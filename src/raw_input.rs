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

use enigo::{Enigo, Key, KeyboardControllable};
use inputbot::KeybdKey;

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

    // Register listener
    KeybdKey::Numrow0Key.blockable_bind(move || {
        let instant_now = start_clone.elapsed();
        let mut lock = timings_clone.lock().unwrap();
        if lock.len() >= 100 {
            stopped_clone.store(true, Ordering::Relaxed);
        } else {
            lock.push(instant_now.as_secs_f64());
        }

        inputbot::BlockInput::DontBlock
    });

    // spawn thread to process key-events
    thread::spawn(move || {
        println!("Waiting for input..");
        inputbot::handle_input_events();
    });

    let handle = if simulate {
        let stopped_clone = stopped.clone();
        let mut keybord = Enigo::new();
        thread::sleep(Duration::from_secs(5));
        Some(thread::spawn(move || {
            while !stopped_clone.load(Ordering::Relaxed) {
                keybord.key_down(Key::Num0);
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
