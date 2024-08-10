use std::fs;
use std::path::Path;

pub fn is_steps_file(name: String) -> bool {
    if name.ends_with(".steps") {
        return true;
    }
    false
}
pub fn get_code_from_file(file_path: String) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

pub fn get_all_files(dir: &Path) -> Vec<String> {
    let mut files = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_file() {
                files.push(path.to_str().unwrap().to_string());
            } else if path.is_dir() {
                files.extend(get_all_files(&path));

            }
        }
    }

    files
}



#[cfg(target_os = "windows")]
pub fn init_environment() -> String {
    create_dir(String::from("C:/Virtel/"))
}
#[cfg(target_os = "android")]
pub fn init_environment() -> String {
    create_dir(String::from("/storage/emulated/0/Virtel/"))
}
#[cfg(target_os = "linux")]
pub fn init_environment() -> String {
    create_dir(String::from("/home/Virtel/"))
}
pub fn create_dir(path: String) -> String {
    match fs::create_dir(path.clone()) {
        Ok(_) => format!("Dir {} created successfully!", path),
        Err(error) => format!("Error while creating dir: {path}: {}", error),
    }
}



#[cfg(target_os = "windows")]
pub fn get_full_path(path: String) -> String {
    format!("{}{}", String::from("C:/Virtel"), path)
}

#[cfg(target_os = "linux")]
pub fn get_full_path(path: String) -> String {
    format!("{}{}", String::from("/home/Virtel"), path)
}

#[cfg(target_os = "android")]//linux
pub fn get_full_path(path: String) -> String {
    format!("{}{}", String::from("/storage/emulated/0/Virtel"), path)
}