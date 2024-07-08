use crate::{Decoder, Encoder};
use serde::{Deserialize, Serialize};

/// A codec for encoding with [`serde_json`] and decoding using [`serde-wasm-bindgen`].
///
/// Only available with the **`json_serde_wasm` feature** enabled.
pub struct JsonSerdeWasmCodec;

impl<T: Serialize> Encoder<T> for JsonSerdeWasmCodec {
    type Error = serde_json::Error;
    type Encoded = String;

    fn encode(val: &T) -> Result<Self::Encoded, Self::Error> {
        serde_json::to_string(val)
    }
}

impl<T> Decoder<T> for JsonSerdeWasmCodec
where
    for<'de> T: Deserialize<'de>,
{
    type Error = wasm_bindgen::JsValue;
    type Encoded = str;

    fn decode(val: &Self::Encoded) -> Result<T, Self::Error> {
        let json = js_sys::JSON::parse(val)?;
        Ok(serde_wasm_bindgen::from_value(json)?)
    }
}

// TODO
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_json_codec() {
//         #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
//         struct Test {
//             s: String,
//             i: i32,
//         }
//         let t = Test {
//             s: String::from("party time 🎉"),
//             i: 42,
//         };
//         let enc = JsonSerdeWasmCodec::encode(&t).unwrap();
//         let dec: Test = JsonSerdeWasmCodec::decode(&enc).unwrap();
//         assert_eq!(dec, t);
//     }
// }
