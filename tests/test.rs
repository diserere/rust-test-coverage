use test_cov_kcov as myMain;

//extern crate test_cov_kcov;
//extern crate main;
//use test_cov_kcov;

#[cfg(test)]
//mod coverage_tests {
mod tests {
//    use super::*;

//#[macro_use]
//extern crate test_cov_kcov;
//use test_cov_kcov::*;
//use test_cov_kcov;

    #[test]
//    #[cfg(feature="pass")]
    fn test_always_pass() {
        assert!(2 == 2);
    }

    #[test]
//    #[cfg(feature="detailed")]
    fn test_say_about_less() {
        myMain::say_about(2,3);
    }
}


