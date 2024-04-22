#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
}
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Number {
    n: usize,
    direction: Direction,
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.n.partial_cmp(&other.n)
    }
}

impl Number {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            direction: Direction::Left,
        }
    }
    pub fn switch_direction(&mut self) {
        self.direction = match self.direction {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
pub fn even_permutation(n: usize) -> Vec<Vec<usize>> {
    if n == 1 {
        return vec![vec![1]];
    }
    let mut arr = Vec::with_capacity(n);
    for i in 1..=n {
        arr.push(Number::new(i));
    }
    let count = calc_permutation_count(n);
    let mut res = Vec::with_capacity(count);
    push_permutation_to_vec(&mut res, &arr);
    for _ in 0..count - 1 {
        let (max_index, neigh) = find_max_moveable_index(&arr);
        let max_node = arr[max_index];
        (arr[max_index], arr[neigh]) = (arr[neigh], arr[max_index]);
        change_direction_bigger_than(&mut arr, &max_node);
        push_permutation_to_vec(&mut res, &arr);
    }
    res
}

fn change_direction_bigger_than(arr: &mut Vec<Number>, n: &Number) {
    for num in arr {
        if *num > *n {
            num.switch_direction();
        }
    }
}

fn find_max_moveable_index(arr: &Vec<Number>) -> (usize, usize) {
    let size = arr.len();
    let mut max_index = None;
    let mut neigh = None;
    for i in 0..size {
        if i == 0 && arr[i].direction == Direction::Left
            || i == size - 1 && arr[i].direction == Direction::Right
        {
            // cannot move
            continue;
        }
        let cur_neigh = if arr[i].direction == Direction::Left {
            i - 1
        } else {
            i + 1
        };
        if arr[i] > arr[cur_neigh] {
            if max_index.is_none() || arr[i] > arr[max_index.unwrap()] {
                max_index = Some(i);
                neigh = Some(cur_neigh);
            }
        }
    }
    (max_index.unwrap(), neigh.unwrap())
}

fn push_permutation_to_vec(vec: &mut Vec<Vec<usize>>, arr: &Vec<Number>) {
    let mut tmp = Vec::with_capacity(arr.len());
    for n in arr {
        tmp.push(n.n);
    }
    vec.push(tmp);
}

pub fn calc_permutation_count(n: usize) -> usize {
    let mut res = 1;
    for i in 2..=n {
        res *= i;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_even_permutation() {
        for n in 1..10 {
            let res = even_permutation(n);
            let cnt = calc_permutation_count(n);
            let mut set = HashSet::new();
            for p in res {
                set.insert(p);
            }
            assert_eq!(set.len(), cnt);
        }
    }
}
