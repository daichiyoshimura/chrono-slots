use thiserror::Error;

use crate::periods::{
    period::{Input, Output, PeriodError},
    slot::Slot,
    span::Span,
};

#[derive(Debug, Error)]
pub enum SlotError {
    #[error("Invalid blocks. Check your arguments are valid.")]
    InvalidPeriod(#[from] PeriodError),
}

pub fn find<In: Input, Out: Output>(
    span: Span,
    mut inputs: Vec<In>,
) -> Result<Vec<Out>, SlotError> {
    inputs.sort_by_key(|p| p.start());

    let mut slots = Vec::new();
    let mut target = span.clone();
    for input in inputs {
        let block = input.to_block()?;

        if block.contains(&target) {
            target.terminate();
            break;
        }

        if block.overlaps_at_start(&target) {
            target.shorten(&block);
            continue;
        }

        if block.is_contained_in(&target) {
            let slot = Slot::create_from(&target, &block)?;
            slots.push(Out::create_from_slot(slot));
            target.shorten(&block);
            continue;
        }

        if block.overlaps_at_end(&target) {
            let slot = Slot::create_from(&target, &block)?;
            slots.push(Out::create_from_slot(slot));
            target.terminate();
            break;
        }
    }

    if !target.remain() {
        return Ok(slots);
    }

    let slot = target.to_slot()?;
    slots.push(Out::create_from_slot(slot));
    Ok(slots)
}

#[cfg(test)]
mod tests {
    use crate::{Block, Period};

    use super::*;
    use chrono::{DateTime, Duration, Utc};
    use chrono_tz::Tz;

    // Mock structures for testing
    #[derive(Debug, Clone)]
    struct MockInput {
        start_at: DateTime<Tz>,
        end_at: DateTime<Tz>,
    }

    impl MockInput {
        fn new(now: DateTime<Tz>, start: i64, end: i64) -> Self {
            MockInput {
                start_at: now + Duration::hours(start),
                end_at: now + Duration::hours(end),
            }
        }
    }

    impl Period for MockInput {
        fn start(&self) -> DateTime<Tz> {
            self.start_at
        }

        fn end(&self) -> DateTime<Tz> {
            self.end_at
        }
    }

    impl Input for MockInput {
        fn to_block(&self) -> Result<Block, PeriodError> {
            Block::new(self.start_at, self.end_at)
        }
    }

    #[derive(Debug, Clone)]
    struct MockOutput {
        start_at: DateTime<Tz>,
        end_at: DateTime<Tz>,
    }

    impl MockOutput {
        fn new(now: DateTime<Tz>, start: i64, end: i64) -> Self {
            MockOutput {
                start_at: now + Duration::hours(start),
                end_at: now + Duration::hours(end),
            }
        }
    }

    impl Period for MockOutput {
        fn start(&self) -> DateTime<Tz> {
            self.start_at
        }

        fn end(&self) -> DateTime<Tz> {
            self.end_at
        }
    }

    impl Output for MockOutput {
        fn create_from_slot(slot: Slot) -> Self {
            MockOutput {
                start_at: slot.start(),
                end_at: slot.end(),
            }
        }
    }

