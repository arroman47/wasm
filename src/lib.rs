use wasm_bindgen::prelude::*;

// Expose this function to JS
#[wasm_bindgen]
pub fn chat_response(_input: &str) -> String {
    "Thank you for your interest! Our tailor-made products are crafted to fit your needs. How can I assist you today?".to_string()
}
