#[derive(Debug, PartialEq)]
struct Fibonacci {
    a: u32,
    b: u32,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let next_fibonacci = Some(self.a);

        let tmp = self.a;
        self.a = self.b;
        self.b = self.a + tmp;

        next_fibonacci
    }
}

#[cfg(test)]
mod tests {
    use super::Fibonacci;

    #[test]
    fn test_new_fibonacci() {
        assert_eq!(Fibonacci::new(), Fibonacci { a: 0, b: 1 })
    }

    #[test]
    fn test_fourth_fibonacci() {
        assert_eq!(Fibonacci::new().nth(5), Some(5))
    }
}

fn main() {
    let result : u32 = Fibonacci::new()
        .take(10)
        .filter(|number| number % 2 != 0)
        .zip(Fibonacci::new())
        .map(|(a, b)| a * b)
        .sum();

    println!("{}", result);
}

