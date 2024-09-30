use std::fmt::{self, Debug};

use std::error::Error;

use chrono::DateTime;
use chrono_tz::Tz;

use super::block::Block;
use super::slot::Slot;

#[derive(Debug)]
pub enum PeriodError {
    InvalidTime,
}

impl fmt::Display for PeriodError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PeriodError::InvalidTime => write!(f, "Start time must be before end time."),
        }
    }
}

impl Error for PeriodError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

/// This is an interface representing a period. Block, Span, and Slot all implement the Period interface.
pub trait Period {
    /// Start time of the period.
    fn start(&self) -> DateTime<Tz>;

    /// End time of the period.
    fn end(&self) -> DateTime<Tz>;

    /// Represents the start time and end time as strings.
    fn to_string(&self) -> String {
        let duration = self.end() - self.start();
        let (hours, minutes) = (duration.num_hours(), duration.num_minutes() % 60);
        format!(
            "start: {}, end: {}, duration: {}h {}m",
            self.start().format(DATETIME_FORMAT),
            self.end().format(DATETIME_FORMAT),
            hours,
            minutes
        )
    }
}

/// input of find
pub trait Input: Period {
    /// To convert internally, define the map function for your input
    fn to_block(&self) -> Result<Block, PeriodError>;
}

/// output of find
pub trait Output: Period {
    /// To convert internally, define the map function for your output
    fn create_from_slot(slot: Slot) -> Self;
}

/// Vec<Period>
pub trait PeriodVec {
    /// Represents the start time and end time as strings.
    fn to_string(&self) -> String;
}

impl<T> PeriodVec for Vec<T>
where
    T: Period,
{
    /// Represents the start time and end time as strings.
    fn to_string(&self) -> String {
        self.iter()
            .map(|period| period.to_string())
            .collect::<Vec<_>>()
            .join("\n ")
    }
}

#[macro_export]
macro_rules! impl_period {
    ($t:ty) => {
        impl Period for $t {
            /// Start time of the period.
            fn start(&self) -> DateTime<Tz> {
                self.start
            }

            /// End time of the period.
            fn end(&self) -> DateTime<Tz> {
                self.end
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    fn dt(now: DateTime<Tz>, hours: i64) -> DateTime<Tz> {
        now + Duration::hours(hours)
    }

    fn block(now: DateTime<Tz>, start: i64, end: i64) -> Result<Block, PeriodError> {
        Block::new(dt(now, start), dt(now, end))
    }

    struct TestCase<T> {
        name: &'static str,
        input: T,
        expected_string: String,
    }

    #[test]
    fn test_period_methods() -> Result<(), PeriodError> {
        let now = Utc::now().with_timezone(&chrono_tz::Japan);
        let block = &block(now, 0, 8)?;

        let cases = vec![TestCase {
            name: "Basic case 3 hour duration",
            input: block,
            expected_string: format!(
                "start: {}, end: {}, duration: 8h 0m",
                block.start().format(DATETIME_FORMAT),
                block.end().format(DATETIME_FORMAT)
            ),
        }];

        Ok(for case in cases {
            let result_string = case.input.to_string();
            assert_eq!(
                result_string, case.expected_string,
                "Failed on to_string: {}",
                case.name
            );
        })
    }

    #[test]
    fn test_invalid_block_creation() {
        let now = Utc::now().with_timezone(&chrono_tz::Japan);

        let valid_block = Block::new(dt(now, 0), dt(now, 8));
        assert!(valid_block.is_ok(), "Valid block creation failed");

        let invalid_block = Block::new(dt(now, 0), dt(now, 0));
        assert!(invalid_block.is_err(), "Invalid block creation should fail");
    }

    #[test]
    fn test_period_vec_to_string() -> Result<(), PeriodError> {
        let now = Utc::now().with_timezone(&chrono_tz::Japan);
        let periods = vec![block(now, 0, 1)?, block(now, 3, 4)?, block(now, 5, 6)?];

        let period_strings = periods.to_string();

        let expected_strings = vec![
            format!(
                "start: {}, end: {}, duration: 1h 0m",
                periods[0].start().format(DATETIME_FORMAT),
                periods[0].end().format(DATETIME_FORMAT),
            ),
            format!(
                "start: {}, end: {}, duration: 1h 0m",
                periods[1].start().format(DATETIME_FORMAT),
                periods[1].end().format(DATETIME_FORMAT),
            ),
            format!(
                "start: {}, end: {}, duration: 1h 0m",
                periods[2].start().format(DATETIME_FORMAT),
                periods[2].end().format(DATETIME_FORMAT),
            ),
        ]
        .join("\n ");

        assert_eq!(
            period_strings, expected_strings,
            "PeriodVec to_string failed"
        );
        Ok(())
    }
}
