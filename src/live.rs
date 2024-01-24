use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    time::{Duration, Instant},
};

use enigo::{Enigo, Key, KeyboardControllable};

use crate::{delay::delay_busy, DOWNLOAD_KEY};

#[allow(clippy::cast_precision_loss)]
pub fn live_simulation<R: AsRef<Path>>(
    input_file_desc: R,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("[Live Simulation]");

    let file = File::open(input_file_desc.as_ref())?;

    println!("Read all input files...");

    println!("Waiting for use to be ready (5 secs) ...");
    std::thread::sleep(Duration::from_secs_f64(5.0));
    println!("Start simulating...");

    let mut keyboard = Enigo::new();

    //read each file to task list
    let reader = BufReader::new(file);

    // read file to vec
    let raw: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    // create task list
    let tasks = create_task_list(&raw).unwrap();

    // execute task list
    let mut last = Option::<Instant>::None;
    let mut last_waited = 0.0;
    for task in tasks.into_iter() {
        match task {
            // wait
            Task::Wait(dur) => {
                delay_busy(dur);

                last_waited = dur;
            }
            // press key
            Task::Key(key) => {
                #[cfg(target_os = "macos")]
                {
                    keyboard.key_down(Key::Layout('a'));
                    keyboard.key_up(Key::Layout('a'));
                }

                #[cfg(not(target_os = "macos"))]
                {
                    keyboard.key_down(key);
                    keyboard.key_up(key);
                }

                let last_elapsed = if let Some(l) = last {
                    let out = l.elapsed().as_secs_f64();
                    last = Some(Instant::now());
                    out
                } else {
                    last = Some(Instant::now());
                    0.0
                };

                debug_assert!((last_elapsed - last_waited).abs() <= 0.001);
            }
        }
    }

    // if task list is finished, trigger download
    keyboard.key_click(DOWNLOAD_KEY);

    std::thread::sleep(Duration::from_secs_f64(4.0));

    Ok(())
}

#[allow(clippy::cast_precision_loss)]
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
            let dif = to_reset + current;
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

#[inline(always)]
fn map_u8_key(c: u8) -> Key {
    // uppercase letters to lowercase letters
    if c >= 65 && c <= 90 {
        return Key::Layout((c + 32) as char);
    }

    // lowercase letters
    if c >= 97 && c <= 122 {
        return Key::Layout(c as char);
    }

    // numbers
    if c >= 48 && c <= 57 {
        return Key::Layout(c as char);
    }

    return match c {
        8 => Key::Backspace,
        13 => Key::Layout(c as char),
        32 => Key::Space,
        44 => Key::Layout(','),
        45 => Key::Layout('-'),
        46 => Key::Layout('.'),
        _ => Key::Layout('#'),
    };
}

#[derive(Debug)]
enum Task {
    Wait(f64),
    Key(Key),
}

#[derive(Debug)]
struct NoCharFoundError;

impl std::error::Error for NoCharFoundError {}

impl Display for NoCharFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Given Key Code could not be parsed as char")
    }
}
