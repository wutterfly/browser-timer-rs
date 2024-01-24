use enigo::{Enigo, Key, KeyboardControllable}; //MouseButton, MouseControllable};

use crate::delay::{delay_busy, delay_sleep};

pub fn browser_timer_sampler(iterations: usize, delay: f64) {
    println!("[Browser timestamp samples]");

    // create new input
    let mut enigo = Enigo::new();

    // wait for user to be ready
    delay_sleep(10.0);

    println!("Starting...");

    // pre-allocate vec, storing how long was really waited
    let mut delays = Vec::with_capacity(iterations);

    for i in 0..iterations {
        // wait specified delay before next mouse click
        delays.push(delay_busy(delay));

        // simulate mouse click
        // enigo.mouse_click(MouseButton::Left);

        // simulate input
        #[cfg(target_os = "macos")]
        enigo.key_down(Key::Layout('a'));

        #[cfg(not(target_os = "macos"))]
        enigo.key_down(KEYS[i % KEYS.len()]);
    }

    // output delays
    // TODO: write to file?
    println!("{delays:?}");
}

const KEYS: [Key; 28] = [
    Key::Layout('a'),
    Key::Layout('b'),
    Key::Layout('c'),
    Key::Layout('d'),
    Key::Layout('e'),
    Key::Layout('f'),
    Key::Layout('g'),
    Key::Layout('h'),
    Key::Layout('i'),
    Key::Layout('j'),
    Key::Layout('k'),
    Key::Layout('l'),
    Key::Layout('m'),
    Key::Layout('n'),
    Key::Layout('o'),
    Key::Layout('p'),
    Key::Layout('q'),
    Key::Layout('r'),
    Key::Layout('s'),
    Key::Layout('t'),
    Key::Layout('u'),
    Key::Layout('v'),
    Key::Layout('w'),
    Key::Layout('x'),
    Key::Layout('y'),
    Key::Layout('z'),
    Key::Delete,
    Key::Layout('.'),
];
