fn main() {
    println!("Hello, world!");
    let msg: &str = get_hello_message();
    println!("{}", msg);
}

fn get_hello_message() -> &'static str {
    "hello message"
}
