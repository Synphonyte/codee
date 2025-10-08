use crate::{Decoder, Encoder};

/// A codec that relies on `bitcode` and `serde` to encode data in the bitcode format.
///
/// This is only available with the **`bitcode_serde` feature** enabled.
pub struct BitcodeSerdeCodec;

impl<T: serde::Serialize> Encoder<T> for BitcodeSerdeCodec {
    type Error = bitcode::Error;
    type Encoded = Vec<u8>;

    fn encode(val: &T) -> Result<Self::Encoded, Self::Error> {
        bitcode::serialize(val)
    }
}

impl<T: serde::de::DeserializeOwned> Decoder<T> for BitcodeSerdeCodec {
    type Error = bitcode::Error;
    type Encoded = [u8];

    fn decode(val: &Self::Encoded) -> Result<T, Self::Error> {
        bitcode::deserialize(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitcode_codec() {
        #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        struct Test {
            s: String,
            i: i32,
        }
        let t = Test {
            s: String::from("party time 🎉"),
            i: 42,
        };
        let enc = BitcodeSerdeCodec::encode(&t).unwrap();
        let dec: Test = BitcodeSerdeCodec::decode(&enc).unwrap();
        assert_eq!(dec, t);
    }
}
