use enigo::{Enigo, MouseButton, MouseControllable};

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

    for _ in 0..iterations {
        // wait specified delay before next mouse click
        delays.push(delay_busy(delay));

        // simulate mouse click
        enigo.mouse_click(MouseButton::Left);
    }

    // output delays
    // TODO: write to file?
    println!("{delays:?}");

    Ok(())
}
