use chrono::{DateTime, Duration, Utc};
use chrono_slots::{find, Block, Input, Output, Period, PeriodError, PeriodVec, Slot, Span};
use chrono_tz::Tz;

// Your struct
struct ScheduledEvent {
    start_at: DateTime<Tz>,
    end_at: DateTime<Tz>,
}

// Please implement this methods in your struct to satisfy the Period interface.
impl Period for ScheduledEvent {
    fn start(&self) -> DateTime<Tz> {
        self.start_at
    }

    fn end(&self) -> DateTime<Tz> {
        self.end_at
    }
}

// To convert internally, define the map function for your output
impl Input for ScheduledEvent {
    fn to_block(&self) -> Result<Block, PeriodError> {
        Block::new(self.start_at, self.end_at)
    }
}

// Your struct
struct AvailableSlot {
    start_at: DateTime<Tz>,
    end_at: DateTime<Tz>,
}

// Please implement this methods in your struct to satisfy the Period interface.
impl Period for AvailableSlot {
    fn start(&self) -> DateTime<Tz> {
        self.start_at
    }

    fn end(&self) -> DateTime<Tz> {
        self.end_at
    }
}

// To convert internally, define the map function for your output
impl Output for AvailableSlot {
    fn create_from_slot(slot: Slot) -> Self {
        AvailableSlot {
            start_at: slot.start(),
            end_at: slot.end(),
        }
    }
}

fn main() {
    let now = Utc::now().with_timezone(&chrono_tz::Japan);

    // This variable will probably be retrieved from something like a request. Since this is an example, we’ll create it artificially.
    let span = Span::new(now + Duration::hours(0), now + Duration::hours(8)).unwrap();
    println!("Span:\n {}\n", span.to_string());

    // This variable will probably be retrieved from something like a database record. Since this is an example, we’ll create it artificially.
    let events = vec![
        ScheduledEvent {
            start_at: now + Duration::hours(1),
            end_at: now + Duration::hours(2),
        },
        ScheduledEvent {
            start_at: now + Duration::hours(3),
            end_at: now + Duration::hours(4),
        },
    ];
    println!("Blocks:\n {}\n", events.to_string());

    // Find available time slots!
    let slots: Vec<AvailableSlot> = find(span, events).unwrap();
    println!("Slots:\n {}\n", slots.to_string());
}
