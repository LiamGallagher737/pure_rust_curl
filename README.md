<div align="center">

# Pure Rust Curl

A basic curl program written in pure Rust, no external libraries. (Linux only)

</div>

## Building

The program can be built with the following command

```sh
rustc main.rs -C panic=abort -C link-arg=-nostartfiles -C debug-assertions=n
```
