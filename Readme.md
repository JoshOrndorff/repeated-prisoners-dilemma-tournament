# Repeated Prisoners Dilemma Tournament

This repo contains (a first draft of) the code for a classroom activity in which students will code strategies for the repeated prisoners dilemma in Rust, and let them compete against each other.

Currently it has hard-coded payouts and 200 rounds. And it only contains the simplest strategies.

## Run it

```bash
cargo run
```

## Goals

Move payouts and rounds to cli
Allow a dynamic number of strategies read from some file
Compile the strategies to wasm so we can play in any language

