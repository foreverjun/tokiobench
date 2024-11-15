use core::fmt;

#[derive(Clone, Copy, Debug)]
pub enum SplitType {
    Uniform,
    Geometric,
}

impl fmt::Display for SplitType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn split(stype: SplitType, len: usize, div: usize) -> Vec<usize> {
    match stype {
        SplitType::Uniform => split_eq(len, div),
        SplitType::Geometric => split_gradient(len, div),
    }
}

fn split_gradient(count: usize, div: usize) -> Vec<usize> {
    if div < 1 {
        panic!("{}", format!("div in split = {div}"));
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
mod split_tests {
    use super::split_gradient;

    fn check(count: usize, div: usize, expected: &[usize]) {
        let result = split_gradient(count, div);
        let exp_sum: usize = expected.iter().sum();

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

fn split_eq(len: usize, n: usize) -> Vec<usize> {
    if n < 1 {
        panic!("{}", format!("n = {n} < 1 in split_eq"));
    }
    let mut res = Vec::with_capacity(n);

    let part = len / n;
    if part == 0 {
        for _ in 0..len {
            res.push(1);
        }

        return res;
    }

    let mut len = len;
    let rem = len % n;
    while len >= part {
        res.push(part);
        len -= part;
    }

    *res.last_mut().unwrap() += rem;

    res
}

#[cfg(test)]
mod split_eq_tests {
    use super::split_eq;

    fn check(count: usize, div: usize, expected: &[usize]) {
        let result = split_eq(count, div);
        let exp_sum: usize = expected.iter().sum();

        assert_eq!(count, exp_sum, "sum must be equal");
        assert_eq!(expected, result.as_slice(), "split result must be equal");
    }

    #[test]
    fn ten_by_one() {
        check(10, 1, &[10])
    }

    #[test]
    fn ten_by_two() {
        check(10, 2, &[5, 5])
    }

    #[test]
    fn ten_by_five() {
        check(10, 5, &[2; 5])
    }

    #[test]
    fn one_by_two() {
        check(1, 2, &[1])
    }

    #[test]
    fn handr_by_one() {
        check(100, 1, &[100])
    }

    #[test]
    fn five_by_two() {
        check(5, 2, &[2, 3])
    }

    #[test]
    fn seven_by_three() {
        check(7, 3, &[2, 2, 3])
    }
}
