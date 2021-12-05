pub fn transpose<T: Clone>(v: &[Vec<T>]) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<_>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose_basic() {
        let v = transpose(&[vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]);
        let r = vec![vec![0, 3, 6], vec![1, 4, 7], vec![2, 5, 8]];

        assert_eq!(v, r);
    }
}
