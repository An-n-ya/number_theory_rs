#![feature(test)]
pub mod max;

pub mod norm;

pub mod permutation;

pub mod inversions;

pub mod determination;

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    #[derive(Clone)]
    struct UnsafeMutator {
        value: *mut u8,
    }

    impl UnsafeMutator {
        fn new(value: &mut Vec<u8>) -> UnsafeMutator {
            return UnsafeMutator {
                value: value.as_mut_ptr(),
            };
        }

        fn nth(&self, index: usize) -> u8 {
            return unsafe { *self.value.add(index) };
        }
        fn mut_value(&self, index: usize, value: u8) {
            unsafe {
                *self.value.add(index) = value;
            };
        }
    }

    unsafe impl Send for UnsafeMutator {}

    #[test]
    fn test_parallel() {
        use std::thread;
        // 创建一个包含数组的 UnsafeSend
        let mut arr = vec![1, 2, 3, 4, 5];
        let array = UnsafeMutator::new(&mut arr);

        // 创建多个线程来访问数组的不同索引数据并修改
        let mut handles = vec![];
        for _ in 0..5 {
            let array = array.clone();
            let handle = thread::spawn(move || {
                sleep(Duration::from_millis(rand::random::<u64>() % 200));
                let v = array.nth(0);
                sleep(Duration::from_millis(rand::random::<u64>() % 200));
                array.mut_value(0, v + 1);
                // println!("Thread {:?} accessed index {}: {}", thread_id, i, *a);
                // *a += 1;
            });
            handles.push(handle);
        }

        // 等待所有线程结束
        for handle in handles {
            handle.join().unwrap();
        }

        // 输出最终的数组
        print!("[");
        for i in 0..5 {
            print!("{}, ", array.nth(i))
        }
        print!("]");

        panic!()
    }
}
