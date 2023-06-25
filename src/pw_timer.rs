use std::{
    fmt::Debug,
    fs::File,
    io::Write,
    path::Path,
    time::{Duration, Instant},
};

use enigo::{Enigo, Key, KeyboardControllable};

use crate::{
    delay::{delay_busy, delay_sleep},
    OpenBrowser, DOWNLOAD_KEY,
};

#[allow(clippy::cast_precision_loss)]
pub fn pw_simulation<S: AsRef<str>, R: AsRef<Path>>(
    browser: &OpenBrowser<S>,
    in_file: R,
    out_file: R,
    sleep: f64,
    download: usize,
    warmup: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // read all rows (1 row == 1 password)
    let rows = read_data(in_file.as_ref())?;
    // create new keyboard
    let mut keyboard = Enigo::new();

    // open the output file
    let mut output_file = File::options()
        .create(true)
        .write(true)
        .append(false)
        .truncate(true)
        .open(out_file.as_ref())?;

    // initilaize csv file
    writeln!(&mut output_file, "i,should_take,took,subject,session,rep")?;

    // try to open default webbrowser
    browser.try_open()?;
    println!("Waiting for user to be ready (10 sec)...");
    // wait for user to be ready
    delay_sleep(10.0);

    // calculate total time needed
    let mut total = 0.0;
    for row in &rows {
        total += row.should_take().as_secs_f32();
        total += 0.2;
    }

    // how many downloads * waits
    total += (rows.len() / download) as f32 * 0.8;
    // if there is a rest to download
    total += (rows.len() % download).min(1) as f32 * 0.8;

    if warmup {
        total += rows.len() as f32 * 8.0 * 0.010;
        total += rows.len() as f32 * 8.0 * 0.100;
    }

    // calculate time needed as hours
    // seconds to hours
    let total_hours_needed = total / 3600.0;

    println!("Using warmup...");
    println!("Total Time needed: {total_hours_needed:.2}h");

    // save total start time
    let start_time = Instant::now();

    // iterate each row/password
    for (i, row) in rows.iter().enumerate() {
        // create precalculated event list, ordered by time
        let events = row.create_events();

        // warmup phase
        if warmup {
            for _ in 0..8 {
                keyboard.key_down(Key::Layout('q'));
                delay_busy(0.100);
                keyboard.key_up(Key::Layout('q'));
                delay_sleep(0.010);
            }
        }

        // save password start time
        let now = Instant::now();
        // .tie5Roanl
        // iterate each input event for row/password
        for (i, event) in events.iter().enumerate() {
            // execute event
            event.execute(&mut keyboard);
            // if there is next event/not last
            if let Some(next_event) = events.get(i + 1) {
                // calculate how long until the next event
                let sleep = next_event.timestamp - event.timestamp;
                // sleep until next event should be executed
                delay_busy(sleep);
            }
        }
        // check how long it took to simulate password
        let elapsed = now.elapsed().as_secs_f64();
        // get how long it should take to simulate password
        let should_take = row.should_take().as_secs_f64();

        // write timing data to output file
        writeln!(
            &mut output_file,
            "{i},{should_take},{elapsed},{},{},{}",
            row.subject, row.session_index, row.rep
        )?;

        // log progess
        println!(
            "{i} / {} ({:.2}%)  --  {:.2}h / {total_hours_needed:.2}h",
            rows.len(),
            (i as f32 / rows.len() as f32) * 100.0,
            start_time.elapsed().as_secs_f64() / 3600.0
        );

        // every 1000 passwords, trigger download
        if i != 0 && i % download == 0 {
            // signal webapp to download data
            keyboard.key_click(DOWNLOAD_KEY);
            // wait a bit for download to finish
            delay_sleep(0.8);
        }

        // wait a bit before beginning with next password simulation
        delay_sleep(sleep);
    }

    // trigger download for rest of data
    keyboard.key_click(DOWNLOAD_KEY);
    Ok(())
}

/// Reads CSV-File and returns vector of rows.
///
/// # Errors
///
/// Returns an error, if file could not be opened or failed parsing CSV-File into rows.
fn read_data<R: AsRef<Path>>(file: R) -> Result<Vec<Row>, Box<dyn std::error::Error>> {
    // open input file
    let file = std::fs::File::open(file.as_ref())?;

    let mut reader = csv::Reader::from_reader(file);

    // read csv file
    let rows = reader.records().collect::<Result<Vec<_>, _>>()?;

    // map raw csv to `Row` struct
    let rows = rows
        .iter()
        .map(|row| row.deserialize(None))
        .collect::<Result<Vec<Row>, _>>()?;

    Ok(rows)
}

/// Struct describing password input
#[derive(Debug, serde::Deserialize)]
struct Row {
    pub subject: String,
    pub session_index: u64,
    pub rep: u64,
    pub h_period: f64,
    pub __dd_period_t: f64,
    pub ud_period_t: f64,
    pub h_t: f64,
    pub _dd_t_i: f64,
    pub ud_t_i: f64,
    pub h_i: f64,
    pub _dd_i_e: f64,
    pub ud_i_e: f64,
    pub h_e: f64,
    pub _dd_e_five: f64,
    pub ud_e_five: f64,
    pub h_five: f64,
    pub _dd_five_shift_r: f64,
    pub ud_five_shift_r: f64,
    pub h_shift_r: f64,
    pub _dd_shift_r_o: f64,
    pub ud_shift_r_o: f64,
    pub h_o: f64,
    pub _dd_o_a: f64,
    pub ud_o_a: f64,
    pub h_a: f64,
    pub _dd_a_n: f64,
    pub ud_a_n: f64,
    pub h_n: f64,
    pub _dd_n_l: f64,
    pub ud_n_l: f64,
    pub h_l: f64,
    pub _dd_l_return: f64,
    pub ud_l_return: f64,
    pub h_return: f64,
}

