extern crate test_cov_kcov;

#[cfg(test)]
mod tests {
    use test_cov_kcov::*;

//    #[macro_use]
//    use test_cov_kcov;

    #[test]
//    #[cfg(feature="pass")]
    fn test_always_pass() {
        assert!(2 == 2);
    }

    #[test]
//    #[cfg(feature="detailed")]
    fn test_say_about_less() {
        say_about(2,3);
    }

    #[test]
    #[should_panic]
    #[ignore]
    fn lib_test_say_lt() {
//        say_lt(2,22);
//        say_lt(22,2);
    }

    #[test]
    //~ #[ignore]
    fn test_should_fail() {
        assert!(0 == 1)
    }
}


