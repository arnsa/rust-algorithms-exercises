pub fn bubble_sort<T>(arr: &mut Vec<T>)
where
    T: PartialOrd
{
    if arr.is_empty() {
        return;
    }

    let mut len = arr.len();

    loop {
        let mut has_swapped = false;

        for index in 0..(len - 1) {
            if arr[index] > arr[index + 1] {
                arr.swap(index, index + 1);
                has_swapped = true;
            }
        }

        if !has_swapped {
            break;
        }

        len -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut nums1: Vec<usize> = Vec::new();
        let mut nums2 = Vec::from([10, 5, 3, 8, 2, 6, 4, 7, 9, 1]);
        let mut nums3 = Vec::from([5, 4, 3, 2, 1]);
        let mut chars = Vec::from(['g', 'd', 'a', 'b', 'f']);

        bubble_sort(&mut nums1);
        bubble_sort(&mut nums2);
        bubble_sort(&mut nums3);
        bubble_sort(&mut chars);

        assert_eq!(nums1, Vec::new());
        assert_eq!(nums2, Vec::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
        assert_eq!(nums3, Vec::from([1, 2, 3, 4, 5]));
        assert_eq!(chars, Vec::from(['a', 'b', 'd', 'f', 'g']));
    }
}
