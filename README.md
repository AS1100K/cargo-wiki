# Cargo wiki

A WIP cargo extension for generating rust doc in markdown especially for wikis like github wiki.

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