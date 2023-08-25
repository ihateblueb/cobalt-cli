pub fn create_end(message: &str) {
    println!("[cobalt] uh-oh! {message}");
    std::process::exit(0);
}

pub fn create_cont(message: &str) {
    println!("[cobalt] uh-oh! {message}");
}
