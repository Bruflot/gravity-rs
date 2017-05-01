#![allow(non_upper_case_globals,
         non_camel_case_types,
         non_snake_case)]
#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
impl<T> ::std::clone::Clone for __IncompleteArrayField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::std::marker::Copy for __IncompleteArrayField<T> {}
#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
pub const GRAVITY_MEMORY_DEBUG: ::std::os::raw::c_uint = 0;
pub const INT8_MIN: ::std::os::raw::c_int = -128;
pub const INT16_MIN: ::std::os::raw::c_int = -32768;
pub const INT32_MIN: ::std::os::raw::c_int = -2147483648;
pub const INT8_MAX: ::std::os::raw::c_uint = 127;
pub const INT16_MAX: ::std::os::raw::c_uint = 32767;
pub const INT32_MAX: ::std::os::raw::c_uint = 2147483647;
pub const UINT8_MAX: ::std::os::raw::c_uint = 255;
pub const UINT16_MAX: ::std::os::raw::c_uint = 65535;
pub const UINT32_MAX: ::std::os::raw::c_uint = 4294967295;
pub const INT_LEAST8_MIN: ::std::os::raw::c_int = -128;
pub const INT_LEAST16_MIN: ::std::os::raw::c_int = -32768;
pub const INT_LEAST32_MIN: ::std::os::raw::c_int = -2147483648;
pub const INT_LEAST8_MAX: ::std::os::raw::c_uint = 127;
pub const INT_LEAST16_MAX: ::std::os::raw::c_uint = 32767;
pub const INT_LEAST32_MAX: ::std::os::raw::c_uint = 2147483647;
pub const UINT_LEAST8_MAX: ::std::os::raw::c_uint = 255;
pub const UINT_LEAST16_MAX: ::std::os::raw::c_uint = 65535;
pub const UINT_LEAST32_MAX: ::std::os::raw::c_uint = 4294967295;
pub const INT_FAST8_MIN: ::std::os::raw::c_int = -128;
pub const INT_FAST16_MIN: ::std::os::raw::c_longlong = -9223372036854775808;
pub const INT_FAST32_MIN: ::std::os::raw::c_longlong = -9223372036854775808;
pub const INT_FAST8_MAX: ::std::os::raw::c_uint = 127;
pub const INT_FAST16_MAX: ::std::os::raw::c_ulonglong = 9223372036854775807;
pub const INT_FAST32_MAX: ::std::os::raw::c_ulonglong = 9223372036854775807;
pub const UINT_FAST8_MAX: ::std::os::raw::c_uint = 255;
pub const UINT_FAST16_MAX: ::std::os::raw::c_int = -1;
pub const UINT_FAST32_MAX: ::std::os::raw::c_int = -1;
pub const INTPTR_MIN: ::std::os::raw::c_longlong = -9223372036854775808;
pub const INTPTR_MAX: ::std::os::raw::c_ulonglong = 9223372036854775807;
pub const UINTPTR_MAX: ::std::os::raw::c_int = -1;
pub const PTRDIFF_MIN: ::std::os::raw::c_longlong = -9223372036854775808;
pub const PTRDIFF_MAX: ::std::os::raw::c_ulonglong = 9223372036854775807;
pub const SIG_ATOMIC_MIN: ::std::os::raw::c_int = -2147483648;
pub const SIG_ATOMIC_MAX: ::std::os::raw::c_uint = 2147483647;
pub const SIZE_MAX: ::std::os::raw::c_int = -1;
pub const WINT_MIN: ::std::os::raw::c_uint = 0;
pub const WINT_MAX: ::std::os::raw::c_uint = 4294967295;
pub const true_: ::std::os::raw::c_uint = 1;
pub const false_: ::std::os::raw::c_uint = 0;
pub const NR_OPEN: ::std::os::raw::c_uint = 1024;
pub const NGROUPS_MAX: ::std::os::raw::c_uint = 65536;
pub const ARG_MAX: ::std::os::raw::c_uint = 131072;
pub const LINK_MAX: ::std::os::raw::c_uint = 127;
pub const MAX_CANON: ::std::os::raw::c_uint = 255;
pub const MAX_INPUT: ::std::os::raw::c_uint = 255;
pub const NAME_MAX: ::std::os::raw::c_uint = 255;
pub const PATH_MAX: ::std::os::raw::c_uint = 4096;
pub const PIPE_BUF: ::std::os::raw::c_uint = 4096;
pub const XATTR_NAME_MAX: ::std::os::raw::c_uint = 255;
pub const XATTR_SIZE_MAX: ::std::os::raw::c_uint = 65536;
pub const XATTR_LIST_MAX: ::std::os::raw::c_uint = 65536;
pub const RTSIG_MAX: ::std::os::raw::c_uint = 32;
pub const PTHREAD_KEYS_MAX: ::std::os::raw::c_uint = 1024;
pub const PTHREAD_DESTRUCTOR_ITERATIONS: ::std::os::raw::c_uint = 4;
pub const AIO_PRIO_DELTA_MAX: ::std::os::raw::c_uint = 20;
pub const PTHREAD_STACK_MIN: ::std::os::raw::c_uint = 16384;
pub const DELAYTIMER_MAX: ::std::os::raw::c_uint = 2147483647;
pub const TTY_NAME_MAX: ::std::os::raw::c_uint = 32;
pub const LOGIN_NAME_MAX: ::std::os::raw::c_uint = 256;
pub const HOST_NAME_MAX: ::std::os::raw::c_uint = 64;
pub const MQ_PRIO_MAX: ::std::os::raw::c_uint = 32768;
pub const SEM_VALUE_MAX: ::std::os::raw::c_uint = 2147483647;
pub const MAXNAMLEN: ::std::os::raw::c_uint = 255;
pub const MARRAY_DEFAULT_SIZE: ::std::os::raw::c_uint = 8;
pub const PRId8: &'static [u8; 2usize] = b"d\x00";
pub const PRId16: &'static [u8; 2usize] = b"d\x00";
pub const PRId32: &'static [u8; 2usize] = b"d\x00";
pub const PRId64: &'static [u8; 3usize] = b"ld\x00";
pub const PRIdLEAST8: &'static [u8; 2usize] = b"d\x00";
pub const PRIdLEAST16: &'static [u8; 2usize] = b"d\x00";
pub const PRIdLEAST32: &'static [u8; 2usize] = b"d\x00";
pub const PRIdLEAST64: &'static [u8; 3usize] = b"ld\x00";
pub const PRIdFAST8: &'static [u8; 2usize] = b"d\x00";
pub const PRIdFAST16: &'static [u8; 3usize] = b"ld\x00";
pub const PRIdFAST32: &'static [u8; 3usize] = b"ld\x00";
pub const PRIdFAST64: &'static [u8; 3usize] = b"ld\x00";
pub const PRIi8: &'static [u8; 2usize] = b"i\x00";
pub const PRIi16: &'static [u8; 2usize] = b"i\x00";
pub const PRIi32: &'static [u8; 2usize] = b"i\x00";
pub const PRIi64: &'static [u8; 3usize] = b"li\x00";
pub const PRIiLEAST8: &'static [u8; 2usize] = b"i\x00";
pub const PRIiLEAST16: &'static [u8; 2usize] = b"i\x00";
pub const PRIiLEAST32: &'static [u8; 2usize] = b"i\x00";
pub const PRIiLEAST64: &'static [u8; 3usize] = b"li\x00";
pub const PRIiFAST8: &'static [u8; 2usize] = b"i\x00";
pub const PRIiFAST16: &'static [u8; 3usize] = b"li\x00";
pub const PRIiFAST32: &'static [u8; 3usize] = b"li\x00";
pub const PRIiFAST64: &'static [u8; 3usize] = b"li\x00";
pub const PRIo8: &'static [u8; 2usize] = b"o\x00";
pub const PRIo16: &'static [u8; 2usize] = b"o\x00";
pub const PRIo32: &'static [u8; 2usize] = b"o\x00";
pub const PRIo64: &'static [u8; 3usize] = b"lo\x00";
pub const PRIoLEAST8: &'static [u8; 2usize] = b"o\x00";
pub const PRIoLEAST16: &'static [u8; 2usize] = b"o\x00";
pub const PRIoLEAST32: &'static [u8; 2usize] = b"o\x00";
pub const PRIoLEAST64: &'static [u8; 3usize] = b"lo\x00";
pub const PRIoFAST8: &'static [u8; 2usize] = b"o\x00";
pub const PRIoFAST16: &'static [u8; 3usize] = b"lo\x00";
pub const PRIoFAST32: &'static [u8; 3usize] = b"lo\x00";
pub const PRIoFAST64: &'static [u8; 3usize] = b"lo\x00";
pub const PRIu8: &'static [u8; 2usize] = b"u\x00";
pub const PRIu16: &'static [u8; 2usize] = b"u\x00";
pub const PRIu32: &'static [u8; 2usize] = b"u\x00";
pub const PRIu64: &'static [u8; 3usize] = b"lu\x00";
pub const PRIuLEAST8: &'static [u8; 2usize] = b"u\x00";
pub const PRIuLEAST16: &'static [u8; 2usize] = b"u\x00";
pub const PRIuLEAST32: &'static [u8; 2usize] = b"u\x00";
pub const PRIuLEAST64: &'static [u8; 3usize] = b"lu\x00";
pub const PRIuFAST8: &'static [u8; 2usize] = b"u\x00";
pub const PRIuFAST16: &'static [u8; 3usize] = b"lu\x00";
pub const PRIuFAST32: &'static [u8; 3usize] = b"lu\x00";
pub const PRIuFAST64: &'static [u8; 3usize] = b"lu\x00";
pub const PRIx8: &'static [u8; 2usize] = b"x\x00";
pub const PRIx16: &'static [u8; 2usize] = b"x\x00";
pub const PRIx32: &'static [u8; 2usize] = b"x\x00";
pub const PRIx64: &'static [u8; 3usize] = b"lx\x00";
pub const PRIxLEAST8: &'static [u8; 2usize] = b"x\x00";
pub const PRIxLEAST16: &'static [u8; 2usize] = b"x\x00";
pub const PRIxLEAST32: &'static [u8; 2usize] = b"x\x00";
pub const PRIxLEAST64: &'static [u8; 3usize] = b"lx\x00";
pub const PRIxFAST8: &'static [u8; 2usize] = b"x\x00";
pub const PRIxFAST16: &'static [u8; 3usize] = b"lx\x00";
pub const PRIxFAST32: &'static [u8; 3usize] = b"lx\x00";
pub const PRIxFAST64: &'static [u8; 3usize] = b"lx\x00";
pub const PRIX8: &'static [u8; 2usize] = b"X\x00";
pub const PRIX16: &'static [u8; 2usize] = b"X\x00";
pub const PRIX32: &'static [u8; 2usize] = b"X\x00";
pub const PRIX64: &'static [u8; 3usize] = b"lX\x00";
pub const PRIXLEAST8: &'static [u8; 2usize] = b"X\x00";
pub const PRIXLEAST16: &'static [u8; 2usize] = b"X\x00";
pub const PRIXLEAST32: &'static [u8; 2usize] = b"X\x00";
pub const PRIXLEAST64: &'static [u8; 3usize] = b"lX\x00";
pub const PRIXFAST8: &'static [u8; 2usize] = b"X\x00";
pub const PRIXFAST16: &'static [u8; 3usize] = b"lX\x00";
pub const PRIXFAST32: &'static [u8; 3usize] = b"lX\x00";
pub const PRIXFAST64: &'static [u8; 3usize] = b"lX\x00";
pub const PRIdMAX: &'static [u8; 3usize] = b"ld\x00";
pub const PRIiMAX: &'static [u8; 3usize] = b"li\x00";
pub const PRIoMAX: &'static [u8; 3usize] = b"lo\x00";
pub const PRIuMAX: &'static [u8; 3usize] = b"lu\x00";
pub const PRIxMAX: &'static [u8; 3usize] = b"lx\x00";
pub const PRIXMAX: &'static [u8; 3usize] = b"lX\x00";
pub const PRIdPTR: &'static [u8; 3usize] = b"ld\x00";
pub const PRIiPTR: &'static [u8; 3usize] = b"li\x00";
pub const PRIoPTR: &'static [u8; 3usize] = b"lo\x00";
pub const PRIuPTR: &'static [u8; 3usize] = b"lu\x00";
pub const PRIxPTR: &'static [u8; 3usize] = b"lx\x00";
pub const PRIXPTR: &'static [u8; 3usize] = b"lX\x00";
pub const SCNd8: &'static [u8; 4usize] = b"hhd\x00";
pub const SCNd16: &'static [u8; 3usize] = b"hd\x00";
pub const SCNd32: &'static [u8; 2usize] = b"d\x00";
pub const SCNd64: &'static [u8; 3usize] = b"ld\x00";
pub const SCNdLEAST8: &'static [u8; 4usize] = b"hhd\x00";
pub const SCNdLEAST16: &'static [u8; 3usize] = b"hd\x00";
pub const SCNdLEAST32: &'static [u8; 2usize] = b"d\x00";
pub const SCNdLEAST64: &'static [u8; 3usize] = b"ld\x00";
pub const SCNdFAST8: &'static [u8; 4usize] = b"hhd\x00";
pub const SCNdFAST16: &'static [u8; 3usize] = b"ld\x00";
pub const SCNdFAST32: &'static [u8; 3usize] = b"ld\x00";
pub const SCNdFAST64: &'static [u8; 3usize] = b"ld\x00";
pub const SCNi8: &'static [u8; 4usize] = b"hhi\x00";
pub const SCNi16: &'static [u8; 3usize] = b"hi\x00";
pub const SCNi32: &'static [u8; 2usize] = b"i\x00";
pub const SCNi64: &'static [u8; 3usize] = b"li\x00";
pub const SCNiLEAST8: &'static [u8; 4usize] = b"hhi\x00";
pub const SCNiLEAST16: &'static [u8; 3usize] = b"hi\x00";
pub const SCNiLEAST32: &'static [u8; 2usize] = b"i\x00";
pub const SCNiLEAST64: &'static [u8; 3usize] = b"li\x00";
pub const SCNiFAST8: &'static [u8; 4usize] = b"hhi\x00";
pub const SCNiFAST16: &'static [u8; 3usize] = b"li\x00";
pub const SCNiFAST32: &'static [u8; 3usize] = b"li\x00";
pub const SCNiFAST64: &'static [u8; 3usize] = b"li\x00";
pub const SCNu8: &'static [u8; 4usize] = b"hhu\x00";
pub const SCNu16: &'static [u8; 3usize] = b"hu\x00";
pub const SCNu32: &'static [u8; 2usize] = b"u\x00";
pub const SCNu64: &'static [u8; 3usize] = b"lu\x00";
pub const SCNuLEAST8: &'static [u8; 4usize] = b"hhu\x00";
pub const SCNuLEAST16: &'static [u8; 3usize] = b"hu\x00";
pub const SCNuLEAST32: &'static [u8; 2usize] = b"u\x00";
pub const SCNuLEAST64: &'static [u8; 3usize] = b"lu\x00";
pub const SCNuFAST8: &'static [u8; 4usize] = b"hhu\x00";
pub const SCNuFAST16: &'static [u8; 3usize] = b"lu\x00";
pub const SCNuFAST32: &'static [u8; 3usize] = b"lu\x00";
pub const SCNuFAST64: &'static [u8; 3usize] = b"lu\x00";
pub const SCNo8: &'static [u8; 4usize] = b"hho\x00";
pub const SCNo16: &'static [u8; 3usize] = b"ho\x00";
pub const SCNo32: &'static [u8; 2usize] = b"o\x00";
pub const SCNo64: &'static [u8; 3usize] = b"lo\x00";
pub const SCNoLEAST8: &'static [u8; 4usize] = b"hho\x00";
pub const SCNoLEAST16: &'static [u8; 3usize] = b"ho\x00";
pub const SCNoLEAST32: &'static [u8; 2usize] = b"o\x00";
pub const SCNoLEAST64: &'static [u8; 3usize] = b"lo\x00";
pub const SCNoFAST8: &'static [u8; 4usize] = b"hho\x00";
pub const SCNoFAST16: &'static [u8; 3usize] = b"lo\x00";
pub const SCNoFAST32: &'static [u8; 3usize] = b"lo\x00";
pub const SCNoFAST64: &'static [u8; 3usize] = b"lo\x00";
pub const SCNx8: &'static [u8; 4usize] = b"hhx\x00";
pub const SCNx16: &'static [u8; 3usize] = b"hx\x00";
pub const SCNx32: &'static [u8; 2usize] = b"x\x00";
pub const SCNx64: &'static [u8; 3usize] = b"lx\x00";
pub const SCNxLEAST8: &'static [u8; 4usize] = b"hhx\x00";
pub const SCNxLEAST16: &'static [u8; 3usize] = b"hx\x00";
pub const SCNxLEAST32: &'static [u8; 2usize] = b"x\x00";
pub const SCNxLEAST64: &'static [u8; 3usize] = b"lx\x00";
pub const SCNxFAST8: &'static [u8; 4usize] = b"hhx\x00";
pub const SCNxFAST16: &'static [u8; 3usize] = b"lx\x00";
pub const SCNxFAST32: &'static [u8; 3usize] = b"lx\x00";
pub const SCNxFAST64: &'static [u8; 3usize] = b"lx\x00";
pub const SCNdMAX: &'static [u8; 3usize] = b"ld\x00";
pub const SCNiMAX: &'static [u8; 3usize] = b"li\x00";
pub const SCNoMAX: &'static [u8; 3usize] = b"lo\x00";
pub const SCNuMAX: &'static [u8; 3usize] = b"lu\x00";
pub const SCNxMAX: &'static [u8; 3usize] = b"lx\x00";
pub const SCNdPTR: &'static [u8; 3usize] = b"ld\x00";
pub const SCNiPTR: &'static [u8; 3usize] = b"li\x00";
pub const SCNoPTR: &'static [u8; 3usize] = b"lo\x00";
pub const SCNuPTR: &'static [u8; 3usize] = b"lu\x00";
pub const SCNxPTR: &'static [u8; 3usize] = b"lx\x00";
pub const json_enable_comments: ::std::os::raw::c_uint = 1;
pub const json_error_max: ::std::os::raw::c_uint = 128;
pub const GRAVITY_VERSION: &'static [u8; 6usize] = b"0.2.6\x00";
pub const GRAVITY_VERSION_NUMBER: ::std::os::raw::c_uint = 518;
pub const GRAVITY_ENABLE_DOUBLE: ::std::os::raw::c_uint = 1;
pub const GRAVITY_ENABLE_INT64: ::std::os::raw::c_uint = 1;
pub const GRAVITY_COMPUTED_GOTO: ::std::os::raw::c_uint = 1;
pub const GRAVITY_NULL_SILENT: ::std::os::raw::c_uint = 1;
pub const GRAVITY_MAP_DOTSUGAR: ::std::os::raw::c_uint = 0;
pub const MAIN_FUNCTION: &'static [u8; 5usize] = b"main\x00";
pub const ITERATOR_INIT_FUNCTION: &'static [u8; 8usize] = b"iterate\x00";
pub const ITERATOR_NEXT_FUNCTION: &'static [u8; 5usize] = b"next\x00";
pub const INITMODULE_NAME: &'static [u8; 12usize] = b"$moduleinit\x00";
pub const CLASS_INTERNAL_INIT_NAME: &'static [u8; 6usize] = b"$init\x00";
pub const CLASS_CONSTRUCTOR_NAME: &'static [u8; 5usize] = b"init\x00";
pub const CLASS_DESTRUCTOR_NAME: &'static [u8; 7usize] = b"deinit\x00";
pub const SELF_PARAMETER_NAME: &'static [u8; 5usize] = b"self\x00";
pub const OUTER_IVAR_NAME: &'static [u8; 6usize] = b"outer\x00";
pub const GETTER_FUNCTION_NAME: &'static [u8; 4usize] = b"get\x00";
pub const SETTER_FUNCTION_NAME: &'static [u8; 4usize] = b"set\x00";
pub const SETTER_PARAMETER_NAME: &'static [u8; 6usize] = b"value\x00";
pub const GLOBALS_DEFAULT_SLOT: ::std::os::raw::c_uint = 4096;
pub const CPOOL_INDEX_MAX: ::std::os::raw::c_uint = 4096;
pub const CPOOL_VALUE_SUPER: ::std::os::raw::c_uint = 4097;
pub const CPOOL_VALUE_NULL: ::std::os::raw::c_uint = 4098;
pub const CPOOL_VALUE_UNDEFINED: ::std::os::raw::c_uint = 4099;
pub const CPOOL_VALUE_ARGUMENTS: ::std::os::raw::c_uint = 4100;
pub const CPOOL_VALUE_TRUE: ::std::os::raw::c_uint = 4101;
pub const CPOOL_VALUE_FALSE: ::std::os::raw::c_uint = 4102;
pub const CPOOL_VALUE_FUNC: ::std::os::raw::c_uint = 4103;
pub const MAX_INSTRUCTION_OPCODE: ::std::os::raw::c_uint = 64;
pub const MAX_REGISTERS: ::std::os::raw::c_uint = 256;
pub const MAX_LOCALS: ::std::os::raw::c_uint = 200;
pub const MAX_UPVALUES: ::std::os::raw::c_uint = 200;
pub const MAX_INLINE_INT: ::std::os::raw::c_uint = 131072;
pub const MAX_FIELDSxFLUSH: ::std::os::raw::c_uint = 64;
pub const MAX_IVARS: ::std::os::raw::c_uint = 768;
pub const MAX_ALLOCATION: ::std::os::raw::c_uint = 4194304;
pub const MAX_CCALLS: ::std::os::raw::c_uint = 100;
pub const DEFAULT_CONTEXT_SIZE: ::std::os::raw::c_uint = 256;
pub const DEFAULT_MINSTRING_SIZE: ::std::os::raw::c_uint = 32;
pub const DEFAULT_MINSTACK_SIZE: ::std::os::raw::c_uint = 256;
pub const DEFAULT_MINCFRAME_SIZE: ::std::os::raw::c_uint = 32;
pub const DEFAULT_CG_THRESHOLD: ::std::os::raw::c_uint = 5242880;
pub const DEFAULT_CG_MINTHRESHOLD: ::std::os::raw::c_uint = 1048576;
pub const DEFAULT_CG_RATIO: f64 = 0.5;
pub const EPSILON: f64 = 0.000001;
pub const GRAVITY_DATA_REGISTER: ::std::os::raw::c_uint = 4294967295;
pub const GRAVITY_FIBER_REGISTER: ::std::os::raw::c_uint = 4294967294;
pub const GRAVITY_MSG_REGISTER: ::std::os::raw::c_uint = 4294967293;
pub const GRAVITY_BRIDGE_INDEX: ::std::os::raw::c_uint = 65535;
pub const GRAVITY_COMPUTED_INDEX: ::std::os::raw::c_uint = 65534;
pub type __u_char = ::std::os::raw::c_uchar;
pub type __u_short = ::std::os::raw::c_ushort;
pub type __u_int = ::std::os::raw::c_uint;
pub type __u_long = ::std::os::raw::c_ulong;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __quad_t = ::std::os::raw::c_long;
pub type __u_quad_t = ::std::os::raw::c_ulong;
pub type __intmax_t = ::std::os::raw::c_long;
pub type __uintmax_t = ::std::os::raw::c_ulong;
pub type __dev_t = ::std::os::raw::c_ulong;
pub type __uid_t = ::std::os::raw::c_uint;
pub type __gid_t = ::std::os::raw::c_uint;
pub type __ino_t = ::std::os::raw::c_ulong;
pub type __ino64_t = ::std::os::raw::c_ulong;
pub type __mode_t = ::std::os::raw::c_uint;
pub type __nlink_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type __pid_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}
impl Clone for __fsid_t {
    fn clone(&self) -> Self {
        *self
    }
}
pub type __clock_t = ::std::os::raw::c_long;
pub type __rlim_t = ::std::os::raw::c_ulong;
pub type __rlim64_t = ::std::os::raw::c_ulong;
pub type __id_t = ::std::os::raw::c_uint;
pub type __time_t = ::std::os::raw::c_long;
pub type __useconds_t = ::std::os::raw::c_uint;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __daddr_t = ::std::os::raw::c_int;
pub type __key_t = ::std::os::raw::c_int;
pub type __clockid_t = ::std::os::raw::c_int;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type __blksize_t = ::std::os::raw::c_long;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __blkcnt64_t = ::std::os::raw::c_long;
pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
pub type __fsword_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __qaddr_t = *mut __quad_t;
pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __socklen_t = ::std::os::raw::c_uint;
pub type int_least8_t = ::std::os::raw::c_schar;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_long;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulong;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: ::std::os::raw::c_ushort,
    pub d_type: ::std::os::raw::c_uchar,
    pub d_name: [::std::os::raw::c_char; 256usize],
}
pub const DT_UNKNOWN: _bindgen_ty_1 = _bindgen_ty_1::DT_UNKNOWN;
pub const DT_FIFO: _bindgen_ty_1 = _bindgen_ty_1::DT_FIFO;
pub const DT_CHR: _bindgen_ty_1 = _bindgen_ty_1::DT_CHR;
pub const DT_DIR: _bindgen_ty_1 = _bindgen_ty_1::DT_DIR;
pub const DT_BLK: _bindgen_ty_1 = _bindgen_ty_1::DT_BLK;
pub const DT_REG: _bindgen_ty_1 = _bindgen_ty_1::DT_REG;
pub const DT_LNK: _bindgen_ty_1 = _bindgen_ty_1::DT_LNK;
pub const DT_SOCK: _bindgen_ty_1 = _bindgen_ty_1::DT_SOCK;
pub const DT_WHT: _bindgen_ty_1 = _bindgen_ty_1::DT_WHT;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _bindgen_ty_1 {
    DT_UNKNOWN = 0,
    DT_FIFO = 1,
    DT_CHR = 2,
    DT_DIR = 4,
    DT_BLK = 6,
    DT_REG = 8,
    DT_LNK = 10,
    DT_SOCK = 12,
    DT_WHT = 14,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __dirstream([u8; 0]);
pub type DIR = __dirstream;
pub type nanotime_t = u64;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct uint16_r {
    pub n: usize,
    pub m: usize,
    pub p: *mut u16,
}
impl Clone for uint16_r {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct uint32_r {
    pub n: usize,
    pub m: usize,
    pub p: *mut u32,
}
impl Clone for uint32_r {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct void_r {
    pub n: usize,
    pub m: usize,
    pub p: *mut *mut ::std::os::raw::c_void,
}
impl Clone for void_r {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct cstring_r {
    pub n: usize,
    pub m: usize,
    pub p: *mut *const ::std::os::raw::c_char,
}
impl Clone for cstring_r {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct json_t([u8; 0]);
pub type __gwchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct imaxdiv_t {
    pub quot: ::std::os::raw::c_long,
    pub rem: ::std::os::raw::c_long,
}
impl Clone for imaxdiv_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct json_settings {
    pub max_memory: ::std::os::raw::c_ulong,
    pub settings: ::std::os::raw::c_int,
    pub memory_alloc:
        ::std::option::Option<unsafe extern "C" fn(arg1: usize,
                                                   zero: ::std::os::raw::c_int,
                                                   user_data: *mut ::std::os::raw::c_void)
                                                   -> *mut ::std::os::raw::c_void>,
    pub memory_free:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void,
                                                   user_data: *mut ::std::os::raw::c_void)>,
    pub user_data: *mut ::std::os::raw::c_void,
    pub value_extra: usize,
}
impl Clone for json_settings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum json_type {
    json_none = 0,
    json_object = 1,
    json_array = 2,
    json_integer = 3,
    json_double = 4,
    json_string = 5,
    json_boolean = 6,
    json_null = 7,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct _json_value {
    pub parent: *mut _json_value,
    pub type_: json_type,
    pub u: _json_value__bindgen_ty_1,
    pub _reserved: _json_value__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct _json_value__bindgen_ty_1 {
    pub boolean: __BindgenUnionField<::std::os::raw::c_int>,
    pub integer: __BindgenUnionField<i64>,
    pub dbl: __BindgenUnionField<f64>,
    pub string: __BindgenUnionField<_json_value__bindgen_ty_1__bindgen_ty_1>,
    pub object: __BindgenUnionField<_json_value__bindgen_ty_1__bindgen_ty_2>,
    pub array: __BindgenUnionField<_json_value__bindgen_ty_1__bindgen_ty_3>,
    pub bindgen_union_field: [u64; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct _json_value__bindgen_ty_1__bindgen_ty_1 {
    pub length: ::std::os::raw::c_uint,
    pub ptr: *mut ::std::os::raw::c_char,
}
impl Clone for _json_value__bindgen_ty_1__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct _json_value__bindgen_ty_1__bindgen_ty_2 {
    pub length: ::std::os::raw::c_uint,
    pub values: *mut json_object_entry,
}
impl Clone for _json_value__bindgen_ty_1__bindgen_ty_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct _json_value__bindgen_ty_1__bindgen_ty_3 {
    pub length: ::std::os::raw::c_uint,
    pub values: *mut *mut _json_value,
}
impl Clone for _json_value__bindgen_ty_1__bindgen_ty_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl Clone for _json_value__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct _json_value__bindgen_ty_2 {
    pub next_alloc: __BindgenUnionField<*mut _json_value>,
    pub object_mem: __BindgenUnionField<*mut ::std::os::raw::c_void>,
    pub bindgen_union_field: u64,
}
impl Clone for _json_value__bindgen_ty_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl Clone for _json_value {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct _json_object_entry {
    pub name: *mut ::std::os::raw::c_char,
    pub name_length: ::std::os::raw::c_uint,
    pub value: *mut _json_value,
}
impl Clone for _json_object_entry {
    fn clone(&self) -> Self {
        *self
    }
}
pub type json_object_entry = _json_object_entry;
pub type json_value = _json_value;
pub type gravity_float_t = f64;
pub type gravity_int_t = i64;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_class_s {
    pub isa: *mut gravity_class_t,
    pub gc: gravity_gc_t,
    pub objclass: *mut gravity_class_t,
    pub identifier: *const ::std::os::raw::c_char,
    pub has_outer: bool,
    pub is_struct: bool,
    pub is_inited: bool,
    pub unused: bool,
    pub xdata: *mut ::std::os::raw::c_void,
    pub superclass: *mut gravity_class_s,
    pub htable: *mut gravity_hash_t,
    pub nivars: u32,
    pub ivars: *mut gravity_value_t,
}
impl Clone for gravity_class_s {
    fn clone(&self) -> Self {
        *self
    }
}
pub type gravity_class_t = gravity_class_s;
pub type gravity_object_t = gravity_class_s;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_value_t {
    pub isa: *mut gravity_class_t,
    pub __bindgen_anon_1: gravity_value_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_value_t__bindgen_ty_1 {
    pub n: __BindgenUnionField<gravity_int_t>,
    pub f: __BindgenUnionField<gravity_float_t>,
    pub p: __BindgenUnionField<*mut gravity_object_t>,
    pub bindgen_union_field: u64,
}
impl Clone for gravity_value_t__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl Clone for gravity_value_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_value_r {
    pub n: usize,
    pub m: usize,
    pub p: *mut gravity_value_t,
}
impl Clone for gravity_value_r {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gravity_hash_t([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gravity_vm([u8; 0]);
pub type gravity_c_internal =
    ::std::option::Option<unsafe extern "C" fn(vm: *mut gravity_vm,
                                               args: *mut gravity_value_t,
                                               nargs: u16,
                                               rindex: u32)
                                               -> bool>;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum gravity_special_index {
    EXEC_TYPE_SPECIAL_GETTER = 0,
    EXEC_TYPE_SPECIAL_SETTER = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum gravity_exec_type {
    EXEC_TYPE_NATIVE = 0,
    EXEC_TYPE_INTERNAL = 1,
    EXEC_TYPE_BRIDGED = 2,
    EXEC_TYPE_SPECIAL = 3,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_gc_t {
    pub isdark: bool,
    pub next: *mut gravity_object_t,
}
impl Clone for gravity_gc_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_function_t {
    pub isa: *mut gravity_class_t,
    pub gc: gravity_gc_t,
    pub xdata: *mut ::std::os::raw::c_void,
    pub identifier: *const ::std::os::raw::c_char,
    pub nparams: u16,
    pub nlocals: u16,
    pub ntemps: u16,
    pub nupvalues: u16,
    pub tag: gravity_exec_type,
    pub __bindgen_anon_1: gravity_function_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_function_t__bindgen_ty_1 {
    pub __bindgen_anon_1: __BindgenUnionField<gravity_function_t__bindgen_ty_1__bindgen_ty_1>,
    pub internal: __BindgenUnionField<gravity_c_internal>,
    pub __bindgen_anon_2: __BindgenUnionField<gravity_function_t__bindgen_ty_1__bindgen_ty_2>,
    pub bindgen_union_field: [u64; 6usize],
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_function_t__bindgen_ty_1__bindgen_ty_1 {
    pub cpool: gravity_value_r,
    pub ninsts: u32,
    pub bytecode: *mut u32,
    pub purity: f32,
    pub useargs: bool,
}
impl Clone for gravity_function_t__bindgen_ty_1__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_function_t__bindgen_ty_1__bindgen_ty_2 {
    pub index: u16,
    pub special: [*mut ::std::os::raw::c_void; 2usize],
}
impl Clone for gravity_function_t__bindgen_ty_1__bindgen_ty_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl Clone for gravity_function_t__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl Clone for gravity_function_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct upvalue_s {
    pub isa: *mut gravity_class_t,
    pub gc: gravity_gc_t,
    pub value: *mut gravity_value_t,
    pub closed: gravity_value_t,
    pub next: *mut upvalue_s,
}
impl Clone for upvalue_s {
    fn clone(&self) -> Self {
        *self
    }
}
pub type gravity_upvalue_t = upvalue_s;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_closure_t {
    pub isa: *mut gravity_class_t,
    pub gc: gravity_gc_t,
    pub f: *mut gravity_function_t,
    pub upvalue: *mut *mut gravity_upvalue_t,
}
impl Clone for gravity_closure_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_list_t {
    pub isa: *mut gravity_class_t,
    pub gc: gravity_gc_t,
    pub array: gravity_value_r,
}
impl Clone for gravity_list_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_map_t {
    pub isa: *mut gravity_class_t,
    pub gc: gravity_gc_t,
    pub hash: *mut gravity_hash_t,
}
impl Clone for gravity_map_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_callframe_t {
    pub ip: *mut u32,
    pub dest: u32,
    pub nargs: u16,
    pub args: *mut gravity_list_t,
    pub closure: *mut gravity_closure_t,
    pub stackstart: *mut gravity_value_t,
    pub outloop: bool,
}
impl Clone for gravity_callframe_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct fiber_s {
    pub isa: *mut gravity_class_t,
    pub gc: gravity_gc_t,
    pub stack: *mut gravity_value_t,
    pub stacktop: *mut gravity_value_t,
    pub stackalloc: u32,
    pub frames: *mut gravity_callframe_t,
    pub nframes: u32,
    pub framesalloc: u32,
    pub upvalues: *mut gravity_upvalue_t,
    pub error: *mut ::std::os::raw::c_char,
    pub trying: bool,
    pub caller: *mut fiber_s,
    pub result: gravity_value_t,
}
impl Clone for fiber_s {
    fn clone(&self) -> Self {
        *self
    }
}
pub type gravity_fiber_t = fiber_s;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_module_t {
    pub isa: *mut gravity_class_t,
    pub gc: gravity_gc_t,
    pub identifier: *const ::std::os::raw::c_char,
    pub htable: *mut gravity_hash_t,
}
impl Clone for gravity_module_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_instance_t {
    pub isa: *mut gravity_class_t,
    pub gc: gravity_gc_t,
    pub objclass: *mut gravity_class_t,
    pub xdata: *mut ::std::os::raw::c_void,
    pub ivars: __IncompleteArrayField<gravity_value_t>,
}
impl Clone for gravity_instance_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_string_t {
    pub isa: *mut gravity_class_t,
    pub gc: gravity_gc_t,
    pub s: *mut ::std::os::raw::c_char,
    pub hash: u32,
    pub len: u32,
    pub alloc: u32,
}
impl Clone for gravity_string_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_range_t {
    pub isa: *mut gravity_class_t,
    pub gc: gravity_gc_t,
    pub from: gravity_int_t,
    pub to: gravity_int_t,
}
impl Clone for gravity_range_t {
    fn clone(&self) -> Self {
        *self
    }
}
pub type code_dump_function =
    ::std::option::Option<unsafe extern "C" fn(code: *mut ::std::os::raw::c_void)>;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_function_r {
    pub n: usize,
    pub m: usize,
    pub p: *mut *mut gravity_function_t,
}
impl Clone for gravity_function_r {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_class_r {
    pub n: usize,
    pub m: usize,
    pub p: *mut *mut gravity_class_t,
}
impl Clone for gravity_class_r {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_object_r {
    pub n: usize,
    pub m: usize,
    pub p: *mut *mut gravity_object_t,
}
impl Clone for gravity_object_r {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum error_type_t {
    GRAVITY_ERROR_NONE = 0,
    GRAVITY_ERROR_SYNTAX = 1,
    GRAVITY_ERROR_SEMANTIC = 2,
    GRAVITY_ERROR_RUNTIME = 3,
    GRAVITY_ERROR_IO = 4,
    GRAVITY_WARNING = 5,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct error_desc_t {
    pub lineno: u32,
    pub colno: u32,
    pub fileid: u32,
    pub offset: u32,
    pub meta: *mut ::std::os::raw::c_void,
}
impl Clone for error_desc_t {
    fn clone(&self) -> Self {
        *self
    }
}
pub type gravity_log_callback =
    ::std::option::Option<unsafe extern "C" fn(message: *const ::std::os::raw::c_char,
                                               xdata: *mut ::std::os::raw::c_void)>;
pub type gravity_error_callback =
    ::std::option::Option<unsafe extern "C" fn(error_type: error_type_t,
                                               description: *const ::std::os::raw::c_char,
                                               error_desc: error_desc_t,
                                               xdata: *mut ::std::os::raw::c_void)>;
pub type gravity_unittest_callback =
    ::std::option::Option<unsafe extern "C" fn(error_type: error_type_t,
                                               desc: *const ::std::os::raw::c_char,
                                               note: *const ::std::os::raw::c_char,
                                               value: gravity_value_t,
                                               row: i32,
                                               col: i32,
                                               xdata: *mut ::std::os::raw::c_void)>;
pub type gravity_parser_callback =
    ::std::option::Option<unsafe extern "C" fn(token: *mut ::std::os::raw::c_void,
                                               xdata: *mut ::std::os::raw::c_void)>;
pub type gravity_precode_callback =
    ::std::option::Option<unsafe extern "C" fn(xdata: *mut ::std::os::raw::c_void)
                                               -> *const ::std::os::raw::c_char>;
pub type gravity_loadfile_callback =
    ::std::option::Option<unsafe extern "C" fn(file: *const ::std::os::raw::c_char,
                                               size: *mut usize,
                                               fileid: *mut u32,
                                               xdata: *mut ::std::os::raw::c_void)
                                               -> *const ::std::os::raw::c_char>;
pub type gravity_filename_callback =
    ::std::option::Option<unsafe extern "C" fn(fileid: u32, xdata: *mut ::std::os::raw::c_void)
                                               -> *const ::std::os::raw::c_char>;
pub type gravity_bridge_initinstance =
    ::std::option::Option<unsafe extern "C" fn(vm: *mut gravity_vm,
                                               xdata: *mut ::std::os::raw::c_void,
                                               instance: *mut gravity_instance_t,
                                               args: *mut gravity_value_t,
                                               nargs: i16)
                                               -> bool>;
pub type gravity_bridge_setvalue =
    ::std::option::Option<unsafe extern "C" fn(vm: *mut gravity_vm,
                                               xdata: *mut ::std::os::raw::c_void,
                                               target: gravity_value_t,
                                               key: *const ::std::os::raw::c_char,
                                               value: gravity_value_t)
                                               -> bool>;
pub type gravity_bridge_getvalue =
    ::std::option::Option<unsafe extern "C" fn(vm: *mut gravity_vm,
                                               xdata: *mut ::std::os::raw::c_void,
                                               target: gravity_value_t,
                                               key: *const ::std::os::raw::c_char,
                                               vindex: u32)
                                               -> bool>;
pub type gravity_bridge_setundef =
    ::std::option::Option<unsafe extern "C" fn(vm: *mut gravity_vm,
                                               xdata: *mut ::std::os::raw::c_void,
                                               target: gravity_value_t,
                                               key: *const ::std::os::raw::c_char,
                                               value: gravity_value_t)
                                               -> bool>;
pub type gravity_bridge_getundef =
    ::std::option::Option<unsafe extern "C" fn(vm: *mut gravity_vm,
                                               xdata: *mut ::std::os::raw::c_void,
                                               target: gravity_value_t,
                                               key: *const ::std::os::raw::c_char,
                                               vindex: u32)
                                               -> bool>;
pub type gravity_bridge_execute =
    ::std::option::Option<unsafe extern "C" fn(vm: *mut gravity_vm,
                                               xdata: *mut ::std::os::raw::c_void,
                                               args: *mut gravity_value_t,
                                               nargs: i16,
                                               vindex: u32)
                                               -> bool>;
pub type gravity_bridge_size =
    ::std::option::Option<unsafe extern "C" fn(vm: *mut gravity_vm, obj: *mut gravity_object_t)
                                               -> u32>;
pub type gravity_bridge_free =
    ::std::option::Option<unsafe extern "C" fn(vm: *mut gravity_vm, obj: *mut gravity_object_t)>;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_delegate_t {
    pub xdata: *mut ::std::os::raw::c_void,
    pub log_callback: gravity_log_callback,
    pub error_callback: gravity_error_callback,
    pub unittest_callback: gravity_unittest_callback,
    pub parser_callback: gravity_parser_callback,
    pub precode_callback: gravity_precode_callback,
    pub loadfile_callback: gravity_loadfile_callback,
    pub filename_callback: gravity_filename_callback,
    pub bridge_initinstance: gravity_bridge_initinstance,
    pub bridge_setvalue: gravity_bridge_setvalue,
    pub bridge_getvalue: gravity_bridge_getvalue,
    pub bridge_setundef: gravity_bridge_setundef,
    pub bridge_getundef: gravity_bridge_getundef,
    pub bridge_execute: gravity_bridge_execute,
    pub bridge_size: gravity_bridge_size,
    pub bridge_free: gravity_bridge_free,
}
impl Clone for gravity_delegate_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum gtoken_t {
    TOK_EOF = 0,
    TOK_ERROR = 1,
    TOK_COMMENT = 2,
    TOK_STRING = 3,
    TOK_NUMBER = 4,
    TOK_IDENTIFIER = 5,
    TOK_SPECIAL = 6,
    TOK_MACRO = 7,
    TOK_KEY_FUNC = 8,
    TOK_KEY_SUPER = 9,
    TOK_KEY_DEFAULT = 10,
    TOK_KEY_TRUE = 11,
    TOK_KEY_FALSE = 12,
    TOK_KEY_IF = 13,
    TOK_KEY_ELSE = 14,
    TOK_KEY_SWITCH = 15,
    TOK_KEY_BREAK = 16,
    TOK_KEY_CONTINUE = 17,
    TOK_KEY_RETURN = 18,
    TOK_KEY_WHILE = 19,
    TOK_KEY_REPEAT = 20,
    TOK_KEY_FOR = 21,
    TOK_KEY_IN = 22,
    TOK_KEY_ENUM = 23,
    TOK_KEY_CLASS = 24,
    TOK_KEY_STRUCT = 25,
    TOK_KEY_PRIVATE = 26,
    TOK_KEY_FILE = 27,
    TOK_KEY_INTERNAL = 28,
    TOK_KEY_PUBLIC = 29,
    TOK_KEY_STATIC = 30,
    TOK_KEY_EXTERN = 31,
    TOK_KEY_LAZY = 32,
    TOK_KEY_CONST = 33,
    TOK_KEY_VAR = 34,
    TOK_KEY_MODULE = 35,
    TOK_KEY_IMPORT = 36,
    TOK_KEY_CASE = 37,
    TOK_KEY_EVENT = 38,
    TOK_KEY_NULL = 39,
    TOK_KEY_UNDEFINED = 40,
    TOK_KEY_ISA = 41,
    TOK_KEY_CURRFUNC = 42,
    TOK_KEY_CURRARGS = 43,
    TOK_OP_SHIFT_LEFT = 44,
    TOK_OP_SHIFT_RIGHT = 45,
    TOK_OP_MUL = 46,
    TOK_OP_DIV = 47,
    TOK_OP_REM = 48,
    TOK_OP_BIT_AND = 49,
    TOK_OP_ADD = 50,
    TOK_OP_SUB = 51,
    TOK_OP_BIT_OR = 52,
    TOK_OP_BIT_XOR = 53,
    TOK_OP_BIT_NOT = 54,
    TOK_OP_RANGE_EXCLUDED = 55,
    TOK_OP_RANGE_INCLUDED = 56,
    TOK_OP_LESS = 57,
    TOK_OP_LESS_EQUAL = 58,
    TOK_OP_GREATER = 59,
    TOK_OP_GREATER_EQUAL = 60,
    TOK_OP_ISEQUAL = 61,
    TOK_OP_ISNOTEQUAL = 62,
    TOK_OP_ISIDENTICAL = 63,
    TOK_OP_ISNOTIDENTICAL = 64,
    TOK_OP_PATTERN_MATCH = 65,
    TOK_OP_AND = 66,
    TOK_OP_OR = 67,
    TOK_OP_TERNARY = 68,
    TOK_OP_ASSIGN = 69,
    TOK_OP_MUL_ASSIGN = 70,
    TOK_OP_DIV_ASSIGN = 71,
    TOK_OP_REM_ASSIGN = 72,
    TOK_OP_ADD_ASSIGN = 73,
    TOK_OP_SUB_ASSIGN = 74,
    TOK_OP_SHIFT_LEFT_ASSIGN = 75,
    TOK_OP_SHIFT_RIGHT_ASSIGN = 76,
    TOK_OP_BIT_AND_ASSIGN = 77,
    TOK_OP_BIT_OR_ASSIGN = 78,
    TOK_OP_BIT_XOR_ASSIGN = 79,
    TOK_OP_NOT = 80,
    TOK_OP_SEMICOLON = 81,
    TOK_OP_OPEN_PARENTHESIS = 82,
    TOK_OP_COLON = 83,
    TOK_OP_COMMA = 84,
    TOK_OP_DOT = 85,
    TOK_OP_CLOSED_PARENTHESIS = 86,
    TOK_OP_OPEN_SQUAREBRACKET = 87,
    TOK_OP_CLOSED_SQUAREBRACKET = 88,
    TOK_OP_OPEN_CURLYBRACE = 89,
    TOK_OP_CLOSED_CURLYBRACE = 90,
    TOK_END = 91,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum gliteral_t {
    LITERAL_STRING = 0,
    LITERAL_FLOAT = 1,
    LITERAL_INT = 2,
    LITERAL_BOOL = 3,
    LITERAL_STRING_INTERPOLATED = 4,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum gbuiltin_t {
    BUILTIN_NONE = 0,
    BUILTIN_LINE = 1,
    BUILTIN_COLUMN = 2,
    BUILTIN_FILE = 3,
    BUILTIN_FUNC = 4,
    BUILTIN_CLASS = 5,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gtoken_s {
    pub type_: gtoken_t,
    pub lineno: u32,
    pub colno: u32,
    pub position: u32,
    pub bytes: u32,
    pub length: u32,
    pub fileid: u32,
    pub builtin: gbuiltin_t,
    pub value: *const ::std::os::raw::c_char,
}
impl Clone for gtoken_s {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum gnode_n {
    NODE_LIST_STAT = 0,
    NODE_COMPOUND_STAT = 1,
    NODE_LABEL_STAT = 2,
    NODE_FLOW_STAT = 3,
    NODE_JUMP_STAT = 4,
    NODE_LOOP_STAT = 5,
    NODE_EMPTY_STAT = 6,
    NODE_ENUM_DECL = 7,
    NODE_FUNCTION_DECL = 8,
    NODE_VARIABLE_DECL = 9,
    NODE_CLASS_DECL = 10,
    NODE_MODULE_DECL = 11,
    NODE_VARIABLE = 12,
    NODE_BINARY_EXPR = 13,
    NODE_UNARY_EXPR = 14,
    NODE_FILE_EXPR = 15,
    NODE_LIST_EXPR = 16,
    NODE_LITERAL_EXPR = 17,
    NODE_IDENTIFIER_EXPR = 18,
    NODE_POSTFIX_EXPR = 19,
    NODE_KEYWORD_EXPR = 20,
    NODE_CALL_EXPR = 21,
    NODE_SUBSCRIPT_EXPR = 22,
    NODE_ACCESS_EXPR = 23,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum gnode_location_type {
    LOCATION_LOCAL = 0,
    LOCATION_GLOBAL = 1,
    LOCATION_UPVALUE = 2,
    LOCATION_CLASS_IVAR_SAME = 3,
    LOCATION_CLASS_IVAR_OUTER = 4,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_t {
    pub tag: gnode_n,
    pub refcount: u32,
    pub token: gtoken_s,
    pub is_assignment: bool,
    pub meta: *mut ::std::os::raw::c_void,
    pub decl: *mut ::std::os::raw::c_void,
}
impl Clone for gnode_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gupvalue_t {
    pub node: *mut gnode_t,
    pub index: u32,
    pub selfindex: u32,
    pub is_direct: bool,
}
impl Clone for gupvalue_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_r {
    pub n: usize,
    pub m: usize,
    pub p: *mut *mut gnode_t,
}
impl Clone for gnode_r {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gupvalue_r {
    pub n: usize,
    pub m: usize,
    pub p: *mut *mut gupvalue_t,
}
impl Clone for gupvalue_r {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct symboltable_t([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_location_t {
    pub type_: gnode_location_type,
    pub index: u16,
    pub nup: u16,
}
impl Clone for gnode_location_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_compound_stmt_t {
    pub base: gnode_t,
    pub symtable: *mut symboltable_t,
    pub stmts: *mut gnode_r,
    pub nclose: u32,
}
impl Clone for gnode_compound_stmt_t {
    fn clone(&self) -> Self {
        *self
    }
}
pub type gnode_list_stmt_t = gnode_compound_stmt_t;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_label_stmt_t {
    pub base: gnode_t,
    pub expr: *mut gnode_t,
    pub stmt: *mut gnode_t,
}
impl Clone for gnode_label_stmt_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_flow_stmt_t {
    pub base: gnode_t,
    pub cond: *mut gnode_t,
    pub stmt: *mut gnode_t,
    pub elsestmt: *mut gnode_t,
}
impl Clone for gnode_flow_stmt_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_loop_stmt_t {
    pub base: gnode_t,
    pub cond: *mut gnode_t,
    pub stmt: *mut gnode_t,
    pub expr: *mut gnode_t,
    pub nclose: u32,
}
impl Clone for gnode_loop_stmt_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_jump_stmt_t {
    pub base: gnode_t,
    pub expr: *mut gnode_t,
}
impl Clone for gnode_jump_stmt_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_function_decl_t {
    pub base: gnode_t,
    pub env: *mut gnode_t,
    pub access: gtoken_t,
    pub storage: gtoken_t,
    pub symtable: *mut symboltable_t,
    pub identifier: *const ::std::os::raw::c_char,
    pub params: *mut gnode_r,
    pub block: *mut gnode_compound_stmt_t,
    pub nlocals: u16,
    pub nparams: u16,
    pub uplist: *mut gupvalue_r,
}
impl Clone for gnode_function_decl_t {
    fn clone(&self) -> Self {
        *self
    }
}
pub type gnode_function_expr_t = gnode_function_decl_t;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_var_t {
    pub base: gnode_t,
    pub env: *mut gnode_t,
    pub identifier: *const ::std::os::raw::c_char,
    pub annotation_type: *const ::std::os::raw::c_char,
    pub expr: *mut gnode_t,
    pub access: gtoken_t,
    pub index: u16,
    pub upvalue: bool,
}
impl Clone for gnode_var_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_variable_decl_t {
    pub base: gnode_t,
    pub type_: gtoken_t,
    pub access: gtoken_t,
    pub storage: gtoken_t,
    pub decls: *mut gnode_r,
}
impl Clone for gnode_variable_decl_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_enum_decl_t {
    pub base: gnode_t,
    pub env: *mut gnode_t,
    pub access: gtoken_t,
    pub storage: gtoken_t,
    pub symtable: *mut symboltable_t,
    pub identifier: *const ::std::os::raw::c_char,
}
impl Clone for gnode_enum_decl_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_class_decl_t {
    pub base: gnode_t,
    pub bridge: bool,
    pub is_struct: bool,
    pub env: *mut gnode_t,
    pub access: gtoken_t,
    pub storage: gtoken_t,
    pub identifier: *const ::std::os::raw::c_char,
    pub superclass: *mut gnode_t,
    pub protocols: *mut gnode_r,
    pub decls: *mut gnode_r,
    pub symtable: *mut symboltable_t,
    pub data: *mut ::std::os::raw::c_void,
    pub nivar: u32,
    pub nsvar: u32,
}
impl Clone for gnode_class_decl_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_module_decl_t {
    pub base: gnode_t,
    pub env: *mut gnode_t,
    pub access: gtoken_t,
    pub storage: gtoken_t,
    pub identifier: *const ::std::os::raw::c_char,
    pub decls: *mut gnode_r,
    pub symtable: *mut symboltable_t,
}
impl Clone for gnode_module_decl_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_binary_expr_t {
    pub base: gnode_t,
    pub op: gtoken_t,
    pub left: *mut gnode_t,
    pub right: *mut gnode_t,
}
impl Clone for gnode_binary_expr_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_unary_expr_t {
    pub base: gnode_t,
    pub op: gtoken_t,
    pub expr: *mut gnode_t,
}
impl Clone for gnode_unary_expr_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_file_expr_t {
    pub base: gnode_t,
    pub identifiers: *mut cstring_r,
    pub location: gnode_location_t,
}
impl Clone for gnode_file_expr_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_literal_expr_t {
    pub base: gnode_t,
    pub type_: gliteral_t,
    pub len: u32,
    pub value: gnode_literal_expr_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_literal_expr_t__bindgen_ty_1 {
    pub str: __BindgenUnionField<*mut ::std::os::raw::c_char>,
    pub d: __BindgenUnionField<f64>,
    pub n64: __BindgenUnionField<i64>,
    pub r: __BindgenUnionField<*mut gnode_r>,
    pub bindgen_union_field: u64,
}
impl Clone for gnode_literal_expr_t__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl Clone for gnode_literal_expr_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_identifier_expr_t {
    pub base: gnode_t,
    pub value: *const ::std::os::raw::c_char,
    pub value2: *const ::std::os::raw::c_char,
    pub symbol: *mut gnode_t,
    pub location: gnode_location_t,
    pub upvalue: *mut gupvalue_t,
}
impl Clone for gnode_identifier_expr_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_keyword_expr_t {
    pub base: gnode_t,
}
impl Clone for gnode_keyword_expr_t {
    fn clone(&self) -> Self {
        *self
    }
}
pub type gnode_empty_stmt_t = gnode_keyword_expr_t;
pub type gnode_base_t = gnode_keyword_expr_t;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_postfix_expr_t {
    pub base: gnode_t,
    pub id: *mut gnode_t,
    pub list: *mut gnode_r,
}
impl Clone for gnode_postfix_expr_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_postfix_subexpr_t {
    pub base: gnode_t,
    pub __bindgen_anon_1: gnode_postfix_subexpr_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_postfix_subexpr_t__bindgen_ty_1 {
    pub expr: __BindgenUnionField<*mut gnode_t>,
    pub args: __BindgenUnionField<*mut gnode_r>,
    pub bindgen_union_field: u64,
}
impl Clone for gnode_postfix_subexpr_t__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl Clone for gnode_postfix_subexpr_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_list_expr_t {
    pub base: gnode_t,
    pub ismap: bool,
    pub list1: *mut gnode_r,
    pub list2: *mut gnode_r,
}
impl Clone for gnode_list_expr_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gravity_compiler_t([u8; 0]);
pub type vm_filter_cb = ::std::option::Option<unsafe extern "C" fn(obj: *mut gravity_object_t)
                                                                   -> bool>;
pub type vm_transfer_cb = ::std::option::Option<unsafe extern "C" fn(vm: *mut gravity_vm,
                                                                     obj: *mut gravity_object_t)>;
pub type vm_cleanup_cb = ::std::option::Option<unsafe extern "C" fn(vm: *mut gravity_vm)>;


extern "C" {
    pub fn opendir(__name: *const ::std::os::raw::c_char) -> *mut DIR;
    pub fn fdopendir(__fd: ::std::os::raw::c_int) -> *mut DIR;
    pub fn closedir(__dirp: *mut DIR) -> ::std::os::raw::c_int;
    pub fn readdir(__dirp: *mut DIR) -> *mut dirent;
    pub fn readdir_r(__dirp: *mut DIR,
                     __entry: *mut dirent,
                     __result: *mut *mut dirent)
                     -> ::std::os::raw::c_int;
    pub fn rewinddir(__dirp: *mut DIR);
    pub fn seekdir(__dirp: *mut DIR, __pos: ::std::os::raw::c_long);
    pub fn telldir(__dirp: *mut DIR) -> ::std::os::raw::c_long;
    pub fn dirfd(__dirp: *mut DIR) -> ::std::os::raw::c_int;
    pub fn scandir(__dir: *const ::std::os::raw::c_char,
                   __namelist: *mut *mut *mut dirent,
                   __selector:
                       ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                      *const dirent)
                                                 -> ::std::os::raw::c_int>,
                   __cmp:
                       ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                      *mut *const dirent,
                                                                  arg2:
                                                                      *mut *const dirent)
                                                 -> ::std::os::raw::c_int>)
-> ::std::os::raw::c_int;
    pub fn alphasort(__e1: *mut *const dirent, __e2: *mut *const dirent) -> ::std::os::raw::c_int;
    pub fn getdirentries(__fd: ::std::os::raw::c_int,
                         __buf: *mut ::std::os::raw::c_char,
                         __nbytes: usize,
                         __basep: *mut __off_t)
                         -> __ssize_t;
    pub fn nanotime() -> nanotime_t;
    pub fn microtime(tstart: nanotime_t, tend: nanotime_t) -> f64;
    pub fn millitime(tstart: nanotime_t, tend: nanotime_t) -> f64;
    pub fn readline(prompt: *mut ::std::os::raw::c_char,
                    length: *mut ::std::os::raw::c_int)
                    -> *mut ::std::os::raw::c_char;
    pub fn file_size(path: *const ::std::os::raw::c_char) -> u64;
    pub fn file_read(path: *const ::std::os::raw::c_char,
                     len: *mut usize)
                     -> *const ::std::os::raw::c_char;
    pub fn file_exists(path: *const ::std::os::raw::c_char) -> bool;
    pub fn file_buildpath(filename: *const ::std::os::raw::c_char,
                          dirpath: *const ::std::os::raw::c_char)
                          -> *const ::std::os::raw::c_char;
    pub fn file_write(path: *const ::std::os::raw::c_char,
                      buffer: *const ::std::os::raw::c_char,
                      len: usize)
                      -> bool;
    pub fn is_directory(path: *const ::std::os::raw::c_char) -> bool;
    pub fn directory_init(path: *const ::std::os::raw::c_char) -> *mut DIR;
    pub fn directory_read(ref_: *mut DIR) -> *const ::std::os::raw::c_char;
    pub fn string_nocasencmp(s1: *const ::std::os::raw::c_char,
                             s2: *const ::std::os::raw::c_char,
                             n: usize)
                             -> ::std::os::raw::c_int;
    pub fn string_casencmp(s1: *const ::std::os::raw::c_char,
                           s2: *const ::std::os::raw::c_char,
                           n: usize)
                           -> ::std::os::raw::c_int;
    pub fn string_cmp(s1: *const ::std::os::raw::c_char,
                      s2: *const ::std::os::raw::c_char)
                      -> ::std::os::raw::c_int;
    pub fn string_dup(s1: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
    pub fn string_ndup(s1: *const ::std::os::raw::c_char,
                       n: usize)
                       -> *const ::std::os::raw::c_char;
    pub fn string_reverse(p: *mut ::std::os::raw::c_char);
    pub fn string_size(p: *const ::std::os::raw::c_char) -> u32;
    pub fn utf8_charbytes(s: *const ::std::os::raw::c_char, i: u32) -> u32;
    pub fn utf8_nbytes(n: u32) -> u32;
    pub fn utf8_encode(buffer: *mut ::std::os::raw::c_char, value: u32) -> u32;
    pub fn utf8_len(s: *const ::std::os::raw::c_char, nbytes: u32) -> u32;
    pub fn utf8_reverse(p: *mut ::std::os::raw::c_char) -> bool;
    pub fn power_of2_ceil(n: u32) -> u32;
    pub fn number_from_hex(s: *const ::std::os::raw::c_char, len: u32) -> i64;
    pub fn number_from_oct(s: *const ::std::os::raw::c_char, len: u32) -> i64;
    pub fn number_from_bin(s: *const ::std::os::raw::c_char, len: u32) -> i64;
    pub fn json_new() -> *mut json_t;
    pub fn json_begin_object(json: *mut json_t, key: *const ::std::os::raw::c_char);
    pub fn json_end_object(json: *mut json_t);
    pub fn json_begin_array(json: *mut json_t, key: *const ::std::os::raw::c_char);
    pub fn json_end_array(json: *mut json_t);
    pub fn json_add_cstring(json: *mut json_t,
                            key: *const ::std::os::raw::c_char,
                            value: *const ::std::os::raw::c_char);
    pub fn json_add_string(json: *mut json_t,
                           key: *const ::std::os::raw::c_char,
                           value: *const ::std::os::raw::c_char,
                           len: usize);
    pub fn json_add_int(json: *mut json_t, key: *const ::std::os::raw::c_char, value: i64);
    pub fn json_add_double(json: *mut json_t, key: *const ::std::os::raw::c_char, value: f64);
    pub fn json_add_bool(json: *mut json_t, key: *const ::std::os::raw::c_char, value: bool);
    pub fn json_add_null(json: *mut json_t, key: *const ::std::os::raw::c_char);
    pub fn json_free(json: *mut json_t);
    pub fn json_buffer(json: *mut json_t, len: *mut usize) -> *const ::std::os::raw::c_char;
    pub fn json_write_file(json: *mut json_t, path: *const ::std::os::raw::c_char) -> bool;
    pub fn json_pop(json: *mut json_t, n: u32);
    pub fn imaxabs(__n: intmax_t) -> intmax_t;
    pub fn imaxdiv(__numer: intmax_t, __denom: intmax_t) -> imaxdiv_t;
    pub fn strtoimax(__nptr: *const ::std::os::raw::c_char,
                     __endptr: *mut *mut ::std::os::raw::c_char,
                     __base: ::std::os::raw::c_int)
                     -> intmax_t;
    pub fn strtoumax(__nptr: *const ::std::os::raw::c_char,
                     __endptr: *mut *mut ::std::os::raw::c_char,
                     __base: ::std::os::raw::c_int)
                     -> uintmax_t;
    pub fn wcstoimax(__nptr: *const __gwchar_t,
                     __endptr: *mut *mut __gwchar_t,
                     __base: ::std::os::raw::c_int)
                     -> intmax_t;
    pub fn wcstoumax(__nptr: *const __gwchar_t,
                     __endptr: *mut *mut __gwchar_t,
                     __base: ::std::os::raw::c_int)
                     -> uintmax_t;
    #[link_name = "json_value_none"]
    pub static json_value_none: _json_value;
    pub fn json_parse(json: *const ::std::os::raw::c_char, length: usize) -> *mut json_value;
    pub fn json_parse_ex(settings: *mut json_settings,
                         json: *const ::std::os::raw::c_char,
                         length: usize,
                         error: *mut ::std::os::raw::c_char)
                         -> *mut json_value;
    pub fn json_value_free(arg1: *mut json_value);
    pub fn json_value_free_ex(settings: *mut json_settings, arg1: *mut json_value);
    #[link_name = "gravity_class_object"]
    pub static mut gravity_class_object: *mut gravity_class_t;
    #[link_name = "gravity_class_bool"]
    pub static mut gravity_class_bool: *mut gravity_class_t;
    #[link_name = "gravity_class_null"]
    pub static mut gravity_class_null: *mut gravity_class_t;
    #[link_name = "gravity_class_int"]
    pub static mut gravity_class_int: *mut gravity_class_t;
    #[link_name = "gravity_class_float"]
    pub static mut gravity_class_float: *mut gravity_class_t;
    #[link_name = "gravity_class_function"]
    pub static mut gravity_class_function: *mut gravity_class_t;
    #[link_name = "gravity_class_closure"]
    pub static mut gravity_class_closure: *mut gravity_class_t;
    #[link_name = "gravity_class_fiber"]
    pub static mut gravity_class_fiber: *mut gravity_class_t;
    #[link_name = "gravity_class_class"]
    pub static mut gravity_class_class: *mut gravity_class_t;
    #[link_name = "gravity_class_string"]
    pub static mut gravity_class_string: *mut gravity_class_t;
    #[link_name = "gravity_class_instance"]
    pub static mut gravity_class_instance: *mut gravity_class_t;
    #[link_name = "gravity_class_list"]
    pub static mut gravity_class_list: *mut gravity_class_t;
    #[link_name = "gravity_class_map"]
    pub static mut gravity_class_map: *mut gravity_class_t;
    #[link_name = "gravity_class_module"]
    pub static mut gravity_class_module: *mut gravity_class_t;
    #[link_name = "gravity_class_range"]
    pub static mut gravity_class_range: *mut gravity_class_t;
    #[link_name = "gravity_class_upvalue"]
    pub static mut gravity_class_upvalue: *mut gravity_class_t;
    pub fn gravity_module_new(vm: *mut gravity_vm,
                              identifier: *const ::std::os::raw::c_char)
                              -> *mut gravity_module_t;
    pub fn gravity_module_free(vm: *mut gravity_vm, m: *mut gravity_module_t);
    pub fn gravity_module_blacken(vm: *mut gravity_vm, m: *mut gravity_module_t);
    pub fn gravity_module_size(vm: *mut gravity_vm, m: *mut gravity_module_t) -> u32;
    pub fn gravity_function_new(vm: *mut gravity_vm,
                                identifier: *const ::std::os::raw::c_char,
                                nparams: u16,
                                nlocals: u16,
                                ntemps: u16,
                                code: *mut ::std::os::raw::c_void)
                                -> *mut gravity_function_t;
    pub fn gravity_function_new_internal(vm: *mut gravity_vm,
                                         identifier: *const ::std::os::raw::c_char,
                                         exec: gravity_c_internal,
                                         nparams: u16)
                                         -> *mut gravity_function_t;
    pub fn gravity_function_new_special(vm: *mut gravity_vm,
                                        identifier: *const ::std::os::raw::c_char,
                                        index: u16,
                                        getter: *mut ::std::os::raw::c_void,
                                        setter: *mut ::std::os::raw::c_void)
                                        -> *mut gravity_function_t;
    pub fn gravity_function_new_bridged(vm: *mut gravity_vm,
                                        identifier: *const ::std::os::raw::c_char,
                                        xdata: *mut ::std::os::raw::c_void)
                                        -> *mut gravity_function_t;
    pub fn gravity_function_cpool_add(vm: *mut gravity_vm,
                                      f: *mut gravity_function_t,
                                      v: gravity_value_t)
                                      -> u16;
    pub fn gravity_function_cpool_get(f: *mut gravity_function_t, i: u16) -> gravity_value_t;
    pub fn gravity_function_dump(f: *mut gravity_function_t, codef: code_dump_function);
    pub fn gravity_function_setouter(f: *mut gravity_function_t, outer: *mut gravity_object_t);
    pub fn gravity_function_setxdata(f: *mut gravity_function_t,
                                     xdata: *mut ::std::os::raw::c_void);
    pub fn gravity_function_serialize(f: *mut gravity_function_t, json: *mut json_t);
    pub fn gravity_bytecode_deserialize(buffer: *const ::std::os::raw::c_char,
                                        len: usize,
                                        ninst: *mut u32)
                                        -> *mut u32;
    pub fn gravity_function_deserialize(vm: *mut gravity_vm,
                                        json: *mut json_value)
                                        -> *mut gravity_function_t;
    pub fn gravity_function_free(vm: *mut gravity_vm, f: *mut gravity_function_t);
    pub fn gravity_function_blacken(vm: *mut gravity_vm, f: *mut gravity_function_t);
    pub fn gravity_function_size(vm: *mut gravity_vm, f: *mut gravity_function_t) -> u32;
    pub fn gravity_closure_new(vm: *mut gravity_vm,
                               f: *mut gravity_function_t)
                               -> *mut gravity_closure_t;
    pub fn gravity_closure_free(vm: *mut gravity_vm, closure: *mut gravity_closure_t);
    pub fn gravity_closure_size(vm: *mut gravity_vm, closure: *mut gravity_closure_t) -> u32;
    pub fn gravity_closure_blacken(vm: *mut gravity_vm, closure: *mut gravity_closure_t);
    pub fn gravity_upvalue_new(vm: *mut gravity_vm,
                               value: *mut gravity_value_t)
                               -> *mut gravity_upvalue_t;
    pub fn gravity_upvalue_size(vm: *mut gravity_vm, upvalue: *mut gravity_upvalue_t) -> u32;
    pub fn gravity_upvalue_blacken(vm: *mut gravity_vm, upvalue: *mut gravity_upvalue_t);
    pub fn gravity_upvalue_free(vm: *mut gravity_vm, upvalue: *mut gravity_upvalue_t);
    pub fn gravity_class_bind(c: *mut gravity_class_t,
                              key: *const ::std::os::raw::c_char,
                              value: gravity_value_t);
    pub fn gravity_class_getsuper(c: *mut gravity_class_t) -> *mut gravity_class_t;
    pub fn gravity_class_grow(c: *mut gravity_class_t, n: u32) -> bool;
    pub fn gravity_class_setsuper(subclass: *mut gravity_class_t,
                                  superclass: *mut gravity_class_t)
                                  -> bool;
    pub fn gravity_class_new_single(vm: *mut gravity_vm,
                                    identifier: *const ::std::os::raw::c_char,
                                    nfields: u32)
                                    -> *mut gravity_class_t;
    pub fn gravity_class_new_pair(vm: *mut gravity_vm,
                                  identifier: *const ::std::os::raw::c_char,
                                  superclass: *mut gravity_class_t,
                                  nivar: u32,
                                  nsvar: u32)
                                  -> *mut gravity_class_t;
    pub fn gravity_class_get_meta(c: *mut gravity_class_t) -> *mut gravity_class_t;
    pub fn gravity_class_is_meta(c: *mut gravity_class_t) -> bool;
    pub fn gravity_class_count_ivars(c: *mut gravity_class_t) -> u32;
    pub fn gravity_class_dump(c: *mut gravity_class_t);
    pub fn gravity_class_setxdata(c: *mut gravity_class_t, xdata: *mut ::std::os::raw::c_void);
    pub fn gravity_class_add_ivar(c: *mut gravity_class_t,
                                  identifier: *const ::std::os::raw::c_char)
                                  -> i16;
    pub fn gravity_class_serialize(c: *mut gravity_class_t, json: *mut json_t);
    pub fn gravity_class_deserialize(vm: *mut gravity_vm,
                                     json: *mut json_value)
                                     -> *mut gravity_class_t;
    pub fn gravity_class_free(vm: *mut gravity_vm, c: *mut gravity_class_t);
    pub fn gravity_class_free_core(vm: *mut gravity_vm, c: *mut gravity_class_t);
    pub fn gravity_class_lookup(c: *mut gravity_class_t,
                                key: gravity_value_t)
                                -> *mut gravity_object_t;
    pub fn gravity_class_lookup_closure(c: *mut gravity_class_t,
                                        key: gravity_value_t)
                                        -> *mut gravity_closure_t;
    pub fn gravity_class_lookup_constructor(c: *mut gravity_class_t,
                                            nparams: u32)
                                            -> *mut gravity_closure_t;
    pub fn gravity_class_blacken(vm: *mut gravity_vm, c: *mut gravity_class_t);
    pub fn gravity_class_size(vm: *mut gravity_vm, c: *mut gravity_class_t) -> u32;
    pub fn gravity_fiber_new(vm: *mut gravity_vm,
                             closure: *mut gravity_closure_t,
                             nstack: u32,
                             nframes: u32)
                             -> *mut gravity_fiber_t;
    pub fn gravity_fiber_reassign(fiber: *mut gravity_fiber_t,
                                  closure: *mut gravity_closure_t,
                                  nargs: u16);
    pub fn gravity_fiber_seterror(fiber: *mut gravity_fiber_t,
                                  error: *const ::std::os::raw::c_char);
    pub fn gravity_fiber_free(vm: *mut gravity_vm, fiber: *mut gravity_fiber_t);
    pub fn gravity_fiber_blacken(vm: *mut gravity_vm, fiber: *mut gravity_fiber_t);
    pub fn gravity_fiber_size(vm: *mut gravity_vm, fiber: *mut gravity_fiber_t) -> u32;
    pub fn gravity_instance_new(vm: *mut gravity_vm,
                                c: *mut gravity_class_t)
                                -> *mut gravity_instance_t;
    pub fn gravity_instance_dup(vm: *mut gravity_vm,
                                src: *mut gravity_instance_t)
                                -> *mut gravity_instance_t;
    pub fn gravity_instance_setivar(instance: *mut gravity_instance_t,
                                    idx: u32,
                                    value: gravity_value_t);
    pub fn gravity_instance_setxdata(i: *mut gravity_instance_t,
                                     xdata: *mut ::std::os::raw::c_void);
    pub fn gravity_instance_free(vm: *mut gravity_vm, i: *mut gravity_instance_t);
    pub fn gravity_instance_lookup_event(i: *mut gravity_instance_t,
                                         name: *const ::std::os::raw::c_char)
                                         -> *mut gravity_closure_t;
    pub fn gravity_instance_blacken(vm: *mut gravity_vm, i: *mut gravity_instance_t);
    pub fn gravity_instance_size(vm: *mut gravity_vm, i: *mut gravity_instance_t) -> u32;
    pub fn gravity_value_equals(v1: gravity_value_t, v2: gravity_value_t) -> bool;
    pub fn gravity_value_hash(value: gravity_value_t) -> u32;
    pub fn gravity_value_getclass(v: gravity_value_t) -> *mut gravity_class_t;
    pub fn gravity_value_getsuper(v: gravity_value_t) -> *mut gravity_class_t;
    pub fn gravity_value_free(vm: *mut gravity_vm, v: gravity_value_t);
    pub fn gravity_value_serialize(v: gravity_value_t, json: *mut json_t);
    pub fn gravity_value_dump(v: gravity_value_t, buffer: *mut ::std::os::raw::c_char, len: u16);
    pub fn gravity_value_isobject(v: gravity_value_t) -> bool;
    pub fn gravity_value_xdata(value: gravity_value_t) -> *mut ::std::os::raw::c_void;
    pub fn gravity_value_blacken(vm: *mut gravity_vm, v: gravity_value_t);
    pub fn gravity_value_size(vm: *mut gravity_vm, v: gravity_value_t) -> u32;
    pub fn gravity_object_serialize(obj: *mut gravity_object_t, json: *mut json_t);
    pub fn gravity_object_deserialize(vm: *mut gravity_vm,
                                      entry: *mut json_value)
                                      -> *mut gravity_object_t;
    pub fn gravity_object_free(vm: *mut gravity_vm, obj: *mut gravity_object_t);
    pub fn gravity_object_blacken(vm: *mut gravity_vm, obj: *mut gravity_object_t);
    pub fn gravity_object_size(vm: *mut gravity_vm, obj: *mut gravity_object_t) -> u32;
    pub fn gravity_object_debug(obj: *mut gravity_object_t) -> *const ::std::os::raw::c_char;
    pub fn gravity_list_new(vm: *mut gravity_vm, n: u32) -> *mut gravity_list_t;
    pub fn gravity_list_from_array(vm: *mut gravity_vm,
                                   n: u32,
                                   p: *mut gravity_value_t)
                                   -> *mut gravity_list_t;
    pub fn gravity_list_free(vm: *mut gravity_vm, list: *mut gravity_list_t);
    pub fn gravity_list_append_list(vm: *mut gravity_vm,
                                    list1: *mut gravity_list_t,
                                    list2: *mut gravity_list_t);
    pub fn gravity_list_blacken(vm: *mut gravity_vm, list: *mut gravity_list_t);
    pub fn gravity_list_size(vm: *mut gravity_vm, list: *mut gravity_list_t) -> u32;
    pub fn gravity_map_new(vm: *mut gravity_vm, n: u32) -> *mut gravity_map_t;
    pub fn gravity_map_free(vm: *mut gravity_vm, map: *mut gravity_map_t);
    pub fn gravity_map_append_map(vm: *mut gravity_vm,
                                  map1: *mut gravity_map_t,
                                  map2: *mut gravity_map_t);
    pub fn gravity_map_insert(vm: *mut gravity_vm,
                              map: *mut gravity_map_t,
                              key: gravity_value_t,
                              value: gravity_value_t);
    pub fn gravity_map_blacken(vm: *mut gravity_vm, map: *mut gravity_map_t);
    pub fn gravity_map_size(vm: *mut gravity_vm, map: *mut gravity_map_t) -> u32;
    pub fn gravity_range_new(vm: *mut gravity_vm,
                             from: gravity_int_t,
                             to: gravity_int_t,
                             inclusive: bool)
                             -> *mut gravity_range_t;
    pub fn gravity_range_free(vm: *mut gravity_vm, range: *mut gravity_range_t);
    pub fn gravity_range_blacken(vm: *mut gravity_vm, range: *mut gravity_range_t);
    pub fn gravity_range_size(vm: *mut gravity_vm, range: *mut gravity_range_t) -> u32;
    /// MARK: - STRING -
    pub fn gravity_string_to_value(vm: *mut gravity_vm,
                                   s: *const ::std::os::raw::c_char,
                                   len: u32)
                                   -> gravity_value_t;
    pub fn gravity_string_new(vm: *mut gravity_vm,
                              s: *mut ::std::os::raw::c_char,
                              len: u32,
                              alloc: u32)
                              -> *mut gravity_string_t;
    pub fn gravity_string_free(vm: *mut gravity_vm, value: *mut gravity_string_t);
    pub fn gravity_string_blacken(vm: *mut gravity_vm, string: *mut gravity_string_t);
    pub fn gravity_string_size(vm: *mut gravity_vm, string: *mut gravity_string_t) -> u32;
    pub fn gravity_hash_keyvaluefree(table: *mut gravity_hash_t,
                                     key: gravity_value_t,
                                     value: gravity_value_t,
                                     data: *mut ::std::os::raw::c_void);
    pub fn gravity_hash_keyfree(table: *mut gravity_hash_t,
                                key: gravity_value_t,
                                value: gravity_value_t,
                                data: *mut ::std::os::raw::c_void);
    pub fn gravity_hash_valuefree(table: *mut gravity_hash_t,
                                  key: gravity_value_t,
                                  value: gravity_value_t,
                                  data: *mut ::std::os::raw::c_void);
    pub fn token_string(token: gtoken_s, len: *mut u32) -> *const ::std::os::raw::c_char;
    pub fn token_name(token: gtoken_t) -> *const ::std::os::raw::c_char;
    pub fn token_keyword(buffer: *const ::std::os::raw::c_char, len: i32) -> gtoken_t;
    pub fn token_special_builtin(token: *mut gtoken_s) -> gtoken_t;
    pub fn token_keywords_indexes(idx_start: *mut u32, idx_end: *mut u32);
    pub fn token_literal_name(value: gliteral_t) -> *const ::std::os::raw::c_char;
    pub fn token_islabel_statement(token: gtoken_t) -> bool;
    pub fn token_isflow_statement(token: gtoken_t) -> bool;
    pub fn token_isloop_statement(token: gtoken_t) -> bool;
    pub fn token_isjump_statement(token: gtoken_t) -> bool;
    pub fn token_iscompound_statement(token: gtoken_t) -> bool;
    pub fn token_isdeclaration_statement(token: gtoken_t) -> bool;
    pub fn token_isempty_statement(token: gtoken_t) -> bool;
    pub fn token_isimport_statement(token: gtoken_t) -> bool;
    pub fn token_isspecial_statement(token: gtoken_t) -> bool;
    pub fn token_isoperator(token: gtoken_t) -> bool;
    pub fn token_ismacro(token: gtoken_t) -> bool;
    pub fn token_iserror(token: gtoken_t) -> bool;
    pub fn token_iseof(token: gtoken_t) -> bool;
    pub fn token_isidentifier(token: gtoken_t) -> bool;
    pub fn token_isvariable_declaration(token: gtoken_t) -> bool;
    pub fn token_isstatement(token: gtoken_t) -> bool;
    pub fn token_isassignment(token: gtoken_t) -> bool;
    pub fn token_isvariable_assignment(token: gtoken_t) -> bool;
    pub fn token_isaccess_specifier(token: gtoken_t) -> bool;
    pub fn token_isstorage_specifier(token: gtoken_t) -> bool;
    pub fn token_isprimary_expression(token: gtoken_t) -> bool;
    pub fn token_isexpression_statement(token: gtoken_t) -> bool;
    pub fn gnode_jump_stat_create(token: gtoken_s,
                                  expr: *mut gnode_t,
                                  decl: *mut gnode_t)
                                  -> *mut gnode_t;
    pub fn gnode_label_stat_create(token: gtoken_s,
                                   expr: *mut gnode_t,
                                   stmt: *mut gnode_t,
                                   decl: *mut gnode_t)
                                   -> *mut gnode_t;
    pub fn gnode_flow_stat_create(token: gtoken_s,
                                  cond: *mut gnode_t,
                                  stmt1: *mut gnode_t,
                                  stmt2: *mut gnode_t,
                                  decl: *mut gnode_t)
                                  -> *mut gnode_t;
    pub fn gnode_loop_stat_create(token: gtoken_s,
                                  cond: *mut gnode_t,
                                  stmt: *mut gnode_t,
                                  expr: *mut gnode_t,
                                  decl: *mut gnode_t)
                                  -> *mut gnode_t;
    pub fn gnode_block_stat_create(type_: gnode_n,
                                   token: gtoken_s,
                                   stmts: *mut gnode_r,
                                   decl: *mut gnode_t)
                                   -> *mut gnode_t;
    pub fn gnode_empty_stat_create(token: gtoken_s, decl: *mut gnode_t) -> *mut gnode_t;
    pub fn gnode_enum_decl_create(token: gtoken_s,
                                  identifier: *const ::std::os::raw::c_char,
                                  access_specifier: gtoken_t,
                                  storage_specifier: gtoken_t,
                                  symtable: *mut symboltable_t,
                                  meta: *mut ::std::os::raw::c_void,
                                  decl: *mut gnode_t)
                                  -> *mut gnode_t;
    pub fn gnode_class_decl_create(token: gtoken_s,
                                   identifier: *const ::std::os::raw::c_char,
                                   access_specifier: gtoken_t,
                                   storage_specifier: gtoken_t,
                                   superclass: *mut gnode_t,
                                   protocols: *mut gnode_r,
                                   declarations: *mut gnode_r,
                                   is_struct: bool,
                                   meta: *mut ::std::os::raw::c_void,
                                   decl: *mut gnode_t)
                                   -> *mut gnode_t;
    pub fn gnode_module_decl_create(token: gtoken_s,
                                    identifier: *const ::std::os::raw::c_char,
                                    access_specifier: gtoken_t,
                                    storage_specifier: gtoken_t,
                                    declarations: *mut gnode_r,
                                    meta: *mut ::std::os::raw::c_void,
                                    decl: *mut gnode_t)
                                    -> *mut gnode_t;
    pub fn gnode_variable_decl_create(token: gtoken_s,
                                      type_: gtoken_t,
                                      access_specifier: gtoken_t,
                                      storage_specifier: gtoken_t,
                                      declarations: *mut gnode_r,
                                      meta: *mut ::std::os::raw::c_void,
                                      decl: *mut gnode_t)
                                      -> *mut gnode_t;
    pub fn gnode_variable_create(token: gtoken_s,
                                 identifier: *const ::std::os::raw::c_char,
                                 annotation_type: *const ::std::os::raw::c_char,
                                 storage_specifier: gtoken_t,
                                 expr: *mut gnode_t,
                                 decl: *mut gnode_t)
                                 -> *mut gnode_t;
    pub fn gnode_function_decl_create(token: gtoken_s,
                                      identifier: *const ::std::os::raw::c_char,
                                      access_specifier: gtoken_t,
                                      storage_specifier: gtoken_t,
                                      params: *mut gnode_r,
                                      block: *mut gnode_compound_stmt_t,
                                      meta: *mut ::std::os::raw::c_void,
                                      decl: *mut gnode_t)
                                      -> *mut gnode_t;
    pub fn gnode_binary_expr_create(op: gtoken_t,
                                    left: *mut gnode_t,
                                    right: *mut gnode_t,
                                    decl: *mut gnode_t)
                                    -> *mut gnode_t;
    pub fn gnode_unary_expr_create(op: gtoken_t,
                                   expr: *mut gnode_t,
                                   decl: *mut gnode_t)
                                   -> *mut gnode_t;
    pub fn gnode_file_expr_create(token: gtoken_s,
                                  list: *mut cstring_r,
                                  decl: *mut gnode_t)
                                  -> *mut gnode_t;
    pub fn gnode_identifier_expr_create(token: gtoken_s,
                                        identifier: *const ::std::os::raw::c_char,
                                        identifier2: *const ::std::os::raw::c_char,
                                        decl: *mut gnode_t)
                                        -> *mut gnode_t;
    pub fn gnode_string_interpolation_create(token: gtoken_s,
                                             r: *mut gnode_r,
                                             decl: *mut gnode_t)
                                             -> *mut gnode_t;
    pub fn gnode_literal_string_expr_create(token: gtoken_s,
                                            s: *mut ::std::os::raw::c_char,
                                            len: u32,
                                            allocated: bool,
                                            decl: *mut gnode_t)
                                            -> *mut gnode_t;
    pub fn gnode_literal_float_expr_create(token: gtoken_s,
                                           f: f64,
                                           decl: *mut gnode_t)
                                           -> *mut gnode_t;
    pub fn gnode_literal_int_expr_create(token: gtoken_s,
                                         n: i64,
                                         decl: *mut gnode_t)
                                         -> *mut gnode_t;
    pub fn gnode_literal_bool_expr_create(token: gtoken_s,
                                          n: i32,
                                          decl: *mut gnode_t)
                                          -> *mut gnode_t;
    pub fn gnode_keyword_expr_create(token: gtoken_s, decl: *mut gnode_t) -> *mut gnode_t;
    pub fn gnode_postfix_subexpr_create(token: gtoken_s,
                                        type_: gnode_n,
                                        expr: *mut gnode_t,
                                        list: *mut gnode_r,
                                        decl: *mut gnode_t)
                                        -> *mut gnode_t;
    pub fn gnode_postfix_expr_create(token: gtoken_s,
                                     id: *mut gnode_t,
                                     list: *mut gnode_r,
                                     decl: *mut gnode_t)
                                     -> *mut gnode_t;
    pub fn gnode_list_expr_create(token: gtoken_s,
                                  list1: *mut gnode_r,
                                  list2: *mut gnode_r,
                                  ismap: bool,
                                  decl: *mut gnode_t)
                                  -> *mut gnode_t;
    pub fn gnode_duplicate(node: *mut gnode_t, deep: bool) -> *mut gnode_t;
    pub fn gnode_array_create() -> *mut gnode_r;
    pub fn gnode_array_remove_byindex(list: *mut gnode_r, index: usize) -> *mut gnode_r;
    pub fn gnode_function_add_upvalue(f: *mut gnode_function_decl_t,
                                      symbol: *mut gnode_var_t,
                                      n: u16)
                                      -> *mut gupvalue_t;
    pub fn cstring_array_create() -> *mut cstring_r;
    pub fn void_array_create() -> *mut void_r;
    pub fn gnode_array_sethead(list: *mut gnode_r, node: *mut gnode_t);
    pub fn gnode_is_equal(node1: *mut gnode_t, node2: *mut gnode_t) -> bool;
    pub fn gnode_is_expression(node: *mut gnode_t) -> bool;
    pub fn gnode_is_literal(node: *mut gnode_t) -> bool;
    pub fn gnode_is_literal_int(node: *mut gnode_t) -> bool;
    pub fn gnode_is_literal_number(node: *mut gnode_t) -> bool;
    pub fn gnode_is_literal_string(node: *mut gnode_t) -> bool;
    pub fn gnode_literal_dump(node: *mut gnode_literal_expr_t,
                              buffer: *mut ::std::os::raw::c_char,
                              buffersize: ::std::os::raw::c_int);
    pub fn gnode_free(node: *mut gnode_t);
    pub fn meta_from_node(node: *mut gnode_t) -> *mut ::std::os::raw::c_void;
    pub fn gravity_compiler_create(delegate: *mut gravity_delegate_t) -> *mut gravity_compiler_t;
    pub fn gravity_compiler_run(compiler: *mut gravity_compiler_t,
                                source: *const ::std::os::raw::c_char,
                                len: usize,
                                fileid: u32,
                                is_static: bool)
                                -> *mut gravity_closure_t;
    pub fn gravity_compiler_serialize(compiler: *mut gravity_compiler_t,
                                      closure: *mut gravity_closure_t)
                                      -> *mut json_t;
    pub fn gravity_compiler_serialize_infile(compiler: *mut gravity_compiler_t,
                                             closure: *mut gravity_closure_t,
                                             path: *const ::std::os::raw::c_char)
                                             -> bool;
    pub fn gravity_compiler_transfer(compiler: *mut gravity_compiler_t, vm: *mut gravity_vm);
    pub fn gravity_compiler_ast(compiler: *mut gravity_compiler_t) -> *mut gnode_t;
    pub fn gravity_compiler_free(compiler: *mut gravity_compiler_t);
    pub fn gravity_vm_new(delegate: *mut gravity_delegate_t) -> *mut gravity_vm;
    pub fn gravity_vm_newmini() -> *mut gravity_vm;
    pub fn gravity_vm_set_callbacks(vm: *mut gravity_vm,
                                    vm_transfer: vm_transfer_cb,
                                    vm_cleanup: vm_cleanup_cb);
    pub fn gravity_vm_free(vm: *mut gravity_vm);
    pub fn gravity_vm_reset(vm: *mut gravity_vm);
    pub fn gravity_vm_runclosure(vm: *mut gravity_vm,
                                 closure: *mut gravity_closure_t,
                                 selfvalue: gravity_value_t,
                                 params: *mut gravity_value_t,
                                 nparams: u16)
                                 -> bool;
    pub fn gravity_vm_runmain(vm: *mut gravity_vm, closure: *mut gravity_closure_t) -> bool;
    pub fn gravity_vm_loadclosure(vm: *mut gravity_vm, closure: *mut gravity_closure_t);
    pub fn gravity_vm_setvalue(vm: *mut gravity_vm,
                               key: *const ::std::os::raw::c_char,
                               value: gravity_value_t);
    pub fn gravity_vm_lookup(vm: *mut gravity_vm, key: gravity_value_t) -> gravity_value_t;
    pub fn gravity_vm_getvalue(vm: *mut gravity_vm,
                               key: *const ::std::os::raw::c_char,
                               keylen: u32)
                               -> gravity_value_t;
    pub fn gravity_vm_time(vm: *mut gravity_vm) -> f64;
    pub fn gravity_vm_result(vm: *mut gravity_vm) -> gravity_value_t;
    pub fn gravity_vm_delegate(vm: *mut gravity_vm) -> *mut gravity_delegate_t;
    pub fn gravity_vm_fiber(vm: *mut gravity_vm) -> *mut gravity_fiber_t;
    pub fn gravity_vm_setfiber(vm: *mut gravity_vm, fiber: *mut gravity_fiber_t);
    pub fn gravity_vm_seterror(vm: *mut gravity_vm, format: *const ::std::os::raw::c_char, ...);
    pub fn gravity_vm_seterror_string(vm: *mut gravity_vm, s: *const ::std::os::raw::c_char);
    pub fn gravity_vm_ismini(vm: *mut gravity_vm) -> bool;
    pub fn gravity_vm_keyindex(vm: *mut gravity_vm, index: u32) -> gravity_value_t;
    pub fn gravity_vm_isaborted(vm: *mut gravity_vm) -> bool;
    pub fn gravity_vm_setaborted(vm: *mut gravity_vm);
    pub fn gravity_vm_getclosure(vm: *mut gravity_vm) -> *mut gravity_closure_t;
    pub fn gravity_gray_value(vm: *mut gravity_vm, v: gravity_value_t);
    pub fn gravity_gray_object(vm: *mut gravity_vm, obj: *mut gravity_object_t);
    pub fn gravity_gc_start(vm: *mut gravity_vm);
    pub fn gravity_gc_setenabled(vm: *mut gravity_vm, enabled: bool);
    pub fn gravity_gc_push(vm: *mut gravity_vm, obj: *mut gravity_object_t);
    pub fn gravity_gc_pop(vm: *mut gravity_vm);
    pub fn gravity_vm_transfer(vm: *mut gravity_vm, obj: *mut gravity_object_t);
    pub fn gravity_vm_cleanup(vm: *mut gravity_vm);
    pub fn gravity_vm_filter(vm: *mut gravity_vm, cleanup_filter: vm_filter_cb);
    pub fn gravity_vm_loadfile(vm: *mut gravity_vm,
                               path: *const ::std::os::raw::c_char)
                               -> *mut gravity_closure_t;
    pub fn gravity_vm_loadbuffer(vm: *mut gravity_vm,
                                 buffer: *const ::std::os::raw::c_char,
                                 len: usize)
                                 -> *mut gravity_closure_t;
    pub fn gravity_vm_initmodule(vm: *mut gravity_vm, f: *mut gravity_function_t);
    pub fn gravity_vm_fastlookup(vm: *mut gravity_vm,
                                 c: *mut gravity_class_t,
                                 index: ::std::os::raw::c_int)
                                 -> *mut gravity_closure_t;
    pub fn gravity_vm_setslot(vm: *mut gravity_vm, value: gravity_value_t, index: u32);
    pub fn gravity_vm_getslot(vm: *mut gravity_vm, index: u32) -> gravity_value_t;
    pub fn gravity_vm_setdata(vm: *mut gravity_vm, data: *mut ::std::os::raw::c_void);
    pub fn gravity_vm_getdata(vm: *mut gravity_vm) -> *mut ::std::os::raw::c_void;
    pub fn gravity_vm_memupdate(vm: *mut gravity_vm, value: gravity_int_t);
    pub fn gravity_vm_get(vm: *mut gravity_vm,
                          key: *const ::std::os::raw::c_char)
                          -> gravity_value_t;
    pub fn gravity_vm_set(vm: *mut gravity_vm,
                          key: *const ::std::os::raw::c_char,
                          value: gravity_value_t)
                          -> bool;
    pub fn gravity_vm_anonymous(vm: *mut gravity_vm) -> *mut ::std::os::raw::c_char;
    pub fn gravity_core_register(vm: *mut gravity_vm);
    pub fn gravity_iscore_class(c: *mut gravity_class_t) -> bool;
    pub fn gravity_core_free();
    pub fn gravity_core_identifiers(id: *mut *mut *const ::std::os::raw::c_char) -> u32;
    pub fn gravity_core_class_from_name(name: *const ::std::os::raw::c_char)
                                        -> *mut gravity_class_t;
    pub fn convert_value2int(vm: *mut gravity_vm, v: gravity_value_t) -> gravity_value_t;
    pub fn convert_value2float(vm: *mut gravity_vm, v: gravity_value_t) -> gravity_value_t;
    pub fn convert_value2bool(vm: *mut gravity_vm, v: gravity_value_t) -> gravity_value_t;
    pub fn convert_value2string(vm: *mut gravity_vm, v: gravity_value_t) -> gravity_value_t;
}
