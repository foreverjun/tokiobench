#[derive(Debug)]
pub struct Split {
    remaining: usize,
    step_remain: usize,
    div: usize,
}

impl Split {
    pub fn new(all: usize, peaces: usize) -> Split {
        assert!(peaces != 0 && all != 0, "attempt to split {all} / {peaces}");

        let res = Split {
            remaining: all,
            step_remain: peaces,
            div: usize::max(all / peaces, 1),
        };
        return res;
    }
}

impl Iterator for Split {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match () {
            () if self.step_remain == 0 && self.remaining == 0 => {
                return None;
            }
            // last step, let's throw all values
            () if self.step_remain == 1 => {
                let result = self.remaining;

                self.remaining = 0;
                self.step_remain = 0;

                return Some(result);
            }
            () => {
                let result = usize::min(self.div, self.remaining);

                self.remaining -= result;
                self.step_remain -= 1;

                return Some(result);
            }
        }
    }
}

#[cfg(test)]
mod split_eq_tests {
    use super::*;

    fn check(count: usize, div: usize, expected: &[usize]) {
        let result: Vec<usize> = Split::new(count, div).into_iter().collect();
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
