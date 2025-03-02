fn is_leap_year(year: u32) -> bool{
    if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
        true
    }
    else {
        false
    }
}

fn num_days_in_month(year: u32, month: u32) -> u32{
    match month {
        2 if is_leap_year(year) => 29,
        2 => 28,
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ => panic!("Not a good month: {}", month),
    }
}

fn month_name(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "Mach",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => panic!("unknown month: {}", month),
    }
}

fn main() {
    let mut year = 1;
    let mut month = 1;
    let mut days_max = num_days_in_month(year, month);
    let mut day = 1;
    let mut weekday = 1;

    loop {
        if weekday == 5 && day == 13 {
            println!("Friday, {} 13, {}", month_name(month), year);
        }

        if weekday == 7 {
            weekday = 1;
        } else {
            weekday += 1;
        }

        if day == days_max {
            day = 1;

            if month == 12 {
                month = 1;
                year += 1;
            } else {
                month += 1;
            }

            days_max = num_days_in_month(year, month);
        } else {
            day += 1;
        }
    }
}
