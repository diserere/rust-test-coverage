fn say_lt(n: i32, eq_const: i32) {
	println!("Less than {}: {}", eq_const, n);
}

fn say_gt(n: i32, eq_const: i32) {
	println!("Greater than {}: {}", eq_const, n);
}

fn say_about(n: i32, eq_const: i32) {
	if n < eq_const {
		say_lt(n, eq_const);
		return;
	} else {
		if n > eq_const {
			say_gt(n, eq_const);
			return;
		}
	}
	println!("Oops! equal {}: {} :))", eq_const, n)
}

fn main() {
    println!("Hello, world!");
    
	let eq_const = 5;
    let n = 2;
    say_about(n, eq_const);
    say_about(8, eq_const);
    say_about(5, eq_const);
}


#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	#[cfg(feature="pass")]
	fn test_always_pass() {
		assert!(2 == 2);
	}

    #[test]
	#[cfg(feature="detailed")]
    fn test_say_about_less() {
        say_about(2,3);
    }

    #[test]
	#[cfg(feature="detailed")]
    fn test_say_about_more() {
        say_about(4,3);
    }

    #[test]
	#[cfg(feature="detailed")]
    fn test_say_about_equal() {
        say_about(3,3);
    }

    #[test]
	#[cfg(feature="nodead")]
    fn test_say_about_deadcode() {
        say_about(0,0);
    }

    #[test]
	#[cfg(feature="main")]
    fn test_main() {
        main();
    }

}
