use super::logger;

use super::files;
use super::system;

use std::path::Path;

pub fn run_application(id: String) -> Result<bool, String> {
    {
        let full_path = files::get_full_path(format!("/apps/{}/",id.clone()));
        let mut all_files = files::get_all_files(&Path::new(format!("{}bin/",full_path).as_str()));
        
        let mut system = system::System::new(id.clone(), all_files);
        logger::log(format!("Application {} environment initialized. Starting...", id), logger::LogType::INFO);
        system.start();
    }
    logger::log(String::from("Application environment destroyed. Finished!"), logger::LogType::INFO);
    Ok(true)
}
