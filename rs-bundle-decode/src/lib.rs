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

    let bundle_bytes_ext = hex::decode(hex_str).map_err(|e| e.to_string())?;
    let bundle_bytes = bundle_bytes_ext[(4 + 32 + 32)..].to_vec();

    let bundle = AngstromBundle::pade_decode(&mut bundle_bytes.as_slice(), None)
        .map_err(|e| e.to_string())?;

    let string_bundle = serde_json::to_string(&bundle).map_err(|e| e.to_string())?;

    Ok(string_bundle)
}
