# cowbird

<img src="https://upload.wikimedia.org/wikipedia/commons/thumb/e/e6/Brown_headed_cowbird_female_in_JBWR_%2825487%29.jpg/784px-Brown_headed_cowbird_female_in_JBWR_%2825487%29.jpg" width="392" height="300" />

<!--
[![Crates.io](https://img.shields.io/crates/v/cowbird.svg)](https://crates.io/crates/cowbird)
[![Docs.rs](https://docs.rs/cowbird/badge.svg)](https://docs.rs/cowbird)
-->
[![CI](https://github.com/erikwilson/cowbird/workflows/CI/badge.svg)](https://github.com/erikwilson/cowbird/actions)
[![Coverage Status](https://coveralls.io/repos/github/erikwilson/cowbird/badge.svg?branch=main)](https://coveralls.io/github/erikwilson/cowbird?branch=main)

## About

cowbird is a command-line tool for triggering scripted activity to be detected
by EDR agents and produces structured logs for regression testing.

## Installation

### Releases

Download and install the latest pre-built binary from [Releases](https://github.com/erikwilson/cowbird/releases).

### Latest from Source

* Clone this repo `git clone https://github.com/erikwilson/cowbird.git`
* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install --path cowbird`

## Commands

```
cowbird 0.1.0
EDR tool

USAGE:
    cowbird [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -l, --log <LOG>    Log file location [default: cowbird.log]
    -h, --help         Print help information
    -V, --version      Print version information

SUBCOMMANDS:
    start     Starts a new process
    create    Create a file
    modify    Modify a file
    delete    Delete a file
    send      Send data over network
    script    Run commands from YAML input
```
```
cowbird-start 
Starts a new process

USAGE:
    cowbird start <EXEC> [ARGS]...

ARGS:
    <EXEC>       
    <ARGS>...    
```
```
cowbird-create 
Create a file

USAGE:
    cowbird create <FILE>

ARGS:
    <FILE>    File to create
```
```
cowbird-modify 
Modify a file

USAGE:
    cowbird modify <FILE> <DATA> [OFFSET]

ARGS:
    <FILE>      File to modify
    <DATA>      Bytes to write, decode hex if value starts with "0x"
    <OFFSET>    Byte offset for writing data [default: 0]
```
```
cowbird-delete 
Delete a file

USAGE:
    cowbird delete <FILE>

ARGS:
    <FILE>    File to delete
```
```
cowbird-send 
Send data over network

USAGE:
    cowbird send <DEST> [ARGS]

ARGS:
    <DEST>     Destination address:port
    <DATA>     Bytes to write, decode hex if value starts with "0x" [default: ]
    <PROTO>    Network protocol to use [default: udp] [possible values: tcp, udp]
```
```
cowbird-script 
Run commands from YAML input

USAGE:
    cowbird script [FILE]

ARGS:
    <FILE>    File to read [default: -]

OPTIONS:
    -h, --help    Print help information
```

## Scripting Example

See [examples/test.yaml](https://github.com/erikwilson/cowbird/blob/main/examples/test.yaml)
as an example for using YAML document separators for ingesting multiple commands, and how
those command inputs are formatted.

If building from source on a unix system the example can be run like `./examples/test.yaml`,
otherwise the script can be run with a pre-build binary using `cowbird script ./examples/test.yaml`.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
