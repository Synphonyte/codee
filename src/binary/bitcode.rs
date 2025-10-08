/// A codec that relies on `bitcode` to encode data.
///
/// This is only available with the **`bitcode` feature** feature enabled.
///
/// If you want to use `serde` with bitcode, please use the `bitcode_serde` feature instead.
pub struct BitcodeCodec;

impl<T: bitcode::Encode> crate::Encoder<T> for BitcodeCodec {
    type Error = ();
    type Encoded = Vec<u8>;

    fn encode(val: &T) -> Result<Self::Encoded, Self::Error> {
        Ok(bitcode::encode(val))
    }
}

impl<T: for<'a> bitcode::Decode<'a>> crate::Decoder<T> for BitcodeCodec {
    type Error = bitcode::Error;
    type Encoded = [u8];

    fn decode(val: &Self::Encoded) -> Result<T, Self::Error> {
        bitcode::decode(val)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Decoder, Encoder};

    use super::*;

    #[test]
    fn test_bitcode_codec() {
        #[derive(Clone, Debug, PartialEq, bitcode::Encode, bitcode::Decode)]
        struct Test {
            s: String,
            i: i32,
        }
        let t = Test {
            s: String::from("party time 🎉"),
            i: 42,
        };
        let enc = BitcodeCodec::encode(&t).unwrap();
        let dec: Test = BitcodeCodec::decode(&enc).unwrap();
        assert_eq!(dec, t);
    }

    #[test]
    fn test_bitcode_codec_complex() {
        use std::collections::BTreeMap;
        use std::collections::HashMap;
        use std::sync::Arc;

        #[derive(Debug, PartialEq, bitcode::Encode, bitcode::Decode)]
        struct Test {
            s: String,
            i: i32,
            m: HashMap<String, i32>,
            b: BTreeMap<String, i32>,
            a: Arc<String>,
        }
        let t = Test {
            s: String::from("party time 🎉"),
            i: 42,
            m: HashMap::from([("a".to_string(), 1), ("b".to_string(), 2)]),
            b: BTreeMap::from([("a".to_string(), 1), ("b".to_string(), 2)]),
            a: Arc::new(String::from("party time 🎉")),
        };
        let enc = BitcodeCodec::encode(&t).unwrap();
        let dec: Test = BitcodeCodec::decode(&enc).unwrap();
        assert_eq!(dec, t);
    }

    #[test]
    fn test_bitcode_codec_error() {
        #[derive(bitcode::Encode, bitcode::Decode)]
        struct Test;
        let enc = [0];
        let dec: Result<Test, bitcode::Error> = BitcodeCodec::decode(&enc);
        assert!(dec.is_err());
    }
}
