use std::{
    fs::File,
    io::Write,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use inputbot::KeybdKey;

use crate::OpenBrowser;

pub fn capture_raw_input<S: AsRef<str>>(
    browser: &OpenBrowser<S>,
    file: Option<S>,
    listen_sec: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    browser.try_open()?;

    let timings = Arc::new(Mutex::new(Vec::with_capacity(256)));
    let timings_clone = timings.clone();

    // Register listener
    KeybdKey::Numrow0Key.blockable_bind(move || {
        let instant_now = Instant::now();
        let mut lock = timings_clone.lock().unwrap();
        lock.push(instant_now);
        inputbot::BlockInput::DontBlock
    });


    // spawn thread to process key-events
    thread::spawn(move || {
        println!("Waiting for input..");
        //timings_clone.lock().unwrap().push(Instant::now());
        inputbot::handle_input_events();
    });

    // wait for user-input
    thread::sleep(Duration::from_secs(listen_sec));
    println!("Finished capturing input!");

    // calculate distances between key-events
    let timestamps = std::mem::take(&mut *timings.lock().unwrap());
    let mut distances = Vec::with_capacity(timestamps.len());

    for (i, timestamp) in timestamps.iter().enumerate() {
        if i == 0 {
            distances.push(0.0);
            continue;
        }

        let distance = timestamp.duration_since(timestamps[i - 1]);
        distances.push(distance.as_secs_f64());
    }

    // output results
    println!("{distances:?}");
    if let Some(file) = file {
        let mut open_file = File::options()
            .truncate(true)
            .create(true)
            .write(true)
            .open(file.as_ref())
            .unwrap();
        writeln!(open_file, "{distances:?}")?;
    }
    Ok(())
}
