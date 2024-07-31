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