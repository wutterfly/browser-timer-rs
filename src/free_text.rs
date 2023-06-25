use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    time::Duration,
};

use enigo::{Enigo, Key, KeyboardControllable};

use crate::{delay::delay_busy, OpenBrowser, DOWNLOAD_KEY};

pub fn free_text_simulation<S: AsRef<str>, R: AsRef<Path>>(
    brower: &OpenBrowser<S>,
    input_file_desc: R,
    warmup: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // get all input files
    let files = get_input_files(input_file_desc).unwrap();

    println!("Read all input files...");

    // check if default browser should be opened
    brower.try_open().unwrap();

    println!("Waiting for use to be ready (5 secs) ...");
    std::thread::sleep(Duration::from_secs_f64(5.0));
    println!("Start simulating...");

    let len = files.len();

    // read each file to task list
    for (i, reader) in files.into_iter().enumerate() {
        println!("Starting file: {i} / {len} ({:.2})", i as f32 / len as f32);
        let mut keyboard = Enigo::new();

        // read file to vec
        let raw: Vec<String> = reader.lines().into_iter().collect::<Result<_, _>>()?;

        // create task list
        let tasks = create_task_list(&raw).unwrap();

        // warm up
        if warmup {
            for _ in 0..8 {
                keyboard.key_click(Key::Control);
            }
        }

        // execute task list
        for task in tasks {
            match task {
                // wait
                Task::Wait(dur) => {
                    delay_busy(dur);
                }
                // press key
                Task::Key(key) => {
                    keyboard.key_click(key);
                }
            }
        }

        // if task list is finished, trigger download
        keyboard.key_click(DOWNLOAD_KEY);

        std::thread::sleep(Duration::from_secs_f64(2.0));
    }

    Ok(())
}

fn create_task_list(raw: &Vec<String>) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    debug_assert!(raw.len() % 2 == 0, "Always a pair of timestamp and key.");
    let mut out = Vec::with_capacity(raw.len() / 2);

    let timestamps = raw
        .iter()
        .step_by(2)
        .map(|t| t.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut iter = raw.iter();
    _ = iter.next();

    let keys = iter
        .step_by(2)
        .map(|key_raw| {
            let key_u8 = key_raw.parse::<u8>()?;
            Ok::<Key, Box<dyn std::error::Error>>(map_u8_key(key_u8))
        })
        .collect::<Result<Vec<_>, _>>()?;

    assert_eq!(timestamps.len() + keys.len(), raw.len());
    assert_eq!(timestamps.len(), keys.len());

    // calculate wait times after each key
    let mut diffs = Vec::new();
    let mut last = None::<u64>;

    for current in &timestamps {
        let c = *current;

        // First element
        if last.is_none() {
            diffs.push(0);
            last = Some(c);
            continue;
        }

        let last_uwr = last.unwrap();

        // monotonic increment
        if last_uwr < c {
            let dif = current - last_uwr;
            diffs.push(dif);
            last = Some(c);
            continue;
        }

        // reset timestamps
        if last_uwr > c {
            let to_reset = 100_000 - last_uwr;
            let dif = to_reset + (current - 0);
            diffs.push(dif);
            last = Some(c);
            continue;
        }

        // no delay to next key
        if last_uwr == c {
            diffs.push(0);
            continue;
        }

        unreachable!()
    }

    assert_eq!(diffs.len(), keys.len());

    let total_time = diffs.iter().sum::<u64>();
    println!(
        "Task Queue takes: {:.2}min",
        (total_time as f32 / 1000.0) / 60.0
    );

    for (t, k) in diffs.into_iter().zip(keys.into_iter()) {
        if t != 0 {
            out.push(Task::Wait(t as f64 / 1000.0)); // convert to seconds
        }

        out.push(Task::Key(k));
    }

    Ok(out)
}

fn get_input_files<R: AsRef<Path>>(
    input_file_desc: R,
) -> Result<Vec<BufReader<File>>, Box<dyn std::error::Error>> {
    let file = File::open(input_file_desc.as_ref()).unwrap();

    let reader = BufReader::new(file);
    let file_paths: Vec<String> = serde_json::from_reader(reader).unwrap();

    let mut out = Vec::with_capacity(file_paths.len());
    for file_p in &file_paths {
        let f = std::fs::File::open(file_p).unwrap();
        out.push(BufReader::new(f));
    }

    Ok(out)
}

fn map_u8_key(c: u8) -> Key {
    if (65..=90).contains(&c) {
        return Key::Layout((c + 32) as char);
    }

    Key::Layout(c as char)
}

#[derive(Debug)]
enum Task {
    Wait(f64),
    Key(enigo::Key),
}

#[derive(Debug)]
struct NoCharFoundError;

impl std::error::Error for NoCharFoundError {}

impl Display for NoCharFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Given Key Code could not be parsed as char")
    }
}
