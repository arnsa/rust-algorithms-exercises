pub fn quick_sort<T>(arr: &[T]) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    if arr.len() <= 1 {
        return arr.to_vec();
    }

    let pivot = arr[arr.len() - 1];
    let mut left = Vec::new();
    let mut right = Vec::new();

    for i in 0..arr.len() - 1 {
        if arr[i] < pivot {
            left.push(arr[i]);
        } else {
            right.push(arr[i]);
        }
    }

    let mut result = Vec::from(quick_sort(&left));

    result.push(pivot);
    result.extend(quick_sort(&right));

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        assert_eq!(quick_sort(&Vec::from([1])), [1]);
        assert_eq!(quick_sort(&Vec::from([3, 2, 1])), [1, 2, 3]);
        assert_eq!(
            quick_sort(&Vec::from([1, 5, 7, 4, 6, 2, 3, 6])),
            [1, 2, 3, 4, 5, 6, 6, 7]
        );
    }
}
