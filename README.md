# Codee

[![Crates.io](https://img.shields.io/crates/v/codee.svg)](https://crates.io/crates/codee)
[![Docs](https://docs.rs/codee/badge.svg)](https://docs.rs/codee/)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/synphonyte/codee#license)
[![Build Status](https://github.com/synphonyte/codee/actions/workflows/ci.yml/badge.svg)](https://github.com/synphonyte/codee/actions/workflows/ci.yml)

Easy and flexible way of encoding and decoding data into either strings or bytes.

This crate provides generic traits for `Encoder`s and `Decoder`s as well as several
implementations for commonly used (de)serializer crates.
    
This makes it easily possible to abstract away the serialization and deserialization independent
of the concrete crate used. You can write a function like this:

```rust
use anyhow::Error;
use codee::{Decoder, Encoder};

fn store_value<T, Codec>(value: T) -> Result<(), Error>
where
    Codec: Encoder<T, Encoded = String> + Decoder<T, Encoded = str>,
{
    let encoded = Codec::encode(&value)?;
    let decoded = Codec::decode(&encoded)?;

    Ok(())
}

// Then we can use it like this:

use codee::string::{JsonSerdeCodec, FromToStringCodec};

#[derive(serde::Serialize, serde::Deserialize)]
struct MyStruct {
    field: usize,
}

store_value::<i32, FromToStringCodec>(42);
store_value::<MyStruct, JsonSerdeCodec>(MyStruct { field: 42 });
```
