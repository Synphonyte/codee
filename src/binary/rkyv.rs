use crate::{Decoder, Encoder};
use rkyv::api::high::{HighSerializer, HighValidator};
use rkyv::de::Pool;
use rkyv::rancor::Strategy;
use rkyv::ser::allocator::ArenaHandle;
pub use rkyv::util::AlignedVec;
use rkyv::{bytecheck, rancor, Archive, Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

/// A codec that relies on `rkyv` to encode data in the msgpack format.
///
/// This is only available with the **`rkyv` feature** enabled.
pub struct RkyvCodec;

impl<T> Encoder<T> for RkyvCodec
where
    T: for<'a> Serialize<HighSerializer<Vec<u8>, ArenaHandle<'a>, rancor::Error>>,
{
    type Error = rancor::Error;
    type Encoded = Vec<u8>;

    fn encode(val: &T) -> Result<Self::Encoded, Self::Error> {
        rkyv::api::high::to_bytes_in(val, Vec::new())
    }
}

impl<T> Decoder<T> for RkyvCodec
where
    T: Archive,
    for<'a> T::Archived: 'a
        + bytecheck::CheckBytes<HighValidator<'a, rancor::Error>>
        + Deserialize<T, Strategy<Pool, rancor::Error>>,
{
    type Error = Arc<dyn Error>;
    type Encoded = [u8];

    fn decode(val: &Self::Encoded) -> Result<T, Self::Error> {
        let mut aligned = AlignedVec::<16>::with_capacity(val.len());
        aligned.extend_from_slice(val);
        rkyv::from_bytes::<T, rancor::Error>(&aligned).map_err(|e| Arc::new(e) as Arc<dyn Error>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rkyv_codec() {
        #[derive(Clone, Debug, PartialEq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
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
