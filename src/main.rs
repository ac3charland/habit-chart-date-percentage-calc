use chrono::{Local, NaiveDate};
use clap::{App, Arg};

fn main() {
    // Define command-line arguments using clap
    let matches = App::new("Percentage Calculator")
        .version("1.0")
        .author("Alex Charland")
        .about("Calculates the percentage until 10/15/2024")
        .arg(
            Arg::with_name("startDate")
                .short("s")
                .long("startDate")
                .value_name("DATE")
                .help("Specify the start date in MM/DD/YYYY format")
                .takes_value(true),
        )
        .get_matches();

    // Get the start date from command-line argument if provided, otherwise use the current date
    let today = if let Some(start_date_str) = matches.value_of("startDate") {
        parse_date(start_date_str).expect("Invalid start date format. Use MM/DD/YYYY")
    } else {
        Local::now().date_naive()
    };

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

// Helper function to parse date from MM/DD/YYYY format
fn parse_date(date_str: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(date_str, "%m/%d/%Y").ok()
}
