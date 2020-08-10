use super::*;
use std::thread;

#[test]
fn basic() {
    let (mut p1, mut p2) = Dekker::new(0);
    *p1.lock() += 1;
    *p2.lock() += 1;
    assert_eq!(*p1.lock(), 2);
}

#[test]
fn count() {
    let (mut p1, mut p2) = Dekker::new(0);

    let other = thread::spawn(move || {
        for _ in 0..1000 {
            *p2.lock() += 1;
        }
    });

    for _ in 0..1000 {
        *p1.lock() += 1;
    }

    other.join().unwrap();

    assert_eq!(*p1.lock(), 2000);
}