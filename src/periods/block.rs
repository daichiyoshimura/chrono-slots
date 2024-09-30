use std::fmt::Debug;

use chrono::DateTime;
use chrono_tz::Tz;

use crate::impl_period;

use super::{
    period::{Period, PeriodError},
    Span,
};

// This refers to already scheduled events. The term ‘Block’ will be standardized here.”
#[derive(Debug, Clone)]
pub struct Block {
    start: DateTime<Tz>,
    end: DateTime<Tz>,
}

impl_period!(Block);

impl Block {
    // constructor
    pub fn new(start: DateTime<Tz>, end: DateTime<Tz>) -> Result<Self, PeriodError> {
        if start >= end {
            return Err(PeriodError::InvalidTime);
        }
        Ok(Block { start, end })
    }

    // Whether the Block contains the given Period.
    pub fn contains(&self, other: &Span) -> bool {
        self.start <= other.start() && other.end() <= self.end
    }

    // Whether the Block is contained within the given Period.
    pub fn is_contained_in(&self, other: &Span) -> bool {
        other.start() <= self.start && self.end <= other.end()
    }

    // Whether a period overlaps across the Block’s end time.
    pub fn overlaps_at_end(&self, other: &Span) -> bool {
        other.start() <= self.start && other.end() <= self.end && self.start <= other.end()
    }

    // Whether a period overlaps across the Block’s start time.
    pub fn overlaps_at_start(&self, other: &Span) -> bool {
        self.start <= other.start() && self.end <= other.end() && other.start() <= self.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use chrono_tz::Tz;

    fn block(now: DateTime<Tz>, start: i64, end: i64) -> Result<Block, PeriodError> {
        Block::new(now + Duration::hours(start), now + Duration::hours(end))
    }

    fn span(now: DateTime<Tz>, start: i64, end: i64) -> Result<Span, PeriodError> {
        Span::new(now + Duration::hours(start), now + Duration::hours(end))
    }

    struct TestCase<T> {
        name: &'static str,
        block: Block,
        span: Span,
        expected: T,
    }

    #[test]
    fn test_block_methods() -> Result<(), PeriodError> {
        let now = Utc::now().with_timezone(&chrono_tz::Japan);
        let cases_contains = vec![
            TestCase {
                name: "Block contains Span",
                block: block(now, 0, 8)?,
                span: span(now, 1, 7)?,
                expected: true,
            },
            TestCase {
                name: "Block does not contain Span (span starts before block)",
                block: block(now, 9, 12)?,
                span: span(now, 0, 8)?,
                expected: false,
            },
            TestCase {
                name: "Block does not contain Span (span ends after block)",
                block: block(now, 10, 12)?,
                span: span(now, 0, 8)?,
                expected: false,
            },
        ];

        let cases_is_contained_in = vec![
            TestCase {
                name: "Block is contained in Span",
                block: block(now, 1, 7)?,
                span: span(now, 0, 8)?,
                expected: true,
            },
            TestCase {
                name: "Block is not contained in Span (span starts after block)",
                block: block(now, 0, 4)?,
                span: span(now, 5, 7)?,
                expected: false,
            },
            TestCase {
                name: "Block is not contained in Span (span ends before block)",
                block: block(now, 7, 8)?,
                span: span(now, 5, 6)?,
                expected: false,
            },
        ];

        let cases_overlaps_at_start = vec![
            TestCase {
                name: "Block overlaps at start of Span",
                block: block(now, 1, 5)?,
                span: span(now, 4, 8)?,
                expected: true,
            },
            TestCase {
                name: "Block does not overlap at start of Span (block entirely before span)",
                block: block(now, 1, 5)?,
                span: span(now, 6, 8)?,
                expected: false,
            },
        ];

        let cases_overlaps_at_end = vec![
            TestCase {
                name: "Block overlaps at end of Span",
                block: block(now, 10, 20)?,
                span: span(now, 5, 15)?,
                expected: true,
            },
            TestCase {
                name: "Block does not overlap at end of Span (block entirely after span)",
                block: block(now, 10, 20)?,
                span: span(now, 0, 8)?,
                expected: false,
            },
        ];

        for case in cases_contains {
            assert_eq!(
                case.block.contains(&case.span),
                case.expected,
                "{} failed",
                case.name
            );
        }

        for case in cases_is_contained_in {
            assert_eq!(
                case.block.is_contained_in(&case.span),
                case.expected,
                "{} failed",
                case.name
            );
        }

        for case in cases_overlaps_at_start {
            assert_eq!(
                case.block.overlaps_at_start(&case.span),
                case.expected,
                "{} failed",
                case.name
            );
        }

        Ok(for case in cases_overlaps_at_end {
            assert_eq!(
                case.block.overlaps_at_end(&case.span),
                case.expected,
                "{} failed",
                case.name
            );
        })
    }

    #[test]
    fn test_block_new() {
        let now = Utc::now().with_timezone(&chrono_tz::Japan);

        fn dt(now: DateTime<Tz>, start: i64) -> DateTime<Tz> {
            now + Duration::hours(start)
        }

        let valid_block = Block::new(dt(now, 0), dt(now, 8));
        assert!(valid_block.is_ok(), "Valid block creation failed");

        let invalid_block = Block::new(dt(now, 8), dt(now, 0));
        assert!(invalid_block.is_err(), "Invalid block creation should fail");
    }
}
