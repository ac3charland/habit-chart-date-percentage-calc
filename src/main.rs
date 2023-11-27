fn main() {
    // Get the current date
    let today = Local::today();

    // Create a date for October 15th, 2024
    let target_date = Local::ymd(2024, 10, 15);

    // Calculate the duration between today and the target date
    let duration = target_date.signed_duration_since(today);

    // Extract the number of days from the duration
    let days = duration.num_days();

    let percentage = (365 - days) / 365 * 100;

    // Print the result
    println!("Percentage complete until October 15th, 2024:\n\n {}\n\n==============================", percentage);
}
