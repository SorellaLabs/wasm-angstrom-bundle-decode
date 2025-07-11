mod types;

use pade::PadeDecode;
use types::AngstromBundle;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn decode_bundle(s: String) -> String {
    match _decode_bundle(s) {
        Ok(v) => v,
        Err(v) => v,
    }
}

fn _decode_bundle(s: String) -> Result<String, String> {
    let hex_str = s.strip_prefix("0x").unwrap_or(&s);

    if !hex_str.starts_with("09c5eabe") {
        return Err(format!(
            "invalid string must start with function selector: `0x09c5eabe` or 09c5eabe"
        ));
    }

    let bundle_bytes_ext = hex::decode(hex_str).map_serde_err()?;
    let bundle_bytes = bundle_bytes_ext[(4 + 32 + 32)..].to_vec();

    let bundle = AngstromBundle::pade_decode(&mut bundle_bytes.as_slice(), None).map_serde_err()?;

    let string_bundle = serde_json::to_string(&bundle).map_serde_err()?;

    Ok(string_bundle)
}

trait MapErrSerdeString<D> {
    fn map_serde_err(self) -> Result<D, String>;
}

impl<D, E: ToString> MapErrSerdeString<D> for Result<D, E> {
    fn map_serde_err(self) -> Result<D, String> {
        self.map_err(|e| serde_json::to_string(&e.to_string()).unwrap())
    }
}
