use std::collections::HashMap;

pub struct ArrayList {
    data: HashMap<usize, usize>,
    length: usize,
}

impl ArrayList {
    pub fn new() -> Self {
        return ArrayList {
            data: HashMap::new(),
            length: 0,
        };
    }

    pub fn push(&mut self, value: usize) {
        self.data.insert(self.length, value);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<usize> {
        return self.delete(self.length - 1);
    }

    pub fn get(&mut self, index: usize) -> Option<&usize> {
        return self.data.get(&index);
    }

    pub fn delete(&mut self, index: usize) -> Option<usize> {
        let value = self.data.remove(&index);

        let iter = self.data.clone().into_iter().skip(index);

        for (key, value) in iter {
            self.data.insert(key - 1, value);
        }

        self.data.remove(&(self.length - 1));
        self.length -= 1;

        return value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_list() {
        let mut array_list = ArrayList::new();

        array_list.push(1);
        array_list.push(2);
        array_list.push(3);
        array_list.push(4);
        array_list.push(5);
        assert_eq!(array_list.get(0), Some(&1));
        assert_eq!(array_list.get(1), Some(&2));
        assert_eq!(array_list.get(4), Some(&5));

        array_list.pop();
        assert_eq!(array_list.get(4), None);

        array_list.delete(0);
        assert_eq!(array_list.get(0), Some(&2));
        assert_eq!(array_list.get(1), Some(&3));
        assert_eq!(array_list.get(2), Some(&4));
        assert_eq!(array_list.get(3), None);
    }
}
