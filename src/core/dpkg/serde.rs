use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::Path;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DebPackage {
    pub str: String,
    pub package: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub section: Option<String>,
    pub installed_size: Option<String>,
    pub maintainer: Option<String>,
    pub architecture: Option<String>,
    pub multi_arch: Option<String>,
    pub source: Option<String>,
    pub version: Option<String>,
    pub replaces: Option<String>,
    pub provides: Option<String>,
    pub depends: Option<String>,
    pub recommends: Option<String>,
    pub suggests: Option<String>,
    pub breaks: Option<String>,
    pub conflicts: Option<String>,
    pub conffiles: Option<String>,
    pub description: String,
    pub other_fields: HashMap<String, String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DpkgStatus {
    pub str: String,
    pub packages: Vec<DebPackage>,
}

impl DebPackage {
    pub fn from_raw_package(package_str: &str) -> Option<Self> {
        let mut package = DebPackage {
            str: package_str.to_string(),
            package: None,
            status: None,
            priority: None,
            section: None,
            installed_size: None,
            maintainer: None,
            architecture: None,
            multi_arch: None,
            source: None,
            version: None,
            replaces: None,
            provides: None,
            depends: None,
            recommends: None,
            suggests: None,
            breaks: None,
            conflicts: None,
            conffiles: None,
            description: String::new(),
            other_fields: HashMap::new(),
        };

        let mut current_field = String::new();
        let mut current_value = String::new();

        for line in package_str.lines() {
            if line.is_empty() {
                continue;
            } else if line.starts_with(' ') {
                current_value.push('\n');
                current_value.push_str(line.trim());
            } else {
                if !current_field.is_empty() {
                    Self::set_field(&mut package, &current_field, &current_value);
                }
                let parts: Vec<&str> = line.splitn(2, ": ").collect();
                if parts.len() == 2 {
                    current_field = parts[0].to_string();
                    current_value = parts[1].to_string();
                }
            }
        }

        if !current_field.is_empty() {
            Self::set_field(&mut package, &current_field, &current_value);
        }

        if package.package.is_some() {
            Some(package)
        } else {
            None
        }
    }

    fn set_field(package: &mut DebPackage, field: &str, value: &str) {
        match field {
            "Package" => package.package = Some(value.to_string()),
            "Status" => package.status = Some(value.to_string()),
            "Priority" => package.priority = Some(value.to_string()),
            "Section" => package.section = Some(value.to_string()),
            "Installed-Size" => package.installed_size = Some(value.to_string()),
            "Maintainer" => package.maintainer = Some(value.to_string()),
            "Architecture" => package.architecture = Some(value.to_string()),
            "Multi-Arch" => package.multi_arch = Some(value.to_string()),
            "Source" => package.source = Some(value.to_string()),
            "Version" => package.version = Some(value.to_string()),
            "Replaces" => package.replaces = Some(value.to_string()),
            "Provides" => package.provides = Some(value.to_string()),
            "Depends" => package.depends = Some(value.to_string()),
            "Recommends" => package.recommends = Some(value.to_string()),
            "Suggests" => package.suggests = Some(value.to_string()),
            "Breaks" => package.breaks = Some(value.to_string()),
            "Conflicts" => package.conflicts = Some(value.to_string()),
            "Conffiles" => package.conffiles = Some(value.to_string()),
            "Description" => package.description = value.to_string(),
            _ => {
                package.other_fields.insert(field.to_string(), value.to_string());
            }
        }
    }

    pub fn to_status_str(&self) -> String {
        let mut result = String::new();
        if let Some(ref value) = self.package {
            result.push_str(&format!("Package: {}\n", value));
        }
        if let Some(ref value) = self.status {
            result.push_str(&format!("Status: {}\n", value));
        }
        if let Some(ref value) = self.priority {
            result.push_str(&format!("Priority: {}\n", value));
        }
        if let Some(ref value) = self.section {
            result.push_str(&format!("Section: {}\n", value));
        }
        if let Some(ref value) = self.installed_size {
            result.push_str(&format!("Installed-Size: {}\n", value));
        }
        if let Some(ref value) = self.maintainer {
            result.push_str(&format!("Maintainer: {}\n", value));
        }
        if let Some(ref value) = self.architecture {
            result.push_str(&format!("Architecture: {}\n", value));
        }
        if let Some(ref value) = self.multi_arch {
            result.push_str(&format!("Multi-Arch: {}\n", value));
        }
        if let Some(ref value) = self.source {
            result.push_str(&format!("Source: {}\n", value));
        }
        if let Some(ref value) = self.version {
            result.push_str(&format!("Version: {}\n", value));
        }
        if let Some(ref value) = self.replaces {
            result.push_str(&format!("Replaces: {}\n", value));
        }
        if let Some(ref value) = self.provides {
            result.push_str(&format!("Provides: {}\n", value));
        }
        if let Some(ref value) = self.depends {
            result.push_str(&format!("Depends: {}\n", value));
        }
        if let Some(ref value) = self.recommends {
            result.push_str(&format!("Recommends: {}\n", value));
        }
        if let Some(ref value) = self.suggests {
            result.push_str(&format!("Suggests: {}\n", value));
        }
        if let Some(ref value) = self.breaks {
            result.push_str(&format!("Breaks: {}\n", value));
        }
        if let Some(ref value) = self.conflicts {
            result.push_str(&format!("Conflicts: {}\n", value));
        }
        if let Some(ref value) = self.conffiles {
            result.push_str(&format!("Conffiles: {}\n", value));
        }

        let formatted_description = self.format_multiline_field(&self.description);
        result.push_str(&format!("Description: {}\n", formatted_description));

        for (key, value) in &self.other_fields {
            let formatted_value = self.format_multiline_field(value);
            result.push_str(&format!("{}: {}\n", key, formatted_value));
        }

        result.push('\n');
        result
    }

    fn format_multiline_field(&self, field_value: &str) -> String {
        let lines: Vec<&str> = field_value.lines().collect();
        if lines.len() > 1 {
            let mut formatted_field = lines[0].to_string();
            for line in &lines[1..] {
                formatted_field.push_str(&format!("\n {}", line));
            }
            formatted_field
        } else {
            field_value.to_string()
        }
    }
}

impl Default for DpkgStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl DpkgStatus {
    pub fn new() -> Self {
        Self {
            str: String::new(),
            packages: Vec::new(),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.packages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn from_status_file<P: AsRef<Path>>(filename: P) -> Self {
        let file = File::open(&filename).unwrap();
        let reader = BufReader::new(file);
        let mut status_str = String::new();

        let mut file = File::open(&filename).unwrap();
        file.read_to_string(&mut status_str).unwrap();

        let mut package_str = String::new();
        let mut packages = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();
            if line.is_empty() {
                if let Some(package) = DebPackage::from_raw_package(&package_str) {
                    packages.push(package);
                }
                package_str.clear();
            } else {
                package_str.push_str(&line);
                package_str.push('\n');
            }
        }

        if !package_str.is_empty() {
            if let Some(package) = DebPackage::from_raw_package(&package_str) {
                packages.push(package);
            }
        }

        Self {
            str: status_str,
            packages,
        }
    }

    pub fn parse_status_file<P: AsRef<Path>>(&mut self, filename: P) -> io::Result<()> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let mut package_str = String::new();

        for line in reader.lines() {
            let line = line?;
            if line.is_empty() {
                if let Some(package) = DebPackage::from_raw_package(&package_str) {
                    self.packages.push(package);
                }
                package_str.clear();
            } else {
                package_str.push_str(&line);
                package_str.push('\n');
            }
        }

        if !package_str.is_empty() {
            if let Some(package) = DebPackage::from_raw_package(&package_str) {
                self.packages.push(package);
            }
        }

        Ok(())
    }

    pub fn get_packages(&self) -> &Vec<DebPackage> {
        &self.packages
    }

    pub fn search_package(&self, name: &str) -> Option<&DebPackage> {
        self.packages.iter().find(|pkg| pkg.package.as_deref() == Some(name))
    }

    pub fn contains(&self, name: &str) -> bool {
        self.packages.iter().any(|pkg| pkg.package.as_deref() == Some(name))
    }

    pub fn update_status(&mut self, old_str: &String, new_str: &str) {
        self.str = self.str.replace(old_str, new_str);
    }

    pub fn write_status_file<P: AsRef<Path>>(&self, filename: P) -> io::Result<()> {
        let mut file = File::create(filename)?;
        file.write_all(self.str.as_ref())?;

        Ok(())
    }
}