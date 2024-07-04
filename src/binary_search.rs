pub fn binary_search(arr: &[isize], num: isize) -> Option<usize> {
    let mut min = 0;
    let mut max = arr.len() - 1;
    let mut idx;

    while min <= max {
        idx = (min + max) / 2;

        let item = arr[idx];

        if item == num {
            return Some(idx);
        } else if item > num {
            max = idx - 1;
        } else {
            min = idx + 1;
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        assert_eq!(binary_search(&Vec::from([1]), 1), Some(0));
        assert_eq!(binary_search(&Vec::from([3, 2, 1]), 2), Some(1));
        assert_eq!(binary_search(&Vec::from([1, 5, 7, 4, 2, 3, 6]), 6), Some(6));
        assert_eq!(binary_search(&Vec::from([1, 3, 4]), 2), None);
        assert_eq!(binary_search(&Vec::from([1, 3, 4, 19]), 555), None);
    }
}
