use std::cell::UnsafeCell;
// core primitive of interior mutability in rust
/*
    At the core of all types that provide interior mutability
    there is unsafe cell
    It holds some type .. you can raw exclusive pointer to it 
    it's upto you to cast that raw exclusive pointer into an exclusive rust 
    ref when you know that its safe to do so.

    Cells - Used for smaller values or flags that need to be mutated from
    multiple different places
*/

pub struct Cell<T> {
    value: UnsafeCell<T>,
}
// As Unsafe does not implement Sync .. it can not be shared acorss the threads
// To make it work .. we can either do Arc

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell { value: UnsafeCell::new(value) }
    }

    // means the &self(Cell) is immutable
    pub fn set(&self, value: T) {
        unsafe {
            // .get() gets a mutable pointer to the wrapped value
            *self.value.get() = value;
        }
    }

    pub fn get(&self) -> T where T: Copy {
        //SAFETY: we know no one else is modifying the value at this time,
        // since only one thread can mutate
        unsafe {
            // I believe we are returning the value because
            // self.value.get() gives *mut T and then **mut T gives T
            // and as line 30 has specified a copy bound
            *self.value.get()
        } // self.value
    }
}

// impl<T> !Sync for Cell<T> {} = implied by unsafe cell

#[cfg(test)]
mod tests {
    use super::Cell;

    #[test]
    fn simple_test() {
        let c = Cell::new(0);
        // I can have this now
        let d = &c;
        // can I do
        let derefed = d.get();
        println!("Before modifying {}", derefed);
        d.set(40);
        println!("After modifying {}", derefed);

        /*
            IF YOU NEED THE LATEST VALUE IN THE CELL , THEN ASK AGAIN FOR
            A FRESH COPY OF THE VALUE.
            - So, its better to not store it in a variable .. just directly call
            the get method
         */

        assert_eq!(c.get(), derefed)
    }
}
