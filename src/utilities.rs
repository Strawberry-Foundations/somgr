use std::path::{Path, PathBuf};



pub fn serializer(text: &str) -> Result<serde_json::Value, serde_json::Error> {
    let serializer = serde_json::from_str(text)?;
    Ok(serializer)
}

