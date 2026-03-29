use std::collections::{ HashMap, HashSet };
use std::sync::{ Arc, Mutex };
use std::thread;

fn cached_squares(inputs: Vec<i32>) -> Vec<i32> {
    let cache = Arc::new(Mutex::new(HashMap::new()));

    let mut handles = vec![];
    // unique find krne vaste assi hun hashset ka use krange

    let set: HashSet<i32> = inputs.iter().cloned().collect();
    for x in set {
        let cache = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            let mut map = cache.lock().unwrap();
            map.insert(x, x * x);
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    // new vecror create
    let map = cache.lock().unwrap();

    let mut result = vec![];
    for x in inputs {
        let item = map.get(&x).unwrap();
        result.push(*item);
    }
    result
}

fn main() {
    let inputs = vec![1, 2, 3];
    cached_squares(inputs);
}