impl Row {
    /// Calculate how long it should take to simulate row
    pub fn should_take(&self) -> Duration {
        let mut total = 0.0;
        // .tie5Roanl
        total += self.h_period;
        total += self.h_t;
        total += self.h_i;
        total += self.h_e;
        total += self.h_five;
        total += self.h_shift_r;
        total += self.h_o;
        total += self.h_a;
        total += self.h_n;
        total += self.h_l;
        total += self.h_return;

        total += self.ud_period_t;
        total += self.ud_t_i;
        total += self.ud_i_e;
        total += self.ud_e_five;
        total += self.ud_five_shift_r;
        total += self.ud_shift_r_o;
        total += self.ud_o_a;
        total += self.ud_a_n;
        total += self.ud_n_l;
        total += self.ud_l_return;

        Duration::from_secs_f64(total)
    }

    /// Precalculate input events, ordered by time.
    pub fn create_events(&self) -> Vec<Event> {
        let mut events = Vec::new();

        let mut timestamp = 0.0;

        // .tie5Roanl

        // .
        events.push(Event::new_down(timestamp, Key::Layout('.')));
        timestamp += self.h_period;
        events.push(Event::new_up(timestamp, Key::Layout('.')));

        timestamp += self.ud_period_t;

        // t
        events.push(Event::new_down(timestamp, Key::Layout('t')));
        timestamp += self.h_t;
        events.push(Event::new_up(timestamp, Key::Layout('t')));

        timestamp += self.ud_t_i;

        // i
        events.push(Event::new_down(timestamp, Key::Layout('i')));
        timestamp += self.h_i;
        events.push(Event::new_up(timestamp, Key::Layout('i')));

        timestamp += self.ud_i_e;

        // e
        events.push(Event::new_down(timestamp, Key::Layout('e')));
        timestamp += self.h_e;
        events.push(Event::new_up(timestamp, Key::Layout('e')));

        timestamp += self.ud_e_five;

        // 5
        events.push(Event::new_down(timestamp, Key::Layout('5')));
        timestamp += self.h_five;
        events.push(Event::new_up(timestamp, Key::Layout('5')));

        timestamp += self.ud_five_shift_r;

        // R
        events.push(Event::new_down(timestamp, Key::Shift));
        events.push(Event::new_down(timestamp, Key::Layout('r')));
        events.push(Event::new_up(timestamp, Key::Shift));
        timestamp += self.h_shift_r;
        events.push(Event::new_up(timestamp, Key::Layout('r')));

        timestamp += self.ud_shift_r_o;

        // o
        events.push(Event::new_down(timestamp, Key::Layout('o')));
        timestamp += self.h_o;
        events.push(Event::new_up(timestamp, Key::Layout('o')));

        timestamp += self.ud_o_a;

        // a
        events.push(Event::new_down(timestamp, Key::Layout('a')));
        timestamp += self.h_a;
        events.push(Event::new_up(timestamp, Key::Layout('a')));

        timestamp += self.ud_a_n;

        // n
        events.push(Event::new_down(timestamp, Key::Layout('n')));
        timestamp += self.h_n;
        events.push(Event::new_up(timestamp, Key::Layout('n')));

        timestamp += self.ud_n_l;

        // l
        events.push(Event::new_down(timestamp, Key::Layout('l')));
        timestamp += self.h_l;
        events.push(Event::new_up(timestamp, Key::Layout('l')));

        timestamp += self.ud_l_return;

        // return
        events.push(Event::new_down(timestamp, Key::Return));
        timestamp += self.h_return;
        events.push(Event::new_up(timestamp, Key::Return));

        events.sort();

        events
    }
}

#[derive(Debug)]
pub struct Event {
    timestamp: f64,
    key_event: KeyEvent,
}

impl Event {
    pub fn execute(&self, keybord: &mut Enigo) {
        self.key_event.exec(keybord);
    }

    pub const fn new_down(timestamp: f64, key: Key) -> Self {
        Self {
            timestamp,
            key_event: KeyEvent::KeyDown(key),
        }
    }

    pub const fn new_up(timestamp: f64, key: Key) -> Self {
        Self {
            timestamp,
            key_event: KeyEvent::KeyUp(key),
        }
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.timestamp.partial_cmp(&other.timestamp)
    }
}

impl Eq for Event {}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.timestamp < other.timestamp {
            return std::cmp::Ordering::Less;
        }

        if self.timestamp > other.timestamp {
            return std::cmp::Ordering::Greater;
        }

        std::cmp::Ordering::Equal
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum KeyEvent {
    KeyDown(Key),
    KeyUp(Key),
}

impl KeyEvent {
    pub fn exec(&self, keybord: &mut Enigo) {
        match self {
            Self::KeyDown(k) => keybord.key_down(*k),
            Self::KeyUp(k) => keybord.key_up(*k),
        }
    }
}
