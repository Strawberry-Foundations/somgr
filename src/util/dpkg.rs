use std::fs::File;
use std::io::BufRead;

pub fn get_packages() -> Vec<String> {
    subprocess::Exec::shell(
        "/usr/sbin/chroot /system dpkg --get-selections | grep -v deinstall | awk '{print $1}' > /var/cache/apt/package_list.txt"
    ).popen().unwrap();

    let file = File::open("/var/cache/apt/package_list.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut packages = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        for word in line.split_whitespace() {
            packages.push(word.to_string());
        }
    }

    packages
}