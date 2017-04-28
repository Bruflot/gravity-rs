extern crate gravity;

use std::mem;
use std::ffi::CString;
use gravity::*;

// Source code for the Gravity application
static SRC: &'static str = "func main(){ \
                                return 11 + 4; \
                            }";

extern "C" fn report_error(error_type: error_type_t,
                           _: *const std::os::raw::c_char,
                           error_desc: error_desc_t,
                           _: *mut std::os::raw::c_void){
    panic!("Error {:?}: {:?}", error_type, error_desc);
}

fn main(){
    let source = CString::new(SRC).unwrap();

    unsafe{
        // Configure a VM delegate
        let mut delegate: gravity_delegate_t = mem::zeroed();
        delegate.error_callback = Some(report_error);

        // Compile source into a closure
        let compiler = gravity_compiler_create(&mut delegate);
        let closure = gravity_compiler_run(compiler, source.as_ptr(), SRC.len(), 0, true);

        if closure.is_null() {
            panic!("Failed to compile source into a closure");
        }

        // Create a new VM
        let vm = gravity_vm_new(&mut delegate);

        // Transfer memory from the compiler to the VM, then free the compiler
        gravity_compiler_transfer(compiler, vm);
        gravity_compiler_free(compiler);

        // Execute the previously compiled closure
        if gravity_vm_runmain(vm, closure){
            let result = gravity_vm_result(vm);
            let time = gravity_vm_time(vm);

            println!("Result: {} (in {} ms)", result.__bindgen_anon_1.bindgen_union_field, time);
        }

        // Free the VM and the core classes
        gravity_vm_free(vm);
        gravity_core_free();
    }
}
