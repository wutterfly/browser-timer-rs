# Browser Timer


This is a tool to generate different timing data for precision timestamps in browsers.

The WebApp can be found at https://wutterfly.com/.

The SourceCode for the WebApp can be found at https://github.com/wutterfly/browser-timer.

Results can be found at https://github.com/wutterfly/browser-timer-results.



# How to run

## With Rust installed

To run this app, you need to have [Rust](https://www.rust-lang.org/tools/install) installed.


Inside the repository directory, run

    cargo r -r -- -h

to get all available commands.

----

Get more information with 

    cargo r -r -- [<command>] -h


## Without Rust installed

### Windows (stable-x86_64-pc-windows-msvc)
Inside the repository directory, run

    browser-timer-rs.exe -h

to get all available commands.

----

Get more information with 

    browser-timer-rs.exe [<command>] -h


## Dataset Sources

### Password Dataset

The Dataset [DSL-StrongPasswordData.csv](./DSL-StrongPasswordData.csv) is part of the research paper [Comparing Anomaly-Detection Algorithms for Keystroke Dynamics](https://www.cs.cmu.edu/~maxion/pubs/KillourhyMaxion09.pdf).

Dataset: https://www.cs.cmu.edu/~keystroke/

### Free-Text Dataset

The Datasets [KEYSTROKE-SAMPLES-31-USERS](./KEYSTROKE-SAMPLES-31-USERS) is part of the research paper [Keystroke Analysis of Free Text](https://dl.acm.org/doi/pdf/10.1145/1085126.1085129).