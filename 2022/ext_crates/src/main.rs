use humantime::format_duration;
use std::time::Duration;

#[allow(unused_imports)]
use chrono::prelude::*;

fn main()
{
    let d = Duration::from_secs(9876);
    println!("{}", format_duration(d));

    let todays_date: DateTime<Utc> = Utc::now();
    let the_custom_date: DateTime<Utc> = Utc.with_ymd_and_hms(2022, 11, 14, 0, 0, 0).unwrap();

    println!("{:?}", todays_date);
    println!("{:?}", the_custom_date);

    let local: DateTime<Local> = Local::now();

    println!("{}", local.format("%Y-%m-%d %H:%M:%S").to_string());
}