    #[test]
    fn test_find() {
        let now = Utc::now().with_timezone(&chrono_tz::Japan);

        // Test cases
        struct TestCase {
            description: &'static str,
            span: Span,
            inputs: Vec<MockInput>,
            expected_slots: Vec<MockOutput>,
            should_error: bool,
        }

        let test_cases = vec![
            TestCase {
                description: "No blocks",
                inputs: vec![],
                span: Span::new(now + Duration::hours(0), now + Duration::hours(8)).unwrap(),
                expected_slots: vec![MockOutput::new(now, 0, 8)],
                should_error: false,
            },
            TestCase {
                description: "One block before slot",
                inputs: vec![MockInput::new(now, -2, -1)],
                span: Span::new(now + Duration::hours(0), now + Duration::hours(8)).unwrap(),
                expected_slots: vec![MockOutput::new(now, 0, 8)],
                should_error: false,
            },
            TestCase {
                description: "One block before slot boundary",
                inputs: vec![MockInput::new(now, -1, 0)],
                span: Span::new(now + Duration::hours(0), now + Duration::hours(8)).unwrap(),
                expected_slots: vec![MockOutput::new(now, 0, 8)],
                should_error: false,
            },
            TestCase {
                description: "One block with overlap at start",
                inputs: vec![MockInput::new(now, -1, 0)],
                span: Span::new(now + Duration::hours(0), now + Duration::hours(8)).unwrap(),
                expected_slots: vec![MockOutput::new(now, 0, 8)],
                should_error: false,
            },
            TestCase {
                description: "One block with overlap at start boundary",
                inputs: vec![MockInput::new(now, 0, 1)],
                span: Span::new(now + Duration::hours(0), now + Duration::hours(8)).unwrap(),
                expected_slots: vec![MockOutput::new(now, 1, 8)],
                should_error: false,
            },
            TestCase {
                description: "One block is contained in slot",
                inputs: vec![MockInput::new(now, 1, 5)],
                span: Span::new(now + Duration::hours(0), now + Duration::hours(8)).unwrap(),
                expected_slots: vec![MockOutput::new(now, 0, 1), MockOutput::new(now, 5, 8)],
                should_error: false,
            },
            TestCase {
                description:
                    "One block is contained in slot boundary (= One block contains slot boundary)",
                inputs: vec![MockInput::new(now, 0, 8)],
                span: Span::new(now + Duration::hours(0), now + Duration::hours(8)).unwrap(),
                expected_slots: vec![],
                should_error: false,
            },
            TestCase {
                description: "One block contains slot",
                inputs: vec![MockInput::new(now, -1, 9)],
                span: Span::new(now + Duration::hours(0), now + Duration::hours(8)).unwrap(),
                expected_slots: vec![],
                should_error: false,
            },
            TestCase {
                description: "One block with overlap at end boundary",
                inputs: vec![MockInput::new(now, 3, 8)],
                span: Span::new(now + Duration::hours(0), now + Duration::hours(8)).unwrap(),
                expected_slots: vec![MockOutput::new(now, 0, 3)],
                should_error: false,
            },
            TestCase {
                description: "One block with overlap at end",
                inputs: vec![MockInput::new(now, 3, 9)],
                span: Span::new(now + Duration::hours(0), now + Duration::hours(8)).unwrap(),
                expected_slots: vec![MockOutput::new(now, 0, 3)],
                should_error: false,
            },
            TestCase {
                description: "One block after slot boundary",
                inputs: vec![MockInput::new(now, 8, 10)],
                span: Span::new(now + Duration::hours(0), now + Duration::hours(8)).unwrap(),
                expected_slots: vec![MockOutput::new(now, 0, 8)],
                should_error: false,
            },
            TestCase {
                description: "One block after slot",
                inputs: vec![MockInput::new(now, 9, 10)],
                span: Span::new(now + Duration::hours(0), now + Duration::hours(8)).unwrap(),
                expected_slots: vec![MockOutput::new(now, 0, 8)],
                should_error: false,
            },
            TestCase {
                description: "Two blocks are contained in slot",
                inputs: vec![MockInput::new(now, 1, 2), MockInput::new(now, 6, 7)],
                span: Span::new(now + Duration::hours(0), now + Duration::hours(8)).unwrap(),
                expected_slots: vec![
                    MockOutput::new(now, 0, 1),
                    MockOutput::new(now, 2, 6),
                    MockOutput::new(now, 7, 8),
                ],
                should_error: false,
            },
            TestCase {
                description: "Two blocks overlap each other and are contained in slot",
                inputs: vec![MockInput::new(now, 1, 4), MockInput::new(now, 2, 5)],
                span: Span::new(now + Duration::hours(0), now + Duration::hours(8)).unwrap(),
                expected_slots: vec![MockOutput::new(now, 0, 1), MockOutput::new(now, 5, 8)],
                should_error: false,
            },
        ];

        // Iterate through each test case
        for case in test_cases {
            let result: Result<Vec<MockOutput>, SlotError> =
                find(case.span.clone(), case.inputs.clone());
            match result {
                Ok(slots) => {
                    assert!(!case.should_error, "{}", case.description);
                    assert_eq!(
                        slots.len(),
                        case.expected_slots.len(),
                        "{}",
                        case.description
                    );
                    for (actual, expected) in slots.iter().zip(case.expected_slots.iter()) {
                        assert_eq!(actual.start(), expected.start(), "{}", case.description);
                        assert_eq!(actual.end(), expected.end(), "{}", case.description);
                    }
                }
                Err(_) => {
                    assert!(case.should_error, "{}", case.description);
                }
            }
        }
    }
}
