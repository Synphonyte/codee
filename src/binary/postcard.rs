use crate::{Decoder, Encoder};

/// A codec that relies on `postcard` to encode data.
///
/// Postcard is a `#![no_std]` focused serializer and deserializer for Serde,
/// designed for embedded and constrained environments.
///
/// This is only available with the **`postcard` feature** enabled.
pub struct PostcardCodec;

impl<T: serde::Serialize> Encoder<T> for PostcardCodec {
    type Error = postcard::Error;
    type Encoded = Vec<u8>;

    fn encode(val: &T) -> Result<Self::Encoded, Self::Error> {
        postcard::to_allocvec(val)
    }
}

impl<T: serde::de::DeserializeOwned> Decoder<T> for PostcardCodec {
    type Error = postcard::Error;
    type Encoded = [u8];

    fn decode(val: &Self::Encoded) -> Result<T, Self::Error> {
        postcard::from_bytes(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_postcard_codec() {
        #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        struct Test {
            s: String,
            i: i32,
        }
        let t = Test {
            s: String::from("party time 🎉"),
            i: 42,
        };
        let enc = PostcardCodec::encode(&t).unwrap();
        let dec: Test = PostcardCodec::decode(&enc).unwrap();
        assert_eq!(dec, t);
    }
}
