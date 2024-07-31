use std::path::{Path, PathBuf};

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