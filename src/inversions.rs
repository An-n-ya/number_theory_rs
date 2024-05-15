pub fn calc_inversions(vec: &[usize]) -> usize {
    calc_inversions_naive(vec)
}

fn calc_inversions_naive(vec: &[usize]) -> usize {
    let mut res = 0;
    let len = vec.len();

    for i in 0..len {
        for j in 0..i {
            if vec[j] > vec[i] {
                res += 1;
            }
        }
    }

    res
}

fn calc_inversions_quick_sort(vec: &[usize]) -> usize {
    fn partition_count(vec: &mut [usize], low: usize, high: usize) -> (usize, usize) {
        let pivot = vec[low];
        let ind_len = high - low + 1;
        let mut i = low;
        let mut count = 0;
        let mut small_ind = Vec::with_capacity(ind_len);
        let mut big_ind = Vec::with_capacity(ind_len);
        let old_v = vec.to_vec();
        for j in low + 1..=high {
            // println!("j:{j}, vec[j]:{}, pivot:{}", vec[j], pivot);
            if vec[j] < pivot {
                count += j - i;
                small_ind.push(j);
                i += 1;
            } else {
                big_ind.push(j);
            }
        }
        let small_ind_len = small_ind.len();
        let big_ind_len = big_ind.len();
        let pivot_pos = small_ind_len + low;
        for j in 0..small_ind_len {
            vec[j + low] = old_v[small_ind[j]];
        }
        vec[pivot_pos] = pivot;
        for j in 0..big_ind_len {
            vec[j + pivot_pos + 1] = old_v[big_ind[j]];
        }
        // let mut vv = vec![];
        // for n in &v {
        //     vv.push(vec[*n]);
        // }
        // vv.push(pivot);
        // let mut j = low + 1;
        // let mut k = 0;
        // while j <= high {
        //     while k < v.len() && v[k] == j {
        //         k += 1;
        //         j += 1;
        //     }
        //     if j > high {
        //         break;
        //     }
        //     vv.push(vec[j]);
        //     j += 1;
        // }

        // for j in low..=high {
        //     vec[j] = vv[j - low];
        // }

        // println!("vec:{:?}, low: {low}, high: {high}", vec);

        (i, count)
    }
    fn quick_sort_count(vec: &mut [usize], low: usize, high: usize) -> usize {
        let mut res = 0;
        if low < high {
            let (pos, count) = partition_count(vec, low, high);
            res += count;
            if pos > low + 1 {
                res += quick_sort_count(vec, low, pos - 1);
            }
            if high > pos + 1 {
                res += quick_sort_count(vec, pos + 1, high);
            }
        }
        res
    }

    let mut v = vec.to_vec();
    let len = vec.len();
    quick_sort_count(&mut v, 0, len - 1)
}

#[cfg(test)]
mod tests {
    use crate::permutation::even_permutation;

    use super::*;

    #[test]
    fn test_quick_sort_inversions() {
        let vecs = [[1, 2, 3], [3, 2, 1], [2, 1, 3]];
        for v in vecs {
            assert_eq!(calc_inversions_naive(&v), calc_inversions_quick_sort(&v));
        }
    }
    #[test]
    fn test_quick_sort_inversions2() {
        let vecs = even_permutation(5);
        for v in vecs {
            assert_eq!(
                calc_inversions_naive(&v),
                calc_inversions_quick_sort(&v),
                "failed {:?}",
                v
            );
        }
    }
}
