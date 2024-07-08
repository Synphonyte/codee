//! Easy and flexible way of encoding and decoding data into either strings or bytes.
//!
//! This crate provides generic traits for [`Encoder`]s and [`Decoder`]s as well as several
//! implementations for commonly used (de)serializer crates.
//!
//! This makes it easily possible to abstract away the serialization and deserialization independent
//! of the concrete crate used. You can write a function like this:
//!
//! ```
//! use codee::{CodecError, Decoder, Encoder};
//!
//! fn store_value<T, Codec>(value: T) -> Result<(), CodecError<<Codec as Encoder<T>>::Error, <Codec as Decoder<T>>::Error>>
//! where
//!     Codec: Encoder<T, Encoded = String> + Decoder<T, Encoded = str>,
//! {
//!     let encoded = Codec::encode(&value).map_err(CodecError::Encode)?;
//!     let decoded = Codec::decode(&encoded).map_err(CodecError::Decode)?;
//!
//!     Ok(())
//! }
//!
//! // Then we can use it like this:
//!
//! use codee::string::{JsonSerdeCodec, FromToStringCodec};
//!
//! #[derive(serde::Serialize, serde::Deserialize)]
//! struct MyStruct {
//!     field: usize,
//! }
//!
//! store_value::<i32, FromToStringCodec>(42);
//! store_value::<MyStruct, JsonSerdeCodec>(MyStruct { field: 42 });
//! ```
//!
//! ## Available Codecs
//!
//! There are two types of codecs: One that encodes as binary data (`Vec[u8]`) in the module [`binary`] and another type that encodes as
//! strings (`String`) in the module [`string`]. There is also an adapter
//! [`Base64`](https://github.com/Synphonyte/leptos-use/blob/main/src/utils/codecs/string/base64.rs) that can be used to
//! wrap a binary codec and make it a string codec by representing the binary data as a base64 string.
//!
//! > Please note that many of the codecs need a feature flag to be enabled. Check the docs of the respective codec to be sure.
//!
//! ### String Codecs
//!
//! Please have look at the module [`string`](crate::string).
//!
//! #### Adapters
//!
//! - [`string::Base64`] —
//!   Wraps a binary codec and make it a string codec by representing the binary data as a base64 string.
//! - [`string::OptionCodec`] —
//!   Wraps a string codec that encodes `T` to create a codec that encodes `Option<T>`.
//!
//! ### Binary Codecs
//!
//! Please have look at the module [`binary`](crate::binary).
//!
//! ## Custom Codecs
//!
//! If you don't find a suitable codecs for your needs, you can implement your own; it's straightforward!
//! If you want to create a string codec, you can look at [`JsonSerdeCodec`] as a starting point.
//! In case it's a binary codec, have a look at [`BincodeSerdeCodec`].
//!
//! ## Versioning
//!
//! Versioning is the process of handling long-term data that can outlive our code.
//!
//! For example, we could have a settings struct whose members change over time. We might eventually
//! add timezone support, and we might then remove support for a thousands separator for numbers.
//! Each change results in a new possible version of the stored data. If we stored these settings
//! in browser storage, we would need to handle all possible versions of the data format that can
//! occur. If we don't offer versioning, then all settings could revert to the default every time we
//! encounter an old format.
//!
//! How best to handle versioning depends on the codec involved:
//!
//! - The `FromToStringCodec` can avoid versioning entirely by keeping
//!   to primitive types. In our example above, we could have decomposed the settings struct into
//!   separate timezone and number separator fields. These would be encoded as strings and stored as
//!   two separate key-value fields in the browser rather than a single field. If a field is missing,
//!   then the value intentionally would fall back to the default without interfering with the other
//!   field.
//!
//! - The `ProstCodec` uses [Protocol buffers](https://protobuf.dev/overview/)
//!   designed to solve the problem of long-term storage. It provides semantics for versioning that
//!   are not present in JSON or other formats.
//!
//! - The codecs that use serde under the hood can rely on serde or by
//!   providing their own manual version handling. See the next sections for more details.
//!
//! ### Rely on `serde`
//!
//! A simple way to avoid complex versioning is to rely on serde's [field attributes](https://serde.rs/field-attrs.html)
//! such as [`serde(default)`](https://serde.rs/field-attrs.html#default)
//! and [`serde(rename = "...")`](https://serde.rs/field-attrs.html#rename).
//!
//! ### Manual Version Handling
//!
//! We look at the example of the `JsonSerdeCodec` in this section.
//!
//! To implement version handling, we parse the JSON generically then transform the
//! resulting `serde_json::Value` before decoding it into our struct again.
//!
//! Let's look at an example.
//!
//! ```
//! use serde::{Deserialize, Serialize};
//! use serde_json::json;
//! use codee::{Encoder, Decoder};
//!
//! #[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
//! pub struct MyState {
//!     pub hello: String,
//!     // This field was added in a later version
//!     pub greeting: String,
//! }
//!
//! pub struct MyStateCodec;
//!
//! impl Encoder<MyState> for MyStateCodec {
//!     type Error = serde_json::Error;
//!     type Encoded = String;
//!
//!     fn encode(val: &MyState) -> Result<Self::Encoded, Self::Error> {
//!         serde_json::to_string(val)
//!     }
//! }
//!
//! impl Decoder<MyState> for MyStateCodec {
//!     type Error = serde_json::Error;
//!     type Encoded = str;
//!
//!     fn decode(stored_value: &Self::Encoded) -> Result<MyState, Self::Error> {
//!         let mut val: serde_json::Value = serde_json::from_str(stored_value)?;
//!         // add "greeting": "Hello" to the object if it's missing
//!         if let Some(obj) = val.as_object_mut() {
//!             if !obj.contains_key("greeting") {
//!                obj.insert("greeting".to_string(), json!("Hello"));
//!             }
//!             serde_json::from_value(val)
//!         } else {
//!             Ok(MyState::default())
//!         }
//!     }
//! }
//!
//! // Then use it just like any other codec.
//! ```
//!
//! ## Hybrid Codecs
//!
//! In case you want to write code that can be used with both, binary and string codecs, there are the
//! [`HybridDecoder`], [`HybridEncoder`] and [`IsBinary`] traits that are implemented automatically
//! for all the codecs.
//!
//! To see them in action, you can have a look at [`leptos_use::use_websocket`](https://github.com/Synphonyte/leptos-use/blob/main/src/use_websocket.rs).

pub mod binary;
mod error;
mod hybrid;
#[cfg(feature = "serde_lite")]
mod serde_lite;
pub mod string;
mod traits;

pub use error::*;
pub use hybrid::*;
#[cfg(feature = "serde_lite")]
pub use serde_lite::*;
pub use traits::*;
