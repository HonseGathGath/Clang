use std::{ops::Add, thread};

pub fn concurrent_add<T>(items: Vec<T>, num: T) -> Vec<thread::JoinHandle<T>>
where
    T: Send + 'static + Copy + Add<Output = T>,
{
    let mut thread_vec: Vec<thread::JoinHandle<T>> = Vec::new();

    items.into_iter().for_each(|item| {
        let handle: thread::JoinHandle<T> = thread::spawn(move || item + num);
        thread_vec.push(handle);
    });

    thread_vec
}

// Example Usage
pub fn main() {
    {
        let mut list = vec![1, 2, 3, 4, 5];

        let handles = concurrent_add(list.clone(), 3);

        for handle in handles {
            let result = handle.join().unwrap();
            let original = list.remove(0);

            assert_eq!(result, original + 3);
        }
    }

    {
        let mut list = vec![10., 20., 30., 40., 50.];

        let handles = concurrent_add(list.clone(), 5.);

        for handle in handles {
            let result = handle.join().unwrap();
            let original = list.remove(0);

            assert_eq!(result, original + 5.);
        }
    }
}
