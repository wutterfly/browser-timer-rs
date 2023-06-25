#![warn(clippy::pedantic)]

mod delay;
mod free_text;
mod pw_timer;
mod raw_input;
mod timer_samples;

use clap::{Parser, Subcommand};
use enigo::Key;

pub const DOWNLOAD_KEY: Key = Key::Escape; // no input character

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse command line args
    let args = Args::parse();

    match args.command {
        // Capture raw use input timestamps
        Commands::Input {
            simulate,
            output,
            wait,
            delay,
            inputs,
            extended,
        } => {
            // check if browser should be opened
            let browser = if args.browser {
                OpenBrowser::Open("https://wutterfly.com/browser/browser_test.html")
            } else {
                OpenBrowser::False
            };
            raw_input::capture_raw_input(
                &browser,
                output.as_str(),
                simulate,
                wait,
                delay,
                inputs,
                extended,
            )?;
        }

        // Simulate typing passwords
        Commands::Password {
            input,
            output,
            sleep,
            download,
            warmup,
        } => {
            // check if browser should be opened
            let browser = if args.browser {
                OpenBrowser::Open("https://wutterfly.com/cont-auth/same_origin.html")
            } else {
                OpenBrowser::False
            };
            pw_timer::pw_simulation(
                &browser,
                input.as_str(),
                output.as_str(),
                sleep,
                download,
                warmup,
            )?;
        }

        // Take timestamp probes every delay(sec)
        Commands::Timer { iterations, delay } => {
            // check if browser should be opened
            let browser = if args.browser {
                OpenBrowser::Open("https://wutterfly.com/timer-precision/same_origin.html")
            } else {
                OpenBrowser::False
            };
            timer_samples::browser_timer_sampler(&browser, iterations, delay)?;
        }

        // Simulate typing random/free text
        Commands::FreeText { input_desc, warmup } => {
            // check if browser should be opened
            let browser = if args.browser {
                OpenBrowser::Open("https://wutterfly.com/free-text/same_origin.html")
            } else {
                OpenBrowser::False
            };

            free_text::free_text_simulation(&browser, input_desc, warmup)?;
        }
    };

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

/// Program for testing different timestamp behavior in browsers.
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    /// If default browser with default webpage should be opened
    #[clap(about)]
    #[arg(short, long)]
    browser: bool,
}

#[derive(Debug, clap::ValueEnum, Clone)]
pub enum Test {
    Input,
    Password,
    Timer,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    /// Simulates user input events (mouse clicks) on a webpage
    Timer {
        /// Specifies how many input events should be
        #[clap(about)]
        #[arg(short, long, default_value_t = 1010)]
        iterations: usize,

        /// Specifies how many seconds to wait between each input event
        #[clap(about)]
        #[arg(short, long, default_value_t = 0.1)]
        delay: f64,
    },
    /// Simulates password input according to given dateset (input)
    Password {
        /// Specifies CSV file to read input from
        #[clap(about)]
        #[arg(short, long, default_value = "./DSL-StrongPasswordData.csv")]
        input: String,

        /// Specifies CSV file to write output to
        #[clap(about)]
        #[arg(short, long, default_value = "./password_data_rs.csv")]
        output: String,

        /// Specifies how long the programm should sleep between each complete password input
        #[clap(about)]
        #[arg(short, long, default_value_t = 0.2)]
        sleep: f64,

        /// Specifies after how many password inputs a download should be triggered
        #[clap(about)]
        #[arg(short, long, default_value_t = 1000)]
        download: usize,

        /// Specifies if some dummy input events should be triggered before each password input
        #[clap(about)]
        #[arg(short, long, default_value_t = false)]
        warmup: bool,
    },
    /// Captures user input (listening on Key `0`) and writes timestamps to output file.
    Input {
        /// If input should be simulated
        #[clap(about)]
        #[arg(short, long, default_value_t = false)]
        simulate: bool,

        /// Specifies how many sec to wait before starting simulation (if enabled)
        #[clap(about)]
        #[arg(short, long, default_value_t = 5.0)]
        wait: f64,

        /// Specifies how many seconds to wait between each input event simulatio (if enabled)
        #[clap(about)]
        #[arg(short, long, default_value_t = 0.2)]
        delay: f64,

        /// Specifies how many inputs are expected and the application should exit
        #[clap(about)]
        #[arg(short, long, default_value_t = 100)]
        inputs: usize,

        /// Specifies CSV file to write output to
        #[clap(about)]
        #[arg(short, long, default_value = "./input_data_rs.csv")]
        output: String,

        /// Specifies if keyup events should also be simulated and captured.
        #[clap(about)]
        #[arg(short, long, default_value_t = false)]
        extended: bool,
    },

    /// Simulates free-text input, based on the given input description ('./KEYSTROKE-SAMPLES-31-USERS/split_')
    FreeText {
        /// Specifies JSON file listing all individual input files to read
        #[clap(about)]
        #[arg(short, long)]
        input_desc: String,

        /// Specifies if some dummy input events should be triggered before each password input
        #[clap(about)]
        #[arg(short, long, default_value_t = false)]
        warmup: bool,
    },
}
