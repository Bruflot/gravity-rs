#![allow(unused_variables, dead_code)]
extern crate gravity_sys as G;

mod builder;

use std::ffi::CString;

pub use builder::*;

const MAX_LEN: u16 = 1024;

pub type Closure = *mut G::gravity_closure_t;
pub type GravityValue = G::gravity_value_t;
pub type ErrorType = G::error_type_t;
pub type ErrorDesc = G::error_desc_t;

// Box?
pub struct Gravity {
    vm: *mut G::gravity_vm,
    compiler: Option<*mut G::gravity_compiler_t>,
    delegate: G::gravity_delegate_t,
}

impl Gravity {
    // #[inline]
    /// Compiles the given closure.
    pub fn compile_closure<'a>(&'a self, source: &'a str) -> Option<Closure> {
        let c_source = match CString::new(source) {
            Ok(x) => x,
            Err(e) => {
                return None;
            }
        };

        let compiler = match self.compiler {
            Some(c) => c,
            None => {
                return None;
            }
        };

        let closure = unsafe { 
            G::gravity_compiler_run(compiler, c_source.as_ptr(), source.len(), 0, false) 
        };

        if closure.is_null(){
            return None;
        }

        Some(closure)
    }

    /// Transfer memory from the compiler to the VM.
    ///
    /// This action will be performed automatically when running the closure unless
    /// it's manually done beforehand.
    #[inline]
    pub fn transfer_mem<'a>(&'a self) {
        let compiler = match self.compiler {
            Some(c) => c,
            None => {
                return ();
            }
        };

        unsafe {
            G::gravity_compiler_transfer(compiler, self.vm);
            G::gravity_compiler_free(compiler);
        }
    }

    /// Load a closure into the VM.
    ///
    /// This action will be performed automatically when running the closure unless
    /// it's manually done beforehand.
    #[inline]
    pub fn load_closure<'a>(&'a self, closure: Closure) {
        unsafe { G::gravity_vm_loadclosure(self.vm, closure); }
    }


    /// Run the main function of the given closure.
    #[inline]
    pub fn run_main<'a>(&'a self, closure: Closure) -> bool {
        // fetch result?
        unsafe { G::gravity_vm_runmain(self.vm, closure) }
    }

    /// Run the specified closure with the given parameters.
    #[inline]
    pub fn run_closure<'a>(&'a self,
                           closure: Closure,
                           value: GravityValue,
                           params: *mut GravityValue,
                           nparams: u16)
                           -> bool {
        unsafe { G::gravity_vm_runclosure(self.vm, closure, value, params, nparams) }
    }

    // TODO: Re-implementation of G::gravity_value_dump to reduce overhead
    // As it stands, the returned result is a string in the following format:
    // `(TYPE) value`
    // Parsing the string and returning the proper type is unacceptable due to the overhead
    // Some changes might have to be made to the VM, but we'll see
    /// Returns the result of the previously ran closure in form of a string.
    /// This method will in the future be reimplemented in Rust, but some changes may
    /// have to be made to the VM to achieve a satisfactory result.
    #[inline]
    pub fn result<'a>(&'a self) -> Option<String>{
        let mut buffer = [0u8; MAX_LEN as usize].to_vec();
        unsafe{
            let result = G::gravity_vm_result(self.vm);
            let _ = G::gravity_value_dump(result, buffer.as_mut_ptr() as *mut i8, MAX_LEN);
        }

        String::from_utf8(buffer).ok()
    }

    /// Returns the time spent executing the previously ran closure.
    #[inline]
    pub fn time<'a>(&'a self) -> f64 {
        unsafe { G::gravity_vm_time(self.vm) }
    }
}

unsafe impl Send for Gravity {}
unsafe impl Sync for Gravity {}

impl Drop for Gravity {
    fn drop(&mut self) {
        unsafe {
            G::gravity_vm_free(self.vm);
            G::gravity_core_free();
        }
    }
}
