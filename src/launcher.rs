use super::logger;
use super::files;
use super::apps;
use std::io;

pub fn launch() -> Result<bool, String> {
    let version = "0.1.0";
    logger::log(format!("Starting Virtel {version}"), logger::LogType::INFO);
    logger::log(files::init_environment(), logger::LogType::INFO);
    logger::log(format!("Initialized Virtel environment"), logger::LogType::INFO);
    logger::log(format!("Running launcher application"), logger::LogType::INFO);

    match apps::run_application(String::from("vladceresna.virtel.launcher")) {
        Ok(_) => {}
        Err(err) => {
            logger::log(format!("Error running launcher application: {err}"), logger::LogType::ERROR);
            return Err(err);
        }
    }
    
    logger::log(String::from("Finish Virtel Platform"), logger::LogType::SUCCESS);
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    Ok(true)
}
