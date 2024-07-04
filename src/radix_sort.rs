fn nth_digit(number: usize, position: usize) -> u8 {
    return ((number / position as usize) % 10) as u8;
}

fn get_longest_number_digits(arr: &[usize]) -> usize {
    return arr
        .iter()
        .map(|num| num.to_string().len())
        .max()
        .unwrap_or(0);
}

pub fn radix_sort(arr: &[usize]) -> Vec<usize> {
    let mut position = 1;
    let mut result = arr.to_vec();
    let mut longest_number_digits = get_longest_number_digits(&arr);
    let mut buckets: Vec<Vec<usize>> = vec![Vec::with_capacity(arr.len()); 10];

    while longest_number_digits > 0 {
        while !result.is_empty() {
            let number = result.remove(0);
            let digit = nth_digit(number, position);

            buckets[digit as usize].push(number);
        }

        for bucket in &mut buckets {
            while !bucket.is_empty() {
                let number = bucket.remove(0);

                result.push(number);
            }
        }

        position *= 10;
        longest_number_digits -= 1;
    }

    return result.to_vec();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_digit() {
        assert_eq!(nth_digit(1, 1), 1);
        assert_eq!(nth_digit(123, 10), 2);
        assert_eq!(nth_digit(123, 100), 1);
        assert_eq!(nth_digit(123, 1000), 0);
    }

    #[test]
    fn test_get_longest_number_digits() {
        assert_eq!(get_longest_number_digits(&[1, 0, 1]), 1);
        assert_eq!(get_longest_number_digits(&[123, 456, 9999]), 4);
    }

    #[test]
    fn test_radix_sort() {
        assert_eq!(radix_sort(&Vec::from([1])), [1]);
        assert_eq!(radix_sort(&Vec::from([3, 2, 1])), [1, 2, 3]);
        assert_eq!(
            radix_sort(&Vec::from([
              20,
              51,
              3,
              801,
              415,
              62,
              4,
              17,
              19,
              11,
              1,
              100,
              1244,
              104,
              944,
              854,
              34,
              3000,
              3001,
              1200,
              633
            ])),
            [
              1,
              3,
              4,
              11,
              17,
              19,
              20,
              34,
              51,
              62,
              100,
              104,
              415,
              633,
              801,
              854,
              944,
              1200,
              1244,
              3000,
              3001
            ]
        );
    }
}
