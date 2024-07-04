pub enum NestedArray<T> {
    Value(T),
    Array(Vec<NestedArray<T>>),
}

pub fn nested_add(arr: &Vec<NestedArray<usize>>) -> usize {
    let mut result = 0;

    for item in arr {
        match item {
            NestedArray::Value(v) => result += v,
            NestedArray::Array(a) => result += nested_add(a),
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nested_add() {
        let nums1: Vec<NestedArray<usize>> = Vec::new();
        let nums2 = Vec::from([
            NestedArray::Value(1),
            NestedArray::Value(2),
            NestedArray::Value(3),
        ]);
        let nums3 = Vec::from([
            NestedArray::Value(1),
            NestedArray::Value(2),
            NestedArray::Value(3),
            NestedArray::Array(Vec::from([
                NestedArray::Value(4),
                NestedArray::Array(Vec::from([NestedArray::Value(5), NestedArray::Value(6)])),
            ])),
        ]);

        assert_eq!(nested_add(&nums1), 0);
        assert_eq!(nested_add(&nums2), 6);
        assert_eq!(nested_add(&nums3), 21);
    }
}
