extern crate gravity_sys as G;

use Gravity;
use std::mem;
use std::os::raw::{c_char, c_void};

pub struct Builder {
    delegate: G::gravity_delegate_t,
    compiler: bool,
}

// FIXME: bridge_* fields
impl Builder {
    #[inline]
    pub fn new() -> Builder {
        let delegate: G::gravity_delegate_t = unsafe { mem::zeroed() };
        Builder {
            delegate: delegate,
            compiler: false,
        }
    }

    #[inline]
    pub fn compiler<'a>(&'a mut self, compiler: bool) -> &'a mut Builder {
        self.compiler = compiler;
        self
    }

    #[inline]
    pub fn log_callback<'a>(&'a mut self,
                            f: extern "C" fn(*const c_char, *mut c_void))
                            -> &'a mut Builder {
        self.delegate.log_callback = Some(f);
        self
    }

    #[inline]
    pub fn error_callback<'a>(&'a mut self,
                              f: extern "C" fn(error_type: ::ErrorType,
                                               description: *const c_char,
                                               error_desc: G::error_desc_t,
                                               xdata: *mut c_void))
                              -> &'a mut Builder {
        self.delegate.error_callback = Some(f);
        self
    }

    #[inline]
    pub fn unit_test_callback<'a>(&'a mut self,
                                  f: extern "C" fn(error_type: ::ErrorType,
                                                   description: *const c_char,
                                                   note: *const c_char,
                                                   value: ::GravityValue,
                                                   row: i32,
                                                   column: i32,
                                                   xdata: *mut c_void))
                                  -> &'a mut Builder {
        self.delegate.unittest_callback = Some(f);
        self
    }

    #[inline]
    pub fn parser_callback<'a>(&'a mut self,
                               f: extern "C" fn(*mut c_void, *mut c_void))
                               -> &'a mut Builder {
        self.delegate.parser_callback = Some(f);
        self
    }

    #[inline]
    pub fn precode_callback<'a>(&'a mut self,
                                f: extern "C" fn(*mut c_void) -> *const c_char)
                                -> &'a mut Builder {
        self.delegate.precode_callback = Some(f);
        self
    }

    #[inline]
    pub fn load_file_callback<'a>(&'a mut self,
                                  f: extern "C" fn(file: *const c_char,
                                                   size: *mut usize,
                                                   file_id: *mut u32,
                                                   xdata: *mut c_void)
                                                   -> *const c_char)
                                  -> &'a mut Builder {
        self.delegate.loadfile_callback = Some(f);
        self
    }

    #[inline]
    pub fn filename_callback<'a>(&'a mut self,
                                 f: extern "C" fn(file_id: u32, xdata: *mut c_void)
                                    -> *const c_char)
                                 -> &'a mut Builder {
        self.delegate.filename_callback = Some(f);
        self
    }

    #[inline]
    pub fn build<'a>(&'a mut self) -> Option<Gravity> {
        let vm = unsafe { G::gravity_vm_new(&mut self.delegate) };

        let compiler = if self.compiler {
            unsafe { Some(G::gravity_compiler_create(&mut self.delegate)) }
        } else {
            None
        };

        Some(Gravity {
            vm: vm,
            compiler: compiler,
            delegate: self.delegate,
        })
    }
}
