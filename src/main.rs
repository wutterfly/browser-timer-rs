#![warn(clippy::pedantic)]

mod browser_timer;
mod delay;
mod pw_timer;

#[cfg(not(target_os = "macos"))]
mod raw_input;

#[cfg(target_os = "macos")]
mod raw_input_macos;

const EXE_OPTIONS: [&str; 3] = ["timer", "password", "input"];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 || !EXE_OPTIONS.contains(&args[1].as_str().to_lowercase().as_str()) {
        println!("Specifiy what test to run:");
        for option in EXE_OPTIONS {
            println!("- {option}");
        }
        return Ok(());
    }

    let test = args[1].as_str().to_lowercase();

    if test == "timer" {
        browser_timer::browser_timer_sampler(
            &OpenBrowser::Open(
                "https://wutterfly.com/timer-precision/extern_input/same_origin.html",
            ),
            1010,
            0.1,
        )?;
    }

    if test == "password" {
        pw_timer::pw_simulation(
            &OpenBrowser::Open("https://wutterfly.com/cont-auth/extern_input/same_origin.html"),
            "./DSL-StrongPasswordData.csv",
        )?;
    }

    if test == "input" {
        let bool = &args[2];
        let simulate = match bool.as_str() {
            "true" => true,

            "false" => false,

            _ => panic!(), // Or whatever appropriate default value or error.
        };

        #[cfg(not(target_os = "macos"))]
        raw_input::capture_raw_input(
            &OpenBrowser::Open("https://wutterfly.com/browser/browser_test.html"),
            Some("./input_data_rs.json"),
            simulate,
        )?;

        #[cfg(target_os = "macos")]
        raw_input_macos::capture_raw_input(
            &OpenBrowser::Open("https://wutterfly.com/browser/browser_test.html"),
            Some("./input_data_rs.json"),
            simulate,
        )?;
    }

    Ok(())
}

#[derive(Debug)]
pub enum OpenBrowser<S: AsRef<str>> {
    False,
    Open(S),
}

impl<S: AsRef<str>> OpenBrowser<S> {
    /// Trys to open a URL inside the systems default browser.
    ///
    /// # Errors
    /// Returns an error if website failed to open.
    pub fn try_open(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::False => Ok(()),
            Self::Open(s) => {
                webbrowser::open(s.as_ref())?;
                Ok(())
            }
        }
    }
}
