use chrono::{DateTime, Utc};

pub fn time_get() -> Result<DateTime<Utc>, String> {
    Ok(Utc::now())
}
pub fn time_get_formatted() -> Result<String, String> {
    Ok(time_get()?.format("%d.%m.%Y %H:%M").to_string())
}
