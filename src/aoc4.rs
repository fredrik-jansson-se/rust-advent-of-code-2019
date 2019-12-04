pub fn run() {
    println!("4:1 {}", run_1(193651, 649729));
    println!("4:2 {}", run_2(193651, 649729));
}

fn run_1(start: u32, end: u32) -> usize {
    (start..end).filter(|i| is_valid_1(*i)).count()
}

fn run_2(start: u32, end: u32) -> usize {
    (start..end).filter(|i| is_valid_2(*i)).count()
}

fn is_valid_1(mut i: u32) -> bool {
    let mut double = false;
    let mut last = i % 10;
    while i > 0 {
        i /= 10;
        let current = i % 10;

        // At least one double digit
        if last == current {
            double = true;
        }

        // Going left to right, it must increase
        if current > last {
            return false;
        }
        last = current;
    }

    double
}

fn is_valid_2(mut i: u32) -> bool {
    let mut double = false;
    let mut last = i % 10;
    let mut last_double_tested = 0;
    while i > 0 {
        i /= 10;
        let current = i % 10;

        // Going left to right, it must increase
        if current > last {
            return false;
        }

        // At exactly one double digit
        if !double && current != last_double_tested {
            let next = (i / 10) % 10;
            if last == current && current != next {
                double = true;
            } else if last == current && current == next {
                last_double_tested = current;
            }
        }

        last = current;
    }

    double
}

#[cfg(test)]
mod tests {

    #[test]
    fn aoc4_is_valid_1() {
        use super::*;
        assert!(is_valid_1(111111));
        assert!(!is_valid_1(223450));
        assert!(!is_valid_1(123789));
    }

    #[test]
    fn aoc4_is_valid_2() {
        use super::*;
        assert!(is_valid_2(112233));
        assert!(!is_valid_2(123444));
        assert!(is_valid_2(111122));
    }
}
