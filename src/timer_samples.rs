use enigo::{Enigo, KeyboardControllable}; //MouseButton, MouseControllable};

use crate::{
    delay::{delay_busy, delay_sleep},
    OpenBrowser,
};

pub fn browser_timer_sampler<S: AsRef<str>>(
    browser: &OpenBrowser<S>,
    iterations: usize,
    delay: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("[Browser timestamp samples]");
    // check if default browser should be opend
    browser.try_open()?;
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
        enigo.key_down(KEYS[i % KEYS.len()])
    }

    // output delays
    // TODO: write to file?
    println!("{delays:?}");

    Ok(())
}

const KEYS: [enigo::Key; 28] = [
    enigo::Key::A,
    enigo::Key::B,
    enigo::Key::C,
    enigo::Key::D,
    enigo::Key::E,
    enigo::Key::F,
    enigo::Key::G,
    enigo::Key::H,
    enigo::Key::I,
    enigo::Key::J,
    enigo::Key::K,
    enigo::Key::L,
    enigo::Key::M,
    enigo::Key::N,
    enigo::Key::O,
    enigo::Key::P,
    enigo::Key::Q,
    enigo::Key::R,
    enigo::Key::S,
    enigo::Key::T,
    enigo::Key::U,
    enigo::Key::V,
    enigo::Key::W,
    enigo::Key::X,
    enigo::Key::Y,
    enigo::Key::Z,
    enigo::Key::Delete,
    enigo::Key::Layout('.'),
];
