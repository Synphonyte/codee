use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodecError<E, D> {
    #[error("failed to encode: {0}")]
    Encode(E),
    #[error("failed to decode: {0}")]
    Decode(D),
}

impl<E, D> Clone for CodecError<E, D>
where
    E: Clone,
    D: Clone,
{
    fn clone(&self) -> Self {
        match self {
            CodecError::Encode(e) => CodecError::Encode(e.clone()),
            CodecError::Decode(d) => CodecError::Decode(d.clone()),
        }
    }
}

impl<E, D> Copy for CodecError<E, D>
where
    E: Copy,
    D: Copy,
{
}
