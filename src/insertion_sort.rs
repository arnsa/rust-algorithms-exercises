pub fn insertion_sort<T>(arr: &mut Vec<T>)
where
    T: PartialOrd,
{
    for index in 1..arr.len() {
        let mut j = index;

        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut nums1: Vec<usize> = Vec::new();
        let mut nums2 = Vec::from([10, 5, 3, 8, 2, 6, 4, 7, 9, 1]);
        let mut nums3 = Vec::from([5, 4, 3, 2, 1]);
        let mut chars = Vec::from(['g', 'd', 'a', 'b', 'f']);

        insertion_sort(&mut nums1);
        insertion_sort(&mut nums2);
        insertion_sort(&mut nums3);
        insertion_sort(&mut chars);

        assert_eq!(nums1, Vec::new());
        assert_eq!(nums2, Vec::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
        assert_eq!(nums3, Vec::from([1, 2, 3, 4, 5]));
        assert_eq!(chars, Vec::from(['a', 'b', 'd', 'f', 'g']));
    }
}
