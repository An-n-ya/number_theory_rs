use crate::{inversions::calc_inversions, permutation::even_permutation};

pub fn determination(mat: Vec<Vec<i32>>) -> i32 {
    let n = mat.len();
    assert_eq!(n, mat[0].len());

    let mut res = 0;
    for p in even_permutation(n) {
        let mut tmp = 1;
        for i in 0..n {
            tmp *= mat[i][p[i] - 1];
        }
        if calc_inversions(&p) % 2 == 1 {
            tmp *= -1;
        }
        res += tmp;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determination() {
        let mat = vec![vec![1, 2], vec![3, 4]];
        let res = determination(mat);
        assert_eq!(res, -2);

        let mat = vec![vec![1, 2, 5], vec![-2, 3, 4], vec![-5, -4, 9]];
        let res = determination(mat);
        assert_eq!(res, 154);
    }
}
