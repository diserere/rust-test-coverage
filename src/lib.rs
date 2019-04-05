fn say_lt(n: i32, eq_const: i32) {
        println!("Less than {}: {}", eq_const, n);
}

fn say_gt(n: i32, eq_const: i32) {
        println!("Greater than {}: {}", eq_const, n);
}

//fn say_about(n: i32, eq_const: i32) {
pub fn say_about(n: i32, eq_const: i32) {
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

