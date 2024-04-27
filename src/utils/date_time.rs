use clokwerk::Interval::{Friday, Monday, Saturday, Sunday, Thursday, Tuesday, Wednesday};

pub(crate)  fn get_day(day: &String) -> Option<clokwerk::Interval> {
    match day.as_str() {
        "sunday" => Some(Sunday),
        "monday" => Some(Monday),
        "tuesday" => Some(Tuesday),
        "wednesday" => Some(Wednesday),
        "thursday" => Some(Thursday),
        "friday" => Some(Friday),
        "saturday" => Some(Saturday),
        _ => {
            std::panic!("Unsupported day");
        }
    }
}

pub(crate) fn get_day_before(day: &String) -> Option<clokwerk::Interval> {
    match day.as_str() {
        "sunday" => Some(Saturday),
        "monday" => Some(Sunday),
        "tuesday" => Some(Monday),
        "wednesday" => Some(Tuesday),
        "thursday" => Some(Wednesday),
        "friday" => Some(Thursday),
        "saturday" => Some(Friday),
        _ => {
            std::panic!("Unsupported day");
        }
    }
}