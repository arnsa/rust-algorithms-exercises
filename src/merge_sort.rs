pub fn merge<T>(left: &[T], right: &[T]) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    let mut i = 0;
    let mut j = 0;
    let mut result = Vec::with_capacity(left.len() + right.len());

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }

    if i < left.len() {
        result.extend(&left[i..]);
    } else if j < right.len() {
        result.extend(&right[j..]);
    }

    return result;
}

pub fn merge_sort<T>(arr: &[T]) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    if arr.len() <= 1 {
        return arr.to_vec();
    }

    let mid = arr.len() / 2;
    let left = merge_sort(&arr[..mid]);
    let right = merge_sort(&arr[mid..]);

    return merge(&left, &right);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        assert_eq!(merge(&[1], &[5]), [1, 5]);
        assert_eq!(merge(&[7], &[4]), [4, 7]);
        assert_eq!(
            merge(&[1, 4, 5, 7], &[2, 3, 6]),
            [1, 2, 3, 4, 5, 6, 7]
        );
    }

    #[test]
    fn test_merge_sort() {
        assert_eq!(merge_sort(&Vec::from([1])), [1]);
        assert_eq!(merge_sort(&Vec::from([3, 2, 1])), [1, 2, 3]);
        assert_eq!(merge_sort(&Vec::from([1, 5, 7, 4, 2, 3, 6])), [1, 2, 3, 4, 5, 6, 7]);
    }
}
