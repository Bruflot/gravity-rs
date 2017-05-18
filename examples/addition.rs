extern crate gravity;

use gravity::{Builder, ErrorType, ErrorDesc};
use std::os::raw::{c_char, c_void};

// Source code for the Gravity script
static SRC: &'static str = "func main(){ \
                                return 11 + 4; \
                            }";


extern fn report_error(error_type: ErrorType, _: *const c_char,
                       error_desc: ErrorDesc, _: *mut c_void) {
    println!("{}:{} - {:?}", 
            error_desc.lineno,
            error_desc.colno,
            error_type);
}

fn main() {
    // Create a Gravity object that contains both the compiler and VM
    let gravity = Builder::new()
        .compiler(true)
        .error_callback(report_error)
        .build()
        .unwrap();

    // Compile the source into a closure
    let closure = gravity
        .compile_closure(SRC)
        .expect("Failed to compile closure!");

    // Runs the `main` function of the closure
    // Returns a bool that indicates whether the closure ran successfully or not
    if gravity.run_main(closure) {
        // Returns the result of the previously ran function 
        let result = gravity.result().unwrap();
        // Time spent executing the function
        let time = gravity.time();
        println!("Result: {} (in {} ms)", result, time);
    }
}
