# Cargo wiki

A WIP cargo extension for generating rust doc in markdown especially for wikis like github wiki.

## Demo
The repository generates it's documentation via `cargo wiki` using `single-file` structure and host it on github wiki 
available [here](https://github.com/AS1100K/cargo-wiki/wiki). Note: This project is still in very early stage of
development and the generated output may contains issue and bug. If you find them, please open [issue here](https://github.com/AS1100K/cargo-wiki/issues)


## Development Setup

```bash
$ cargo build
$ cp target/debug/cargo-wiki .
$ cargo wiki
```

Make sure to setup your path to the current directory
```bash
$ export PATH=/home/<user>/<path/to/cargo-wiki>:$PATH
```