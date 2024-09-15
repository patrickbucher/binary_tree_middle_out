pub fn middle_out<T: Ord + Clone>(values: &[T]) -> Vec<T> {
    if values.is_empty() {
        Vec::<T>::new()
    } else {
        do_middle_out(&vec![values.to_vec()], &mut Vec::new())
    }
}

fn do_middle_out<T: Ord + Clone>(splits: &Vec<Vec<T>>, acc: &mut Vec<T>) -> Vec<T> {
    if splits.iter().all(|s| s.is_empty()) {
        return acc.clone();
    }
    let mut remainder = Vec::new();
    for split in splits {
        let (median, splits) = split_median(split);
        if let Some(median) = median {
            acc.push(median.clone());
        }
        for split in splits {
            if !split.is_empty() {
                remainder.push(split);
            }
        }
    }
    do_middle_out(&remainder, acc)
}

fn split_median<T: Ord + Clone>(values: &[T]) -> (Option<T>, Vec<Vec<T>>) {
    let n = values.len();
    if n == 0 {
        (None, Vec::new())
    } else if n == 1 {
        (Some(values[0].clone()), Vec::new())
    } else {
        let mut values = values.to_owned();
        values.sort();
        let m = if n % 2 == 0 { n / 2 - 1 } else { n / 2 };
        let median = &values[m];
        let left = values[0..m].to_owned();
        let right = values[m + 1..n].to_owned();
        (Some(median.clone()), vec![left, right])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_middle_out_empty() {
        assert_eq!(middle_out(&Vec::<usize>::new()), Vec::new());
    }

    #[test]
    fn test_middle_out_small() {
        assert_eq!(
            middle_out(&(1..=8).collect::<Vec<usize>>()),
            vec![4, 2, 6, 1, 3, 5, 7, 8]
        );
    }

    #[test]
    fn test_middle_out_big() {
        assert_eq!(
            middle_out(&(0..=14).collect::<Vec<usize>>()),
            vec![7, 3, 11, 1, 5, 9, 13, 0, 2, 4, 6, 8, 10, 12, 14]
        );
    }
}
