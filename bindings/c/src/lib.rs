

#[no_mangle]
pub extern "C" fn print_hello_from_rust() {
    println!("{}", multilib_example::greet());
}