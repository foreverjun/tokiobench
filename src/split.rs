#[derive(Debug)]
pub struct Split {
    remaining: usize,
    step_remain: usize,
    div: usize,
}

impl Split {
    pub fn new(all: usize, peaces: usize) -> Split {
        assert!(peaces != 0 && all != 0, "attempt to split {all} / {peaces}");

        Split {
            remaining: all,
            step_remain: peaces,
            div: usize::max(all / peaces, 1),
        }
    }
}

impl Iterator for Split {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match () {
            () if self.step_remain == 0 && self.remaining == 0 => None,
            // last step, let's throw all values
            () if self.step_remain == 1 => {
                let result = self.remaining;

                self.remaining = 0;
                self.step_remain = 0;

                Some(result)
            }
            () => {
                let result = usize::min(self.div, self.remaining);

                self.remaining -= result;
                self.step_remain -= 1;

                Some(result)
            }
        }
    }
}

#[cfg(test)]
mod split_tests {
    use super::*;

    fn check(count: usize, div: usize, expected: &[usize]) {
        let result: Vec<usize> = Split::new(count, div).collect();
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
        check(1, 2, &[1, 0])
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

pub struct EqSplit {
    remain: usize,
    val: usize,
}

impl EqSplit {
    pub fn new(all: usize, peaces: usize) -> EqSplit {
        assert_ne!(peaces, 0, "Peaces is zero");
        assert_eq!(all % peaces, 0, "Cannot div equolly");

        let val = all / peaces;
        assert_ne!(val, 0, "Eq peace in zero");

        EqSplit {
            remain: peaces,
            val,
        }
    }

    pub fn item(&self) -> usize {
        self.val
    }
}

impl Iterator for EqSplit {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remain > 0 {
            self.remain -= 1;
            return Some(self.val);
        }

        None
    }
}

#[cfg(test)]
mod eq_split_tests {
    use super::*;

    fn check(count: usize, div: usize, expected: &[usize]) {
        let result: Vec<usize> = EqSplit::new(count, div).collect();
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
    #[should_panic(
        expected = "assertion `left == right` failed: Cannot div equolly\n  left: 1\n right: 0"
    )]
    fn one_by_two() {
        check(1, 2, &[1, 0])
    }

    #[test]
    fn handr_by_one() {
        check(100, 1, &[100])
    }

    #[test]
    #[should_panic(
        expected = "assertion `left == right` failed: Cannot div equolly\n  left: 1\n right: 0"
    )]
    fn five_by_two() {
        check(5, 2, &[2, 3])
    }

    #[test]
    #[should_panic(expected = "left == right` failed: Cannot div equolly\n  left: 1\n right: 0")]
    fn seven_by_three() {
        check(7, 3, &[2, 2, 3])
    }
}
