use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodecError<E, D> {
    #[error("failed to encode: {0}")]
    Encode(E),
    #[error("failed to decode: {0}")]
    Decode(D),
}
