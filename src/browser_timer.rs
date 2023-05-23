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
    let mut enigo = Enigo::new();
    browser.try_open()?;
    delay_sleep(10.0);

    let mut delays = Vec::with_capacity(iterations);

    for _ in 0..iterations {
        delays.push(delay_busy(delay));
        enigo.mouse_click(MouseButton::Left);
    }

    println!("{delays:?}");

    Ok(())
}
