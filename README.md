# Finger-tokio (WIP)

## Why? 

The final task in a programming book had me implement a version of `fingerd` in the target language. I thought this might be a good way to familiarize myself with `tokio` & friends.

## About

Included is an implementation for `finger` for `tokio-proto` `Encoder` and `Decoder` traits. Those are re-exported from `main`, though I doubt anyone will find much value in them. `FingerCodec` works similarly to the `LineCodec` struct from the tokio examples; the difference being that in addition to parsing each `\n` block, it looks for the `@` symbol and creates a struct with a `username` and `hostname` value.

In main is a simple `Service` implementation in `tokio-core`. 
