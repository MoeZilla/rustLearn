#![allow(non_snake_case)]
fn main() {
    let mut mom = String::from("fuck off,  duck, fucker, fuck");

    change(&mut mom);
    println!("{}", mom);
}

fn kk(some_string: &mut String) {
    some_string.push_str(", fuck");
}
