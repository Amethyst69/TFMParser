# TFMParser
A tool to extract TFM Encryption keys (needed for connecting bots). In order not to allow people to use spambots and other malicious projects, I've removed the packet keys algorithm.

## Instructions
1) The first thing that we have to do is going into the TFMParser directory (use the `cd` command).
2) If you don't have cargo, [download it](https://doc.rust-lang.org/cargo/getting-started/installation.html)
3) Run `cargo run src\main.rs`, after that, you will have to wait for it to build the project.
4) When the project compiled, you can run the parser by running the executable generated => `target\debug\parser.exe`
