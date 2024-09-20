# ChronoSlots

Find available time slots in Rust.

Searching for “available time” is done in various situations.

-	Check the “available time” of your work schedule.
-	Check the “available time” of the restaurant you want to use for a meal.
-	Check the “available time” of a hotel room you want to stay at during your trip.
-	Check the “available time” of a car-sharing service you want to use on your day off.

If your application has information about “scheduled events” that are already booked, this library will provide “available time” easily!

## Quick Start

// TODO

## Note

- Do not mix schedules (Blocks) held by different entities. (You should know a smarter way to handle this.)
- In the implementation of the Period, ensure that the start time is always before the end time. (Be mindful of cases where this may inadvertently happen.)

## Specification

- It is acceptable for different schedules (Blocks) to overlap.