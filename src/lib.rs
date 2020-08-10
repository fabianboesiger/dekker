use std::ops::{Deref, DerefMut};

#[cfg(test)]
mod tests;

/// Provides mutually exclusive access to a shared object using Dekker's algorithm.
pub struct Dekker<T> {
    shared: T,
    turn: usize,
    wants_to_enter: [bool; 2],
    processes: u8,
}

impl<T> Dekker<T> {
    /// Allocates a new shared object and returns two processes to access it.
    pub fn new(shared: T) -> (Process<T>, Process<T>) {
        let dekker = Dekker {
            shared,
            turn: 0,
            wants_to_enter: [false; 2],
            processes: 2,
        };

        let dekker = Box::leak(Box::new(dekker));

        (Process::new(0, dekker), Process::new(1, dekker))
    }
}

/// Used to access the critical section from one of two concurrent processes.
pub struct Process<T> {
    index: usize,
    owner: *mut Dekker<T>,
    locked: bool,
}

unsafe impl<T> Send for Process<T> {}

impl<T> Process<T> {
    fn new(index: usize, owner: *mut Dekker<T>) -> Process<T> {
        Process {
            index,
            owner,
            locked: false,
        }
    }

    /// Acquire the lock for the critical section.
    pub fn lock(&mut self) -> Guard<T> {
        unsafe {
            (*self.owner).wants_to_enter[self.index] = true;
            while (*self.owner).wants_to_enter[1 - self.index] {
                if (*self.owner).turn != self.index {
                    (*self.owner).wants_to_enter[self.index] = false;
                    while (*self.owner).turn != self.index {}
                    (*self.owner).wants_to_enter[self.index] = true;
                }
            }
        }

        self.locked = true;

        Guard::new(self as *mut Process<T>)
    }
}

impl<T> Drop for Process<T> {
    // We can deallocate if both processes are finished.
    fn drop(&mut self) {
        let guard = self.lock();

        unsafe {
            (*self.owner).processes -= 1;
            if (*self.owner).processes == 0 {
                let dekker = Box::from_raw(self.owner);
                drop(dekker);
            }
        }

        drop(guard);
    }
}

/// Holds the lock until dropped.
pub struct Guard<T> {
    owner: *mut Process<T>,
}

impl<T> Guard<T> {
    fn new(owner: *mut Process<T>) -> Guard<T> {
        Guard {
            owner,
        }
    }
}

impl<T> Deref for Guard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &(*(*self.owner).owner).shared
        }
    }
}

impl<T> DerefMut for Guard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut (*(*self.owner).owner).shared
        }
    }
}

impl<T> Drop for Guard<T> {
    // Let other process enter the critical section if guard is dropped.
    fn drop(&mut self) {
        unsafe {
            (*self.owner).locked = false;
            (*(*self.owner).owner).turn = 1 - (*self.owner).index;
            (*(*self.owner).owner).wants_to_enter[(*self.owner).index] = false;
        }
    }
}