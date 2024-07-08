use crate::{Decoder, Encoder};
use miniserde::{json, Deserialize, Serialize};

/// A codec that relies on `miniserde` to encode data in the json format.
///
/// This is only available with the **`miniserde` feature** enabled.
pub struct MiniserdeCodec;

impl<T: Serialize> Encoder<T> for MiniserdeCodec {
    type Error = ();
    type Encoded = String;

    fn encode(val: &T) -> Result<Self::Encoded, Self::Error> {
        Ok(json::to_string(val))
    }
}

impl<T: Deserialize> Decoder<T> for MiniserdeCodec {
    type Error = miniserde::Error;
    type Encoded = str;

    fn decode(val: &Self::Encoded) -> Result<T, Self::Error> {
        json::from_str(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_codec() {
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        struct Test {
            s: String,
            i: i32,
        }
        let t = Test {
            s: String::from("party time 🎉"),
            i: 42,
        };
        let enc = MiniserdeCodec::encode(&t).unwrap();
        let dec: Test = MiniserdeCodec::decode(&enc).unwrap();
        assert_eq!(dec, t);
    }
}
