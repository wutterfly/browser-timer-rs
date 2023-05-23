#![warn(clippy::pedantic)]


mod browser_timer;
mod delay;
mod pw_timer;
mod raw_input;

const EXE_OPTIONS: [& str; 3] = ["timer", "password", "input"];

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
        raw_input::capture_raw_input(
            &OpenBrowser::Open("https://wutterfly.com/browser/browser_test.html"),
            Some("./input_data_rs.json"),
            15,
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
    /// # Results
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
