use std::fmt::Debug;

use chrono::DateTime;
use chrono_tz::Tz;

use crate::impl_period;

use super::{
    block::Block,
    period::{Period, PeriodError},
    slot::Slot,
};

#[derive(Debug, Clone)]
pub struct Span {
    start: DateTime<Tz>,
    end: DateTime<Tz>,
}

impl_period!(Span);

impl Span {
    pub fn new(start: DateTime<Tz>, end: DateTime<Tz>) -> Result<Self, PeriodError> {
        if start >= end {
            return Err(PeriodError::InvalidTime);
        }
        Ok(Span { start, end })
    }

    pub fn remain(&self) -> bool {
        self.start < self.end
    }

    pub fn shorten(&mut self, other: &Block) {
        self.start = other.end()
    }

    pub fn terminate(&mut self) {
        self.start = self.end
    }

    pub fn to_slot(&self) -> Result<Slot, PeriodError> {
        Slot::new(self.start(), self.end())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    fn dt(now: DateTime<Tz>, hours: i64) -> DateTime<Tz> {
        now + Duration::hours(hours)
    }

    fn span(now: DateTime<Tz>, start: i64, end: i64) -> Result<Span, PeriodError> {
        if start >= end {
            let mut s = Span::new(dt(now, start), dt(now, end + 8))?;
            s.shorten(&Block::new(dt(now, start), dt(now, end + 8))?);
            s.terminate();
            return Ok(s);
        }
        Span::new(dt(now, start), dt(now, end))
    }

    fn slot(now: DateTime<Tz>, start: i64, end: i64) -> Result<Slot, PeriodError> {
        Slot::new(dt(now, start), dt(now, end))
    }

    fn block(now: DateTime<Tz>, start: i64, end: i64) -> Result<Block, PeriodError> {
        Block::new(dt(now, start), dt(now, end))
    }

    #[test]
    fn test_span_to_slot() -> Result<(), PeriodError> {
        let now = Utc::now().with_timezone(&chrono_tz::Japan);

        struct TestCase {
            name: &'static str,
            span: Span,
            result: Result<Slot, PeriodError>,
        }
        let cases = vec![
            TestCase {
                name: "valid",
                span: span(now, 0, 8)?,
                result: slot(now, 0, 8),
            },
            TestCase {
                name: "invalid",
                span: span(now, 0, 0)?,
                result: Err(PeriodError::InvalidTime),
            },
        ];

        Ok(for case in cases {
            let span = case.span.clone();
            match span.to_slot() {
                Ok(slot) => {
                    assert_eq!(
                        slot.to_string(),
                        case.result.unwrap().to_string(),
                        "Test case failed: {}",
                        case.name
                    );
                }
                Err(err) => {
                    assert_eq!(
                        err.to_string(),
                        case.result.unwrap_err().to_string(),
                        "Test case failed: {}",
                        case.name
                    );
                }
            }
        })
    }

    #[test]
    fn test_span_remain() -> Result<(), PeriodError> {
        let now = Utc::now().with_timezone(&chrono_tz::Japan);
        struct TestCase {
            name: &'static str,
            span: Span,
            expected: bool,
        }
        let cases = vec![
            TestCase {
                name: "valid",
                span: span(now, 0, 8)?,
                expected: true,
            },
            TestCase {
                name: "terminateped",
                span: span(now, 0, 0)?,
                expected: false,
            },
        ];

        Ok(for case in cases {
            let span = case.span.clone();
            assert_eq!(
                span.remain(),
                case.expected,
                "Test case failed: {}",
                case.name
            );
        })
    }

    #[test]
    fn test_span_shorten() -> Result<(), PeriodError> {
        let now = Utc::now().with_timezone(&chrono_tz::Japan);
        struct TestCase {
            name: &'static str,
            span: Span,
            block: Block,
            expected: Span,
        }
        let cases = vec![
            TestCase {
                name: "shorten",
                span: span(now, 0, 8)?,
                block: block(now, 3, 4)?,
                expected: span(now, 4, 8)?,
            },
            TestCase {
                name: "terminateped",
                span: span(now, 0, 8)?,
                block: block(now, 0, 8)?,
                expected: span(now, 0, 0)?,
            },
        ];

        Ok(for case in cases {
            let mut span = case.span.clone();
            span.shorten(&case.block);
            assert_eq!(
                span.to_string(),
                case.expected.to_string(),
                "Test case failed: {}",
                case.name
            );
            assert_eq!(
                span.remain(),
                case.expected.remain(),
                "Test case failed: {}",
                case.name
            );
        })
    }
}
