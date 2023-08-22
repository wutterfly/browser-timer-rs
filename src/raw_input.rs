use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

use enigo::{Enigo, KeyboardControllable};
use rdev::{grab, simulate, Event, EventType, Key};

use crate::{
    delay::{delay_busy, delay_sleep},
    OpenBrowser,
};

pub fn capture_raw_input<S: AsRef<str>, R: AsRef<Path>>(
    browser: &OpenBrowser<S>,
    file: R,
    simulate: bool,
    wait_before_start: f64,
    delay: f64,
    mut inputs: usize,
    extended: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("[Browser - OS differnce]");
    // check if browser should be opened
    browser.try_open()?;

    // output results
    let mut open_file = BufWriter::new(
        File::options()
            .truncate(true)
            .create(true)
            .write(true)
            .open(file.as_ref())?,
    );

    if extended {
        println!("Triggering Key Ups and Downs...");
        inputs *= 2;
    }

    let mut enigo = Enigo::new();

    let start = Arc::new(Instant::now());

    // create vec for timing information
    let timings = Arc::new(Mutex::new(Vec::with_capacity(inputs)));
    let timings_clone = timings.clone();

    // create flag if simulation should be stopped
    let stopped = Arc::new(AtomicBool::new(false));
    let stopped_clone = stopped.clone();

    // register callback on "0" Key
    let callback = move |event: Event| -> Option<Event> {
        if let EventType::KeyPress(k) = event.event_type {
            // get time since start
            let instant_now = start.elapsed();
            let mut lock = timings_clone.lock().unwrap();

            // check if expected number of inputs was reached
            if lock.len() >= inputs {
                // signal application should exit
                stopped_clone.store(true, Ordering::Relaxed);
            } else {
                // push timestamp to list
                lock.push(Captured::KeyDown(instant_now.as_secs_f64(), k));
            }
        }

        if !extended {
            return Some(event);
        }

        if let EventType::KeyRelease(k) = event.event_type {
            // get time since start
            let instant_now = start.elapsed();
            let mut lock = timings_clone.lock().unwrap();

            // check if expected number of inputs was reached
            if lock.len() >= inputs {
                // signal application should exit
                stopped_clone.store(true, Ordering::Relaxed);
            } else {
                // push timestamp to list
                lock.push(Captured::KeyUp(instant_now.as_secs_f64(), k));
            }
        }

        Some(event)
    };

    // spawn thread to process key-events
    thread::spawn(move || {
        // This will block.
        if let Err(error) = grab(callback) {
            println!("Error: {:?}", error);
        }
    });

    delay_sleep(1.0);

    // if inputs should be simulated
    let handle = if simulate {
        println!("Triggering {inputs} inputs after {wait_before_start} sec...");
        // get the status flag
        let stopped_clone = stopped.clone();
        // wait for user to be ready
        delay_sleep(wait_before_start);
        println!("Starting...");

        // spawn thread to simulate inputs
        Some(thread::spawn(move || {
            let mut rng = rand::thread_rng();
            // while should not exit
            while !stopped_clone.load(Ordering::Relaxed) {
                // send "0" key down events
                //send(&EventType::KeyPress(Key::Num0));
                enigo.key_down(enigo::Key::Layout('0'));

                if extended {
                    // hold key down between 90ms - 200ms
                    let rand_num: f64 = rand::Rng::gen_range(&mut rng, 0.090..0.200);
                    delay_busy(rand_num);

                    //send(&EventType::KeyRelease(Key::Num0));
                    enigo.key_up(enigo::Key::Layout('0'));
                }

                // wait for specified delay (doesn't have to super precise)
                delay_sleep(delay);
            }
        }))
    } else {
        println!("Press '0' to trigger input events! {inputs} inputs needed...");
        None
    };

    // wait for user-input
    while !stopped.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_millis(500));
    }

    // stop simulation thread
    stopped.store(true, Ordering::Relaxed);
    // wait for simulation thread to exit
    handle.map(std::thread::JoinHandle::join);

    println!("Finished capturing input!");

    // take list from mutex
    let timestamps = std::mem::take(&mut *timings.lock().unwrap());

    // check for correct length
    assert_eq!(timestamps.len(), inputs);

    // output results
    writeln!(open_file, "timestamp,key,type")?;

    for timestamp in timestamps {
        match timestamp {
            Captured::KeyDown(t, k) => writeln!(open_file, "{t},{k:?},keydown")?,
            Captured::KeyUp(t, k) => writeln!(open_file, "{t},{k:?},keyup")?,
        }
    }

    Ok(())
}

#[inline(always)]
fn send(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => (),
        Err(_) => {
            println!("We could not send {event_type:?}");
        }
    }
}

#[derive(Clone, Copy)]
enum Captured {
    KeyDown(f64, Key),
    KeyUp(f64, Key),
}

impl std::fmt::Debug for Captured {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KeyDown(arg0, arg1) => write!(f, "{arg0}, {arg1:?}, keydown "),
            Self::KeyUp(arg0, arg1) => write!(f, "{arg0}, {arg1:?}, keyup"),
        }
    }
}
