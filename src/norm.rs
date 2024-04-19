extern crate test;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn par_norm(v: &[f32]) -> f32 {
    let sum: f32 = v.par_iter().map(|&x| x * x).sum();
    sum.sqrt()
}

pub fn norm(v: &[f32]) -> f32 {
    let sum: f32 = v.iter().map(|&x| x * x).sum();
    sum.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    #[bench]
    fn bench_par_norm(b: &mut Bencher) {
        let data = vec![20.0; 2000000];
        b.iter(|| par_norm(&data));
    }
    #[bench]
    fn bench_norm_slice(b: &mut Bencher) {
        let data = vec![20.0; 2000000];
        b.iter(|| norm(&data));
    }
}
