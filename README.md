# About

This is a small program used to test some rust concepts. The idea is to have a cli tool able
to query maven dependencies based on inputs similarly to how you can add dependencies
using the rider dependency manager.

This was inspired in the way `fzf` works

## Requirements

- rustc 1.75.0
- cargo 1.75.0


## How to run

It's as simple as executing

```bash
cargo run
```

Use the up and down keys to go over the dependencies and `CTRL+c` to exit.
