use std::{
    fs::File,
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
    file: Option<R>,
    simulate: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    browser.try_open()?;

    let start = Arc::new(Instant::now());
    let start_clone = start.clone();

    let timings = Arc::new(Mutex::new(Vec::with_capacity(100)));
    let timings_clone = timings.clone();

    let stopped = Arc::new(AtomicBool::new(false));
    let stopped_clone = stopped.clone();

    let callback = move |event: Event| -> Option<Event> {
        if let EventType::KeyPress(Key::Num0) = event.event_type {
            let instant_now = start_clone.elapsed();
            let mut lock = timings_clone.lock().unwrap();
            if lock.len() >= 100 {
                stopped_clone.store(true, Ordering::Relaxed);
            } else {
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
            println!("Error: {:?}", error)
        }
    });

    let handle = if simulate {
        let stopped_clone = stopped.clone();
        thread::sleep(Duration::from_secs(5));
        Some(thread::spawn(move || {
            while !stopped_clone.load(Ordering::Relaxed) {
                send(&EventType::KeyPress(Key::Num0));
                thread::sleep(Duration::from_millis(200));
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
    handle.map(|x| x.join());

    println!("Finished capturing input!");

    // calculate distances between key-events
    let timestamps = std::mem::take(&mut *timings.lock().unwrap());

    assert_eq!(timestamps.len(), 100);

    // output results
    if let Some(file) = file {
        use std::io::Write;
        let mut open_file = File::options()
            .truncate(true)
            .create(true)
            .write(true)
            .open(file.as_ref())
            .unwrap();
        writeln!(open_file, "{timestamps:#?}")?;
    }
    Ok(())
}

fn send(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
}
