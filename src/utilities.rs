use std::path::{Path, PathBuf};

pub fn format_size(kilobytes: i64) -> String {
    const KB: i64 = 1024;
    const MB: i64 = 1024 * KB;

    if kilobytes < 0 {
        return String::from("0 kB")
    }

    let bytes = kilobytes * KB;

    if bytes < KB {
        format!("{} B", bytes)
    } else if bytes < MB {
        format!("{:.1} kB", bytes as f64 / KB as f64)
    } else {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    }
}

pub fn serializer(text: &str) -> Result<serde_json::Value, serde_json::Error> {
    let serializer = serde_json::from_str(text)?;
    Ok(serializer)
}

pub fn capitalize_first(input: &str) -> String {
    if input.is_empty() {
        String::new()
    } else {
        let mut chars = input.chars();
        let first_char = chars.next().unwrap().to_uppercase();
        let rest_of_string: String = chars.collect();
        format!("{}{}", first_char, rest_of_string)
    }
}

pub fn calc_percent(part: u64, val: u64) -> f64 {
    let result = if val != 0 {
        (part as f64 / val as f64) * 100.0
    } else {
        0.0
    };

    result.ceil()
}

pub fn make_absolute_path(input_path: &str) -> PathBuf {
    let path = Path::new(input_path);

    if path.is_absolute() {
        PathBuf::from(path)
    } else {
        // Wenn der Pfad relativ ist, füge das aktuelle Arbeitsverzeichnis hinzu
        let mut absolute_path = std::env::current_dir().expect("Konnte das aktuelle Verzeichnis nicht abrufen.");
        absolute_path.push(path);
        absolute_path
    }
}