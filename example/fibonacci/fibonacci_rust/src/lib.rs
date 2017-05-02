#[no_mangle]
pub extern "C" fn fibonacci(number: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;

    for _ in 0..number {
        let tmp = a;
        a = b;
        b = a + tmp;
    }

    a
}


#[cfg(test)]
mod tests {
    use ::fibonacci;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(5), 8);
        assert_eq!(fibonacci(6), 13);
    }
}
