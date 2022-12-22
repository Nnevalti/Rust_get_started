fn print_a_string(str: &'static str) {
    println!("{}", str);
}

fn return_a_str() -> &'static str {
    "Return a str"
}

fn main() {
    let str = "My first string";
    println!("Hello, world! {}", str);
    print_a_string("A string");
    println!("{}", return_a_str());
}
