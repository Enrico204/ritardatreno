use colored::*;

pub fn format_real_time(expected_time: Option<chrono::DateTime<chrono::Local>>, real_time: Option<chrono::DateTime<chrono::Local>>) -> String {
    let formatted_time = expected_time.unwrap().format("%I:%M").to_string();

    if real_time.is_some() {
        let real_time_str = String::from(real_time.unwrap().format("%I:%M").to_string());

        return if real_time.gt(&expected_time) {
            format!("{} ({})", formatted_time, real_time_str.red())
        } else if real_time.lt(&expected_time) {
            format!("{} ({})", formatted_time, real_time_str.green())
        } else {
            format!("{} ({})", formatted_time, real_time_str)
        };
    }
    return formatted_time;
}