mod intcode;

pub use self::intcode::*;

#[cfg(test)]
mod tests {
    use crate::Intcode;

    #[test]
    fn parse() {
        assert_eq!(
            Intcode::parse("1,9,10,3,2,3,11,0,99,30,40,50"),
            vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[test]
    fn testinput_2() {
        let mut ic = Intcode::new(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        ic.run();
        assert_eq!(ic[0], 3500);
    }
}
