use crate::println;

#[no_mangle]
pub extern "C" fn system_call() {
    println!("System Call triggered");
}
