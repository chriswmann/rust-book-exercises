use std::collections::{HashMap, HashSet};


#[allow(dead_code)]
fn mean<'a, T>(data: &'a [T]) -> Option<T> 
where T: core::iter::Sum<&'a T> + std::ops::Div<usize, Output = T>
{
    let sum = data.iter().sum::<T>();
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count),
        _ => None
    }
}

#[allow(dead_code)]
fn median(list: &mut [u32]) -> Option<f64> {
    if list.len() == 0 {
        return None
    }
    list.sort();
    let len = list.len();
    let mid = len / 2;
    if len % 2 == 0 {
        Some((list.get(mid - 1).unwrap().to_owned() as f64 + list.get(mid).unwrap().to_owned() as f64)
            / 2.0)
    } else {
        Some(list.get(mid).unwrap().to_owned() as f64)
    }
}

#[allow(dead_code)]
fn mode(list: &[u32]) -> HashSet<u32> {
    let mut mode: HashSet<u32> = HashSet::new();
    if list.len() == 0 {
        return mode
    }
    let counts: HashMap<u32, u32> = list.iter().fold(HashMap::new(), |mut acc, &x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });
    let mut max_count = 0;
    for (key, _) in counts.iter() {
        if *key > max_count {
           max_count = *key; 
        }
    }
    for (key, value) in counts.iter() {
        if *key == max_count {
            mode.insert(*value);
        }
    }
    mode


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn median_with_even_number_of_items() {
        let mut list = vec![1, 2, 3, 4];
        let result = median(&mut list);
        assert_eq!(result, Some(2.5));
    }

    #[test]
    fn median_with_odd_number_of_items() {
        let mut list = vec![2, 3, 4];
        let result = median(&mut list);
        assert_eq!(result, Some(3.0));
    }

    #[test]
    fn median_with_empty_list() {
        let mut list = vec![];
        let result = median(&mut list);
        assert_eq!(result, None);
    }

    #[test]
    fn mode_with_items() {
        let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 1, 1, 1, 1];
        let result = mode(&list);
        let mut expected = HashSet::new();
        expected.insert(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn mode_with_empty_list() {
        let list = vec![];
        let result = mode(&list);
        let expected = HashSet::new();
        assert_eq!(result, expected);
    }

    #[test]
    fn mean_with_items() {
        let list = vec![1, 2, 3, 4, 5];
        let result = mean(&list);
        assert_eq!(result, Some(3));
    }
}
