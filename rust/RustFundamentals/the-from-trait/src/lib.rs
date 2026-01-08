pub struct Minutes(pub i32);
pub struct Hours(pub i32);
pub struct Days(pub i32);

impl From<Minutes> for Hours {
    fn from(minutes: Minutes) -> Hours {
        // Implement the minute to hour conversion here
        Hours(minutes.0 / 60)
    }
}

// TODO: implement from hours to days
impl From<Hours> for Days {
    fn from(hours: Hours) -> Days {
        // Implement the minute to hour conversion here
        Days(hours.0 / 24)
    }
}

impl From<Minutes> for Days {
    fn from(minutes: Minutes) -> Days {
        // Implement the minute to hour conversion here
        Days::from(Hours::from(minutes))
    }
}

impl From<Days> for Hours {
    fn from(days: Days) -> Hours {
        // Implement the minute to hour conversion here
        Hours(days.0 * 24)
    }
}

// TODO: implement from minutes to days

// TODO: implement from days to hours

// Example usage
pub fn main() {
    let minutes = Minutes(60);
    let hours: Hours = minutes.into();
    assert_eq!(hours.0, 1);
}
