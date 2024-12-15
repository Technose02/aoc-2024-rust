pub fn extended_modulo(n: isize, m: usize) -> usize {
    if n >= 0 {
        (n as usize) % m
    } else {
        let m = m as isize;
        let res = (m - (-n % m)) as usize;
        if res == m as usize {
            0
        } else {
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::extended_modulo;

    #[test]
    fn extended_modulo_works() {
        assert_eq!(extended_modulo(0, 7), 0);
        assert_eq!(extended_modulo(1, 7), 1);
        assert_eq!(extended_modulo(2, 7), 2);
        assert_eq!(extended_modulo(3, 7), 3);
        assert_eq!(extended_modulo(4, 7), 4);
        assert_eq!(extended_modulo(5, 7), 5);
        assert_eq!(extended_modulo(6, 7), 6);
        assert_eq!(extended_modulo(7, 7), 0);
        assert_eq!(extended_modulo(8, 7), 1);
        assert_eq!(extended_modulo(9, 7), 2);
        assert_eq!(extended_modulo(10, 7), 3);
        assert_eq!(extended_modulo(20, 7), 6);
        assert_eq!(extended_modulo(51, 7), 2);

        assert_eq!(extended_modulo(-1, 7), 6);
        assert_eq!(extended_modulo(-2, 7), 5);
        assert_eq!(extended_modulo(-3, 7), 4);
        assert_eq!(extended_modulo(-4, 7), 3);
        assert_eq!(extended_modulo(-5, 7), 2);
        assert_eq!(extended_modulo(-6, 7), 1);
        assert_eq!(extended_modulo(-7, 7), 0);
        assert_eq!(extended_modulo(-8, 7), 6);
        assert_eq!(extended_modulo(-9, 7), 5);
        assert_eq!(extended_modulo(-10, 7), 4);
        assert_eq!(extended_modulo(-20, 7), 1);
        assert_eq!(extended_modulo(-51, 7), 5);
    }
}
