use chrono::Local;

pub fn curr_time(format: Option<&str>) -> String {
    Local::now().format(format.unwrap_or("%c")).to_string()
}
