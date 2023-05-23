use std::{
    fmt::Debug,
    fs::File,
    io::Write,
    time::{Duration, Instant},
};

use enigo::{Enigo, Key, KeyboardControllable};

use crate::{
    delay::{delay_busy, delay_sleep},
    OpenBrowser,
};

pub fn pw_simulation<S: AsRef<str>>(
    browser: OpenBrowser<S>,
    file: S,
) -> Result<(), Box<dyn std::error::Error>> {
    let rows = read_data(file)?;
    let mut keybord = Enigo::new();

    let mut output_file = File::options()
        .create(true)
        .write(true)
        .append(true)
        .open("./password_data_rs.csv")?;

    writeln!(&mut output_file, "i,should_take,took,subject,session,rep")?;

    browser.try_open()?;
    delay_sleep(5.0);

    let mut total = 0.0;
    for row in &rows {
        total += row.should_take().as_secs_f64();
        total += 0.2;
    }

    println!(
        "Total Time needed: {:?}h",
        Duration::from_secs_f64(total).as_secs() / 3600
    );

    for (i, row) in rows.iter().enumerate() {
        let events = row.create_events();
        let now = Instant::now();

        // .tie5Roanl
        for (i, event) in events.iter().enumerate() {
            event.execute(&mut keybord);
            if let Some(next_event) = events.get(i + 1) {
                let sleep = next_event.timestamp - event.timestamp;
                delay_busy(sleep);
            }
        }

        let elapsed = now.elapsed().as_secs_f64();
        let should_take = row.should_take().as_secs_f64();

        writeln!(
            &mut output_file,
            "{i},{should_take},{elapsed},{},{},{}",
            row.subject, row.session_index, row.rep
        )?;

        println!("{i} / {}", rows.len());

        delay_sleep(0.2);
    }

    keybord.key_down(Key::F2);
    keybord.key_up(Key::F2);
    Ok(())
}

/// Reads CSV-File and returns vector of rows.
///
/// //  Results
/// Returns an error, if file could not be opened or failed parsing CSV-File into rows.
fn read_data<S: AsRef<str>>(file: S) -> Result<Vec<Row>, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(file.as_ref())?;

    let mut reader = csv::Reader::from_reader(file);

    let rows = reader.records().collect::<Result<Vec<_>, _>>()?;

    let rows = rows
        .iter()
        .map(|row| row.deserialize(None))
        .collect::<Result<Vec<Row>, _>>()?;

    Ok(rows)
}

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

    pub fn create_events(&self) -> Vec<Event> {
        let mut events = Vec::new();

        let mut timestamp = 0.0;

        // .tie5Roanl

        // .
        events.push(Event {
            timestamp,
            key_event: KeyEvent::KeyDown(Key::Layout('.')),
        });
        timestamp += self.h_period;
        events.push(Event {
            timestamp,
            key_event: KeyEvent::KeyUp(Key::Layout('.')),
        });

        timestamp += self.ud_period_t;

        // t
        events.push(Event {
            timestamp,
            key_event: KeyEvent::KeyDown(Key::Layout('t')),
        });
        timestamp += self.h_t;
        events.push(Event {
            timestamp,
            key_event: KeyEvent::KeyUp(Key::Layout('t')),
        });

        timestamp += self.ud_t_i;

        // i
        events.push(Event {
            timestamp,
            key_event: KeyEvent::KeyDown(Key::Layout('i')),
        });
        timestamp += self.h_i;
        events.push(Event {
            timestamp,
            key_event: KeyEvent::KeyUp(Key::Layout('i')),
        });

        timestamp += self.ud_i_e;

        // e
        events.push(Event {
            timestamp,
            key_event: KeyEvent::KeyDown(Key::Layout('e')),
        });
        timestamp += self.h_e;
        events.push(Event {
            timestamp,
            key_event: KeyEvent::KeyUp(Key::Layout('e')),
        });

        timestamp += self.ud_e_five;

        // 5
        events.push(Event {
            timestamp,
            key_event: KeyEvent::KeyDown(Key::Layout('5')),
        });
        timestamp += self.h_five;
        events.push(Event {
            timestamp,
            key_event: KeyEvent::KeyUp(Key::Layout('5')),
        });

        timestamp += self.ud_five_shift_r;

        // R
        events.push(Event {
            timestamp,
            key_event: KeyEvent::KeyDown(Key::Shift),
        });
        events.push(Event {
            timestamp,
            key_event: KeyEvent::KeyDown(Key::Layout('r')),
        });
        events.push(Event {
            timestamp,
            key_event: KeyEvent::KeyUp(Key::Shift),
        });
        timestamp += self.h_shift_r;
        events.push(Event {
            timestamp,
            key_event: KeyEvent::KeyUp(Key::Layout('r')),
        });

        timestamp += self.ud_shift_r_o;

        // o
        events.push(Event {
            timestamp,
            key_event: KeyEvent::KeyDown(Key::Layout('o')),
        });
        timestamp += self.h_o;
        events.push(Event {
            timestamp,
            key_event: KeyEvent::KeyUp(Key::Layout('o')),
        });

        timestamp += self.ud_o_a;

        // a
        events.push(Event {
            timestamp,
            key_event: KeyEvent::KeyDown(Key::Layout('a')),
        });
        timestamp += self.h_a;
        events.push(Event {
            timestamp,
            key_event: KeyEvent::KeyUp(Key::Layout('a')),
        });

        timestamp += self.ud_a_n;

        // n
        events.push(Event {
            timestamp,
            key_event: KeyEvent::KeyDown(Key::Layout('n')),
        });
        timestamp += self.h_n;
        events.push(Event {
            timestamp,
            key_event: KeyEvent::KeyUp(Key::Layout('n')),
        });

        timestamp += self.ud_n_l;

        // l
        events.push(Event {
            timestamp,
            key_event: KeyEvent::KeyDown(Key::Layout('l')),
        });
        timestamp += self.h_l;
        events.push(Event {
            timestamp,
            key_event: KeyEvent::KeyUp(Key::Layout('l')),
        });

        timestamp += self.ud_l_return;

        // return
        events.push(Event {
            timestamp,
            key_event: KeyEvent::KeyDown(Key::Return),
        });
        timestamp += self.h_return;
        events.push(Event {
            timestamp,
            key_event: KeyEvent::KeyUp(Key::Return),
        });

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
