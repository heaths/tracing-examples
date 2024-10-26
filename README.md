# Tracing examples

This crate contains examples of tracing for use in [Azure SDK for Rust](https://github.com/Azure/azure-sdk-for-rust).

## Rotate secret

An example of convenience methods calling generated methods. You can pass "foo", "bar", or "baz" to rotate an existing secret,
or specify any other value to emit an error.

```bash
cargo run --example rotate_secret -- foo secret --level debug
```
