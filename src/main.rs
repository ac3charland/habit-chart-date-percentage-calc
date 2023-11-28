use chrono::{Local, NaiveDate};

fn main() {
    // Get the current date
    let today = Local::now().date_naive();

    // Create a date for October 15th, 2024
    let target_date = NaiveDate::from_ymd_opt(2024, 10, 15).expect("Invalid date");

    // Calculate the duration between today and the target date
    let duration = target_date.signed_duration_since(today);

    // Extract the number of days from the duration
    let days = duration.num_days();

    // Calculate percentage
    let percentage = (365 - days) as f32 * 100.0 / 365.0;

    // Print the result
    println!("\nPercentage complete until October 15th, 2024:\n\n {}%\n\n=============================================\n", percentage.round());
}
