# Tracing examples

This crate contains examples of tracing for use in [Azure SDK for Rust](https://github.com/Azure/azure-sdk-for-rust).

## Rotate secret

An example of convenience methods calling generated methods. You can pass "foo", "bar", or "baz" to rotate an existing secret,
or specify any other value to emit an error.

```bash
cargo run --example rotate_secret -- foo secret --level debug
```

## Links

Some interesting documentation to consider:

* [`tracing`](https://docs.rs/tracing/latest/tracing/index.html)
* [`tracing_subscriber`](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/index.html)
* [`tracing_opentelemetry`](https://docs.rs/tracing-opentelemetry/latest/tracing_opentelemetry/index.html)
* [`reqwest_tracing`](https://docs.rs/reqwest-tracing/latest/reqwest_tracing/index.html)
