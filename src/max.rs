extern crate test;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn max(arr: &[usize]) -> usize {
    arr.par_iter()
        .cloned()
        .reduce(|| 0, |a, b| if a > b { a } else { b })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};
    use test::Bencher;
    #[bench]
    fn bench_seq_max(b: &mut Bencher) {
        let mut arr = vec![0usize; 300_0000];
        thread_rng().fill(&mut arr[..]);
        b.iter(|| *arr.iter().max().unwrap());
    }
    #[bench]
    fn bench_par_max(b: &mut Bencher) {
        let mut arr = vec![0usize; 300_0000];
        thread_rng().fill(&mut arr[..]);
        b.iter(|| max(&arr));
    }

    #[test]
    fn test_max() {
        let input = [1, 2, 4, 1, 3, 0, 9];
        let max_value = max(&input);
        assert_eq!(max_value, 9);
    }
}
