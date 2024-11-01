pub fn split(count: usize, div: usize) -> Vec<usize> {
    if div == 0 {
        panic!("split by zero")
    }

    let mut res = Vec::new();

    if div == 1 {
        res.push(count);
        return res;
    }

    let mut count = count;

    while count > 0 {
        // always decreasing, since div >= 2
        let rem = count / div;
        let part = count - rem;

        if part == 0 {
            break;
        }

        res.push(part);
        count = rem;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(count: usize, div: usize, expected: &[usize]) {
        let result = split(count, div);
        let exp_sum = expected.iter().sum();

        assert_eq!(count, exp_sum, "sum must be equal");
        assert_eq!(expected, result.as_slice(), "split result must be equal");
    }

    #[test]
    fn by_one() {
        check(1000, 1, &[1000])
    }

    #[test]
    fn by_two() {
        check(10, 2, &[5, 3, 1, 1])
    }

    #[test]
    fn by_three() {
        check(1000, 3, &[667, 222, 74, 25, 8, 3, 1])
    }

    #[test]
    fn by_same() {
        check(1000, 1000, &[999, 1])
    }
}
