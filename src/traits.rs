/// Trait every encoder must implement.
pub trait Encoder<T>: 'static {
    type Error;
    type Encoded;

    fn encode(val: &T) -> Result<Self::Encoded, Self::Error>;
}

/// Trait every decoder must implement.
pub trait Decoder<T>: 'static {
    type Error;
    type Encoded: ?Sized;

    fn decode(val: &Self::Encoded) -> Result<T, Self::Error>;
}
