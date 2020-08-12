# Dekker

This crate provides an implementation of Dekker's algorithm. Dekker's algorithm is the first known correct solution to the mutual exclusion problem in concurrent programming.

## Example

```rust
use dekker::Dekker;
use std::thread;

fn main() {
    // Create two process handles.
    let (mut p1, mut p2) = Dekker::new(0);

    // Create a new thread and move one handle.
    let other = thread::spawn(move || {
        // Increment by one five times.
        for _ in 0..5 {
            println!("Incrementing in secondary thread.");
            *p2.lock() += 1;
        }
    });

    // Increment by one another five times.
    for _ in 0..5 {
        println!("Incrementing in main thread.");
        *p1.lock() += 1;
    }

    // Join the threads.
    other.join().unwrap();

    println!("The counter is now at {}.", *p1.lock());
}
```

A possible output looks like this:

```text
Incrementing in main thread.
Incrementing in secondary thread.
Incrementing in secondary thread.
Incrementing in main thread.
Incrementing in secondary thread.
Incrementing in main thread.
Incrementing in main thread.
Incrementing in secondary thread.
Incrementing in secondary thread.
Incrementing in main thread.
The counter is now at 10.
```

In the end, the counter will always be at 10.
