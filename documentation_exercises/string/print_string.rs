fn main() {
    let and_string = "hello";
    print_string(and_string);
}

fn print_string(s: &str) -> &str {
    print!("{}", s);
    s
}