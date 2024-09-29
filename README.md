# ChronoSlots

Find available time slots in Rust.

Searching for “available time” is done in various situations.

-	Check the “available time” of your work schedule.
-	Check the “available time” of the restaurant you want to use for a meal.
-	Check the “available time” of a hotel room you want to stay at during your trip.
-	Check the “available time” of a car-sharing service you want to use on your day off.

If your application has information about “scheduled events” that are already booked, this library will provide “available time” easily!

## Quick Start

```rust
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
    fn to_block(&self) -> Block {
        Block::new(self.start_at, self.end_at).unwrap()
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
```

## Note

- Do not mix schedules (Blocks) held by different entities. (You should know a smarter way to handle this.)
- In the implementation of the Period, ensure that the start time is always before the end time. (Be mindful of cases where this may inadvertently happen.)

## Specification

- It is acceptable for different schedules (Blocks) to overlap.