## Browser Timer


This is a tool to generate different timing data for precision timestamps in browsers.

The WebApp can be found at https://wutterfly.com/.

The SourceCode for the WebApp can be found at https://github.com/wutterfly/browser-timer.



## How to run

### With Rust installed

To run this app, you need to have [Rust](https://www.rust-lang.org/tools/install) installed.


Inside the repository directory, run

    cargo r -r -- -h

to get all available commands.


Get more information with 

    cargo r -r -- [<command>] -h


### Without Rust installed

There are no binarys provided as of now. It is nessary to have Rust installed.


## Dataset Sources

### Password Dataset

The Dataset [DSL-StrongPasswordData.csv](./DSL-StrongPasswordData.csv) is part of the research paper [Comparing Anomaly-Detection Algorithms for Keystroke Dynamics](https://www.cs.cmu.edu/~maxion/pubs/KillourhyMaxion09.pdf).

Dataset: https://www.cs.cmu.edu/~keystroke/

### Free-Text Dataset

The Datasets [KEYSTROKE-SAMPLES-31-USERS](./KEYSTROKE-SAMPLES-31-USERS) is part of the research paper [A Replication of Two Free Text Keystroke Dynamics Experiments](https://cs.emis.de/LNI/Proceedings/Proceedings260/147.pdf).