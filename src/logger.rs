use super::time;

pub enum LogType {
    ERROR,
    SUCCESS,
    INFO,
    WARNING,
    DEBUG,
    APP,
}

pub fn log(message: String, log_type: LogType) -> Result<bool, String> {
    println!(
        "{} {} {}",
        get_prefix(log_type),
        time::time_get_formatted()?,
        message
    );
    Ok(true)
}

fn get_prefix(log_type: LogType) -> String {
    match log_type {
        LogType::ERROR => String::from("[ERROR]   "),
        LogType::SUCCESS => String::from("[SUCCESS] "),
        LogType::INFO => String::from("[INFO]    "),
        LogType::WARNING => String::from("[WARNING] "),
        LogType::DEBUG => String::from("[DEBUG]   "),
        LogType::APP => String::from("[APP]     "),
    }
}
