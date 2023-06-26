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
    // check if browse should be opened
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
        inputs *= 2;
    }

    let start = Arc::new(Instant::now());

    // create vec for timing information
    let timings = Arc::new(Mutex::new(Vec::with_capacity(100)));
    let timings_clone = timings.clone();

    // create flag if simulation should be stopped
    let stopped = Arc::new(AtomicBool::new(false));
    let stopped_clone = stopped.clone();

    // register callback on "0" Key
    let callback = move |event: Event| -> Option<Event> {
        if let EventType::KeyPress(Key::Num0) = event.event_type {
            // get time since start
            let instant_now = start.elapsed();
            let mut lock = timings_clone.lock().unwrap();

            // check if expected number of inputs was reached
            if lock.len() >= inputs {
                // signal application should exit
                stopped_clone.store(true, Ordering::Relaxed);
            } else {
                // push timestamp to list
                lock.push(Captured::KeyDown(instant_now.as_secs_f64()));
            }
        }

        if !extended {
            return Some(event);
        }
        debug_assert!(extended);

        if let EventType::KeyRelease(Key::Num0) = event.event_type {
            // get time since start
            let instant_now = start.elapsed();
            let mut lock = timings_clone.lock().unwrap();

            // check if expected number of inputs was reached
            if lock.len() >= inputs {
                // signal application should exit
                stopped_clone.store(true, Ordering::Relaxed);
            } else {
                // push timestamp to list
                lock.push(Captured::KeyUp(instant_now.as_secs_f64()));
            }
        }

        Some(event)
    };

    // spawn thread to process key-events
    thread::spawn(move || {
        println!("Waiting for input..");
        // This will block.
        if let Err(error) = grab(callback) {
            println!("Error: {:?}", error);
        }
    });

    // if inputs should be simulated
    let handle = if simulate {
        // get the status flag
        let stopped_clone = stopped.clone();
        // wait for user to be ready
        delay_sleep(wait_before_start);

        // spawn thread to simulate inputs
        Some(thread::spawn(move || {
            // while should not exit
            while !stopped_clone.load(Ordering::Relaxed) {
                // send "0" key down events
                send(&EventType::KeyPress(Key::Num0));

                if extended {
                    // hold key down for 10ms
                    delay_busy(0.01);

                    send(&EventType::KeyRelease(Key::Num0));
                }

                // wait for specified delay (doesn't have to super precise)
                delay_sleep(delay);
            }
        }))
    } else {
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

    //
    assert_eq!(timestamps.len(), inputs);

    // output results

    writeln!(open_file, "timestamp,type")?;

    for timestamp in timestamps {
        match timestamp {
            Captured::KeyDown(t) => writeln!(open_file, "{t},keydown")?,
            Captured::KeyUp(t) => writeln!(open_file, "{t},keyup")?,
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
    KeyDown(f64),
    KeyUp(f64),
}

impl std::fmt::Debug for Captured {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KeyDown(arg0) => write!(f, "{arg0}"),
            Self::KeyUp(arg0) => write!(f, "{arg0}"),
        }
    }
}
