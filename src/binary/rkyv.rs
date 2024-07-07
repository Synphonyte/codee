use crate::{Decoder, Encoder};
use rkyv::de::deserializers::SharedDeserializeMap;
use rkyv::ser::serializers::AllocSerializer;
use rkyv::validation::validators::DefaultValidator;
use rkyv::{Archive, CheckBytes, Deserialize, Fallible, Serialize};
use std::error::Error;
use std::sync::Arc;

/// A codec that relies on `rkyv` to encode data in the msgpack format.
///
/// This is only available with the **`rkyv` feature** enabled.
pub struct RkyvCodec;

impl<T> Encoder<T> for RkyvCodec
where
    T: Serialize<AllocSerializer<1024>>,
{
    type Error = <AllocSerializer<1024> as Fallible>::Error;
    type Encoded = Vec<u8>;

    fn encode(val: &T) -> Result<Self::Encoded, Self::Error> {
        Ok(rkyv::to_bytes::<T, 1024>(val)?.to_vec())
    }
}

impl<T> Decoder<T> for RkyvCodec
where
    T: Archive,
    for<'a> T::Archived:
        'a + CheckBytes<DefaultValidator<'a>> + Deserialize<T, SharedDeserializeMap>,
{
    type Error = Arc<dyn Error>;
    type Encoded = [u8];

    fn decode(val: &Self::Encoded) -> Result<T, Self::Error> {
        rkyv::from_bytes::<T>(val).map_err(|e| Arc::new(e) as Arc<dyn Error>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rkyv_codec() {
        #[derive(Clone, Debug, PartialEq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
        #[archive(check_bytes)]
        struct Test {
            s: String,
            i: i32,
        }
        let t = Test {
            s: String::from("party time 🎉"),
            i: 42,
        };
        let enc = RkyvCodec::encode(&t).unwrap();
        let dec: Test = RkyvCodec::decode(&enc).unwrap();
        assert_eq!(dec, t);
    }
}
