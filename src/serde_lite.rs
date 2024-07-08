use crate::{Decoder, Encoder};
use serde_lite::{Deserialize, Intermediate, Serialize};

/// A wrapper codec that relies on [`serde_lite`]. With this, you can wrap serde based codecs that
/// also work with serde-lite like the [`JsonSerdeCodec`] or the [`MsgpackSerdeCodec`].
///
/// Only available with the **`serde_lite` feature** enabled plus the feature you need for the
/// wrapped codec.
///
/// ## Example
///
/// ```
/// use codee::{Encoder, Decoder, SerdeLite, string::JsonSerdeCodec, binary::MsgpackSerdeCodec};
/// use serde_lite::{Deserialize, Serialize};
///
/// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
/// struct Test {
///     s: String,
///     i: i32,
/// }
///
/// let t = Test {
///     s: String::from("party time 🎉"),
///     i: 42,
/// };
///
/// let enc = SerdeLite::<JsonSerdeCodec>::encode(&t).unwrap();
/// let dec: Test = SerdeLite::<JsonSerdeCodec>::decode(&enc).unwrap();
///
/// let enc = SerdeLite::<MsgpackSerdeCodec>::encode(&t).unwrap();
/// let dec: Test = SerdeLite::<MsgpackSerdeCodec>::decode(&enc).unwrap();
/// assert_eq!(dec, t);
/// ```
pub struct SerdeLite<C>(C);

#[derive(Debug, thiserror::Error)]
pub enum SerdeLiteEncodeError<E> {
    #[error("Error from serde-lite: {0}")]
    SerdeLite(serde_lite::Error),
    #[error("Error from wrapped encoder")]
    Encoder(#[from] E),
}

#[derive(Debug, thiserror::Error)]
pub enum SerdeLiteDecodeError<E> {
    #[error("Error from serde-lite: {0}")]
    SerdeLite(serde_lite::Error),
    #[error("Error from wrapped decoder")]
    Decoder(#[from] E),
}

impl<T, E> Encoder<T> for SerdeLite<E>
where
    T: Serialize,
    E: Encoder<Intermediate>,
{
    type Error = SerdeLiteEncodeError<<E as Encoder<Intermediate>>::Error>;
    type Encoded = <E as Encoder<Intermediate>>::Encoded;

    fn encode(val: &T) -> Result<Self::Encoded, Self::Error> {
        let intermediate = val.serialize().map_err(SerdeLiteEncodeError::SerdeLite)?;
        Ok(E::encode(&intermediate)?)
    }
}

impl<T, D> Decoder<T> for SerdeLite<D>
where
    T: Deserialize,
    D: Decoder<Intermediate>,
{
    type Error = SerdeLiteDecodeError<<D as Decoder<Intermediate>>::Error>;
    type Encoded = <D as Decoder<Intermediate>>::Encoded;

    fn decode(val: &Self::Encoded) -> Result<T, Self::Error> {
        let intermediate = D::decode(val)?;
        T::deserialize(&intermediate).map_err(SerdeLiteDecodeError::SerdeLite)
    }
}
