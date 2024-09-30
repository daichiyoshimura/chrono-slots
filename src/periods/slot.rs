use std::fmt::Debug;

use chrono::DateTime;
use chrono_tz::Tz;

use crate::impl_period;

use super::{
    block::Block,
    period::{Period, PeriodError},
    span::Span,
};

/// This refers to available free time. The term ‘Slot’ will be standardized here.
#[derive(Debug, Clone)]
pub struct Slot {
    start: DateTime<Tz>,
    end: DateTime<Tz>,
}

impl_period!(Slot);

impl Slot {

    /// constructor
    pub fn new(start: DateTime<Tz>, end: DateTime<Tz>) -> Result<Self, PeriodError> {
        if start >= end {
            return Err(PeriodError::InvalidTime);
        }
        Ok(Slot { start, end })
    }

    /// constructor
    pub fn create_from(target: &Span, block: &Block) -> Result<Self, PeriodError> {
        if target.start() > block.start() {
            return Err(PeriodError::InvalidTime);
        }
        Ok(Slot {
            start: target.start(),
            end: block.start(),
        })
    }
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

    fn span(now: DateTime<Tz>, start: i64, end: i64) -> Result<Span, PeriodError> {
        Span::new(dt(now, start), dt(now, end))
    }

    fn slot(now: DateTime<Tz>, start: i64, end: i64) -> Result<Slot, PeriodError> {
        Slot::new(dt(now, start), dt(now, end))
    }

    #[test]
    fn test_slot_create_from() -> Result<(), PeriodError> {
        let now = Utc::now().with_timezone(&chrono_tz::Japan);

        struct TestCase {
            name: &'static str,
            span: Span,
            block: Block,
            expected: Result<Slot, PeriodError>,
        }

        let cases = vec![
            TestCase {
                name: "Valid Slot creation from Span and Block",
                span: span(now, 0, 8)?,
                block: block(now, 4, 9)?,
                expected: Ok(slot(now, 0, 4)?),
            },
            TestCase {
                name: "Invalid Slot creation (Span starts after Block)",
                span: span(now, 4, 8)?,
                block: block(now, 1, 5)?,
                expected: Err(PeriodError::InvalidTime),
            },
        ];

        Ok(for case in cases {
            let result = Slot::create_from(&case.span, &case.block);
            match &result {
                Ok(actual) => {
                    let expected = case.expected.unwrap();
                    assert_eq!(
                        actual.start(),
                        expected.start(),
                        "Failed on start: {}",
                        case.name
                    );
                    assert_eq!(actual.end(), expected.end(), "Failed on end: {}", case.name);
                }
                Err(err) => {
                    assert_eq!(
                        err.to_string(),
                        case.expected.unwrap_err().to_string(),
                        "Failed on error case: {}",
                        case.name
                    );
                }
            }
        })
    }
}
