use std::{
    fs::File,
    io::Write,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

use rdev::{grab, simulate, Event, EventType, Key};

use crate::OpenBrowser;

pub fn capture_raw_input<S: AsRef<str>, R: AsRef<Path>>(
    browser: &OpenBrowser<S>,
    file: R,
    simulate: bool,
    wait_before_start: f64,
    delay: f64,
    inputs: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    // check if browse should be opened
    browser.try_open()?;

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
                lock.push(instant_now.as_secs_f64());
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
        thread::sleep(Duration::from_secs_f64(wait_before_start));
        // spawn thread to simulate inputs
        Some(thread::spawn(move || {
            // while should not exit
            while !stopped_clone.load(Ordering::Relaxed) {
                // send "0" key down events
                send(&EventType::KeyPress(Key::Num0));
                // wait for specified delay (doesn't have to super precise)
                thread::sleep(Duration::from_secs_f64(delay));
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
    let mut open_file = File::options()
        .truncate(true)
        .create(true)
        .write(true)
        .open(file.as_ref())
        .unwrap();
    writeln!(open_file, "{timestamps:#?}")?;

    Ok(())
}

fn send(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => (),
        Err(_) => {
            println!("We could not send {event_type:?}");
        }
    }
}
