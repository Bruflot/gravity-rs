#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>);
impl <T> __IncompleteArrayField<T> {
    #[inline]
    pub fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const T { ::std::mem::transmute(self) }
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
impl <T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
impl <T> ::std::clone::Clone for __IncompleteArrayField<T> {
    #[inline]
    fn clone(&self) -> Self { Self::new() }
}
impl <T> ::std::marker::Copy for __IncompleteArrayField<T> { }
#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl <T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self { __BindgenUnionField(::std::marker::PhantomData) }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T { ::std::mem::transmute(self) }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T { ::std::mem::transmute(self) }
}
impl <T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self { Self::new() }
}
impl <T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self { Self::new() }
}
impl <T> ::std::marker::Copy for __BindgenUnionField<T> { }
impl <T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
pub const _STDINT_H: ::std::os::raw::c_uint = 1;
pub const _FEATURES_H: ::std::os::raw::c_uint = 1;
pub const _DEFAULT_SOURCE: ::std::os::raw::c_uint = 1;
pub const __USE_ISOC11: ::std::os::raw::c_uint = 1;
pub const __USE_ISOC99: ::std::os::raw::c_uint = 1;
pub const __USE_ISOC95: ::std::os::raw::c_uint = 1;
pub const __USE_POSIX_IMPLICITLY: ::std::os::raw::c_uint = 1;
pub const _POSIX_SOURCE: ::std::os::raw::c_uint = 1;
pub const _POSIX_C_SOURCE: ::std::os::raw::c_uint = 200809;
pub const __USE_POSIX: ::std::os::raw::c_uint = 1;
pub const __USE_POSIX2: ::std::os::raw::c_uint = 1;
pub const __USE_POSIX199309: ::std::os::raw::c_uint = 1;
pub const __USE_POSIX199506: ::std::os::raw::c_uint = 1;
pub const __USE_XOPEN2K: ::std::os::raw::c_uint = 1;
pub const __USE_XOPEN2K8: ::std::os::raw::c_uint = 1;
pub const _ATFILE_SOURCE: ::std::os::raw::c_uint = 1;
pub const __USE_MISC: ::std::os::raw::c_uint = 1;
pub const __USE_ATFILE: ::std::os::raw::c_uint = 1;
pub const __USE_FORTIFY_LEVEL: ::std::os::raw::c_uint = 0;
pub const _STDC_PREDEF_H: ::std::os::raw::c_uint = 1;
pub const __STDC_IEC_559__: ::std::os::raw::c_uint = 1;
pub const __STDC_IEC_559_COMPLEX__: ::std::os::raw::c_uint = 1;
pub const __STDC_ISO_10646__: ::std::os::raw::c_uint = 201505;
pub const __STDC_NO_THREADS__: ::std::os::raw::c_uint = 1;
pub const __GNU_LIBRARY__: ::std::os::raw::c_uint = 6;
pub const __GLIBC__: ::std::os::raw::c_uint = 2;
pub const __GLIBC_MINOR__: ::std::os::raw::c_uint = 25;
pub const _SYS_CDEFS_H: ::std::os::raw::c_uint = 1;
pub const __glibc_c99_flexarr_available: ::std::os::raw::c_uint = 1;
pub const __WORDSIZE: ::std::os::raw::c_uint = 64;
pub const __WORDSIZE_TIME64_COMPAT32: ::std::os::raw::c_uint = 1;
pub const __SYSCALL_WORDSIZE: ::std::os::raw::c_uint = 64;
pub const __GLIBC_USE_LIB_EXT2: ::std::os::raw::c_uint = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: ::std::os::raw::c_uint = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: ::std::os::raw::c_uint = 0;
pub const _BITS_TYPES_H: ::std::os::raw::c_uint = 1;
pub const _BITS_TYPESIZES_H: ::std::os::raw::c_uint = 1;
pub const __OFF_T_MATCHES_OFF64_T: ::std::os::raw::c_uint = 1;
pub const __INO_T_MATCHES_INO64_T: ::std::os::raw::c_uint = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: ::std::os::raw::c_uint = 1;
pub const __FD_SETSIZE: ::std::os::raw::c_uint = 1024;
pub const _BITS_WCHAR_H: ::std::os::raw::c_uint = 1;
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
pub const _STDIO_H: ::std::os::raw::c_uint = 1;
pub const __FILE_defined: ::std::os::raw::c_uint = 1;
pub const ____FILE_defined: ::std::os::raw::c_uint = 1;
pub const _G_config_h: ::std::os::raw::c_uint = 1;
pub const ____mbstate_t_defined: ::std::os::raw::c_uint = 1;
pub const _G_HAVE_MMAP: ::std::os::raw::c_uint = 1;
pub const _G_HAVE_MREMAP: ::std::os::raw::c_uint = 1;
pub const _G_IO_IO_FILE_VERSION: ::std::os::raw::c_uint = 131073;
pub const _G_BUFSIZ: ::std::os::raw::c_uint = 8192;
pub const _IO_BUFSIZ: ::std::os::raw::c_uint = 8192;
pub const __GNUC_VA_LIST: ::std::os::raw::c_uint = 1;
pub const _IO_UNIFIED_JUMPTABLES: ::std::os::raw::c_uint = 1;
pub const EOF: ::std::os::raw::c_int = -1;
pub const _IOS_INPUT: ::std::os::raw::c_uint = 1;
pub const _IOS_OUTPUT: ::std::os::raw::c_uint = 2;
pub const _IOS_ATEND: ::std::os::raw::c_uint = 4;
pub const _IOS_APPEND: ::std::os::raw::c_uint = 8;
pub const _IOS_TRUNC: ::std::os::raw::c_uint = 16;
pub const _IOS_NOCREATE: ::std::os::raw::c_uint = 32;
pub const _IOS_NOREPLACE: ::std::os::raw::c_uint = 64;
pub const _IOS_BIN: ::std::os::raw::c_uint = 128;
pub const _IO_MAGIC: ::std::os::raw::c_uint = 4222418944;
pub const _OLD_STDIO_MAGIC: ::std::os::raw::c_uint = 4206624768;
pub const _IO_MAGIC_MASK: ::std::os::raw::c_uint = 4294901760;
pub const _IO_USER_BUF: ::std::os::raw::c_uint = 1;
pub const _IO_UNBUFFERED: ::std::os::raw::c_uint = 2;
pub const _IO_NO_READS: ::std::os::raw::c_uint = 4;
pub const _IO_NO_WRITES: ::std::os::raw::c_uint = 8;
pub const _IO_EOF_SEEN: ::std::os::raw::c_uint = 16;
pub const _IO_ERR_SEEN: ::std::os::raw::c_uint = 32;
pub const _IO_DELETE_DONT_CLOSE: ::std::os::raw::c_uint = 64;
pub const _IO_LINKED: ::std::os::raw::c_uint = 128;
pub const _IO_IN_BACKUP: ::std::os::raw::c_uint = 256;
pub const _IO_LINE_BUF: ::std::os::raw::c_uint = 512;
pub const _IO_TIED_PUT_GET: ::std::os::raw::c_uint = 1024;
pub const _IO_CURRENTLY_PUTTING: ::std::os::raw::c_uint = 2048;
pub const _IO_IS_APPENDING: ::std::os::raw::c_uint = 4096;
pub const _IO_IS_FILEBUF: ::std::os::raw::c_uint = 8192;
pub const _IO_BAD_SEEN: ::std::os::raw::c_uint = 16384;
pub const _IO_USER_LOCK: ::std::os::raw::c_uint = 32768;
pub const _IO_FLAGS2_MMAP: ::std::os::raw::c_uint = 1;
pub const _IO_FLAGS2_NOTCANCEL: ::std::os::raw::c_uint = 2;
pub const _IO_FLAGS2_USER_WBUF: ::std::os::raw::c_uint = 8;
pub const _IO_SKIPWS: ::std::os::raw::c_uint = 1;
pub const _IO_LEFT: ::std::os::raw::c_uint = 2;
pub const _IO_RIGHT: ::std::os::raw::c_uint = 4;
pub const _IO_INTERNAL: ::std::os::raw::c_uint = 8;
pub const _IO_DEC: ::std::os::raw::c_uint = 16;
pub const _IO_OCT: ::std::os::raw::c_uint = 32;
pub const _IO_HEX: ::std::os::raw::c_uint = 64;
pub const _IO_SHOWBASE: ::std::os::raw::c_uint = 128;
pub const _IO_SHOWPOINT: ::std::os::raw::c_uint = 256;
pub const _IO_UPPERCASE: ::std::os::raw::c_uint = 512;
pub const _IO_SHOWPOS: ::std::os::raw::c_uint = 1024;
pub const _IO_SCIENTIFIC: ::std::os::raw::c_uint = 2048;
pub const _IO_FIXED: ::std::os::raw::c_uint = 4096;
pub const _IO_UNITBUF: ::std::os::raw::c_uint = 8192;
pub const _IO_STDIO: ::std::os::raw::c_uint = 16384;
pub const _IO_DONT_CLOSE: ::std::os::raw::c_uint = 32768;
pub const _IO_BOOLALPHA: ::std::os::raw::c_uint = 65536;
pub const _IOFBF: ::std::os::raw::c_uint = 0;
pub const _IOLBF: ::std::os::raw::c_uint = 1;
pub const _IONBF: ::std::os::raw::c_uint = 2;
pub const BUFSIZ: ::std::os::raw::c_uint = 8192;
pub const SEEK_SET: ::std::os::raw::c_uint = 0;
pub const SEEK_CUR: ::std::os::raw::c_uint = 1;
pub const SEEK_END: ::std::os::raw::c_uint = 2;
pub const P_tmpdir: &'static [u8; 5usize] = b"/tmp\x00";
pub const L_tmpnam: ::std::os::raw::c_uint = 20;
pub const TMP_MAX: ::std::os::raw::c_uint = 238328;
pub const FILENAME_MAX: ::std::os::raw::c_uint = 4096;
pub const L_ctermid: ::std::os::raw::c_uint = 9;
pub const FOPEN_MAX: ::std::os::raw::c_uint = 16;
pub const _STDLIB_H: ::std::os::raw::c_uint = 1;
pub const WNOHANG: ::std::os::raw::c_uint = 1;
pub const WUNTRACED: ::std::os::raw::c_uint = 2;
pub const WSTOPPED: ::std::os::raw::c_uint = 2;
pub const WEXITED: ::std::os::raw::c_uint = 4;
pub const WCONTINUED: ::std::os::raw::c_uint = 8;
pub const WNOWAIT: ::std::os::raw::c_uint = 16777216;
pub const __WNOTHREAD: ::std::os::raw::c_uint = 536870912;
pub const __WALL: ::std::os::raw::c_uint = 1073741824;
pub const __WCLONE: ::std::os::raw::c_uint = 2147483648;
pub const __W_CONTINUED: ::std::os::raw::c_uint = 65535;
pub const __WCOREFLAG: ::std::os::raw::c_uint = 128;
pub const __ldiv_t_defined: ::std::os::raw::c_uint = 1;
pub const __lldiv_t_defined: ::std::os::raw::c_uint = 1;
pub const RAND_MAX: ::std::os::raw::c_uint = 2147483647;
pub const EXIT_FAILURE: ::std::os::raw::c_uint = 1;
pub const EXIT_SUCCESS: ::std::os::raw::c_uint = 0;
pub const _SYS_TYPES_H: ::std::os::raw::c_uint = 1;
pub const __clock_t_defined: ::std::os::raw::c_uint = 1;
pub const __clockid_t_defined: ::std::os::raw::c_uint = 1;
pub const __time_t_defined: ::std::os::raw::c_uint = 1;
pub const __timer_t_defined: ::std::os::raw::c_uint = 1;
pub const __BIT_TYPES_DEFINED__: ::std::os::raw::c_uint = 1;
pub const _ENDIAN_H: ::std::os::raw::c_uint = 1;
pub const __LITTLE_ENDIAN: ::std::os::raw::c_uint = 1234;
pub const __BIG_ENDIAN: ::std::os::raw::c_uint = 4321;
pub const __PDP_ENDIAN: ::std::os::raw::c_uint = 3412;
pub const __BYTE_ORDER: ::std::os::raw::c_uint = 1234;
pub const __FLOAT_WORD_ORDER: ::std::os::raw::c_uint = 1234;
pub const LITTLE_ENDIAN: ::std::os::raw::c_uint = 1234;
pub const BIG_ENDIAN: ::std::os::raw::c_uint = 4321;
pub const PDP_ENDIAN: ::std::os::raw::c_uint = 3412;
pub const BYTE_ORDER: ::std::os::raw::c_uint = 1234;
pub const _BITS_BYTESWAP_H: ::std::os::raw::c_uint = 1;
pub const _BITS_UINTN_IDENTITY_H: ::std::os::raw::c_uint = 1;
pub const _SYS_SELECT_H: ::std::os::raw::c_uint = 1;
pub const __FD_ZERO_STOS: &'static [u8; 6usize] = b"stosq\x00";
pub const _SIGSET_H_types: ::std::os::raw::c_uint = 1;
pub const __timeval_defined: ::std::os::raw::c_uint = 1;
pub const __timespec_defined: ::std::os::raw::c_uint = 1;
pub const FD_SETSIZE: ::std::os::raw::c_uint = 1024;
pub const _SYS_SYSMACROS_H: ::std::os::raw::c_uint = 1;
pub const _BITS_SYSMACROS_H: ::std::os::raw::c_uint = 1;
pub const _BITS_PTHREADTYPES_H: ::std::os::raw::c_uint = 1;
pub const __SIZEOF_PTHREAD_ATTR_T: ::std::os::raw::c_uint = 56;
pub const __SIZEOF_PTHREAD_MUTEX_T: ::std::os::raw::c_uint = 40;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: ::std::os::raw::c_uint = 4;
pub const __SIZEOF_PTHREAD_COND_T: ::std::os::raw::c_uint = 48;
pub const __SIZEOF_PTHREAD_CONDATTR_T: ::std::os::raw::c_uint = 4;
pub const __SIZEOF_PTHREAD_RWLOCK_T: ::std::os::raw::c_uint = 56;
pub const __SIZEOF_PTHREAD_RWLOCKATTR_T: ::std::os::raw::c_uint = 8;
pub const __SIZEOF_PTHREAD_BARRIER_T: ::std::os::raw::c_uint = 32;
pub const __SIZEOF_PTHREAD_BARRIERATTR_T: ::std::os::raw::c_uint = 4;
pub const __have_pthread_attr_t: ::std::os::raw::c_uint = 1;
pub const __PTHREAD_MUTEX_HAVE_PREV: ::std::os::raw::c_uint = 1;
pub const __PTHREAD_RWLOCK_INT_FLAGS_SHARED: ::std::os::raw::c_uint = 1;
pub const _ALLOCA_H: ::std::os::raw::c_uint = 1;
pub const true_: ::std::os::raw::c_uint = 1;
pub const false_: ::std::os::raw::c_uint = 0;
pub const __bool_true_false_are_defined: ::std::os::raw::c_uint = 1;
pub const GRAVITY_MEMORY_DEBUG: ::std::os::raw::c_uint = 0;
pub const _DIRENT_H: ::std::os::raw::c_uint = 1;
pub const _DIRENT_MATCHES_DIRENT64: ::std::os::raw::c_uint = 1;
pub const _BITS_POSIX1_LIM_H: ::std::os::raw::c_uint = 1;
pub const _POSIX_AIO_LISTIO_MAX: ::std::os::raw::c_uint = 2;
pub const _POSIX_AIO_MAX: ::std::os::raw::c_uint = 1;
pub const _POSIX_ARG_MAX: ::std::os::raw::c_uint = 4096;
pub const _POSIX_CHILD_MAX: ::std::os::raw::c_uint = 25;
pub const _POSIX_DELAYTIMER_MAX: ::std::os::raw::c_uint = 32;
pub const _POSIX_HOST_NAME_MAX: ::std::os::raw::c_uint = 255;
pub const _POSIX_LINK_MAX: ::std::os::raw::c_uint = 8;
pub const _POSIX_LOGIN_NAME_MAX: ::std::os::raw::c_uint = 9;
pub const _POSIX_MAX_CANON: ::std::os::raw::c_uint = 255;
pub const _POSIX_MAX_INPUT: ::std::os::raw::c_uint = 255;
pub const _POSIX_MQ_OPEN_MAX: ::std::os::raw::c_uint = 8;
pub const _POSIX_MQ_PRIO_MAX: ::std::os::raw::c_uint = 32;
pub const _POSIX_NAME_MAX: ::std::os::raw::c_uint = 14;
pub const _POSIX_NGROUPS_MAX: ::std::os::raw::c_uint = 8;
pub const _POSIX_OPEN_MAX: ::std::os::raw::c_uint = 20;
pub const _POSIX_PATH_MAX: ::std::os::raw::c_uint = 256;
pub const _POSIX_PIPE_BUF: ::std::os::raw::c_uint = 512;
pub const _POSIX_RE_DUP_MAX: ::std::os::raw::c_uint = 255;
pub const _POSIX_RTSIG_MAX: ::std::os::raw::c_uint = 8;
pub const _POSIX_SEM_NSEMS_MAX: ::std::os::raw::c_uint = 256;
pub const _POSIX_SEM_VALUE_MAX: ::std::os::raw::c_uint = 32767;
pub const _POSIX_SIGQUEUE_MAX: ::std::os::raw::c_uint = 32;
pub const _POSIX_SSIZE_MAX: ::std::os::raw::c_uint = 32767;
pub const _POSIX_STREAM_MAX: ::std::os::raw::c_uint = 8;
pub const _POSIX_SYMLINK_MAX: ::std::os::raw::c_uint = 255;
pub const _POSIX_SYMLOOP_MAX: ::std::os::raw::c_uint = 8;
pub const _POSIX_TIMER_MAX: ::std::os::raw::c_uint = 32;
pub const _POSIX_TTY_NAME_MAX: ::std::os::raw::c_uint = 9;
pub const _POSIX_TZNAME_MAX: ::std::os::raw::c_uint = 6;
pub const _POSIX_CLOCKRES_MIN: ::std::os::raw::c_uint = 20000000;
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
pub const _POSIX_THREAD_KEYS_MAX: ::std::os::raw::c_uint = 128;
pub const PTHREAD_KEYS_MAX: ::std::os::raw::c_uint = 1024;
pub const _POSIX_THREAD_DESTRUCTOR_ITERATIONS: ::std::os::raw::c_uint = 4;
pub const PTHREAD_DESTRUCTOR_ITERATIONS: ::std::os::raw::c_uint = 4;
pub const _POSIX_THREAD_THREADS_MAX: ::std::os::raw::c_uint = 64;
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
pub const _INTTYPES_H: ::std::os::raw::c_uint = 1;
pub const ____gwchar_t_defined: ::std::os::raw::c_uint = 1;
pub const __PRI64_PREFIX: &'static [u8; 2usize] = b"l\x00";
pub const __PRIPTR_PREFIX: &'static [u8; 2usize] = b"l\x00";
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
pub const _CTYPE_H: ::std::os::raw::c_uint = 1;
pub const _XLOCALE_H: ::std::os::raw::c_uint = 1;
pub const _FCNTL_H: ::std::os::raw::c_uint = 1;
pub const __O_LARGEFILE: ::std::os::raw::c_uint = 0;
pub const F_GETLK64: ::std::os::raw::c_uint = 5;
pub const F_SETLK64: ::std::os::raw::c_uint = 6;
pub const F_SETLKW64: ::std::os::raw::c_uint = 7;
pub const O_ACCMODE: ::std::os::raw::c_uint = 3;
pub const O_RDONLY: ::std::os::raw::c_uint = 0;
pub const O_WRONLY: ::std::os::raw::c_uint = 1;
pub const O_RDWR: ::std::os::raw::c_uint = 2;
pub const O_CREAT: ::std::os::raw::c_uint = 64;
pub const O_EXCL: ::std::os::raw::c_uint = 128;
pub const O_NOCTTY: ::std::os::raw::c_uint = 256;
pub const O_TRUNC: ::std::os::raw::c_uint = 512;
pub const O_APPEND: ::std::os::raw::c_uint = 1024;
pub const O_NONBLOCK: ::std::os::raw::c_uint = 2048;
pub const O_NDELAY: ::std::os::raw::c_uint = 2048;
pub const O_SYNC: ::std::os::raw::c_uint = 1052672;
pub const O_FSYNC: ::std::os::raw::c_uint = 1052672;
pub const O_ASYNC: ::std::os::raw::c_uint = 8192;
pub const __O_DIRECTORY: ::std::os::raw::c_uint = 65536;
pub const __O_NOFOLLOW: ::std::os::raw::c_uint = 131072;
pub const __O_CLOEXEC: ::std::os::raw::c_uint = 524288;
pub const __O_DIRECT: ::std::os::raw::c_uint = 16384;
pub const __O_NOATIME: ::std::os::raw::c_uint = 262144;
pub const __O_PATH: ::std::os::raw::c_uint = 2097152;
pub const __O_DSYNC: ::std::os::raw::c_uint = 4096;
pub const __O_TMPFILE: ::std::os::raw::c_uint = 4259840;
pub const F_GETLK: ::std::os::raw::c_uint = 5;
pub const F_SETLK: ::std::os::raw::c_uint = 6;
pub const F_SETLKW: ::std::os::raw::c_uint = 7;
pub const O_DIRECTORY: ::std::os::raw::c_uint = 65536;
pub const O_NOFOLLOW: ::std::os::raw::c_uint = 131072;
pub const O_CLOEXEC: ::std::os::raw::c_uint = 524288;
pub const O_DSYNC: ::std::os::raw::c_uint = 4096;
pub const O_RSYNC: ::std::os::raw::c_uint = 1052672;
pub const F_DUPFD: ::std::os::raw::c_uint = 0;
pub const F_GETFD: ::std::os::raw::c_uint = 1;
pub const F_SETFD: ::std::os::raw::c_uint = 2;
pub const F_GETFL: ::std::os::raw::c_uint = 3;
pub const F_SETFL: ::std::os::raw::c_uint = 4;
pub const __F_SETOWN: ::std::os::raw::c_uint = 8;
pub const __F_GETOWN: ::std::os::raw::c_uint = 9;
pub const F_SETOWN: ::std::os::raw::c_uint = 8;
pub const F_GETOWN: ::std::os::raw::c_uint = 9;
pub const __F_SETSIG: ::std::os::raw::c_uint = 10;
pub const __F_GETSIG: ::std::os::raw::c_uint = 11;
pub const __F_SETOWN_EX: ::std::os::raw::c_uint = 15;
pub const __F_GETOWN_EX: ::std::os::raw::c_uint = 16;
pub const F_DUPFD_CLOEXEC: ::std::os::raw::c_uint = 1030;
pub const FD_CLOEXEC: ::std::os::raw::c_uint = 1;
pub const F_RDLCK: ::std::os::raw::c_uint = 0;
pub const F_WRLCK: ::std::os::raw::c_uint = 1;
pub const F_UNLCK: ::std::os::raw::c_uint = 2;
pub const F_EXLCK: ::std::os::raw::c_uint = 4;
pub const F_SHLCK: ::std::os::raw::c_uint = 8;
pub const LOCK_SH: ::std::os::raw::c_uint = 1;
pub const LOCK_EX: ::std::os::raw::c_uint = 2;
pub const LOCK_NB: ::std::os::raw::c_uint = 4;
pub const LOCK_UN: ::std::os::raw::c_uint = 8;
pub const FAPPEND: ::std::os::raw::c_uint = 1024;
pub const FFSYNC: ::std::os::raw::c_uint = 1052672;
pub const FASYNC: ::std::os::raw::c_uint = 8192;
pub const FNONBLOCK: ::std::os::raw::c_uint = 2048;
pub const FNDELAY: ::std::os::raw::c_uint = 2048;
pub const __POSIX_FADV_DONTNEED: ::std::os::raw::c_uint = 4;
pub const __POSIX_FADV_NOREUSE: ::std::os::raw::c_uint = 5;
pub const POSIX_FADV_NORMAL: ::std::os::raw::c_uint = 0;
pub const POSIX_FADV_RANDOM: ::std::os::raw::c_uint = 1;
pub const POSIX_FADV_SEQUENTIAL: ::std::os::raw::c_uint = 2;
pub const POSIX_FADV_WILLNEED: ::std::os::raw::c_uint = 3;
pub const POSIX_FADV_DONTNEED: ::std::os::raw::c_uint = 4;
pub const POSIX_FADV_NOREUSE: ::std::os::raw::c_uint = 5;
pub const _BITS_STAT_H: ::std::os::raw::c_uint = 1;
pub const _STAT_VER_KERNEL: ::std::os::raw::c_uint = 0;
pub const _STAT_VER_LINUX: ::std::os::raw::c_uint = 1;
pub const _MKNOD_VER_LINUX: ::std::os::raw::c_uint = 0;
pub const _STAT_VER: ::std::os::raw::c_uint = 1;
pub const __S_IFMT: ::std::os::raw::c_uint = 61440;
pub const __S_IFDIR: ::std::os::raw::c_uint = 16384;
pub const __S_IFCHR: ::std::os::raw::c_uint = 8192;
pub const __S_IFBLK: ::std::os::raw::c_uint = 24576;
pub const __S_IFREG: ::std::os::raw::c_uint = 32768;
pub const __S_IFIFO: ::std::os::raw::c_uint = 4096;
pub const __S_IFLNK: ::std::os::raw::c_uint = 40960;
pub const __S_IFSOCK: ::std::os::raw::c_uint = 49152;
pub const __S_ISUID: ::std::os::raw::c_uint = 2048;
pub const __S_ISGID: ::std::os::raw::c_uint = 1024;
pub const __S_ISVTX: ::std::os::raw::c_uint = 512;
pub const __S_IREAD: ::std::os::raw::c_uint = 256;
pub const __S_IWRITE: ::std::os::raw::c_uint = 128;
pub const __S_IEXEC: ::std::os::raw::c_uint = 64;
pub const UTIME_NOW: ::std::os::raw::c_uint = 1073741823;
pub const UTIME_OMIT: ::std::os::raw::c_uint = 1073741822;
pub const S_IFMT: ::std::os::raw::c_uint = 61440;
pub const S_IFDIR: ::std::os::raw::c_uint = 16384;
pub const S_IFCHR: ::std::os::raw::c_uint = 8192;
pub const S_IFBLK: ::std::os::raw::c_uint = 24576;
pub const S_IFREG: ::std::os::raw::c_uint = 32768;
pub const S_IFIFO: ::std::os::raw::c_uint = 4096;
pub const S_IFLNK: ::std::os::raw::c_uint = 40960;
pub const S_IFSOCK: ::std::os::raw::c_uint = 49152;
pub const S_ISUID: ::std::os::raw::c_uint = 2048;
pub const S_ISGID: ::std::os::raw::c_uint = 1024;
pub const S_ISVTX: ::std::os::raw::c_uint = 512;
pub const S_IRUSR: ::std::os::raw::c_uint = 256;
pub const S_IWUSR: ::std::os::raw::c_uint = 128;
pub const S_IXUSR: ::std::os::raw::c_uint = 64;
pub const S_IRWXU: ::std::os::raw::c_uint = 448;
pub const S_IRGRP: ::std::os::raw::c_uint = 32;
pub const S_IWGRP: ::std::os::raw::c_uint = 16;
pub const S_IXGRP: ::std::os::raw::c_uint = 8;
pub const S_IRWXG: ::std::os::raw::c_uint = 56;
pub const S_IROTH: ::std::os::raw::c_uint = 4;
pub const S_IWOTH: ::std::os::raw::c_uint = 2;
pub const S_IXOTH: ::std::os::raw::c_uint = 1;
pub const S_IRWXO: ::std::os::raw::c_uint = 7;
pub const R_OK: ::std::os::raw::c_uint = 4;
pub const W_OK: ::std::os::raw::c_uint = 2;
pub const X_OK: ::std::os::raw::c_uint = 1;
pub const F_OK: ::std::os::raw::c_uint = 0;
pub const AT_FDCWD: ::std::os::raw::c_int = -100;
pub const AT_SYMLINK_NOFOLLOW: ::std::os::raw::c_uint = 256;
pub const AT_REMOVEDIR: ::std::os::raw::c_uint = 512;
pub const AT_SYMLINK_FOLLOW: ::std::os::raw::c_uint = 1024;
pub const AT_EACCESS: ::std::os::raw::c_uint = 512;
pub const F_ULOCK: ::std::os::raw::c_uint = 0;
pub const F_LOCK: ::std::os::raw::c_uint = 1;
pub const F_TLOCK: ::std::os::raw::c_uint = 2;
pub const F_TEST: ::std::os::raw::c_uint = 3;
pub const _STRING_H: ::std::os::raw::c_uint = 1;
pub const _ASSERT_H: ::std::os::raw::c_uint = 1;
pub const _UNISTD_H: ::std::os::raw::c_uint = 1;
pub const _POSIX_VERSION: ::std::os::raw::c_uint = 200809;
pub const __POSIX2_THIS_VERSION: ::std::os::raw::c_uint = 200809;
pub const _POSIX2_VERSION: ::std::os::raw::c_uint = 200809;
pub const _POSIX2_C_VERSION: ::std::os::raw::c_uint = 200809;
pub const _POSIX2_C_BIND: ::std::os::raw::c_uint = 200809;
pub const _POSIX2_C_DEV: ::std::os::raw::c_uint = 200809;
pub const _POSIX2_SW_DEV: ::std::os::raw::c_uint = 200809;
pub const _POSIX2_LOCALEDEF: ::std::os::raw::c_uint = 200809;
pub const _XOPEN_VERSION: ::std::os::raw::c_uint = 700;
pub const _XOPEN_XCU_VERSION: ::std::os::raw::c_uint = 4;
pub const _XOPEN_XPG2: ::std::os::raw::c_uint = 1;
pub const _XOPEN_XPG3: ::std::os::raw::c_uint = 1;
pub const _XOPEN_XPG4: ::std::os::raw::c_uint = 1;
pub const _XOPEN_UNIX: ::std::os::raw::c_uint = 1;
pub const _XOPEN_CRYPT: ::std::os::raw::c_uint = 1;
pub const _XOPEN_ENH_I18N: ::std::os::raw::c_uint = 1;
pub const _XOPEN_LEGACY: ::std::os::raw::c_uint = 1;
pub const _BITS_POSIX_OPT_H: ::std::os::raw::c_uint = 1;
pub const _POSIX_JOB_CONTROL: ::std::os::raw::c_uint = 1;
pub const _POSIX_SAVED_IDS: ::std::os::raw::c_uint = 1;
pub const _POSIX_PRIORITY_SCHEDULING: ::std::os::raw::c_uint = 200809;
pub const _POSIX_SYNCHRONIZED_IO: ::std::os::raw::c_uint = 200809;
pub const _POSIX_FSYNC: ::std::os::raw::c_uint = 200809;
pub const _POSIX_MAPPED_FILES: ::std::os::raw::c_uint = 200809;
pub const _POSIX_MEMLOCK: ::std::os::raw::c_uint = 200809;
pub const _POSIX_MEMLOCK_RANGE: ::std::os::raw::c_uint = 200809;
pub const _POSIX_MEMORY_PROTECTION: ::std::os::raw::c_uint = 200809;
pub const _POSIX_CHOWN_RESTRICTED: ::std::os::raw::c_uint = 0;
pub const _POSIX_VDISABLE: u8 = b'\x00';
pub const _POSIX_NO_TRUNC: ::std::os::raw::c_uint = 1;
pub const _XOPEN_REALTIME: ::std::os::raw::c_uint = 1;
pub const _XOPEN_REALTIME_THREADS: ::std::os::raw::c_uint = 1;
pub const _XOPEN_SHM: ::std::os::raw::c_uint = 1;
pub const _POSIX_THREADS: ::std::os::raw::c_uint = 200809;
pub const _POSIX_REENTRANT_FUNCTIONS: ::std::os::raw::c_uint = 1;
pub const _POSIX_THREAD_SAFE_FUNCTIONS: ::std::os::raw::c_uint = 200809;
pub const _POSIX_THREAD_PRIORITY_SCHEDULING: ::std::os::raw::c_uint = 200809;
pub const _POSIX_THREAD_ATTR_STACKSIZE: ::std::os::raw::c_uint = 200809;
pub const _POSIX_THREAD_ATTR_STACKADDR: ::std::os::raw::c_uint = 200809;
pub const _POSIX_THREAD_PRIO_INHERIT: ::std::os::raw::c_uint = 200809;
pub const _POSIX_THREAD_PRIO_PROTECT: ::std::os::raw::c_uint = 200809;
pub const _POSIX_THREAD_ROBUST_PRIO_INHERIT: ::std::os::raw::c_uint = 200809;
pub const _POSIX_THREAD_ROBUST_PRIO_PROTECT: ::std::os::raw::c_int = -1;
pub const _POSIX_SEMAPHORES: ::std::os::raw::c_uint = 200809;
pub const _POSIX_REALTIME_SIGNALS: ::std::os::raw::c_uint = 200809;
pub const _POSIX_ASYNCHRONOUS_IO: ::std::os::raw::c_uint = 200809;
pub const _POSIX_ASYNC_IO: ::std::os::raw::c_uint = 1;
pub const _LFS_ASYNCHRONOUS_IO: ::std::os::raw::c_uint = 1;
pub const _POSIX_PRIORITIZED_IO: ::std::os::raw::c_uint = 200809;
pub const _LFS64_ASYNCHRONOUS_IO: ::std::os::raw::c_uint = 1;
pub const _LFS_LARGEFILE: ::std::os::raw::c_uint = 1;
pub const _LFS64_LARGEFILE: ::std::os::raw::c_uint = 1;
pub const _LFS64_STDIO: ::std::os::raw::c_uint = 1;
pub const _POSIX_SHARED_MEMORY_OBJECTS: ::std::os::raw::c_uint = 200809;
pub const _POSIX_CPUTIME: ::std::os::raw::c_uint = 0;
pub const _POSIX_THREAD_CPUTIME: ::std::os::raw::c_uint = 0;
pub const _POSIX_REGEXP: ::std::os::raw::c_uint = 1;
pub const _POSIX_READER_WRITER_LOCKS: ::std::os::raw::c_uint = 200809;
pub const _POSIX_SHELL: ::std::os::raw::c_uint = 1;
pub const _POSIX_TIMEOUTS: ::std::os::raw::c_uint = 200809;
pub const _POSIX_SPIN_LOCKS: ::std::os::raw::c_uint = 200809;
pub const _POSIX_SPAWN: ::std::os::raw::c_uint = 200809;
pub const _POSIX_TIMERS: ::std::os::raw::c_uint = 200809;
pub const _POSIX_BARRIERS: ::std::os::raw::c_uint = 200809;
pub const _POSIX_MESSAGE_PASSING: ::std::os::raw::c_uint = 200809;
pub const _POSIX_THREAD_PROCESS_SHARED: ::std::os::raw::c_uint = 200809;
pub const _POSIX_MONOTONIC_CLOCK: ::std::os::raw::c_uint = 0;
pub const _POSIX_CLOCK_SELECTION: ::std::os::raw::c_uint = 200809;
pub const _POSIX_ADVISORY_INFO: ::std::os::raw::c_uint = 200809;
pub const _POSIX_IPV6: ::std::os::raw::c_uint = 200809;
pub const _POSIX_RAW_SOCKETS: ::std::os::raw::c_uint = 200809;
pub const _POSIX2_CHAR_TERM: ::std::os::raw::c_uint = 200809;
pub const _POSIX_SPORADIC_SERVER: ::std::os::raw::c_int = -1;
pub const _POSIX_THREAD_SPORADIC_SERVER: ::std::os::raw::c_int = -1;
pub const _POSIX_TRACE: ::std::os::raw::c_int = -1;
pub const _POSIX_TRACE_EVENT_FILTER: ::std::os::raw::c_int = -1;
pub const _POSIX_TRACE_INHERIT: ::std::os::raw::c_int = -1;
pub const _POSIX_TRACE_LOG: ::std::os::raw::c_int = -1;
pub const _POSIX_TYPED_MEMORY_OBJECTS: ::std::os::raw::c_int = -1;
pub const _POSIX_V7_LPBIG_OFFBIG: ::std::os::raw::c_int = -1;
pub const _POSIX_V6_LPBIG_OFFBIG: ::std::os::raw::c_int = -1;
pub const _XBS5_LPBIG_OFFBIG: ::std::os::raw::c_int = -1;
pub const _POSIX_V7_LP64_OFF64: ::std::os::raw::c_uint = 1;
pub const _POSIX_V6_LP64_OFF64: ::std::os::raw::c_uint = 1;
pub const _XBS5_LP64_OFF64: ::std::os::raw::c_uint = 1;
pub const __ILP32_OFF32_CFLAGS: &'static [u8; 5usize] = b"-m32\x00";
pub const __ILP32_OFF32_LDFLAGS: &'static [u8; 5usize] = b"-m32\x00";
pub const __ILP32_OFFBIG_CFLAGS: &'static [u8; 48usize] =
    b"-m32 -D_LARGEFILE_SOURCE -D_FILE_OFFSET_BITS=64\x00";
pub const __ILP32_OFFBIG_LDFLAGS: &'static [u8; 5usize] = b"-m32\x00";
pub const __LP64_OFF64_CFLAGS: &'static [u8; 5usize] = b"-m64\x00";
pub const __LP64_OFF64_LDFLAGS: &'static [u8; 5usize] = b"-m64\x00";
pub const STDIN_FILENO: ::std::os::raw::c_uint = 0;
pub const STDOUT_FILENO: ::std::os::raw::c_uint = 1;
pub const STDERR_FILENO: ::std::os::raw::c_uint = 2;
pub const L_SET: ::std::os::raw::c_uint = 0;
pub const L_INCR: ::std::os::raw::c_uint = 1;
pub const L_XTND: ::std::os::raw::c_uint = 2;
pub const _LIBC_LIMITS_H_: ::std::os::raw::c_uint = 1;
pub const MB_LEN_MAX: ::std::os::raw::c_uint = 16;
pub const _BITS_POSIX2_LIM_H: ::std::os::raw::c_uint = 1;
pub const _POSIX2_BC_BASE_MAX: ::std::os::raw::c_uint = 99;
pub const _POSIX2_BC_DIM_MAX: ::std::os::raw::c_uint = 2048;
pub const _POSIX2_BC_SCALE_MAX: ::std::os::raw::c_uint = 99;
pub const _POSIX2_BC_STRING_MAX: ::std::os::raw::c_uint = 1000;
pub const _POSIX2_COLL_WEIGHTS_MAX: ::std::os::raw::c_uint = 2;
pub const _POSIX2_EXPR_NEST_MAX: ::std::os::raw::c_uint = 32;
pub const _POSIX2_LINE_MAX: ::std::os::raw::c_uint = 2048;
pub const _POSIX2_RE_DUP_MAX: ::std::os::raw::c_uint = 255;
pub const _POSIX2_CHARCLASS_NAME_MAX: ::std::os::raw::c_uint = 14;
pub const BC_BASE_MAX: ::std::os::raw::c_uint = 99;
pub const BC_DIM_MAX: ::std::os::raw::c_uint = 2048;
pub const BC_SCALE_MAX: ::std::os::raw::c_uint = 99;
pub const BC_STRING_MAX: ::std::os::raw::c_uint = 1000;
pub const COLL_WEIGHTS_MAX: ::std::os::raw::c_uint = 255;
pub const EXPR_NEST_MAX: ::std::os::raw::c_uint = 32;
pub const LINE_MAX: ::std::os::raw::c_uint = 2048;
pub const CHARCLASS_NAME_MAX: ::std::os::raw::c_uint = 2048;
pub const RE_DUP_MAX: ::std::os::raw::c_uint = 32767;
pub const _MATH_H: ::std::os::raw::c_uint = 1;
pub const _BITS_LIBM_SIMD_DECL_STUBS_H: ::std::os::raw::c_uint = 1;
pub const __FP_LOGB0_IS_MIN: ::std::os::raw::c_uint = 1;
pub const __FP_LOGBNAN_IS_MIN: ::std::os::raw::c_uint = 1;
pub const FP_ILOGB0: ::std::os::raw::c_int = -2147483648;
pub const FP_ILOGBNAN: ::std::os::raw::c_int = -2147483648;
pub const __MATH_DECLARING_DOUBLE: ::std::os::raw::c_uint = 1;
pub const __MATH_DECLARE_LDOUBLE: ::std::os::raw::c_uint = 1;
pub const FP_NAN: ::std::os::raw::c_uint = 0;
pub const FP_INFINITE: ::std::os::raw::c_uint = 1;
pub const FP_ZERO: ::std::os::raw::c_uint = 2;
pub const FP_SUBNORMAL: ::std::os::raw::c_uint = 3;
pub const FP_NORMAL: ::std::os::raw::c_uint = 4;
pub const MATH_ERRNO: ::std::os::raw::c_uint = 1;
pub const MATH_ERREXCEPT: ::std::os::raw::c_uint = 2;
pub const math_errhandling: ::std::os::raw::c_uint = 3;
pub const DOMAIN: ::std::os::raw::c_uint = 1;
pub const SING: ::std::os::raw::c_uint = 2;
pub const OVERFLOW: ::std::os::raw::c_uint = 3;
pub const UNDERFLOW: ::std::os::raw::c_uint = 4;
pub const TLOSS: ::std::os::raw::c_uint = 5;
pub const PLOSS: ::std::os::raw::c_uint = 6;
pub const M_E: f64 = 2.718281828459045;
pub const M_LOG2E: f64 = 1.4426950408889634;
pub const M_LOG10E: f64 = 0.4342944819032518;
pub const M_LN2: f64 = 0.6931471805599453;
pub const M_LN10: f64 = 2.302585092994046;
pub const M_PI: f64 = 3.141592653589793;
pub const M_PI_2: f64 = 1.5707963267948966;
pub const M_PI_4: f64 = 0.7853981633974483;
pub const M_1_PI: f64 = 0.3183098861837907;
pub const M_2_PI: f64 = 0.6366197723675814;
pub const M_2_SQRTPI: f64 = 1.1283791670955126;
pub const M_SQRT2: f64 = 1.4142135623730951;
pub const M_SQRT1_2: f64 = 0.7071067811865476;
pub const GRAVITY_LEXEM_DEBUG: ::std::os::raw::c_uint = 0;
pub const GRAVITY_LEXER_DEGUB: ::std::os::raw::c_uint = 0;
pub const GRAVITY_PARSER_DEBUG: ::std::os::raw::c_uint = 0;
pub const GRAVITY_SEMANTIC_DEBUG: ::std::os::raw::c_uint = 0;
pub const GRAVITY_AST_DEBUG: ::std::os::raw::c_uint = 0;
pub const GRAVITY_LOOKUP_DEBUG: ::std::os::raw::c_uint = 0;
pub const GRAVITY_SYMTABLE_DEBUG: ::std::os::raw::c_uint = 0;
pub const GRAVITY_CODEGEN_DEBUG: ::std::os::raw::c_uint = 0;
pub const GRAVITY_OPCODE_DEBUG: ::std::os::raw::c_uint = 0;
pub const GRAVITY_BYTECODE_DEBUG: ::std::os::raw::c_uint = 0;
pub const GRAVITY_REGISTER_DEBUG: ::std::os::raw::c_uint = 0;
pub const GRAVITY_FREE_DEBUG: ::std::os::raw::c_uint = 0;
pub const GRAVITY_DESERIALIZE_DEBUG: ::std::os::raw::c_uint = 0;
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
pub const AUTOLENGTH: ::std::os::raw::c_uint = 4294967295;
pub const GRAVITY_JSON_FUNCTION: &'static [u8; 9usize] = b"function\x00";
pub const GRAVITY_JSON_CLASS: &'static [u8; 6usize] = b"class\x00";
pub const GRAVITY_JSON_ENUM: &'static [u8; 5usize] = b"enum\x00";
pub const GRAVITY_JSON_MAP: &'static [u8; 4usize] = b"map\x00";
pub const GRAVITY_JSON_VAR: &'static [u8; 4usize] = b"var\x00";
pub const GRAVITY_JSON_GETTER: &'static [u8; 5usize] = b"$get\x00";
pub const GRAVITY_JSON_SETTER: &'static [u8; 5usize] = b"$set\x00";
pub const GRAVITY_JSON_LABELTAG: &'static [u8; 4usize] = b"tag\x00";
pub const GRAVITY_JSON_LABELNAME: &'static [u8; 5usize] = b"name\x00";
pub const GRAVITY_JSON_LABELTYPE: &'static [u8; 5usize] = b"type\x00";
pub const GRAVITY_JSON_LABELIDENTIFIER: &'static [u8; 11usize] =
    b"identifier\x00";
pub const GRAVITY_JSON_LABELPOOL: &'static [u8; 5usize] = b"pool\x00";
pub const GRAVITY_JSON_LABELMETA: &'static [u8; 5usize] = b"meta\x00";
pub const GRAVITY_JSON_LABELBYTECODE: &'static [u8; 9usize] = b"bytecode\x00";
pub const GRAVITY_JSON_LABELNPARAM: &'static [u8; 7usize] = b"nparam\x00";
pub const GRAVITY_JSON_LABELNLOCAL: &'static [u8; 7usize] = b"nlocal\x00";
pub const GRAVITY_JSON_LABELNTEMP: &'static [u8; 6usize] = b"ntemp\x00";
pub const GRAVITY_JSON_LABELNUPV: &'static [u8; 4usize] = b"nup\x00";
pub const GRAVITY_JSON_LABELARGS: &'static [u8; 5usize] = b"args\x00";
pub const GRAVITY_JSON_LABELINDEX: &'static [u8; 6usize] = b"index\x00";
pub const GRAVITY_JSON_LABELSUPER: &'static [u8; 6usize] = b"super\x00";
pub const GRAVITY_JSON_LABELNIVAR: &'static [u8; 6usize] = b"nivar\x00";
pub const GRAVITY_JSON_LABELSIVAR: &'static [u8; 6usize] = b"sivar\x00";
pub const GRAVITY_JSON_LABELPURITY: &'static [u8; 7usize] = b"purity\x00";
pub const GRAVITY_JSON_LABELREADONLY: &'static [u8; 9usize] = b"readonly\x00";
pub const GRAVITY_JSON_LABELSTORE: &'static [u8; 6usize] = b"store\x00";
pub const GRAVITY_JSON_LABELINIT: &'static [u8; 5usize] = b"init\x00";
pub const GRAVITY_JSON_LABELSTATIC: &'static [u8; 7usize] = b"static\x00";
pub const GRAVITY_JSON_LABELPARAMS: &'static [u8; 7usize] = b"params\x00";
pub const GRAVITY_JSON_LABELSTRUCT: &'static [u8; 7usize] = b"struct\x00";
pub const GRAVITY_VM_ANONYMOUS_PREFIX: &'static [u8; 3usize] = b"$$\x00";
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
#[test]
fn bindgen_test_layout___fsid_t() {
    assert_eq!(::std::mem::size_of::<__fsid_t>() , 8usize , concat ! (
               "Size of: " , stringify ! ( __fsid_t ) ));
    assert_eq! (::std::mem::align_of::<__fsid_t>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( __fsid_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const __fsid_t ) ) . __val as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( __fsid_t ) , "::" ,
                stringify ! ( __val ) ));
}
impl Clone for __fsid_t {
    fn clone(&self) -> Self { *self }
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
#[derive(Debug, Copy)]
pub struct _IO_FILE {
    pub _flags: ::std::os::raw::c_int,
    pub _IO_read_ptr: *mut ::std::os::raw::c_char,
    pub _IO_read_end: *mut ::std::os::raw::c_char,
    pub _IO_read_base: *mut ::std::os::raw::c_char,
    pub _IO_write_base: *mut ::std::os::raw::c_char,
    pub _IO_write_ptr: *mut ::std::os::raw::c_char,
    pub _IO_write_end: *mut ::std::os::raw::c_char,
    pub _IO_buf_base: *mut ::std::os::raw::c_char,
    pub _IO_buf_end: *mut ::std::os::raw::c_char,
    pub _IO_save_base: *mut ::std::os::raw::c_char,
    pub _IO_backup_base: *mut ::std::os::raw::c_char,
    pub _IO_save_end: *mut ::std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::std::os::raw::c_int,
    pub _flags2: ::std::os::raw::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::std::os::raw::c_ushort,
    pub _vtable_offset: ::std::os::raw::c_schar,
    pub _shortbuf: [::std::os::raw::c_char; 1usize],
    pub _lock: *mut _IO_lock_t,
    pub _offset: __off64_t,
    pub __pad1: *mut ::std::os::raw::c_void,
    pub __pad2: *mut ::std::os::raw::c_void,
    pub __pad3: *mut ::std::os::raw::c_void,
    pub __pad4: *mut ::std::os::raw::c_void,
    pub __pad5: usize,
    pub _mode: ::std::os::raw::c_int,
    pub _unused2: [::std::os::raw::c_char; 20usize],
}
#[test]
fn bindgen_test_layout__IO_FILE() {
    assert_eq!(::std::mem::size_of::<_IO_FILE>() , 216usize , concat ! (
               "Size of: " , stringify ! ( _IO_FILE ) ));
    assert_eq! (::std::mem::align_of::<_IO_FILE>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( _IO_FILE ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . _flags as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( _flags ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . _IO_read_ptr as * const _
                as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( _IO_read_ptr ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . _IO_read_end as * const _
                as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( _IO_read_end ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . _IO_read_base as * const _
                as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( _IO_read_base ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . _IO_write_base as * const
                _ as usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( _IO_write_base ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . _IO_write_ptr as * const _
                as usize } , 40usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( _IO_write_ptr ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . _IO_write_end as * const _
                as usize } , 48usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( _IO_write_end ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . _IO_buf_base as * const _
                as usize } , 56usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( _IO_buf_base ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . _IO_buf_end as * const _
                as usize } , 64usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( _IO_buf_end ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . _IO_save_base as * const _
                as usize } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( _IO_save_base ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . _IO_backup_base as * const
                _ as usize } , 80usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( _IO_backup_base ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . _IO_save_end as * const _
                as usize } , 88usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( _IO_save_end ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . _markers as * const _ as
                usize } , 96usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( _markers ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . _chain as * const _ as
                usize } , 104usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( _chain ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . _fileno as * const _ as
                usize } , 112usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( _fileno ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . _flags2 as * const _ as
                usize } , 116usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( _flags2 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . _old_offset as * const _
                as usize } , 120usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( _old_offset ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . _cur_column as * const _
                as usize } , 128usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( _cur_column ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . _vtable_offset as * const
                _ as usize } , 130usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( _vtable_offset ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . _shortbuf as * const _ as
                usize } , 131usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( _shortbuf ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . _lock as * const _ as
                usize } , 136usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( _lock ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . _offset as * const _ as
                usize } , 144usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( _offset ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . __pad1 as * const _ as
                usize } , 152usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( __pad1 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . __pad2 as * const _ as
                usize } , 160usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( __pad2 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . __pad3 as * const _ as
                usize } , 168usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( __pad3 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . __pad4 as * const _ as
                usize } , 176usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( __pad4 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . __pad5 as * const _ as
                usize } , 184usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( __pad5 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . _mode as * const _ as
                usize } , 192usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( _mode ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_FILE ) ) . _unused2 as * const _ as
                usize } , 196usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_FILE ) , "::" ,
                stringify ! ( _unused2 ) ));
}
impl Clone for _IO_FILE {
    fn clone(&self) -> Self { *self }
}
pub type FILE = _IO_FILE;
pub type __FILE = _IO_FILE;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct __mbstate_t {
    pub __count: ::std::os::raw::c_int,
    pub __value: __mbstate_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct __mbstate_t__bindgen_ty_1 {
    pub __wch: __BindgenUnionField<::std::os::raw::c_uint>,
    pub __wchb: __BindgenUnionField<[::std::os::raw::c_char; 4usize]>,
    pub bindgen_union_field: u32,
}
#[test]
fn bindgen_test_layout___mbstate_t__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<__mbstate_t__bindgen_ty_1>() , 4usize ,
               concat ! (
               "Size of: " , stringify ! ( __mbstate_t__bindgen_ty_1 ) ));
    assert_eq! (::std::mem::align_of::<__mbstate_t__bindgen_ty_1>() , 4usize ,
                concat ! (
                "Alignment of " , stringify ! ( __mbstate_t__bindgen_ty_1 )
                ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const __mbstate_t__bindgen_ty_1 ) ) . __wch as
                * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                __mbstate_t__bindgen_ty_1 ) , "::" , stringify ! ( __wch ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const __mbstate_t__bindgen_ty_1 ) ) . __wchb as
                * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                __mbstate_t__bindgen_ty_1 ) , "::" , stringify ! ( __wchb )
                ));
}
impl Clone for __mbstate_t__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout___mbstate_t() {
    assert_eq!(::std::mem::size_of::<__mbstate_t>() , 8usize , concat ! (
               "Size of: " , stringify ! ( __mbstate_t ) ));
    assert_eq! (::std::mem::align_of::<__mbstate_t>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( __mbstate_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const __mbstate_t ) ) . __count as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( __mbstate_t ) , "::" ,
                stringify ! ( __count ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const __mbstate_t ) ) . __value as * const _ as
                usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( __mbstate_t ) , "::" ,
                stringify ! ( __value ) ));
}
impl Clone for __mbstate_t {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct _G_fpos_t {
    pub __pos: __off_t,
    pub __state: __mbstate_t,
}
#[test]
fn bindgen_test_layout__G_fpos_t() {
    assert_eq!(::std::mem::size_of::<_G_fpos_t>() , 16usize , concat ! (
               "Size of: " , stringify ! ( _G_fpos_t ) ));
    assert_eq! (::std::mem::align_of::<_G_fpos_t>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( _G_fpos_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _G_fpos_t ) ) . __pos as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( _G_fpos_t ) , "::" ,
                stringify ! ( __pos ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _G_fpos_t ) ) . __state as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( _G_fpos_t ) , "::" ,
                stringify ! ( __state ) ));
}
impl Clone for _G_fpos_t {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct _G_fpos64_t {
    pub __pos: __off64_t,
    pub __state: __mbstate_t,
}
#[test]
fn bindgen_test_layout__G_fpos64_t() {
    assert_eq!(::std::mem::size_of::<_G_fpos64_t>() , 16usize , concat ! (
               "Size of: " , stringify ! ( _G_fpos64_t ) ));
    assert_eq! (::std::mem::align_of::<_G_fpos64_t>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( _G_fpos64_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _G_fpos64_t ) ) . __pos as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( _G_fpos64_t ) , "::" ,
                stringify ! ( __pos ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _G_fpos64_t ) ) . __state as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( _G_fpos64_t ) , "::" ,
                stringify ! ( __state ) ));
}
impl Clone for _G_fpos64_t {
    fn clone(&self) -> Self { *self }
}
pub type va_list = __builtin_va_list;
pub type __gnuc_va_list = __builtin_va_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_jump_t([u8; 0]);
pub type _IO_lock_t = ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout__IO_marker() {
    assert_eq!(::std::mem::size_of::<_IO_marker>() , 24usize , concat ! (
               "Size of: " , stringify ! ( _IO_marker ) ));
    assert_eq! (::std::mem::align_of::<_IO_marker>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( _IO_marker ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_marker ) ) . _next as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_marker ) , "::" ,
                stringify ! ( _next ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_marker ) ) . _sbuf as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_marker ) , "::" ,
                stringify ! ( _sbuf ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _IO_marker ) ) . _pos as * const _ as
                usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( _IO_marker ) , "::" ,
                stringify ! ( _pos ) ));
}
impl Clone for _IO_marker {
    fn clone(&self) -> Self { *self }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum __codecvt_result {
    __codecvt_ok = 0,
    __codecvt_partial = 1,
    __codecvt_error = 2,
    __codecvt_noconv = 3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_FILE_plus([u8; 0]);
extern "C" {
    #[link_name = "_IO_2_1_stdin_"]
    pub static mut _IO_2_1_stdin_: _IO_FILE_plus;
}
extern "C" {
    #[link_name = "_IO_2_1_stdout_"]
    pub static mut _IO_2_1_stdout_: _IO_FILE_plus;
}
extern "C" {
    #[link_name = "_IO_2_1_stderr_"]
    pub static mut _IO_2_1_stderr_: _IO_FILE_plus;
}
pub type __io_read_fn =
    ::std::option::Option<unsafe extern "C" fn(__cookie:
                                                   *mut ::std::os::raw::c_void,
                                               __buf:
                                                   *mut ::std::os::raw::c_char,
                                               __nbytes: usize) -> __ssize_t>;
pub type __io_write_fn =
    ::std::option::Option<unsafe extern "C" fn(__cookie:
                                                   *mut ::std::os::raw::c_void,
                                               __buf:
                                                   *const ::std::os::raw::c_char,
                                               __n: usize) -> __ssize_t>;
pub type __io_seek_fn =
    ::std::option::Option<unsafe extern "C" fn(__cookie:
                                                   *mut ::std::os::raw::c_void,
                                               __pos: *mut __off64_t,
                                               __w: ::std::os::raw::c_int)
                              -> ::std::os::raw::c_int>;
pub type __io_close_fn =
    ::std::option::Option<unsafe extern "C" fn(__cookie:
                                                   *mut ::std::os::raw::c_void)
                              -> ::std::os::raw::c_int>;
extern "C" {
    pub fn __underflow(arg1: *mut _IO_FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __uflow(arg1: *mut _IO_FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __overflow(arg1: *mut _IO_FILE, arg2: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_getc(__fp: *mut _IO_FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_putc(__c: ::std::os::raw::c_int, __fp: *mut _IO_FILE)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_feof(__fp: *mut _IO_FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_ferror(__fp: *mut _IO_FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_peekc_locked(__fp: *mut _IO_FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_flockfile(arg1: *mut _IO_FILE);
}
extern "C" {
    pub fn _IO_funlockfile(arg1: *mut _IO_FILE);
}
extern "C" {
    pub fn _IO_ftrylockfile(arg1: *mut _IO_FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_vfscanf(arg1: *mut _IO_FILE,
                       arg2: *const ::std::os::raw::c_char,
                       arg3: *mut __va_list_tag,
                       arg4: *mut ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_vfprintf(arg1: *mut _IO_FILE,
                        arg2: *const ::std::os::raw::c_char,
                        arg3: *mut __va_list_tag) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_padn(arg1: *mut _IO_FILE, arg2: ::std::os::raw::c_int,
                    arg3: __ssize_t) -> __ssize_t;
}
extern "C" {
    pub fn _IO_sgetn(arg1: *mut _IO_FILE, arg2: *mut ::std::os::raw::c_void,
                     arg3: usize) -> usize;
}
extern "C" {
    pub fn _IO_seekoff(arg1: *mut _IO_FILE, arg2: __off64_t,
                       arg3: ::std::os::raw::c_int,
                       arg4: ::std::os::raw::c_int) -> __off64_t;
}
extern "C" {
    pub fn _IO_seekpos(arg1: *mut _IO_FILE, arg2: __off64_t,
                       arg3: ::std::os::raw::c_int) -> __off64_t;
}
extern "C" {
    pub fn _IO_free_backup_area(arg1: *mut _IO_FILE);
}
pub type off_t = __off_t;
pub type fpos_t = _G_fpos_t;
extern "C" {
    #[link_name = "stdin"]
    pub static mut stdin: *mut _IO_FILE;
}
extern "C" {
    #[link_name = "stdout"]
    pub static mut stdout: *mut _IO_FILE;
}
extern "C" {
    #[link_name = "stderr"]
    pub static mut stderr: *mut _IO_FILE;
}
extern "C" {
    pub fn remove(__filename: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rename(__old: *const ::std::os::raw::c_char,
                  __new: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn renameat(__oldfd: ::std::os::raw::c_int,
                    __old: *const ::std::os::raw::c_char,
                    __newfd: ::std::os::raw::c_int,
                    __new: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tmpfile() -> *mut FILE;
}
extern "C" {
    pub fn tmpnam(__s: *mut ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tmpnam_r(__s: *mut ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tempnam(__dir: *const ::std::os::raw::c_char,
                   __pfx: *const ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fflush(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fflush_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fopen(__filename: *const ::std::os::raw::c_char,
                 __modes: *const ::std::os::raw::c_char) -> *mut FILE;
}
extern "C" {
    pub fn freopen(__filename: *const ::std::os::raw::c_char,
                   __modes: *const ::std::os::raw::c_char,
                   __stream: *mut FILE) -> *mut FILE;
}
extern "C" {
    pub fn fdopen(__fd: ::std::os::raw::c_int,
                  __modes: *const ::std::os::raw::c_char) -> *mut FILE;
}
extern "C" {
    pub fn fmemopen(__s: *mut ::std::os::raw::c_void, __len: usize,
                    __modes: *const ::std::os::raw::c_char) -> *mut FILE;
}
extern "C" {
    pub fn open_memstream(__bufloc: *mut *mut ::std::os::raw::c_char,
                          __sizeloc: *mut usize) -> *mut FILE;
}
extern "C" {
    pub fn setbuf(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn setvbuf(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char,
                   __modes: ::std::os::raw::c_int, __n: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setbuffer(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char,
                     __size: usize);
}
extern "C" {
    pub fn setlinebuf(__stream: *mut FILE);
}
extern "C" {
    pub fn fprintf(__stream: *mut FILE,
                   __format: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn printf(__format: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sprintf(__s: *mut ::std::os::raw::c_char,
                   __format: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfprintf(__s: *mut FILE, __format: *const ::std::os::raw::c_char,
                    __arg: *mut __va_list_tag) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vprintf(__format: *const ::std::os::raw::c_char,
                   __arg: *mut __va_list_tag) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsprintf(__s: *mut ::std::os::raw::c_char,
                    __format: *const ::std::os::raw::c_char,
                    __arg: *mut __va_list_tag) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn snprintf(__s: *mut ::std::os::raw::c_char, __maxlen: usize,
                    __format: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsnprintf(__s: *mut ::std::os::raw::c_char, __maxlen: usize,
                     __format: *const ::std::os::raw::c_char,
                     __arg: *mut __va_list_tag) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vdprintf(__fd: ::std::os::raw::c_int,
                    __fmt: *const ::std::os::raw::c_char,
                    __arg: *mut __va_list_tag) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn dprintf(__fd: ::std::os::raw::c_int,
                   __fmt: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fscanf(__stream: *mut FILE,
                  __format: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scanf(__format: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sscanf(__s: *const ::std::os::raw::c_char,
                  __format: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "__isoc99_fscanf"]
    pub fn fscanf1(__stream: *mut FILE,
                   __format: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "__isoc99_scanf"]
    pub fn scanf1(__format: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "__isoc99_sscanf"]
    pub fn sscanf1(__s: *const ::std::os::raw::c_char,
                   __format: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfscanf(__s: *mut FILE, __format: *const ::std::os::raw::c_char,
                   __arg: *mut __va_list_tag) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vscanf(__format: *const ::std::os::raw::c_char,
                  __arg: *mut __va_list_tag) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsscanf(__s: *const ::std::os::raw::c_char,
                   __format: *const ::std::os::raw::c_char,
                   __arg: *mut __va_list_tag) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "__isoc99_vfscanf"]
    pub fn vfscanf1(__s: *mut FILE, __format: *const ::std::os::raw::c_char,
                    __arg: *mut __va_list_tag) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "__isoc99_vscanf"]
    pub fn vscanf1(__format: *const ::std::os::raw::c_char,
                   __arg: *mut __va_list_tag) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "__isoc99_vsscanf"]
    pub fn vsscanf1(__s: *const ::std::os::raw::c_char,
                    __format: *const ::std::os::raw::c_char,
                    __arg: *mut __va_list_tag) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetc(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getc(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getchar() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getc_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getchar_unlocked() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetc_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputc(__c: ::std::os::raw::c_int, __stream: *mut FILE)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putc(__c: ::std::os::raw::c_int, __stream: *mut FILE)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putchar(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputc_unlocked(__c: ::std::os::raw::c_int, __stream: *mut FILE)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putc_unlocked(__c: ::std::os::raw::c_int, __stream: *mut FILE)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putchar_unlocked(__c: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getw(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putw(__w: ::std::os::raw::c_int, __stream: *mut FILE)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgets(__s: *mut ::std::os::raw::c_char, __n: ::std::os::raw::c_int,
                 __stream: *mut FILE) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __getdelim(__lineptr: *mut *mut ::std::os::raw::c_char,
                      __n: *mut usize, __delimiter: ::std::os::raw::c_int,
                      __stream: *mut FILE) -> __ssize_t;
}
extern "C" {
    pub fn getdelim(__lineptr: *mut *mut ::std::os::raw::c_char,
                    __n: *mut usize, __delimiter: ::std::os::raw::c_int,
                    __stream: *mut FILE) -> __ssize_t;
}
extern "C" {
    pub fn getline(__lineptr: *mut *mut ::std::os::raw::c_char,
                   __n: *mut usize, __stream: *mut FILE) -> __ssize_t;
}
extern "C" {
    pub fn fputs(__s: *const ::std::os::raw::c_char, __stream: *mut FILE)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn puts(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ungetc(__c: ::std::os::raw::c_int, __stream: *mut FILE)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fread(__ptr: *mut ::std::os::raw::c_void, __size: usize,
                 __n: usize, __stream: *mut FILE) -> usize;
}
extern "C" {
    pub fn fwrite(__ptr: *const ::std::os::raw::c_void, __size: usize,
                  __n: usize, __s: *mut FILE) -> usize;
}
extern "C" {
    pub fn fread_unlocked(__ptr: *mut ::std::os::raw::c_void, __size: usize,
                          __n: usize, __stream: *mut FILE) -> usize;
}
extern "C" {
    pub fn fwrite_unlocked(__ptr: *const ::std::os::raw::c_void,
                           __size: usize, __n: usize, __stream: *mut FILE)
     -> usize;
}
extern "C" {
    pub fn fseek(__stream: *mut FILE, __off: ::std::os::raw::c_long,
                 __whence: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftell(__stream: *mut FILE) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn rewind(__stream: *mut FILE);
}
extern "C" {
    pub fn fseeko(__stream: *mut FILE, __off: __off_t,
                  __whence: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftello(__stream: *mut FILE) -> __off_t;
}
extern "C" {
    pub fn fgetpos(__stream: *mut FILE, __pos: *mut fpos_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fsetpos(__stream: *mut FILE, __pos: *const fpos_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearerr(__stream: *mut FILE);
}
extern "C" {
    pub fn feof(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ferror(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearerr_unlocked(__stream: *mut FILE);
}
extern "C" {
    pub fn feof_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ferror_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn perror(__s: *const ::std::os::raw::c_char);
}
extern "C" {
    #[link_name = "sys_nerr"]
    pub static mut sys_nerr: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "sys_errlist"]
    pub static mut sys_errlist: [*const ::std::os::raw::c_char; 0usize];
}
extern "C" {
    pub fn fileno(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fileno_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn popen(__command: *const ::std::os::raw::c_char,
                 __modes: *const ::std::os::raw::c_char) -> *mut FILE;
}
extern "C" {
    pub fn pclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ctermid(__s: *mut ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn flockfile(__stream: *mut FILE);
}
extern "C" {
    pub fn ftrylockfile(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn funlockfile(__stream: *mut FILE);
}
pub type wchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct div_t {
    pub quot: ::std::os::raw::c_int,
    pub rem: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_div_t() {
    assert_eq!(::std::mem::size_of::<div_t>() , 8usize , concat ! (
               "Size of: " , stringify ! ( div_t ) ));
    assert_eq! (::std::mem::align_of::<div_t>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( div_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const div_t ) ) . quot as * const _ as usize }
                , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( div_t ) , "::" ,
                stringify ! ( quot ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const div_t ) ) . rem as * const _ as usize } ,
                4usize , concat ! (
                "Alignment of field: " , stringify ! ( div_t ) , "::" ,
                stringify ! ( rem ) ));
}
impl Clone for div_t {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct ldiv_t {
    pub quot: ::std::os::raw::c_long,
    pub rem: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_ldiv_t() {
    assert_eq!(::std::mem::size_of::<ldiv_t>() , 16usize , concat ! (
               "Size of: " , stringify ! ( ldiv_t ) ));
    assert_eq! (::std::mem::align_of::<ldiv_t>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( ldiv_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ldiv_t ) ) . quot as * const _ as usize }
                , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( ldiv_t ) , "::" ,
                stringify ! ( quot ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ldiv_t ) ) . rem as * const _ as usize }
                , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( ldiv_t ) , "::" ,
                stringify ! ( rem ) ));
}
impl Clone for ldiv_t {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct lldiv_t {
    pub quot: ::std::os::raw::c_longlong,
    pub rem: ::std::os::raw::c_longlong,
}
#[test]
fn bindgen_test_layout_lldiv_t() {
    assert_eq!(::std::mem::size_of::<lldiv_t>() , 16usize , concat ! (
               "Size of: " , stringify ! ( lldiv_t ) ));
    assert_eq! (::std::mem::align_of::<lldiv_t>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( lldiv_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const lldiv_t ) ) . quot as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( lldiv_t ) , "::" ,
                stringify ! ( quot ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const lldiv_t ) ) . rem as * const _ as usize }
                , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( lldiv_t ) , "::" ,
                stringify ! ( rem ) ));
}
impl Clone for lldiv_t {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    pub fn __ctype_get_mb_cur_max() -> usize;
}
extern "C" {
    pub fn atof(__nptr: *const ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn atoi(__nptr: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn atol(__nptr: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn atoll(__nptr: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtod(__nptr: *const ::std::os::raw::c_char,
                  __endptr: *mut *mut ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn strtof(__nptr: *const ::std::os::raw::c_char,
                  __endptr: *mut *mut ::std::os::raw::c_char) -> f32;
}
extern "C" {
    pub fn strtold(__nptr: *const ::std::os::raw::c_char,
                   __endptr: *mut *mut ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn strtol(__nptr: *const ::std::os::raw::c_char,
                  __endptr: *mut *mut ::std::os::raw::c_char,
                  __base: ::std::os::raw::c_int) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn strtoul(__nptr: *const ::std::os::raw::c_char,
                   __endptr: *mut *mut ::std::os::raw::c_char,
                   __base: ::std::os::raw::c_int) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strtoq(__nptr: *const ::std::os::raw::c_char,
                  __endptr: *mut *mut ::std::os::raw::c_char,
                  __base: ::std::os::raw::c_int)
     -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtouq(__nptr: *const ::std::os::raw::c_char,
                   __endptr: *mut *mut ::std::os::raw::c_char,
                   __base: ::std::os::raw::c_int)
     -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn strtoll(__nptr: *const ::std::os::raw::c_char,
                   __endptr: *mut *mut ::std::os::raw::c_char,
                   __base: ::std::os::raw::c_int)
     -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtoull(__nptr: *const ::std::os::raw::c_char,
                    __endptr: *mut *mut ::std::os::raw::c_char,
                    __base: ::std::os::raw::c_int)
     -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn l64a(__n: ::std::os::raw::c_long) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn a64l(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type u_long = __u_long;
pub type quad_t = __quad_t;
pub type u_quad_t = __u_quad_t;
pub type fsid_t = __fsid_t;
pub type loff_t = __loff_t;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type nlink_t = __nlink_t;
pub type uid_t = __uid_t;
pub type pid_t = __pid_t;
pub type id_t = __id_t;
pub type daddr_t = __daddr_t;
pub type caddr_t = __caddr_t;
pub type key_t = __key_t;
pub type clock_t = __clock_t;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type timer_t = __timer_t;
pub type ulong = ::std::os::raw::c_ulong;
pub type ushort = ::std::os::raw::c_ushort;
pub type uint = ::std::os::raw::c_uint;
pub type u_int8_t = ::std::os::raw::c_uchar;
pub type u_int16_t = ::std::os::raw::c_ushort;
pub type u_int32_t = ::std::os::raw::c_uint;
pub type u_int64_t = ::std::os::raw::c_ulong;
pub type register_t = ::std::os::raw::c_long;
pub type __sig_atomic_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct __sigset_t {
    pub __val: [::std::os::raw::c_ulong; 16usize],
}
#[test]
fn bindgen_test_layout___sigset_t() {
    assert_eq!(::std::mem::size_of::<__sigset_t>() , 128usize , concat ! (
               "Size of: " , stringify ! ( __sigset_t ) ));
    assert_eq! (::std::mem::align_of::<__sigset_t>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( __sigset_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const __sigset_t ) ) . __val as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( __sigset_t ) , "::" ,
                stringify ! ( __val ) ));
}
impl Clone for __sigset_t {
    fn clone(&self) -> Self { *self }
}
pub type sigset_t = __sigset_t;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[test]
fn bindgen_test_layout_timeval() {
    assert_eq!(::std::mem::size_of::<timeval>() , 16usize , concat ! (
               "Size of: " , stringify ! ( timeval ) ));
    assert_eq! (::std::mem::align_of::<timeval>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( timeval ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const timeval ) ) . tv_sec as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( timeval ) , "::" ,
                stringify ! ( tv_sec ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const timeval ) ) . tv_usec as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( timeval ) , "::" ,
                stringify ! ( tv_usec ) ));
}
impl Clone for timeval {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[test]
fn bindgen_test_layout_timespec() {
    assert_eq!(::std::mem::size_of::<timespec>() , 16usize , concat ! (
               "Size of: " , stringify ! ( timespec ) ));
    assert_eq! (::std::mem::align_of::<timespec>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( timespec ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const timespec ) ) . tv_sec as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( timespec ) , "::" ,
                stringify ! ( tv_sec ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const timespec ) ) . tv_nsec as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( timespec ) , "::" ,
                stringify ! ( tv_nsec ) ));
}
impl Clone for timespec {
    fn clone(&self) -> Self { *self }
}
pub type suseconds_t = __suseconds_t;
pub type __fd_mask = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16usize],
}
#[test]
fn bindgen_test_layout_fd_set() {
    assert_eq!(::std::mem::size_of::<fd_set>() , 128usize , concat ! (
               "Size of: " , stringify ! ( fd_set ) ));
    assert_eq! (::std::mem::align_of::<fd_set>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( fd_set ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const fd_set ) ) . __fds_bits as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( fd_set ) , "::" ,
                stringify ! ( __fds_bits ) ));
}
impl Clone for fd_set {
    fn clone(&self) -> Self { *self }
}
pub type fd_mask = __fd_mask;
extern "C" {
    pub fn select(__nfds: ::std::os::raw::c_int, __readfds: *mut fd_set,
                  __writefds: *mut fd_set, __exceptfds: *mut fd_set,
                  __timeout: *mut timeval) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pselect(__nfds: ::std::os::raw::c_int, __readfds: *mut fd_set,
                   __writefds: *mut fd_set, __exceptfds: *mut fd_set,
                   __timeout: *const timespec, __sigmask: *const __sigset_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gnu_dev_major(__dev: __dev_t) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn gnu_dev_minor(__dev: __dev_t) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn gnu_dev_makedev(__major: ::std::os::raw::c_uint,
                           __minor: ::std::os::raw::c_uint) -> __dev_t;
}
pub type blksize_t = __blksize_t;
pub type blkcnt_t = __blkcnt_t;
pub type fsblkcnt_t = __fsblkcnt_t;
pub type fsfilcnt_t = __fsfilcnt_t;
pub type pthread_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct pthread_attr_t {
    pub __size: __BindgenUnionField<[::std::os::raw::c_char; 56usize]>,
    pub __align: __BindgenUnionField<::std::os::raw::c_long>,
    pub bindgen_union_field: [u64; 7usize],
}
#[test]
fn bindgen_test_layout_pthread_attr_t() {
    assert_eq!(::std::mem::size_of::<pthread_attr_t>() , 56usize , concat ! (
               "Size of: " , stringify ! ( pthread_attr_t ) ));
    assert_eq! (::std::mem::align_of::<pthread_attr_t>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( pthread_attr_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_attr_t ) ) . __size as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( pthread_attr_t ) , "::"
                , stringify ! ( __size ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_attr_t ) ) . __align as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( pthread_attr_t ) , "::"
                , stringify ! ( __align ) ));
}
impl Clone for pthread_attr_t {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
#[test]
fn bindgen_test_layout___pthread_internal_list() {
    assert_eq!(::std::mem::size_of::<__pthread_internal_list>() , 16usize ,
               concat ! (
               "Size of: " , stringify ! ( __pthread_internal_list ) ));
    assert_eq! (::std::mem::align_of::<__pthread_internal_list>() , 8usize ,
                concat ! (
                "Alignment of " , stringify ! ( __pthread_internal_list ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const __pthread_internal_list ) ) . __prev as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( __pthread_internal_list
                ) , "::" , stringify ! ( __prev ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const __pthread_internal_list ) ) . __next as *
                const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( __pthread_internal_list
                ) , "::" , stringify ! ( __next ) ));
}
impl Clone for __pthread_internal_list {
    fn clone(&self) -> Self { *self }
}
pub type __pthread_list_t = __pthread_internal_list;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct pthread_mutex_t {
    pub __data: __BindgenUnionField<pthread_mutex_t___pthread_mutex_s>,
    pub __size: __BindgenUnionField<[::std::os::raw::c_char; 40usize]>,
    pub __align: __BindgenUnionField<::std::os::raw::c_long>,
    pub bindgen_union_field: [u64; 5usize],
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct pthread_mutex_t___pthread_mutex_s {
    pub __lock: ::std::os::raw::c_int,
    pub __count: ::std::os::raw::c_uint,
    pub __owner: ::std::os::raw::c_int,
    pub __nusers: ::std::os::raw::c_uint,
    pub __kind: ::std::os::raw::c_int,
    pub __spins: ::std::os::raw::c_short,
    pub __elision: ::std::os::raw::c_short,
    pub __list: __pthread_list_t,
}
#[test]
fn bindgen_test_layout_pthread_mutex_t___pthread_mutex_s() {
    assert_eq!(::std::mem::size_of::<pthread_mutex_t___pthread_mutex_s>() ,
               40usize , concat ! (
               "Size of: " , stringify ! ( pthread_mutex_t___pthread_mutex_s )
               ));
    assert_eq! (::std::mem::align_of::<pthread_mutex_t___pthread_mutex_s>() ,
                8usize , concat ! (
                "Alignment of " , stringify ! (
                pthread_mutex_t___pthread_mutex_s ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_mutex_t___pthread_mutex_s ) ) .
                __lock as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_mutex_t___pthread_mutex_s ) , "::" , stringify ! (
                __lock ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_mutex_t___pthread_mutex_s ) ) .
                __count as * const _ as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_mutex_t___pthread_mutex_s ) , "::" , stringify ! (
                __count ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_mutex_t___pthread_mutex_s ) ) .
                __owner as * const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_mutex_t___pthread_mutex_s ) , "::" , stringify ! (
                __owner ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_mutex_t___pthread_mutex_s ) ) .
                __nusers as * const _ as usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_mutex_t___pthread_mutex_s ) , "::" , stringify ! (
                __nusers ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_mutex_t___pthread_mutex_s ) ) .
                __kind as * const _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_mutex_t___pthread_mutex_s ) , "::" , stringify ! (
                __kind ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_mutex_t___pthread_mutex_s ) ) .
                __spins as * const _ as usize } , 20usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_mutex_t___pthread_mutex_s ) , "::" , stringify ! (
                __spins ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_mutex_t___pthread_mutex_s ) ) .
                __elision as * const _ as usize } , 22usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_mutex_t___pthread_mutex_s ) , "::" , stringify ! (
                __elision ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_mutex_t___pthread_mutex_s ) ) .
                __list as * const _ as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_mutex_t___pthread_mutex_s ) , "::" , stringify ! (
                __list ) ));
}
impl Clone for pthread_mutex_t___pthread_mutex_s {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_pthread_mutex_t() {
    assert_eq!(::std::mem::size_of::<pthread_mutex_t>() , 40usize , concat ! (
               "Size of: " , stringify ! ( pthread_mutex_t ) ));
    assert_eq! (::std::mem::align_of::<pthread_mutex_t>() , 8usize , concat !
                ( "Alignment of " , stringify ! ( pthread_mutex_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_mutex_t ) ) . __data as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( pthread_mutex_t ) ,
                "::" , stringify ! ( __data ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_mutex_t ) ) . __size as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( pthread_mutex_t ) ,
                "::" , stringify ! ( __size ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_mutex_t ) ) . __align as * const
                _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( pthread_mutex_t ) ,
                "::" , stringify ! ( __align ) ));
}
impl Clone for pthread_mutex_t {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct pthread_mutexattr_t {
    pub __size: __BindgenUnionField<[::std::os::raw::c_char; 4usize]>,
    pub __align: __BindgenUnionField<::std::os::raw::c_int>,
    pub bindgen_union_field: u32,
}
#[test]
fn bindgen_test_layout_pthread_mutexattr_t() {
    assert_eq!(::std::mem::size_of::<pthread_mutexattr_t>() , 4usize , concat
               ! ( "Size of: " , stringify ! ( pthread_mutexattr_t ) ));
    assert_eq! (::std::mem::align_of::<pthread_mutexattr_t>() , 4usize ,
                concat ! (
                "Alignment of " , stringify ! ( pthread_mutexattr_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_mutexattr_t ) ) . __size as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( pthread_mutexattr_t ) ,
                "::" , stringify ! ( __size ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_mutexattr_t ) ) . __align as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( pthread_mutexattr_t ) ,
                "::" , stringify ! ( __align ) ));
}
impl Clone for pthread_mutexattr_t {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct pthread_cond_t {
    pub __data: __BindgenUnionField<pthread_cond_t__bindgen_ty_1>,
    pub __size: __BindgenUnionField<[::std::os::raw::c_char; 48usize]>,
    pub __align: __BindgenUnionField<::std::os::raw::c_longlong>,
    pub bindgen_union_field: [u64; 6usize],
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct pthread_cond_t__bindgen_ty_1 {
    pub __bindgen_anon_1: pthread_cond_t__bindgen_ty_1__bindgen_ty_1,
    pub __bindgen_anon_2: pthread_cond_t__bindgen_ty_1__bindgen_ty_2,
    pub __g_refs: [::std::os::raw::c_uint; 2usize],
    pub __g_size: [::std::os::raw::c_uint; 2usize],
    pub __g1_orig_size: ::std::os::raw::c_uint,
    pub __wrefs: ::std::os::raw::c_uint,
    pub __g_signals: [::std::os::raw::c_uint; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct pthread_cond_t__bindgen_ty_1__bindgen_ty_1 {
    pub __wseq: __BindgenUnionField<::std::os::raw::c_ulonglong>,
    pub __wseq32: __BindgenUnionField<pthread_cond_t__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1>,
    pub bindgen_union_field: u64,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct pthread_cond_t__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 {
    pub __low: ::std::os::raw::c_uint,
    pub __high: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_pthread_cond_t__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<pthread_cond_t__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1>()
               , 8usize , concat ! (
               "Size of: " , stringify ! (
               pthread_cond_t__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 ) ));
    assert_eq! (::std::mem::align_of::<pthread_cond_t__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1>()
                , 4usize , concat ! (
                "Alignment of " , stringify ! (
                pthread_cond_t__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 ) ));
    assert_eq! (unsafe {
                & (
                * (
                0 as * const
                pthread_cond_t__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 ) ) .
                __low as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_cond_t__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 ) ,
                "::" , stringify ! ( __low ) ));
    assert_eq! (unsafe {
                & (
                * (
                0 as * const
                pthread_cond_t__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 ) ) .
                __high as * const _ as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_cond_t__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 ) ,
                "::" , stringify ! ( __high ) ));
}
impl Clone for pthread_cond_t__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_pthread_cond_t__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<pthread_cond_t__bindgen_ty_1__bindgen_ty_1>()
               , 8usize , concat ! (
               "Size of: " , stringify ! (
               pthread_cond_t__bindgen_ty_1__bindgen_ty_1 ) ));
    assert_eq! (::std::mem::align_of::<pthread_cond_t__bindgen_ty_1__bindgen_ty_1>()
                , 8usize , concat ! (
                "Alignment of " , stringify ! (
                pthread_cond_t__bindgen_ty_1__bindgen_ty_1 ) ));
    assert_eq! (unsafe {
                & (
                * ( 0 as * const pthread_cond_t__bindgen_ty_1__bindgen_ty_1 )
                ) . __wseq as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_cond_t__bindgen_ty_1__bindgen_ty_1 ) , "::" ,
                stringify ! ( __wseq ) ));
    assert_eq! (unsafe {
                & (
                * ( 0 as * const pthread_cond_t__bindgen_ty_1__bindgen_ty_1 )
                ) . __wseq32 as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_cond_t__bindgen_ty_1__bindgen_ty_1 ) , "::" ,
                stringify ! ( __wseq32 ) ));
}
impl Clone for pthread_cond_t__bindgen_ty_1__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct pthread_cond_t__bindgen_ty_1__bindgen_ty_2 {
    pub __g1_start: __BindgenUnionField<::std::os::raw::c_ulonglong>,
    pub __g1_start32: __BindgenUnionField<pthread_cond_t__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1>,
    pub bindgen_union_field: u64,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct pthread_cond_t__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1 {
    pub __low: ::std::os::raw::c_uint,
    pub __high: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_pthread_cond_t__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<pthread_cond_t__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1>()
               , 8usize , concat ! (
               "Size of: " , stringify ! (
               pthread_cond_t__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1 ) ));
    assert_eq! (::std::mem::align_of::<pthread_cond_t__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1>()
                , 4usize , concat ! (
                "Alignment of " , stringify ! (
                pthread_cond_t__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1 ) ));
    assert_eq! (unsafe {
                & (
                * (
                0 as * const
                pthread_cond_t__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1 ) ) .
                __low as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_cond_t__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1 ) ,
                "::" , stringify ! ( __low ) ));
    assert_eq! (unsafe {
                & (
                * (
                0 as * const
                pthread_cond_t__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1 ) ) .
                __high as * const _ as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_cond_t__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1 ) ,
                "::" , stringify ! ( __high ) ));
}
impl Clone for pthread_cond_t__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_pthread_cond_t__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(::std::mem::size_of::<pthread_cond_t__bindgen_ty_1__bindgen_ty_2>()
               , 8usize , concat ! (
               "Size of: " , stringify ! (
               pthread_cond_t__bindgen_ty_1__bindgen_ty_2 ) ));
    assert_eq! (::std::mem::align_of::<pthread_cond_t__bindgen_ty_1__bindgen_ty_2>()
                , 8usize , concat ! (
                "Alignment of " , stringify ! (
                pthread_cond_t__bindgen_ty_1__bindgen_ty_2 ) ));
    assert_eq! (unsafe {
                & (
                * ( 0 as * const pthread_cond_t__bindgen_ty_1__bindgen_ty_2 )
                ) . __g1_start as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_cond_t__bindgen_ty_1__bindgen_ty_2 ) , "::" ,
                stringify ! ( __g1_start ) ));
    assert_eq! (unsafe {
                & (
                * ( 0 as * const pthread_cond_t__bindgen_ty_1__bindgen_ty_2 )
                ) . __g1_start32 as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_cond_t__bindgen_ty_1__bindgen_ty_2 ) , "::" ,
                stringify ! ( __g1_start32 ) ));
}
impl Clone for pthread_cond_t__bindgen_ty_1__bindgen_ty_2 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_pthread_cond_t__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<pthread_cond_t__bindgen_ty_1>() , 48usize
               , concat ! (
               "Size of: " , stringify ! ( pthread_cond_t__bindgen_ty_1 ) ));
    assert_eq! (::std::mem::align_of::<pthread_cond_t__bindgen_ty_1>() ,
                8usize , concat ! (
                "Alignment of " , stringify ! ( pthread_cond_t__bindgen_ty_1 )
                ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_cond_t__bindgen_ty_1 ) ) .
                __g_refs as * const _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_cond_t__bindgen_ty_1 ) , "::" , stringify ! ( __g_refs
                ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_cond_t__bindgen_ty_1 ) ) .
                __g_size as * const _ as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_cond_t__bindgen_ty_1 ) , "::" , stringify ! ( __g_size
                ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_cond_t__bindgen_ty_1 ) ) .
                __g1_orig_size as * const _ as usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_cond_t__bindgen_ty_1 ) , "::" , stringify ! (
                __g1_orig_size ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_cond_t__bindgen_ty_1 ) ) .
                __wrefs as * const _ as usize } , 36usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_cond_t__bindgen_ty_1 ) , "::" , stringify ! ( __wrefs
                ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_cond_t__bindgen_ty_1 ) ) .
                __g_signals as * const _ as usize } , 40usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_cond_t__bindgen_ty_1 ) , "::" , stringify ! (
                __g_signals ) ));
}
impl Clone for pthread_cond_t__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_pthread_cond_t() {
    assert_eq!(::std::mem::size_of::<pthread_cond_t>() , 48usize , concat ! (
               "Size of: " , stringify ! ( pthread_cond_t ) ));
    assert_eq! (::std::mem::align_of::<pthread_cond_t>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( pthread_cond_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_cond_t ) ) . __data as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( pthread_cond_t ) , "::"
                , stringify ! ( __data ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_cond_t ) ) . __size as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( pthread_cond_t ) , "::"
                , stringify ! ( __size ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_cond_t ) ) . __align as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( pthread_cond_t ) , "::"
                , stringify ! ( __align ) ));
}
impl Clone for pthread_cond_t {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct pthread_condattr_t {
    pub __size: __BindgenUnionField<[::std::os::raw::c_char; 4usize]>,
    pub __align: __BindgenUnionField<::std::os::raw::c_int>,
    pub bindgen_union_field: u32,
}
#[test]
fn bindgen_test_layout_pthread_condattr_t() {
    assert_eq!(::std::mem::size_of::<pthread_condattr_t>() , 4usize , concat !
               ( "Size of: " , stringify ! ( pthread_condattr_t ) ));
    assert_eq! (::std::mem::align_of::<pthread_condattr_t>() , 4usize , concat
                ! ( "Alignment of " , stringify ! ( pthread_condattr_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_condattr_t ) ) . __size as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( pthread_condattr_t ) ,
                "::" , stringify ! ( __size ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_condattr_t ) ) . __align as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( pthread_condattr_t ) ,
                "::" , stringify ! ( __align ) ));
}
impl Clone for pthread_condattr_t {
    fn clone(&self) -> Self { *self }
}
pub type pthread_key_t = ::std::os::raw::c_uint;
pub type pthread_once_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct pthread_rwlock_t {
    pub __data: __BindgenUnionField<pthread_rwlock_t__bindgen_ty_1>,
    pub __size: __BindgenUnionField<[::std::os::raw::c_char; 56usize]>,
    pub __align: __BindgenUnionField<::std::os::raw::c_long>,
    pub bindgen_union_field: [u64; 7usize],
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct pthread_rwlock_t__bindgen_ty_1 {
    pub __readers: ::std::os::raw::c_uint,
    pub __writers: ::std::os::raw::c_uint,
    pub __wrphase_futex: ::std::os::raw::c_uint,
    pub __writers_futex: ::std::os::raw::c_uint,
    pub __pad3: ::std::os::raw::c_uint,
    pub __pad4: ::std::os::raw::c_uint,
    pub __cur_writer: ::std::os::raw::c_int,
    pub __shared: ::std::os::raw::c_int,
    pub __rwelision: ::std::os::raw::c_schar,
    pub __pad1: [::std::os::raw::c_uchar; 7usize],
    pub __pad2: ::std::os::raw::c_ulong,
    pub __flags: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_pthread_rwlock_t__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<pthread_rwlock_t__bindgen_ty_1>() ,
               56usize , concat ! (
               "Size of: " , stringify ! ( pthread_rwlock_t__bindgen_ty_1 )
               ));
    assert_eq! (::std::mem::align_of::<pthread_rwlock_t__bindgen_ty_1>() ,
                8usize , concat ! (
                "Alignment of " , stringify ! ( pthread_rwlock_t__bindgen_ty_1
                ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_rwlock_t__bindgen_ty_1 ) ) .
                __readers as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_rwlock_t__bindgen_ty_1 ) , "::" , stringify ! (
                __readers ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_rwlock_t__bindgen_ty_1 ) ) .
                __writers as * const _ as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_rwlock_t__bindgen_ty_1 ) , "::" , stringify ! (
                __writers ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_rwlock_t__bindgen_ty_1 ) ) .
                __wrphase_futex as * const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_rwlock_t__bindgen_ty_1 ) , "::" , stringify ! (
                __wrphase_futex ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_rwlock_t__bindgen_ty_1 ) ) .
                __writers_futex as * const _ as usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_rwlock_t__bindgen_ty_1 ) , "::" , stringify ! (
                __writers_futex ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_rwlock_t__bindgen_ty_1 ) ) .
                __pad3 as * const _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_rwlock_t__bindgen_ty_1 ) , "::" , stringify ! ( __pad3
                ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_rwlock_t__bindgen_ty_1 ) ) .
                __pad4 as * const _ as usize } , 20usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_rwlock_t__bindgen_ty_1 ) , "::" , stringify ! ( __pad4
                ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_rwlock_t__bindgen_ty_1 ) ) .
                __cur_writer as * const _ as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_rwlock_t__bindgen_ty_1 ) , "::" , stringify ! (
                __cur_writer ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_rwlock_t__bindgen_ty_1 ) ) .
                __shared as * const _ as usize } , 28usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_rwlock_t__bindgen_ty_1 ) , "::" , stringify ! (
                __shared ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_rwlock_t__bindgen_ty_1 ) ) .
                __rwelision as * const _ as usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_rwlock_t__bindgen_ty_1 ) , "::" , stringify ! (
                __rwelision ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_rwlock_t__bindgen_ty_1 ) ) .
                __pad1 as * const _ as usize } , 33usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_rwlock_t__bindgen_ty_1 ) , "::" , stringify ! ( __pad1
                ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_rwlock_t__bindgen_ty_1 ) ) .
                __pad2 as * const _ as usize } , 40usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_rwlock_t__bindgen_ty_1 ) , "::" , stringify ! ( __pad2
                ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_rwlock_t__bindgen_ty_1 ) ) .
                __flags as * const _ as usize } , 48usize , concat ! (
                "Alignment of field: " , stringify ! (
                pthread_rwlock_t__bindgen_ty_1 ) , "::" , stringify ! (
                __flags ) ));
}
impl Clone for pthread_rwlock_t__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_pthread_rwlock_t() {
    assert_eq!(::std::mem::size_of::<pthread_rwlock_t>() , 56usize , concat !
               ( "Size of: " , stringify ! ( pthread_rwlock_t ) ));
    assert_eq! (::std::mem::align_of::<pthread_rwlock_t>() , 8usize , concat !
                ( "Alignment of " , stringify ! ( pthread_rwlock_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_rwlock_t ) ) . __data as * const
                _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( pthread_rwlock_t ) ,
                "::" , stringify ! ( __data ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_rwlock_t ) ) . __size as * const
                _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( pthread_rwlock_t ) ,
                "::" , stringify ! ( __size ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_rwlock_t ) ) . __align as * const
                _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( pthread_rwlock_t ) ,
                "::" , stringify ! ( __align ) ));
}
impl Clone for pthread_rwlock_t {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct pthread_rwlockattr_t {
    pub __size: __BindgenUnionField<[::std::os::raw::c_char; 8usize]>,
    pub __align: __BindgenUnionField<::std::os::raw::c_long>,
    pub bindgen_union_field: u64,
}
#[test]
fn bindgen_test_layout_pthread_rwlockattr_t() {
    assert_eq!(::std::mem::size_of::<pthread_rwlockattr_t>() , 8usize , concat
               ! ( "Size of: " , stringify ! ( pthread_rwlockattr_t ) ));
    assert_eq! (::std::mem::align_of::<pthread_rwlockattr_t>() , 8usize ,
                concat ! (
                "Alignment of " , stringify ! ( pthread_rwlockattr_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_rwlockattr_t ) ) . __size as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( pthread_rwlockattr_t )
                , "::" , stringify ! ( __size ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_rwlockattr_t ) ) . __align as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( pthread_rwlockattr_t )
                , "::" , stringify ! ( __align ) ));
}
impl Clone for pthread_rwlockattr_t {
    fn clone(&self) -> Self { *self }
}
pub type pthread_spinlock_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct pthread_barrier_t {
    pub __size: __BindgenUnionField<[::std::os::raw::c_char; 32usize]>,
    pub __align: __BindgenUnionField<::std::os::raw::c_long>,
    pub bindgen_union_field: [u64; 4usize],
}
#[test]
fn bindgen_test_layout_pthread_barrier_t() {
    assert_eq!(::std::mem::size_of::<pthread_barrier_t>() , 32usize , concat !
               ( "Size of: " , stringify ! ( pthread_barrier_t ) ));
    assert_eq! (::std::mem::align_of::<pthread_barrier_t>() , 8usize , concat
                ! ( "Alignment of " , stringify ! ( pthread_barrier_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_barrier_t ) ) . __size as * const
                _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( pthread_barrier_t ) ,
                "::" , stringify ! ( __size ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_barrier_t ) ) . __align as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( pthread_barrier_t ) ,
                "::" , stringify ! ( __align ) ));
}
impl Clone for pthread_barrier_t {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct pthread_barrierattr_t {
    pub __size: __BindgenUnionField<[::std::os::raw::c_char; 4usize]>,
    pub __align: __BindgenUnionField<::std::os::raw::c_int>,
    pub bindgen_union_field: u32,
}
#[test]
fn bindgen_test_layout_pthread_barrierattr_t() {
    assert_eq!(::std::mem::size_of::<pthread_barrierattr_t>() , 4usize ,
               concat ! ( "Size of: " , stringify ! ( pthread_barrierattr_t )
               ));
    assert_eq! (::std::mem::align_of::<pthread_barrierattr_t>() , 4usize ,
                concat ! (
                "Alignment of " , stringify ! ( pthread_barrierattr_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_barrierattr_t ) ) . __size as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( pthread_barrierattr_t )
                , "::" , stringify ! ( __size ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const pthread_barrierattr_t ) ) . __align as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( pthread_barrierattr_t )
                , "::" , stringify ! ( __align ) ));
}
impl Clone for pthread_barrierattr_t {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    pub fn random() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn srandom(__seed: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn initstate(__seed: ::std::os::raw::c_uint,
                     __statebuf: *mut ::std::os::raw::c_char,
                     __statelen: usize) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn setstate(__statebuf: *mut ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct random_data {
    pub fptr: *mut i32,
    pub rptr: *mut i32,
    pub state: *mut i32,
    pub rand_type: ::std::os::raw::c_int,
    pub rand_deg: ::std::os::raw::c_int,
    pub rand_sep: ::std::os::raw::c_int,
    pub end_ptr: *mut i32,
}
#[test]
fn bindgen_test_layout_random_data() {
    assert_eq!(::std::mem::size_of::<random_data>() , 48usize , concat ! (
               "Size of: " , stringify ! ( random_data ) ));
    assert_eq! (::std::mem::align_of::<random_data>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( random_data ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const random_data ) ) . fptr as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( random_data ) , "::" ,
                stringify ! ( fptr ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const random_data ) ) . rptr as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( random_data ) , "::" ,
                stringify ! ( rptr ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const random_data ) ) . state as * const _ as
                usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( random_data ) , "::" ,
                stringify ! ( state ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const random_data ) ) . rand_type as * const _
                as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( random_data ) , "::" ,
                stringify ! ( rand_type ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const random_data ) ) . rand_deg as * const _
                as usize } , 28usize , concat ! (
                "Alignment of field: " , stringify ! ( random_data ) , "::" ,
                stringify ! ( rand_deg ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const random_data ) ) . rand_sep as * const _
                as usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( random_data ) , "::" ,
                stringify ! ( rand_sep ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const random_data ) ) . end_ptr as * const _ as
                usize } , 40usize , concat ! (
                "Alignment of field: " , stringify ! ( random_data ) , "::" ,
                stringify ! ( end_ptr ) ));
}
impl Clone for random_data {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    pub fn random_r(__buf: *mut random_data, __result: *mut i32)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srandom_r(__seed: ::std::os::raw::c_uint, __buf: *mut random_data)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn initstate_r(__seed: ::std::os::raw::c_uint,
                       __statebuf: *mut ::std::os::raw::c_char,
                       __statelen: usize, __buf: *mut random_data)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setstate_r(__statebuf: *mut ::std::os::raw::c_char,
                      __buf: *mut random_data) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rand() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srand(__seed: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn rand_r(__seed: *mut ::std::os::raw::c_uint)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn drand48() -> f64;
}
extern "C" {
    pub fn erand48(__xsubi: *mut ::std::os::raw::c_ushort) -> f64;
}
extern "C" {
    pub fn lrand48() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn nrand48(__xsubi: *mut ::std::os::raw::c_ushort)
     -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn mrand48() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn jrand48(__xsubi: *mut ::std::os::raw::c_ushort)
     -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn srand48(__seedval: ::std::os::raw::c_long);
}
extern "C" {
    pub fn seed48(__seed16v: *mut ::std::os::raw::c_ushort)
     -> *mut ::std::os::raw::c_ushort;
}
extern "C" {
    pub fn lcong48(__param: *mut ::std::os::raw::c_ushort);
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct drand48_data {
    pub __x: [::std::os::raw::c_ushort; 3usize],
    pub __old_x: [::std::os::raw::c_ushort; 3usize],
    pub __c: ::std::os::raw::c_ushort,
    pub __init: ::std::os::raw::c_ushort,
    pub __a: ::std::os::raw::c_ulonglong,
}
#[test]
fn bindgen_test_layout_drand48_data() {
    assert_eq!(::std::mem::size_of::<drand48_data>() , 24usize , concat ! (
               "Size of: " , stringify ! ( drand48_data ) ));
    assert_eq! (::std::mem::align_of::<drand48_data>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( drand48_data ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const drand48_data ) ) . __x as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( drand48_data ) , "::" ,
                stringify ! ( __x ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const drand48_data ) ) . __old_x as * const _
                as usize } , 6usize , concat ! (
                "Alignment of field: " , stringify ! ( drand48_data ) , "::" ,
                stringify ! ( __old_x ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const drand48_data ) ) . __c as * const _ as
                usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( drand48_data ) , "::" ,
                stringify ! ( __c ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const drand48_data ) ) . __init as * const _ as
                usize } , 14usize , concat ! (
                "Alignment of field: " , stringify ! ( drand48_data ) , "::" ,
                stringify ! ( __init ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const drand48_data ) ) . __a as * const _ as
                usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( drand48_data ) , "::" ,
                stringify ! ( __a ) ));
}
impl Clone for drand48_data {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    pub fn drand48_r(__buffer: *mut drand48_data, __result: *mut f64)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn erand48_r(__xsubi: *mut ::std::os::raw::c_ushort,
                     __buffer: *mut drand48_data, __result: *mut f64)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lrand48_r(__buffer: *mut drand48_data,
                     __result: *mut ::std::os::raw::c_long)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nrand48_r(__xsubi: *mut ::std::os::raw::c_ushort,
                     __buffer: *mut drand48_data,
                     __result: *mut ::std::os::raw::c_long)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mrand48_r(__buffer: *mut drand48_data,
                     __result: *mut ::std::os::raw::c_long)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jrand48_r(__xsubi: *mut ::std::os::raw::c_ushort,
                     __buffer: *mut drand48_data,
                     __result: *mut ::std::os::raw::c_long)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srand48_r(__seedval: ::std::os::raw::c_long,
                     __buffer: *mut drand48_data) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn seed48_r(__seed16v: *mut ::std::os::raw::c_ushort,
                    __buffer: *mut drand48_data) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lcong48_r(__param: *mut ::std::os::raw::c_ushort,
                     __buffer: *mut drand48_data) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn malloc(__size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn calloc(__nmemb: usize, __size: usize)
     -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn realloc(__ptr: *mut ::std::os::raw::c_void, __size: usize)
     -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn free(__ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn cfree(__ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn alloca(__size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn valloc(__size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn posix_memalign(__memptr: *mut *mut ::std::os::raw::c_void,
                          __alignment: usize, __size: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn aligned_alloc(__alignment: usize, __size: usize)
     -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn abort();
}
extern "C" {
    pub fn atexit(__func: ::std::option::Option<unsafe extern "C" fn()>)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn at_quick_exit(__func:
                             ::std::option::Option<unsafe extern "C" fn()>)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn on_exit(__func:
                       ::std::option::Option<unsafe extern "C" fn(__status:
                                                                      ::std::os::raw::c_int,
                                                                  __arg:
                                                                      *mut ::std::os::raw::c_void)>,
                   __arg: *mut ::std::os::raw::c_void)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn exit(__status: ::std::os::raw::c_int);
}
extern "C" {
    pub fn quick_exit(__status: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _Exit(__status: ::std::os::raw::c_int);
}
extern "C" {
    pub fn getenv(__name: *const ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn putenv(__string: *mut ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setenv(__name: *const ::std::os::raw::c_char,
                  __value: *const ::std::os::raw::c_char,
                  __replace: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn unsetenv(__name: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearenv() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mktemp(__template: *mut ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mkstemp(__template: *mut ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkstemps(__template: *mut ::std::os::raw::c_char,
                    __suffixlen: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkdtemp(__template: *mut ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn system(__command: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn realpath(__name: *const ::std::os::raw::c_char,
                    __resolved: *mut ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
}
pub type __compar_fn_t =
    ::std::option::Option<unsafe extern "C" fn(arg1:
                                                   *const ::std::os::raw::c_void,
                                               arg2:
                                                   *const ::std::os::raw::c_void)
                              -> ::std::os::raw::c_int>;
extern "C" {
    pub fn bsearch(__key: *const ::std::os::raw::c_void,
                   __base: *const ::std::os::raw::c_void, __nmemb: usize,
                   __size: usize, __compar: __compar_fn_t)
     -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn qsort(__base: *mut ::std::os::raw::c_void, __nmemb: usize,
                 __size: usize, __compar: __compar_fn_t);
}
extern "C" {
    pub fn abs(__x: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn labs(__x: ::std::os::raw::c_long) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llabs(__x: ::std::os::raw::c_longlong)
     -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn div(__numer: ::std::os::raw::c_int, __denom: ::std::os::raw::c_int)
     -> div_t;
}
extern "C" {
    pub fn ldiv(__numer: ::std::os::raw::c_long,
                __denom: ::std::os::raw::c_long) -> ldiv_t;
}
extern "C" {
    pub fn lldiv(__numer: ::std::os::raw::c_longlong,
                 __denom: ::std::os::raw::c_longlong) -> lldiv_t;
}
extern "C" {
    pub fn ecvt(__value: f64, __ndigit: ::std::os::raw::c_int,
                __decpt: *mut ::std::os::raw::c_int,
                __sign: *mut ::std::os::raw::c_int)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fcvt(__value: f64, __ndigit: ::std::os::raw::c_int,
                __decpt: *mut ::std::os::raw::c_int,
                __sign: *mut ::std::os::raw::c_int)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn gcvt(__value: f64, __ndigit: ::std::os::raw::c_int,
                __buf: *mut ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn qecvt(__value: f64, __ndigit: ::std::os::raw::c_int,
                 __decpt: *mut ::std::os::raw::c_int,
                 __sign: *mut ::std::os::raw::c_int)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn qfcvt(__value: f64, __ndigit: ::std::os::raw::c_int,
                 __decpt: *mut ::std::os::raw::c_int,
                 __sign: *mut ::std::os::raw::c_int)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn qgcvt(__value: f64, __ndigit: ::std::os::raw::c_int,
                 __buf: *mut ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ecvt_r(__value: f64, __ndigit: ::std::os::raw::c_int,
                  __decpt: *mut ::std::os::raw::c_int,
                  __sign: *mut ::std::os::raw::c_int,
                  __buf: *mut ::std::os::raw::c_char, __len: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fcvt_r(__value: f64, __ndigit: ::std::os::raw::c_int,
                  __decpt: *mut ::std::os::raw::c_int,
                  __sign: *mut ::std::os::raw::c_int,
                  __buf: *mut ::std::os::raw::c_char, __len: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn qecvt_r(__value: f64, __ndigit: ::std::os::raw::c_int,
                   __decpt: *mut ::std::os::raw::c_int,
                   __sign: *mut ::std::os::raw::c_int,
                   __buf: *mut ::std::os::raw::c_char, __len: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn qfcvt_r(__value: f64, __ndigit: ::std::os::raw::c_int,
                   __decpt: *mut ::std::os::raw::c_int,
                   __sign: *mut ::std::os::raw::c_int,
                   __buf: *mut ::std::os::raw::c_char, __len: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mblen(__s: *const ::std::os::raw::c_char, __n: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbtowc(__pwc: *mut wchar_t, __s: *const ::std::os::raw::c_char,
                  __n: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wctomb(__s: *mut ::std::os::raw::c_char, __wchar: wchar_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbstowcs(__pwcs: *mut wchar_t, __s: *const ::std::os::raw::c_char,
                    __n: usize) -> usize;
}
extern "C" {
    pub fn wcstombs(__s: *mut ::std::os::raw::c_char, __pwcs: *const wchar_t,
                    __n: usize) -> usize;
}
extern "C" {
    pub fn rpmatch(__response: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getsubopt(__optionp: *mut *mut ::std::os::raw::c_char,
                     __tokens: *const *const ::std::os::raw::c_char,
                     __valuep: *mut *mut ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getloadavg(__loadavg: *mut f64, __nelem: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: ::std::os::raw::c_ushort,
    pub d_type: ::std::os::raw::c_uchar,
    pub d_name: [::std::os::raw::c_char; 256usize],
}
#[test]
fn bindgen_test_layout_dirent() {
    assert_eq!(::std::mem::size_of::<dirent>() , 280usize , concat ! (
               "Size of: " , stringify ! ( dirent ) ));
    assert_eq! (::std::mem::align_of::<dirent>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( dirent ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const dirent ) ) . d_ino as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( dirent ) , "::" ,
                stringify ! ( d_ino ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const dirent ) ) . d_off as * const _ as usize
                } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( dirent ) , "::" ,
                stringify ! ( d_off ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const dirent ) ) . d_reclen as * const _ as
                usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( dirent ) , "::" ,
                stringify ! ( d_reclen ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const dirent ) ) . d_type as * const _ as usize
                } , 18usize , concat ! (
                "Alignment of field: " , stringify ! ( dirent ) , "::" ,
                stringify ! ( d_type ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const dirent ) ) . d_name as * const _ as usize
                } , 19usize , concat ! (
                "Alignment of field: " , stringify ! ( dirent ) , "::" ,
                stringify ! ( d_name ) ));
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
extern "C" {
    pub fn opendir(__name: *const ::std::os::raw::c_char) -> *mut DIR;
}
extern "C" {
    pub fn fdopendir(__fd: ::std::os::raw::c_int) -> *mut DIR;
}
extern "C" {
    pub fn closedir(__dirp: *mut DIR) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn readdir(__dirp: *mut DIR) -> *mut dirent;
}
extern "C" {
    pub fn readdir_r(__dirp: *mut DIR, __entry: *mut dirent,
                     __result: *mut *mut dirent) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rewinddir(__dirp: *mut DIR);
}
extern "C" {
    pub fn seekdir(__dirp: *mut DIR, __pos: ::std::os::raw::c_long);
}
extern "C" {
    pub fn telldir(__dirp: *mut DIR) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn dirfd(__dirp: *mut DIR) -> ::std::os::raw::c_int;
}
extern "C" {
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
}
extern "C" {
    pub fn alphasort(__e1: *mut *const dirent, __e2: *mut *const dirent)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getdirentries(__fd: ::std::os::raw::c_int,
                         __buf: *mut ::std::os::raw::c_char, __nbytes: usize,
                         __basep: *mut __off_t) -> __ssize_t;
}
pub type nanotime_t = u64;
extern "C" {
    pub fn nanotime() -> nanotime_t;
}
extern "C" {
    pub fn microtime(tstart: nanotime_t, tend: nanotime_t) -> f64;
}
extern "C" {
    pub fn millitime(tstart: nanotime_t, tend: nanotime_t) -> f64;
}
extern "C" {
    pub fn readline(prompt: *mut ::std::os::raw::c_char,
                    length: *mut ::std::os::raw::c_int)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn file_size(path: *const ::std::os::raw::c_char) -> u64;
}
extern "C" {
    pub fn file_read(path: *const ::std::os::raw::c_char, len: *mut usize)
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn file_exists(path: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn file_buildpath(filename: *const ::std::os::raw::c_char,
                          dirpath: *const ::std::os::raw::c_char)
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn file_write(path: *const ::std::os::raw::c_char,
                      buffer: *const ::std::os::raw::c_char, len: usize)
     -> bool;
}
extern "C" {
    pub fn is_directory(path: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn directory_init(path: *const ::std::os::raw::c_char) -> *mut DIR;
}
extern "C" {
    pub fn directory_read(ref_: *mut DIR) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn string_nocasencmp(s1: *const ::std::os::raw::c_char,
                             s2: *const ::std::os::raw::c_char, n: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn string_casencmp(s1: *const ::std::os::raw::c_char,
                           s2: *const ::std::os::raw::c_char, n: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn string_cmp(s1: *const ::std::os::raw::c_char,
                      s2: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn string_dup(s1: *const ::std::os::raw::c_char)
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn string_ndup(s1: *const ::std::os::raw::c_char, n: usize)
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn string_reverse(p: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn string_size(p: *const ::std::os::raw::c_char) -> u32;
}
extern "C" {
    pub fn utf8_charbytes(s: *const ::std::os::raw::c_char, i: u32) -> u32;
}
extern "C" {
    pub fn utf8_nbytes(n: u32) -> u32;
}
extern "C" {
    pub fn utf8_encode(buffer: *mut ::std::os::raw::c_char, value: u32)
     -> u32;
}
extern "C" {
    pub fn utf8_len(s: *const ::std::os::raw::c_char, nbytes: u32) -> u32;
}
extern "C" {
    pub fn utf8_reverse(p: *mut ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn power_of2_ceil(n: u32) -> u32;
}
extern "C" {
    pub fn number_from_hex(s: *const ::std::os::raw::c_char, len: u32) -> i64;
}
extern "C" {
    pub fn number_from_oct(s: *const ::std::os::raw::c_char, len: u32) -> i64;
}
extern "C" {
    pub fn number_from_bin(s: *const ::std::os::raw::c_char, len: u32) -> i64;
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct uint16_r {
    pub n: usize,
    pub m: usize,
    pub p: *mut u16,
}
#[test]
fn bindgen_test_layout_uint16_r() {
    assert_eq!(::std::mem::size_of::<uint16_r>() , 24usize , concat ! (
               "Size of: " , stringify ! ( uint16_r ) ));
    assert_eq! (::std::mem::align_of::<uint16_r>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( uint16_r ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const uint16_r ) ) . n as * const _ as usize }
                , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( uint16_r ) , "::" ,
                stringify ! ( n ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const uint16_r ) ) . m as * const _ as usize }
                , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( uint16_r ) , "::" ,
                stringify ! ( m ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const uint16_r ) ) . p as * const _ as usize }
                , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( uint16_r ) , "::" ,
                stringify ! ( p ) ));
}
impl Clone for uint16_r {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct uint32_r {
    pub n: usize,
    pub m: usize,
    pub p: *mut u32,
}
#[test]
fn bindgen_test_layout_uint32_r() {
    assert_eq!(::std::mem::size_of::<uint32_r>() , 24usize , concat ! (
               "Size of: " , stringify ! ( uint32_r ) ));
    assert_eq! (::std::mem::align_of::<uint32_r>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( uint32_r ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const uint32_r ) ) . n as * const _ as usize }
                , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( uint32_r ) , "::" ,
                stringify ! ( n ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const uint32_r ) ) . m as * const _ as usize }
                , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( uint32_r ) , "::" ,
                stringify ! ( m ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const uint32_r ) ) . p as * const _ as usize }
                , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( uint32_r ) , "::" ,
                stringify ! ( p ) ));
}
impl Clone for uint32_r {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct void_r {
    pub n: usize,
    pub m: usize,
    pub p: *mut *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_void_r() {
    assert_eq!(::std::mem::size_of::<void_r>() , 24usize , concat ! (
               "Size of: " , stringify ! ( void_r ) ));
    assert_eq! (::std::mem::align_of::<void_r>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( void_r ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const void_r ) ) . n as * const _ as usize } ,
                0usize , concat ! (
                "Alignment of field: " , stringify ! ( void_r ) , "::" ,
                stringify ! ( n ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const void_r ) ) . m as * const _ as usize } ,
                8usize , concat ! (
                "Alignment of field: " , stringify ! ( void_r ) , "::" ,
                stringify ! ( m ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const void_r ) ) . p as * const _ as usize } ,
                16usize , concat ! (
                "Alignment of field: " , stringify ! ( void_r ) , "::" ,
                stringify ! ( p ) ));
}
impl Clone for void_r {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct cstring_r {
    pub n: usize,
    pub m: usize,
    pub p: *mut *const ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_cstring_r() {
    assert_eq!(::std::mem::size_of::<cstring_r>() , 24usize , concat ! (
               "Size of: " , stringify ! ( cstring_r ) ));
    assert_eq! (::std::mem::align_of::<cstring_r>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( cstring_r ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const cstring_r ) ) . n as * const _ as usize }
                , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( cstring_r ) , "::" ,
                stringify ! ( n ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const cstring_r ) ) . m as * const _ as usize }
                , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( cstring_r ) , "::" ,
                stringify ! ( m ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const cstring_r ) ) . p as * const _ as usize }
                , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( cstring_r ) , "::" ,
                stringify ! ( p ) ));
}
impl Clone for cstring_r {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: f64,
}
#[test]
fn bindgen_test_layout_max_align_t() {
    assert_eq!(::std::mem::size_of::<max_align_t>() , 24usize , concat ! (
               "Size of: " , stringify ! ( max_align_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const max_align_t ) ) .
                __clang_max_align_nonce1 as * const _ as usize } , 0usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( max_align_t ) , "::" ,
                stringify ! ( __clang_max_align_nonce1 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const max_align_t ) ) .
                __clang_max_align_nonce2 as * const _ as usize } , 16usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( max_align_t ) , "::" ,
                stringify ! ( __clang_max_align_nonce2 ) ));
}
impl Clone for max_align_t {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct json_t([u8; 0]);
extern "C" {
    pub fn json_new() -> *mut json_t;
}
extern "C" {
    pub fn json_begin_object(json: *mut json_t,
                             key: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn json_end_object(json: *mut json_t);
}
extern "C" {
    pub fn json_begin_array(json: *mut json_t,
                            key: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn json_end_array(json: *mut json_t);
}
extern "C" {
    pub fn json_add_cstring(json: *mut json_t,
                            key: *const ::std::os::raw::c_char,
                            value: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn json_add_string(json: *mut json_t,
                           key: *const ::std::os::raw::c_char,
                           value: *const ::std::os::raw::c_char, len: usize);
}
extern "C" {
    pub fn json_add_int(json: *mut json_t, key: *const ::std::os::raw::c_char,
                        value: i64);
}
extern "C" {
    pub fn json_add_double(json: *mut json_t,
                           key: *const ::std::os::raw::c_char, value: f64);
}
extern "C" {
    pub fn json_add_bool(json: *mut json_t,
                         key: *const ::std::os::raw::c_char, value: bool);
}
extern "C" {
    pub fn json_add_null(json: *mut json_t,
                         key: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn json_free(json: *mut json_t);
}
extern "C" {
    pub fn json_buffer(json: *mut json_t, len: *mut usize)
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn json_write_file(json: *mut json_t,
                           path: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn json_pop(json: *mut json_t, n: u32);
}
pub type __gwchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct imaxdiv_t {
    pub quot: ::std::os::raw::c_long,
    pub rem: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_imaxdiv_t() {
    assert_eq!(::std::mem::size_of::<imaxdiv_t>() , 16usize , concat ! (
               "Size of: " , stringify ! ( imaxdiv_t ) ));
    assert_eq! (::std::mem::align_of::<imaxdiv_t>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( imaxdiv_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const imaxdiv_t ) ) . quot as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( imaxdiv_t ) , "::" ,
                stringify ! ( quot ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const imaxdiv_t ) ) . rem as * const _ as usize
                } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( imaxdiv_t ) , "::" ,
                stringify ! ( rem ) ));
}
impl Clone for imaxdiv_t {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    pub fn imaxabs(__n: intmax_t) -> intmax_t;
}
extern "C" {
    pub fn imaxdiv(__numer: intmax_t, __denom: intmax_t) -> imaxdiv_t;
}
extern "C" {
    pub fn strtoimax(__nptr: *const ::std::os::raw::c_char,
                     __endptr: *mut *mut ::std::os::raw::c_char,
                     __base: ::std::os::raw::c_int) -> intmax_t;
}
extern "C" {
    pub fn strtoumax(__nptr: *const ::std::os::raw::c_char,
                     __endptr: *mut *mut ::std::os::raw::c_char,
                     __base: ::std::os::raw::c_int) -> uintmax_t;
}
extern "C" {
    pub fn wcstoimax(__nptr: *const __gwchar_t,
                     __endptr: *mut *mut __gwchar_t,
                     __base: ::std::os::raw::c_int) -> intmax_t;
}
extern "C" {
    pub fn wcstoumax(__nptr: *const __gwchar_t,
                     __endptr: *mut *mut __gwchar_t,
                     __base: ::std::os::raw::c_int) -> uintmax_t;
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct json_settings {
    pub max_memory: ::std::os::raw::c_ulong,
    pub settings: ::std::os::raw::c_int,
    pub memory_alloc: ::std::option::Option<unsafe extern "C" fn(arg1: usize,
                                                                 zero:
                                                                     ::std::os::raw::c_int,
                                                                 user_data:
                                                                     *mut ::std::os::raw::c_void)
                                                ->
                                                    *mut ::std::os::raw::c_void>,
    pub memory_free: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                    *mut ::std::os::raw::c_void,
                                                                user_data:
                                                                    *mut ::std::os::raw::c_void)>,
    pub user_data: *mut ::std::os::raw::c_void,
    pub value_extra: usize,
}
#[test]
fn bindgen_test_layout_json_settings() {
    assert_eq!(::std::mem::size_of::<json_settings>() , 48usize , concat ! (
               "Size of: " , stringify ! ( json_settings ) ));
    assert_eq! (::std::mem::align_of::<json_settings>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( json_settings ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const json_settings ) ) . max_memory as * const
                _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( json_settings ) , "::"
                , stringify ! ( max_memory ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const json_settings ) ) . settings as * const _
                as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( json_settings ) , "::"
                , stringify ! ( settings ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const json_settings ) ) . memory_alloc as *
                const _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( json_settings ) , "::"
                , stringify ! ( memory_alloc ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const json_settings ) ) . memory_free as *
                const _ as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( json_settings ) , "::"
                , stringify ! ( memory_free ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const json_settings ) ) . user_data as * const
                _ as usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( json_settings ) , "::"
                , stringify ! ( user_data ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const json_settings ) ) . value_extra as *
                const _ as usize } , 40usize , concat ! (
                "Alignment of field: " , stringify ! ( json_settings ) , "::"
                , stringify ! ( value_extra ) ));
}
impl Clone for json_settings {
    fn clone(&self) -> Self { *self }
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
#[test]
fn bindgen_test_layout__json_value__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<_json_value__bindgen_ty_1__bindgen_ty_1>()
               , 16usize , concat ! (
               "Size of: " , stringify ! (
               _json_value__bindgen_ty_1__bindgen_ty_1 ) ));
    assert_eq! (::std::mem::align_of::<_json_value__bindgen_ty_1__bindgen_ty_1>()
                , 8usize , concat ! (
                "Alignment of " , stringify ! (
                _json_value__bindgen_ty_1__bindgen_ty_1 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _json_value__bindgen_ty_1__bindgen_ty_1 )
                ) . length as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                _json_value__bindgen_ty_1__bindgen_ty_1 ) , "::" , stringify !
                ( length ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _json_value__bindgen_ty_1__bindgen_ty_1 )
                ) . ptr as * const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! (
                _json_value__bindgen_ty_1__bindgen_ty_1 ) , "::" , stringify !
                ( ptr ) ));
}
impl Clone for _json_value__bindgen_ty_1__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct _json_value__bindgen_ty_1__bindgen_ty_2 {
    pub length: ::std::os::raw::c_uint,
    pub values: *mut json_object_entry,
}
#[test]
fn bindgen_test_layout__json_value__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(::std::mem::size_of::<_json_value__bindgen_ty_1__bindgen_ty_2>()
               , 16usize , concat ! (
               "Size of: " , stringify ! (
               _json_value__bindgen_ty_1__bindgen_ty_2 ) ));
    assert_eq! (::std::mem::align_of::<_json_value__bindgen_ty_1__bindgen_ty_2>()
                , 8usize , concat ! (
                "Alignment of " , stringify ! (
                _json_value__bindgen_ty_1__bindgen_ty_2 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _json_value__bindgen_ty_1__bindgen_ty_2 )
                ) . length as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                _json_value__bindgen_ty_1__bindgen_ty_2 ) , "::" , stringify !
                ( length ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _json_value__bindgen_ty_1__bindgen_ty_2 )
                ) . values as * const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! (
                _json_value__bindgen_ty_1__bindgen_ty_2 ) , "::" , stringify !
                ( values ) ));
}
impl Clone for _json_value__bindgen_ty_1__bindgen_ty_2 {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct _json_value__bindgen_ty_1__bindgen_ty_3 {
    pub length: ::std::os::raw::c_uint,
    pub values: *mut *mut _json_value,
}
#[test]
fn bindgen_test_layout__json_value__bindgen_ty_1__bindgen_ty_3() {
    assert_eq!(::std::mem::size_of::<_json_value__bindgen_ty_1__bindgen_ty_3>()
               , 16usize , concat ! (
               "Size of: " , stringify ! (
               _json_value__bindgen_ty_1__bindgen_ty_3 ) ));
    assert_eq! (::std::mem::align_of::<_json_value__bindgen_ty_1__bindgen_ty_3>()
                , 8usize , concat ! (
                "Alignment of " , stringify ! (
                _json_value__bindgen_ty_1__bindgen_ty_3 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _json_value__bindgen_ty_1__bindgen_ty_3 )
                ) . length as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                _json_value__bindgen_ty_1__bindgen_ty_3 ) , "::" , stringify !
                ( length ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _json_value__bindgen_ty_1__bindgen_ty_3 )
                ) . values as * const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! (
                _json_value__bindgen_ty_1__bindgen_ty_3 ) , "::" , stringify !
                ( values ) ));
}
impl Clone for _json_value__bindgen_ty_1__bindgen_ty_3 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout__json_value__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<_json_value__bindgen_ty_1>() , 16usize ,
               concat ! (
               "Size of: " , stringify ! ( _json_value__bindgen_ty_1 ) ));
    assert_eq! (::std::mem::align_of::<_json_value__bindgen_ty_1>() , 8usize ,
                concat ! (
                "Alignment of " , stringify ! ( _json_value__bindgen_ty_1 )
                ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _json_value__bindgen_ty_1 ) ) . boolean
                as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                _json_value__bindgen_ty_1 ) , "::" , stringify ! ( boolean )
                ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _json_value__bindgen_ty_1 ) ) . integer
                as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                _json_value__bindgen_ty_1 ) , "::" , stringify ! ( integer )
                ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _json_value__bindgen_ty_1 ) ) . dbl as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                _json_value__bindgen_ty_1 ) , "::" , stringify ! ( dbl ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _json_value__bindgen_ty_1 ) ) . string as
                * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                _json_value__bindgen_ty_1 ) , "::" , stringify ! ( string )
                ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _json_value__bindgen_ty_1 ) ) . object as
                * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                _json_value__bindgen_ty_1 ) , "::" , stringify ! ( object )
                ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _json_value__bindgen_ty_1 ) ) . array as
                * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                _json_value__bindgen_ty_1 ) , "::" , stringify ! ( array ) ));
}
impl Clone for _json_value__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct _json_value__bindgen_ty_2 {
    pub next_alloc: __BindgenUnionField<*mut _json_value>,
    pub object_mem: __BindgenUnionField<*mut ::std::os::raw::c_void>,
    pub bindgen_union_field: u64,
}
#[test]
fn bindgen_test_layout__json_value__bindgen_ty_2() {
    assert_eq!(::std::mem::size_of::<_json_value__bindgen_ty_2>() , 8usize ,
               concat ! (
               "Size of: " , stringify ! ( _json_value__bindgen_ty_2 ) ));
    assert_eq! (::std::mem::align_of::<_json_value__bindgen_ty_2>() , 8usize ,
                concat ! (
                "Alignment of " , stringify ! ( _json_value__bindgen_ty_2 )
                ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _json_value__bindgen_ty_2 ) ) .
                next_alloc as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                _json_value__bindgen_ty_2 ) , "::" , stringify ! ( next_alloc
                ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _json_value__bindgen_ty_2 ) ) .
                object_mem as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                _json_value__bindgen_ty_2 ) , "::" , stringify ! ( object_mem
                ) ));
}
impl Clone for _json_value__bindgen_ty_2 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout__json_value() {
    assert_eq!(::std::mem::size_of::<_json_value>() , 40usize , concat ! (
               "Size of: " , stringify ! ( _json_value ) ));
    assert_eq! (::std::mem::align_of::<_json_value>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( _json_value ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _json_value ) ) . parent as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( _json_value ) , "::" ,
                stringify ! ( parent ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _json_value ) ) . type_ as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( _json_value ) , "::" ,
                stringify ! ( type_ ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _json_value ) ) . u as * const _ as usize
                } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( _json_value ) , "::" ,
                stringify ! ( u ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _json_value ) ) . _reserved as * const _
                as usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( _json_value ) , "::" ,
                stringify ! ( _reserved ) ));
}
impl Clone for _json_value {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    #[link_name = "json_value_none"]
    pub static json_value_none: _json_value;
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct _json_object_entry {
    pub name: *mut ::std::os::raw::c_char,
    pub name_length: ::std::os::raw::c_uint,
    pub value: *mut _json_value,
}
#[test]
fn bindgen_test_layout__json_object_entry() {
    assert_eq!(::std::mem::size_of::<_json_object_entry>() , 24usize , concat
               ! ( "Size of: " , stringify ! ( _json_object_entry ) ));
    assert_eq! (::std::mem::align_of::<_json_object_entry>() , 8usize , concat
                ! ( "Alignment of " , stringify ! ( _json_object_entry ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _json_object_entry ) ) . name as * const
                _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( _json_object_entry ) ,
                "::" , stringify ! ( name ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _json_object_entry ) ) . name_length as *
                const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( _json_object_entry ) ,
                "::" , stringify ! ( name_length ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _json_object_entry ) ) . value as * const
                _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( _json_object_entry ) ,
                "::" , stringify ! ( value ) ));
}
impl Clone for _json_object_entry {
    fn clone(&self) -> Self { *self }
}
pub type json_object_entry = _json_object_entry;
pub type json_value = _json_value;
extern "C" {
    pub fn json_parse(json: *const ::std::os::raw::c_char, length: usize)
     -> *mut json_value;
}
extern "C" {
    pub fn json_parse_ex(settings: *mut json_settings,
                         json: *const ::std::os::raw::c_char, length: usize,
                         error: *mut ::std::os::raw::c_char)
     -> *mut json_value;
}
extern "C" {
    pub fn json_value_free(arg1: *mut json_value);
}
extern "C" {
    pub fn json_value_free_ex(settings: *mut json_settings,
                              arg1: *mut json_value);
}
pub const _ISupper: _bindgen_ty_2 = _bindgen_ty_2::_ISupper;
pub const _ISlower: _bindgen_ty_2 = _bindgen_ty_2::_ISlower;
pub const _ISalpha: _bindgen_ty_2 = _bindgen_ty_2::_ISalpha;
pub const _ISdigit: _bindgen_ty_2 = _bindgen_ty_2::_ISdigit;
pub const _ISxdigit: _bindgen_ty_2 = _bindgen_ty_2::_ISxdigit;
pub const _ISspace: _bindgen_ty_2 = _bindgen_ty_2::_ISspace;
pub const _ISprint: _bindgen_ty_2 = _bindgen_ty_2::_ISprint;
pub const _ISgraph: _bindgen_ty_2 = _bindgen_ty_2::_ISgraph;
pub const _ISblank: _bindgen_ty_2 = _bindgen_ty_2::_ISblank;
pub const _IScntrl: _bindgen_ty_2 = _bindgen_ty_2::_IScntrl;
pub const _ISpunct: _bindgen_ty_2 = _bindgen_ty_2::_ISpunct;
pub const _ISalnum: _bindgen_ty_2 = _bindgen_ty_2::_ISalnum;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _bindgen_ty_2 {
    _ISupper = 256,
    _ISlower = 512,
    _ISalpha = 1024,
    _ISdigit = 2048,
    _ISxdigit = 4096,
    _ISspace = 8192,
    _ISprint = 16384,
    _ISgraph = 32768,
    _ISblank = 1,
    _IScntrl = 2,
    _ISpunct = 4,
    _ISalnum = 8,
}
extern "C" {
    pub fn __ctype_b_loc() -> *mut *const ::std::os::raw::c_ushort;
}
extern "C" {
    pub fn __ctype_tolower_loc() -> *mut *const __int32_t;
}
extern "C" {
    pub fn __ctype_toupper_loc() -> *mut *const __int32_t;
}
extern "C" {
    pub fn isalnum(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isalpha(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iscntrl(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isdigit(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn islower(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isgraph(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isprint(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ispunct(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isspace(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isupper(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isxdigit(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tolower(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn toupper(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isblank(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isascii(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn toascii(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _toupper(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _tolower(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13usize],
    pub __ctype_b: *const ::std::os::raw::c_ushort,
    pub __ctype_tolower: *const ::std::os::raw::c_int,
    pub __ctype_toupper: *const ::std::os::raw::c_int,
    pub __names: [*const ::std::os::raw::c_char; 13usize],
}
#[test]
fn bindgen_test_layout___locale_struct() {
    assert_eq!(::std::mem::size_of::<__locale_struct>() , 232usize , concat !
               ( "Size of: " , stringify ! ( __locale_struct ) ));
    assert_eq! (::std::mem::align_of::<__locale_struct>() , 8usize , concat !
                ( "Alignment of " , stringify ! ( __locale_struct ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const __locale_struct ) ) . __locales as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( __locale_struct ) ,
                "::" , stringify ! ( __locales ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const __locale_struct ) ) . __ctype_b as *
                const _ as usize } , 104usize , concat ! (
                "Alignment of field: " , stringify ! ( __locale_struct ) ,
                "::" , stringify ! ( __ctype_b ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const __locale_struct ) ) . __ctype_tolower as
                * const _ as usize } , 112usize , concat ! (
                "Alignment of field: " , stringify ! ( __locale_struct ) ,
                "::" , stringify ! ( __ctype_tolower ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const __locale_struct ) ) . __ctype_toupper as
                * const _ as usize } , 120usize , concat ! (
                "Alignment of field: " , stringify ! ( __locale_struct ) ,
                "::" , stringify ! ( __ctype_toupper ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const __locale_struct ) ) . __names as * const
                _ as usize } , 128usize , concat ! (
                "Alignment of field: " , stringify ! ( __locale_struct ) ,
                "::" , stringify ! ( __names ) ));
}
impl Clone for __locale_struct {
    fn clone(&self) -> Self { *self }
}
pub type __locale_t = *mut __locale_struct;
pub type locale_t = __locale_t;
extern "C" {
    pub fn isalnum_l(arg1: ::std::os::raw::c_int, arg2: __locale_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isalpha_l(arg1: ::std::os::raw::c_int, arg2: __locale_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iscntrl_l(arg1: ::std::os::raw::c_int, arg2: __locale_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isdigit_l(arg1: ::std::os::raw::c_int, arg2: __locale_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn islower_l(arg1: ::std::os::raw::c_int, arg2: __locale_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isgraph_l(arg1: ::std::os::raw::c_int, arg2: __locale_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isprint_l(arg1: ::std::os::raw::c_int, arg2: __locale_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ispunct_l(arg1: ::std::os::raw::c_int, arg2: __locale_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isspace_l(arg1: ::std::os::raw::c_int, arg2: __locale_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isupper_l(arg1: ::std::os::raw::c_int, arg2: __locale_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isxdigit_l(arg1: ::std::os::raw::c_int, arg2: __locale_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isblank_l(arg1: ::std::os::raw::c_int, arg2: __locale_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __tolower_l(__c: ::std::os::raw::c_int, __l: __locale_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tolower_l(__c: ::std::os::raw::c_int, __l: __locale_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __toupper_l(__c: ::std::os::raw::c_int, __l: __locale_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn toupper_l(__c: ::std::os::raw::c_int, __l: __locale_t)
     -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct flock {
    pub l_type: ::std::os::raw::c_short,
    pub l_whence: ::std::os::raw::c_short,
    pub l_start: __off_t,
    pub l_len: __off_t,
    pub l_pid: __pid_t,
}
#[test]
fn bindgen_test_layout_flock() {
    assert_eq!(::std::mem::size_of::<flock>() , 32usize , concat ! (
               "Size of: " , stringify ! ( flock ) ));
    assert_eq! (::std::mem::align_of::<flock>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( flock ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const flock ) ) . l_type as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( flock ) , "::" ,
                stringify ! ( l_type ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const flock ) ) . l_whence as * const _ as
                usize } , 2usize , concat ! (
                "Alignment of field: " , stringify ! ( flock ) , "::" ,
                stringify ! ( l_whence ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const flock ) ) . l_start as * const _ as usize
                } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( flock ) , "::" ,
                stringify ! ( l_start ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const flock ) ) . l_len as * const _ as usize }
                , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( flock ) , "::" ,
                stringify ! ( l_len ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const flock ) ) . l_pid as * const _ as usize }
                , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( flock ) , "::" ,
                stringify ! ( l_pid ) ));
}
impl Clone for flock {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::std::os::raw::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3usize],
}
#[test]
fn bindgen_test_layout_stat() {
    assert_eq!(::std::mem::size_of::<stat>() , 144usize , concat ! (
               "Size of: " , stringify ! ( stat ) ));
    assert_eq! (::std::mem::align_of::<stat>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( stat ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const stat ) ) . st_dev as * const _ as usize }
                , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( stat ) , "::" ,
                stringify ! ( st_dev ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const stat ) ) . st_ino as * const _ as usize }
                , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( stat ) , "::" ,
                stringify ! ( st_ino ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const stat ) ) . st_nlink as * const _ as usize
                } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( stat ) , "::" ,
                stringify ! ( st_nlink ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const stat ) ) . st_mode as * const _ as usize
                } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( stat ) , "::" ,
                stringify ! ( st_mode ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const stat ) ) . st_uid as * const _ as usize }
                , 28usize , concat ! (
                "Alignment of field: " , stringify ! ( stat ) , "::" ,
                stringify ! ( st_uid ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const stat ) ) . st_gid as * const _ as usize }
                , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( stat ) , "::" ,
                stringify ! ( st_gid ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const stat ) ) . __pad0 as * const _ as usize }
                , 36usize , concat ! (
                "Alignment of field: " , stringify ! ( stat ) , "::" ,
                stringify ! ( __pad0 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const stat ) ) . st_rdev as * const _ as usize
                } , 40usize , concat ! (
                "Alignment of field: " , stringify ! ( stat ) , "::" ,
                stringify ! ( st_rdev ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const stat ) ) . st_size as * const _ as usize
                } , 48usize , concat ! (
                "Alignment of field: " , stringify ! ( stat ) , "::" ,
                stringify ! ( st_size ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const stat ) ) . st_blksize as * const _ as
                usize } , 56usize , concat ! (
                "Alignment of field: " , stringify ! ( stat ) , "::" ,
                stringify ! ( st_blksize ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const stat ) ) . st_blocks as * const _ as
                usize } , 64usize , concat ! (
                "Alignment of field: " , stringify ! ( stat ) , "::" ,
                stringify ! ( st_blocks ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const stat ) ) . st_atim as * const _ as usize
                } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( stat ) , "::" ,
                stringify ! ( st_atim ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const stat ) ) . st_mtim as * const _ as usize
                } , 88usize , concat ! (
                "Alignment of field: " , stringify ! ( stat ) , "::" ,
                stringify ! ( st_mtim ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const stat ) ) . st_ctim as * const _ as usize
                } , 104usize , concat ! (
                "Alignment of field: " , stringify ! ( stat ) , "::" ,
                stringify ! ( st_ctim ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const stat ) ) . __glibc_reserved as * const _
                as usize } , 120usize , concat ! (
                "Alignment of field: " , stringify ! ( stat ) , "::" ,
                stringify ! ( __glibc_reserved ) ));
}
impl Clone for stat {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    pub fn fcntl(__fd: ::std::os::raw::c_int,
                 __cmd: ::std::os::raw::c_int, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn open(__file: *const ::std::os::raw::c_char,
                __oflag: ::std::os::raw::c_int, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn openat(__fd: ::std::os::raw::c_int,
                  __file: *const ::std::os::raw::c_char,
                  __oflag: ::std::os::raw::c_int, ...)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn creat(__file: *const ::std::os::raw::c_char, __mode: mode_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lockf(__fd: ::std::os::raw::c_int, __cmd: ::std::os::raw::c_int,
                 __len: off_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn posix_fadvise(__fd: ::std::os::raw::c_int, __offset: off_t,
                         __len: off_t, __advise: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn posix_fallocate(__fd: ::std::os::raw::c_int, __offset: off_t,
                           __len: off_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn memcpy(__dest: *mut ::std::os::raw::c_void,
                  __src: *const ::std::os::raw::c_void, __n: usize)
     -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memmove(__dest: *mut ::std::os::raw::c_void,
                   __src: *const ::std::os::raw::c_void, __n: usize)
     -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memccpy(__dest: *mut ::std::os::raw::c_void,
                   __src: *const ::std::os::raw::c_void,
                   __c: ::std::os::raw::c_int, __n: usize)
     -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memset(__s: *mut ::std::os::raw::c_void,
                  __c: ::std::os::raw::c_int, __n: usize)
     -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memcmp(__s1: *const ::std::os::raw::c_void,
                  __s2: *const ::std::os::raw::c_void, __n: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn memchr(__s: *const ::std::os::raw::c_void,
                  __c: ::std::os::raw::c_int, __n: usize)
     -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn strcpy(__dest: *mut ::std::os::raw::c_char,
                  __src: *const ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strncpy(__dest: *mut ::std::os::raw::c_char,
                   __src: *const ::std::os::raw::c_char, __n: usize)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strcat(__dest: *mut ::std::os::raw::c_char,
                  __src: *const ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strncat(__dest: *mut ::std::os::raw::c_char,
                   __src: *const ::std::os::raw::c_char, __n: usize)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strcmp(__s1: *const ::std::os::raw::c_char,
                  __s2: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strncmp(__s1: *const ::std::os::raw::c_char,
                   __s2: *const ::std::os::raw::c_char, __n: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strcoll(__s1: *const ::std::os::raw::c_char,
                   __s2: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strxfrm(__dest: *mut ::std::os::raw::c_char,
                   __src: *const ::std::os::raw::c_char, __n: usize)
     -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strcoll_l(__s1: *const ::std::os::raw::c_char,
                     __s2: *const ::std::os::raw::c_char, __l: __locale_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strxfrm_l(__dest: *mut ::std::os::raw::c_char,
                     __src: *const ::std::os::raw::c_char, __n: usize,
                     __l: __locale_t) -> usize;
}
extern "C" {
    pub fn strdup(__s: *const ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strndup(__string: *const ::std::os::raw::c_char, __n: usize)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strchr(__s: *const ::std::os::raw::c_char,
                  __c: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strrchr(__s: *const ::std::os::raw::c_char,
                   __c: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strcspn(__s: *const ::std::os::raw::c_char,
                   __reject: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strspn(__s: *const ::std::os::raw::c_char,
                  __accept: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strpbrk(__s: *const ::std::os::raw::c_char,
                   __accept: *const ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strstr(__haystack: *const ::std::os::raw::c_char,
                  __needle: *const ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strtok(__s: *mut ::std::os::raw::c_char,
                  __delim: *const ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __strtok_r(__s: *mut ::std::os::raw::c_char,
                      __delim: *const ::std::os::raw::c_char,
                      __save_ptr: *mut *mut ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strtok_r(__s: *mut ::std::os::raw::c_char,
                    __delim: *const ::std::os::raw::c_char,
                    __save_ptr: *mut *mut ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strlen(__s: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strnlen(__string: *const ::std::os::raw::c_char, __maxlen: usize)
     -> usize;
}
extern "C" {
    pub fn strerror(__errnum: ::std::os::raw::c_int)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "__xpg_strerror_r"]
    pub fn strerror_r(__errnum: ::std::os::raw::c_int,
                      __buf: *mut ::std::os::raw::c_char, __buflen: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strerror_l(__errnum: ::std::os::raw::c_int, __l: __locale_t)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __bzero(__s: *mut ::std::os::raw::c_void, __n: usize);
}
extern "C" {
    pub fn bcopy(__src: *const ::std::os::raw::c_void,
                 __dest: *mut ::std::os::raw::c_void, __n: usize);
}
extern "C" {
    pub fn bzero(__s: *mut ::std::os::raw::c_void, __n: usize);
}
extern "C" {
    pub fn explicit_bzero(__s: *mut ::std::os::raw::c_void, __n: usize);
}
extern "C" {
    pub fn bcmp(__s1: *const ::std::os::raw::c_void,
                __s2: *const ::std::os::raw::c_void, __n: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn index(__s: *const ::std::os::raw::c_char,
                 __c: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn rindex(__s: *const ::std::os::raw::c_char,
                  __c: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ffs(__i: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strcasecmp(__s1: *const ::std::os::raw::c_char,
                      __s2: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strncasecmp(__s1: *const ::std::os::raw::c_char,
                       __s2: *const ::std::os::raw::c_char, __n: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strsep(__stringp: *mut *mut ::std::os::raw::c_char,
                  __delim: *const ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strsignal(__sig: ::std::os::raw::c_int)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __stpcpy(__dest: *mut ::std::os::raw::c_char,
                    __src: *const ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn stpcpy(__dest: *mut ::std::os::raw::c_char,
                  __src: *const ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __stpncpy(__dest: *mut ::std::os::raw::c_char,
                     __src: *const ::std::os::raw::c_char, __n: usize)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn stpncpy(__dest: *mut ::std::os::raw::c_char,
                   __src: *const ::std::os::raw::c_char, __n: usize)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __assert_fail(__assertion: *const ::std::os::raw::c_char,
                         __file: *const ::std::os::raw::c_char,
                         __line: ::std::os::raw::c_uint,
                         __function: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn __assert_perror_fail(__errnum: ::std::os::raw::c_int,
                                __file: *const ::std::os::raw::c_char,
                                __line: ::std::os::raw::c_uint,
                                __function: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn __assert(__assertion: *const ::std::os::raw::c_char,
                    __file: *const ::std::os::raw::c_char,
                    __line: ::std::os::raw::c_int);
}
pub type useconds_t = __useconds_t;
pub type socklen_t = __socklen_t;
extern "C" {
    pub fn access(__name: *const ::std::os::raw::c_char,
                  __type: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faccessat(__fd: ::std::os::raw::c_int,
                     __file: *const ::std::os::raw::c_char,
                     __type: ::std::os::raw::c_int,
                     __flag: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lseek(__fd: ::std::os::raw::c_int, __offset: __off_t,
                 __whence: ::std::os::raw::c_int) -> __off_t;
}
extern "C" {
    pub fn close(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn read(__fd: ::std::os::raw::c_int,
                __buf: *mut ::std::os::raw::c_void, __nbytes: usize) -> isize;
}
extern "C" {
    pub fn write(__fd: ::std::os::raw::c_int,
                 __buf: *const ::std::os::raw::c_void, __n: usize) -> isize;
}
extern "C" {
    pub fn pread(__fd: ::std::os::raw::c_int,
                 __buf: *mut ::std::os::raw::c_void, __nbytes: usize,
                 __offset: __off_t) -> isize;
}
extern "C" {
    pub fn pwrite(__fd: ::std::os::raw::c_int,
                  __buf: *const ::std::os::raw::c_void, __n: usize,
                  __offset: __off_t) -> isize;
}
extern "C" {
    pub fn pipe(__pipedes: *mut ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn alarm(__seconds: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn sleep(__seconds: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn ualarm(__value: __useconds_t, __interval: __useconds_t)
     -> __useconds_t;
}
extern "C" {
    pub fn usleep(__useconds: __useconds_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pause() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn chown(__file: *const ::std::os::raw::c_char, __owner: __uid_t,
                 __group: __gid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fchown(__fd: ::std::os::raw::c_int, __owner: __uid_t,
                  __group: __gid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lchown(__file: *const ::std::os::raw::c_char, __owner: __uid_t,
                  __group: __gid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fchownat(__fd: ::std::os::raw::c_int,
                    __file: *const ::std::os::raw::c_char, __owner: __uid_t,
                    __group: __gid_t, __flag: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn chdir(__path: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fchdir(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getcwd(__buf: *mut ::std::os::raw::c_char, __size: usize)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn getwd(__buf: *mut ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn dup(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn dup2(__fd: ::std::os::raw::c_int, __fd2: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "__environ"]
    pub static mut __environ: *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn execve(__path: *const ::std::os::raw::c_char,
                  __argv: *const *const ::std::os::raw::c_char,
                  __envp: *const *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fexecve(__fd: ::std::os::raw::c_int,
                   __argv: *const *const ::std::os::raw::c_char,
                   __envp: *const *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn execv(__path: *const ::std::os::raw::c_char,
                 __argv: *const *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn execle(__path: *const ::std::os::raw::c_char,
                  __arg: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn execl(__path: *const ::std::os::raw::c_char,
                 __arg: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn execvp(__file: *const ::std::os::raw::c_char,
                  __argv: *const *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn execlp(__file: *const ::std::os::raw::c_char,
                  __arg: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nice(__inc: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _exit(__status: ::std::os::raw::c_int);
}
pub const _PC_LINK_MAX: _bindgen_ty_3 = _bindgen_ty_3::_PC_LINK_MAX;
pub const _PC_MAX_CANON: _bindgen_ty_3 = _bindgen_ty_3::_PC_MAX_CANON;
pub const _PC_MAX_INPUT: _bindgen_ty_3 = _bindgen_ty_3::_PC_MAX_INPUT;
pub const _PC_NAME_MAX: _bindgen_ty_3 = _bindgen_ty_3::_PC_NAME_MAX;
pub const _PC_PATH_MAX: _bindgen_ty_3 = _bindgen_ty_3::_PC_PATH_MAX;
pub const _PC_PIPE_BUF: _bindgen_ty_3 = _bindgen_ty_3::_PC_PIPE_BUF;
pub const _PC_CHOWN_RESTRICTED: _bindgen_ty_3 =
    _bindgen_ty_3::_PC_CHOWN_RESTRICTED;
pub const _PC_NO_TRUNC: _bindgen_ty_3 = _bindgen_ty_3::_PC_NO_TRUNC;
pub const _PC_VDISABLE: _bindgen_ty_3 = _bindgen_ty_3::_PC_VDISABLE;
pub const _PC_SYNC_IO: _bindgen_ty_3 = _bindgen_ty_3::_PC_SYNC_IO;
pub const _PC_ASYNC_IO: _bindgen_ty_3 = _bindgen_ty_3::_PC_ASYNC_IO;
pub const _PC_PRIO_IO: _bindgen_ty_3 = _bindgen_ty_3::_PC_PRIO_IO;
pub const _PC_SOCK_MAXBUF: _bindgen_ty_3 = _bindgen_ty_3::_PC_SOCK_MAXBUF;
pub const _PC_FILESIZEBITS: _bindgen_ty_3 = _bindgen_ty_3::_PC_FILESIZEBITS;
pub const _PC_REC_INCR_XFER_SIZE: _bindgen_ty_3 =
    _bindgen_ty_3::_PC_REC_INCR_XFER_SIZE;
pub const _PC_REC_MAX_XFER_SIZE: _bindgen_ty_3 =
    _bindgen_ty_3::_PC_REC_MAX_XFER_SIZE;
pub const _PC_REC_MIN_XFER_SIZE: _bindgen_ty_3 =
    _bindgen_ty_3::_PC_REC_MIN_XFER_SIZE;
pub const _PC_REC_XFER_ALIGN: _bindgen_ty_3 =
    _bindgen_ty_3::_PC_REC_XFER_ALIGN;
pub const _PC_ALLOC_SIZE_MIN: _bindgen_ty_3 =
    _bindgen_ty_3::_PC_ALLOC_SIZE_MIN;
pub const _PC_SYMLINK_MAX: _bindgen_ty_3 = _bindgen_ty_3::_PC_SYMLINK_MAX;
pub const _PC_2_SYMLINKS: _bindgen_ty_3 = _bindgen_ty_3::_PC_2_SYMLINKS;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _bindgen_ty_3 {
    _PC_LINK_MAX = 0,
    _PC_MAX_CANON = 1,
    _PC_MAX_INPUT = 2,
    _PC_NAME_MAX = 3,
    _PC_PATH_MAX = 4,
    _PC_PIPE_BUF = 5,
    _PC_CHOWN_RESTRICTED = 6,
    _PC_NO_TRUNC = 7,
    _PC_VDISABLE = 8,
    _PC_SYNC_IO = 9,
    _PC_ASYNC_IO = 10,
    _PC_PRIO_IO = 11,
    _PC_SOCK_MAXBUF = 12,
    _PC_FILESIZEBITS = 13,
    _PC_REC_INCR_XFER_SIZE = 14,
    _PC_REC_MAX_XFER_SIZE = 15,
    _PC_REC_MIN_XFER_SIZE = 16,
    _PC_REC_XFER_ALIGN = 17,
    _PC_ALLOC_SIZE_MIN = 18,
    _PC_SYMLINK_MAX = 19,
    _PC_2_SYMLINKS = 20,
}
pub const _SC_ARG_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_ARG_MAX;
pub const _SC_CHILD_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_CHILD_MAX;
pub const _SC_CLK_TCK: _bindgen_ty_4 = _bindgen_ty_4::_SC_CLK_TCK;
pub const _SC_NGROUPS_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_NGROUPS_MAX;
pub const _SC_OPEN_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_OPEN_MAX;
pub const _SC_STREAM_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_STREAM_MAX;
pub const _SC_TZNAME_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_TZNAME_MAX;
pub const _SC_JOB_CONTROL: _bindgen_ty_4 = _bindgen_ty_4::_SC_JOB_CONTROL;
pub const _SC_SAVED_IDS: _bindgen_ty_4 = _bindgen_ty_4::_SC_SAVED_IDS;
pub const _SC_REALTIME_SIGNALS: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_REALTIME_SIGNALS;
pub const _SC_PRIORITY_SCHEDULING: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_PRIORITY_SCHEDULING;
pub const _SC_TIMERS: _bindgen_ty_4 = _bindgen_ty_4::_SC_TIMERS;
pub const _SC_ASYNCHRONOUS_IO: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_ASYNCHRONOUS_IO;
pub const _SC_PRIORITIZED_IO: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_PRIORITIZED_IO;
pub const _SC_SYNCHRONIZED_IO: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_SYNCHRONIZED_IO;
pub const _SC_FSYNC: _bindgen_ty_4 = _bindgen_ty_4::_SC_FSYNC;
pub const _SC_MAPPED_FILES: _bindgen_ty_4 = _bindgen_ty_4::_SC_MAPPED_FILES;
pub const _SC_MEMLOCK: _bindgen_ty_4 = _bindgen_ty_4::_SC_MEMLOCK;
pub const _SC_MEMLOCK_RANGE: _bindgen_ty_4 = _bindgen_ty_4::_SC_MEMLOCK_RANGE;
pub const _SC_MEMORY_PROTECTION: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_MEMORY_PROTECTION;
pub const _SC_MESSAGE_PASSING: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_MESSAGE_PASSING;
pub const _SC_SEMAPHORES: _bindgen_ty_4 = _bindgen_ty_4::_SC_SEMAPHORES;
pub const _SC_SHARED_MEMORY_OBJECTS: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_SHARED_MEMORY_OBJECTS;
pub const _SC_AIO_LISTIO_MAX: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_AIO_LISTIO_MAX;
pub const _SC_AIO_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_AIO_MAX;
pub const _SC_AIO_PRIO_DELTA_MAX: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_AIO_PRIO_DELTA_MAX;
pub const _SC_DELAYTIMER_MAX: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_DELAYTIMER_MAX;
pub const _SC_MQ_OPEN_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_MQ_OPEN_MAX;
pub const _SC_MQ_PRIO_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_MQ_PRIO_MAX;
pub const _SC_VERSION: _bindgen_ty_4 = _bindgen_ty_4::_SC_VERSION;
pub const _SC_PAGESIZE: _bindgen_ty_4 = _bindgen_ty_4::_SC_PAGESIZE;
pub const _SC_RTSIG_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_RTSIG_MAX;
pub const _SC_SEM_NSEMS_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_SEM_NSEMS_MAX;
pub const _SC_SEM_VALUE_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_SEM_VALUE_MAX;
pub const _SC_SIGQUEUE_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_SIGQUEUE_MAX;
pub const _SC_TIMER_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_TIMER_MAX;
pub const _SC_BC_BASE_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_BC_BASE_MAX;
pub const _SC_BC_DIM_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_BC_DIM_MAX;
pub const _SC_BC_SCALE_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_BC_SCALE_MAX;
pub const _SC_BC_STRING_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_BC_STRING_MAX;
pub const _SC_COLL_WEIGHTS_MAX: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_COLL_WEIGHTS_MAX;
pub const _SC_EQUIV_CLASS_MAX: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_EQUIV_CLASS_MAX;
pub const _SC_EXPR_NEST_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_EXPR_NEST_MAX;
pub const _SC_LINE_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_LINE_MAX;
pub const _SC_RE_DUP_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_RE_DUP_MAX;
pub const _SC_CHARCLASS_NAME_MAX: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_CHARCLASS_NAME_MAX;
pub const _SC_2_VERSION: _bindgen_ty_4 = _bindgen_ty_4::_SC_2_VERSION;
pub const _SC_2_C_BIND: _bindgen_ty_4 = _bindgen_ty_4::_SC_2_C_BIND;
pub const _SC_2_C_DEV: _bindgen_ty_4 = _bindgen_ty_4::_SC_2_C_DEV;
pub const _SC_2_FORT_DEV: _bindgen_ty_4 = _bindgen_ty_4::_SC_2_FORT_DEV;
pub const _SC_2_FORT_RUN: _bindgen_ty_4 = _bindgen_ty_4::_SC_2_FORT_RUN;
pub const _SC_2_SW_DEV: _bindgen_ty_4 = _bindgen_ty_4::_SC_2_SW_DEV;
pub const _SC_2_LOCALEDEF: _bindgen_ty_4 = _bindgen_ty_4::_SC_2_LOCALEDEF;
pub const _SC_PII: _bindgen_ty_4 = _bindgen_ty_4::_SC_PII;
pub const _SC_PII_XTI: _bindgen_ty_4 = _bindgen_ty_4::_SC_PII_XTI;
pub const _SC_PII_SOCKET: _bindgen_ty_4 = _bindgen_ty_4::_SC_PII_SOCKET;
pub const _SC_PII_INTERNET: _bindgen_ty_4 = _bindgen_ty_4::_SC_PII_INTERNET;
pub const _SC_PII_OSI: _bindgen_ty_4 = _bindgen_ty_4::_SC_PII_OSI;
pub const _SC_POLL: _bindgen_ty_4 = _bindgen_ty_4::_SC_POLL;
pub const _SC_SELECT: _bindgen_ty_4 = _bindgen_ty_4::_SC_SELECT;
pub const _SC_UIO_MAXIOV: _bindgen_ty_4 = _bindgen_ty_4::_SC_UIO_MAXIOV;
pub const _SC_IOV_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_UIO_MAXIOV;
pub const _SC_PII_INTERNET_STREAM: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_PII_INTERNET_STREAM;
pub const _SC_PII_INTERNET_DGRAM: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_PII_INTERNET_DGRAM;
pub const _SC_PII_OSI_COTS: _bindgen_ty_4 = _bindgen_ty_4::_SC_PII_OSI_COTS;
pub const _SC_PII_OSI_CLTS: _bindgen_ty_4 = _bindgen_ty_4::_SC_PII_OSI_CLTS;
pub const _SC_PII_OSI_M: _bindgen_ty_4 = _bindgen_ty_4::_SC_PII_OSI_M;
pub const _SC_T_IOV_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_T_IOV_MAX;
pub const _SC_THREADS: _bindgen_ty_4 = _bindgen_ty_4::_SC_THREADS;
pub const _SC_THREAD_SAFE_FUNCTIONS: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_THREAD_SAFE_FUNCTIONS;
pub const _SC_GETGR_R_SIZE_MAX: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_GETGR_R_SIZE_MAX;
pub const _SC_GETPW_R_SIZE_MAX: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_GETPW_R_SIZE_MAX;
pub const _SC_LOGIN_NAME_MAX: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_LOGIN_NAME_MAX;
pub const _SC_TTY_NAME_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_TTY_NAME_MAX;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_THREAD_DESTRUCTOR_ITERATIONS;
pub const _SC_THREAD_KEYS_MAX: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_THREAD_KEYS_MAX;
pub const _SC_THREAD_STACK_MIN: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_THREAD_STACK_MIN;
pub const _SC_THREAD_THREADS_MAX: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_THREAD_THREADS_MAX;
pub const _SC_THREAD_ATTR_STACKADDR: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_THREAD_ATTR_STACKADDR;
pub const _SC_THREAD_ATTR_STACKSIZE: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_THREAD_ATTR_STACKSIZE;
pub const _SC_THREAD_PRIORITY_SCHEDULING: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_THREAD_PRIORITY_SCHEDULING;
pub const _SC_THREAD_PRIO_INHERIT: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_THREAD_PRIO_INHERIT;
pub const _SC_THREAD_PRIO_PROTECT: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_THREAD_PRIO_PROTECT;
pub const _SC_THREAD_PROCESS_SHARED: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_THREAD_PROCESS_SHARED;
pub const _SC_NPROCESSORS_CONF: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_NPROCESSORS_CONF;
pub const _SC_NPROCESSORS_ONLN: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_NPROCESSORS_ONLN;
pub const _SC_PHYS_PAGES: _bindgen_ty_4 = _bindgen_ty_4::_SC_PHYS_PAGES;
pub const _SC_AVPHYS_PAGES: _bindgen_ty_4 = _bindgen_ty_4::_SC_AVPHYS_PAGES;
pub const _SC_ATEXIT_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_ATEXIT_MAX;
pub const _SC_PASS_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_PASS_MAX;
pub const _SC_XOPEN_VERSION: _bindgen_ty_4 = _bindgen_ty_4::_SC_XOPEN_VERSION;
pub const _SC_XOPEN_XCU_VERSION: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_XOPEN_XCU_VERSION;
pub const _SC_XOPEN_UNIX: _bindgen_ty_4 = _bindgen_ty_4::_SC_XOPEN_UNIX;
pub const _SC_XOPEN_CRYPT: _bindgen_ty_4 = _bindgen_ty_4::_SC_XOPEN_CRYPT;
pub const _SC_XOPEN_ENH_I18N: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_XOPEN_ENH_I18N;
pub const _SC_XOPEN_SHM: _bindgen_ty_4 = _bindgen_ty_4::_SC_XOPEN_SHM;
pub const _SC_2_CHAR_TERM: _bindgen_ty_4 = _bindgen_ty_4::_SC_2_CHAR_TERM;
pub const _SC_2_C_VERSION: _bindgen_ty_4 = _bindgen_ty_4::_SC_2_C_VERSION;
pub const _SC_2_UPE: _bindgen_ty_4 = _bindgen_ty_4::_SC_2_UPE;
pub const _SC_XOPEN_XPG2: _bindgen_ty_4 = _bindgen_ty_4::_SC_XOPEN_XPG2;
pub const _SC_XOPEN_XPG3: _bindgen_ty_4 = _bindgen_ty_4::_SC_XOPEN_XPG3;
pub const _SC_XOPEN_XPG4: _bindgen_ty_4 = _bindgen_ty_4::_SC_XOPEN_XPG4;
pub const _SC_CHAR_BIT: _bindgen_ty_4 = _bindgen_ty_4::_SC_CHAR_BIT;
pub const _SC_CHAR_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_CHAR_MAX;
pub const _SC_CHAR_MIN: _bindgen_ty_4 = _bindgen_ty_4::_SC_CHAR_MIN;
pub const _SC_INT_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_INT_MAX;
pub const _SC_INT_MIN: _bindgen_ty_4 = _bindgen_ty_4::_SC_INT_MIN;
pub const _SC_LONG_BIT: _bindgen_ty_4 = _bindgen_ty_4::_SC_LONG_BIT;
pub const _SC_WORD_BIT: _bindgen_ty_4 = _bindgen_ty_4::_SC_WORD_BIT;
pub const _SC_MB_LEN_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_MB_LEN_MAX;
pub const _SC_NZERO: _bindgen_ty_4 = _bindgen_ty_4::_SC_NZERO;
pub const _SC_SSIZE_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_SSIZE_MAX;
pub const _SC_SCHAR_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_SCHAR_MAX;
pub const _SC_SCHAR_MIN: _bindgen_ty_4 = _bindgen_ty_4::_SC_SCHAR_MIN;
pub const _SC_SHRT_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_SHRT_MAX;
pub const _SC_SHRT_MIN: _bindgen_ty_4 = _bindgen_ty_4::_SC_SHRT_MIN;
pub const _SC_UCHAR_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_UCHAR_MAX;
pub const _SC_UINT_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_UINT_MAX;
pub const _SC_ULONG_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_ULONG_MAX;
pub const _SC_USHRT_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_USHRT_MAX;
pub const _SC_NL_ARGMAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_NL_ARGMAX;
pub const _SC_NL_LANGMAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_NL_LANGMAX;
pub const _SC_NL_MSGMAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_NL_MSGMAX;
pub const _SC_NL_NMAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_NL_NMAX;
pub const _SC_NL_SETMAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_NL_SETMAX;
pub const _SC_NL_TEXTMAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_NL_TEXTMAX;
pub const _SC_XBS5_ILP32_OFF32: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_XBS5_ILP32_OFF32;
pub const _SC_XBS5_ILP32_OFFBIG: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_XBS5_ILP32_OFFBIG;
pub const _SC_XBS5_LP64_OFF64: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_XBS5_LP64_OFF64;
pub const _SC_XBS5_LPBIG_OFFBIG: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_XBS5_LPBIG_OFFBIG;
pub const _SC_XOPEN_LEGACY: _bindgen_ty_4 = _bindgen_ty_4::_SC_XOPEN_LEGACY;
pub const _SC_XOPEN_REALTIME: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_XOPEN_REALTIME;
pub const _SC_XOPEN_REALTIME_THREADS: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_XOPEN_REALTIME_THREADS;
pub const _SC_ADVISORY_INFO: _bindgen_ty_4 = _bindgen_ty_4::_SC_ADVISORY_INFO;
pub const _SC_BARRIERS: _bindgen_ty_4 = _bindgen_ty_4::_SC_BARRIERS;
pub const _SC_BASE: _bindgen_ty_4 = _bindgen_ty_4::_SC_BASE;
pub const _SC_C_LANG_SUPPORT: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_C_LANG_SUPPORT;
pub const _SC_C_LANG_SUPPORT_R: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_C_LANG_SUPPORT_R;
pub const _SC_CLOCK_SELECTION: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_CLOCK_SELECTION;
pub const _SC_CPUTIME: _bindgen_ty_4 = _bindgen_ty_4::_SC_CPUTIME;
pub const _SC_THREAD_CPUTIME: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_THREAD_CPUTIME;
pub const _SC_DEVICE_IO: _bindgen_ty_4 = _bindgen_ty_4::_SC_DEVICE_IO;
pub const _SC_DEVICE_SPECIFIC: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_DEVICE_SPECIFIC;
pub const _SC_DEVICE_SPECIFIC_R: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_DEVICE_SPECIFIC_R;
pub const _SC_FD_MGMT: _bindgen_ty_4 = _bindgen_ty_4::_SC_FD_MGMT;
pub const _SC_FIFO: _bindgen_ty_4 = _bindgen_ty_4::_SC_FIFO;
pub const _SC_PIPE: _bindgen_ty_4 = _bindgen_ty_4::_SC_PIPE;
pub const _SC_FILE_ATTRIBUTES: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_FILE_ATTRIBUTES;
pub const _SC_FILE_LOCKING: _bindgen_ty_4 = _bindgen_ty_4::_SC_FILE_LOCKING;
pub const _SC_FILE_SYSTEM: _bindgen_ty_4 = _bindgen_ty_4::_SC_FILE_SYSTEM;
pub const _SC_MONOTONIC_CLOCK: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_MONOTONIC_CLOCK;
pub const _SC_MULTI_PROCESS: _bindgen_ty_4 = _bindgen_ty_4::_SC_MULTI_PROCESS;
pub const _SC_SINGLE_PROCESS: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_SINGLE_PROCESS;
pub const _SC_NETWORKING: _bindgen_ty_4 = _bindgen_ty_4::_SC_NETWORKING;
pub const _SC_READER_WRITER_LOCKS: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_READER_WRITER_LOCKS;
pub const _SC_SPIN_LOCKS: _bindgen_ty_4 = _bindgen_ty_4::_SC_SPIN_LOCKS;
pub const _SC_REGEXP: _bindgen_ty_4 = _bindgen_ty_4::_SC_REGEXP;
pub const _SC_REGEX_VERSION: _bindgen_ty_4 = _bindgen_ty_4::_SC_REGEX_VERSION;
pub const _SC_SHELL: _bindgen_ty_4 = _bindgen_ty_4::_SC_SHELL;
pub const _SC_SIGNALS: _bindgen_ty_4 = _bindgen_ty_4::_SC_SIGNALS;
pub const _SC_SPAWN: _bindgen_ty_4 = _bindgen_ty_4::_SC_SPAWN;
pub const _SC_SPORADIC_SERVER: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_SPORADIC_SERVER;
pub const _SC_THREAD_SPORADIC_SERVER: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_THREAD_SPORADIC_SERVER;
pub const _SC_SYSTEM_DATABASE: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_SYSTEM_DATABASE;
pub const _SC_SYSTEM_DATABASE_R: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_SYSTEM_DATABASE_R;
pub const _SC_TIMEOUTS: _bindgen_ty_4 = _bindgen_ty_4::_SC_TIMEOUTS;
pub const _SC_TYPED_MEMORY_OBJECTS: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_TYPED_MEMORY_OBJECTS;
pub const _SC_USER_GROUPS: _bindgen_ty_4 = _bindgen_ty_4::_SC_USER_GROUPS;
pub const _SC_USER_GROUPS_R: _bindgen_ty_4 = _bindgen_ty_4::_SC_USER_GROUPS_R;
pub const _SC_2_PBS: _bindgen_ty_4 = _bindgen_ty_4::_SC_2_PBS;
pub const _SC_2_PBS_ACCOUNTING: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_2_PBS_ACCOUNTING;
pub const _SC_2_PBS_LOCATE: _bindgen_ty_4 = _bindgen_ty_4::_SC_2_PBS_LOCATE;
pub const _SC_2_PBS_MESSAGE: _bindgen_ty_4 = _bindgen_ty_4::_SC_2_PBS_MESSAGE;
pub const _SC_2_PBS_TRACK: _bindgen_ty_4 = _bindgen_ty_4::_SC_2_PBS_TRACK;
pub const _SC_SYMLOOP_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_SYMLOOP_MAX;
pub const _SC_STREAMS: _bindgen_ty_4 = _bindgen_ty_4::_SC_STREAMS;
pub const _SC_2_PBS_CHECKPOINT: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_2_PBS_CHECKPOINT;
pub const _SC_V6_ILP32_OFF32: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_V6_ILP32_OFF32;
pub const _SC_V6_ILP32_OFFBIG: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_V6_ILP32_OFFBIG;
pub const _SC_V6_LP64_OFF64: _bindgen_ty_4 = _bindgen_ty_4::_SC_V6_LP64_OFF64;
pub const _SC_V6_LPBIG_OFFBIG: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_V6_LPBIG_OFFBIG;
pub const _SC_HOST_NAME_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_HOST_NAME_MAX;
pub const _SC_TRACE: _bindgen_ty_4 = _bindgen_ty_4::_SC_TRACE;
pub const _SC_TRACE_EVENT_FILTER: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_TRACE_EVENT_FILTER;
pub const _SC_TRACE_INHERIT: _bindgen_ty_4 = _bindgen_ty_4::_SC_TRACE_INHERIT;
pub const _SC_TRACE_LOG: _bindgen_ty_4 = _bindgen_ty_4::_SC_TRACE_LOG;
pub const _SC_LEVEL1_ICACHE_SIZE: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_LEVEL1_ICACHE_SIZE;
pub const _SC_LEVEL1_ICACHE_ASSOC: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_LEVEL1_ICACHE_ASSOC;
pub const _SC_LEVEL1_ICACHE_LINESIZE: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_LEVEL1_ICACHE_LINESIZE;
pub const _SC_LEVEL1_DCACHE_SIZE: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_LEVEL1_DCACHE_SIZE;
pub const _SC_LEVEL1_DCACHE_ASSOC: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_LEVEL1_DCACHE_ASSOC;
pub const _SC_LEVEL1_DCACHE_LINESIZE: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_LEVEL1_DCACHE_LINESIZE;
pub const _SC_LEVEL2_CACHE_SIZE: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_LEVEL2_CACHE_SIZE;
pub const _SC_LEVEL2_CACHE_ASSOC: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_LEVEL2_CACHE_ASSOC;
pub const _SC_LEVEL2_CACHE_LINESIZE: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_LEVEL2_CACHE_LINESIZE;
pub const _SC_LEVEL3_CACHE_SIZE: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_LEVEL3_CACHE_SIZE;
pub const _SC_LEVEL3_CACHE_ASSOC: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_LEVEL3_CACHE_ASSOC;
pub const _SC_LEVEL3_CACHE_LINESIZE: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_LEVEL3_CACHE_LINESIZE;
pub const _SC_LEVEL4_CACHE_SIZE: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_LEVEL4_CACHE_SIZE;
pub const _SC_LEVEL4_CACHE_ASSOC: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_LEVEL4_CACHE_ASSOC;
pub const _SC_LEVEL4_CACHE_LINESIZE: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_LEVEL4_CACHE_LINESIZE;
pub const _SC_IPV6: _bindgen_ty_4 = _bindgen_ty_4::_SC_IPV6;
pub const _SC_RAW_SOCKETS: _bindgen_ty_4 = _bindgen_ty_4::_SC_RAW_SOCKETS;
pub const _SC_V7_ILP32_OFF32: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_V7_ILP32_OFF32;
pub const _SC_V7_ILP32_OFFBIG: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_V7_ILP32_OFFBIG;
pub const _SC_V7_LP64_OFF64: _bindgen_ty_4 = _bindgen_ty_4::_SC_V7_LP64_OFF64;
pub const _SC_V7_LPBIG_OFFBIG: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_V7_LPBIG_OFFBIG;
pub const _SC_SS_REPL_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_SS_REPL_MAX;
pub const _SC_TRACE_EVENT_NAME_MAX: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_TRACE_EVENT_NAME_MAX;
pub const _SC_TRACE_NAME_MAX: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_TRACE_NAME_MAX;
pub const _SC_TRACE_SYS_MAX: _bindgen_ty_4 = _bindgen_ty_4::_SC_TRACE_SYS_MAX;
pub const _SC_TRACE_USER_EVENT_MAX: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_TRACE_USER_EVENT_MAX;
pub const _SC_XOPEN_STREAMS: _bindgen_ty_4 = _bindgen_ty_4::_SC_XOPEN_STREAMS;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_THREAD_ROBUST_PRIO_INHERIT;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: _bindgen_ty_4 =
    _bindgen_ty_4::_SC_THREAD_ROBUST_PRIO_PROTECT;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _bindgen_ty_4 {
    _SC_ARG_MAX = 0,
    _SC_CHILD_MAX = 1,
    _SC_CLK_TCK = 2,
    _SC_NGROUPS_MAX = 3,
    _SC_OPEN_MAX = 4,
    _SC_STREAM_MAX = 5,
    _SC_TZNAME_MAX = 6,
    _SC_JOB_CONTROL = 7,
    _SC_SAVED_IDS = 8,
    _SC_REALTIME_SIGNALS = 9,
    _SC_PRIORITY_SCHEDULING = 10,
    _SC_TIMERS = 11,
    _SC_ASYNCHRONOUS_IO = 12,
    _SC_PRIORITIZED_IO = 13,
    _SC_SYNCHRONIZED_IO = 14,
    _SC_FSYNC = 15,
    _SC_MAPPED_FILES = 16,
    _SC_MEMLOCK = 17,
    _SC_MEMLOCK_RANGE = 18,
    _SC_MEMORY_PROTECTION = 19,
    _SC_MESSAGE_PASSING = 20,
    _SC_SEMAPHORES = 21,
    _SC_SHARED_MEMORY_OBJECTS = 22,
    _SC_AIO_LISTIO_MAX = 23,
    _SC_AIO_MAX = 24,
    _SC_AIO_PRIO_DELTA_MAX = 25,
    _SC_DELAYTIMER_MAX = 26,
    _SC_MQ_OPEN_MAX = 27,
    _SC_MQ_PRIO_MAX = 28,
    _SC_VERSION = 29,
    _SC_PAGESIZE = 30,
    _SC_RTSIG_MAX = 31,
    _SC_SEM_NSEMS_MAX = 32,
    _SC_SEM_VALUE_MAX = 33,
    _SC_SIGQUEUE_MAX = 34,
    _SC_TIMER_MAX = 35,
    _SC_BC_BASE_MAX = 36,
    _SC_BC_DIM_MAX = 37,
    _SC_BC_SCALE_MAX = 38,
    _SC_BC_STRING_MAX = 39,
    _SC_COLL_WEIGHTS_MAX = 40,
    _SC_EQUIV_CLASS_MAX = 41,
    _SC_EXPR_NEST_MAX = 42,
    _SC_LINE_MAX = 43,
    _SC_RE_DUP_MAX = 44,
    _SC_CHARCLASS_NAME_MAX = 45,
    _SC_2_VERSION = 46,
    _SC_2_C_BIND = 47,
    _SC_2_C_DEV = 48,
    _SC_2_FORT_DEV = 49,
    _SC_2_FORT_RUN = 50,
    _SC_2_SW_DEV = 51,
    _SC_2_LOCALEDEF = 52,
    _SC_PII = 53,
    _SC_PII_XTI = 54,
    _SC_PII_SOCKET = 55,
    _SC_PII_INTERNET = 56,
    _SC_PII_OSI = 57,
    _SC_POLL = 58,
    _SC_SELECT = 59,
    _SC_UIO_MAXIOV = 60,
    _SC_PII_INTERNET_STREAM = 61,
    _SC_PII_INTERNET_DGRAM = 62,
    _SC_PII_OSI_COTS = 63,
    _SC_PII_OSI_CLTS = 64,
    _SC_PII_OSI_M = 65,
    _SC_T_IOV_MAX = 66,
    _SC_THREADS = 67,
    _SC_THREAD_SAFE_FUNCTIONS = 68,
    _SC_GETGR_R_SIZE_MAX = 69,
    _SC_GETPW_R_SIZE_MAX = 70,
    _SC_LOGIN_NAME_MAX = 71,
    _SC_TTY_NAME_MAX = 72,
    _SC_THREAD_DESTRUCTOR_ITERATIONS = 73,
    _SC_THREAD_KEYS_MAX = 74,
    _SC_THREAD_STACK_MIN = 75,
    _SC_THREAD_THREADS_MAX = 76,
    _SC_THREAD_ATTR_STACKADDR = 77,
    _SC_THREAD_ATTR_STACKSIZE = 78,
    _SC_THREAD_PRIORITY_SCHEDULING = 79,
    _SC_THREAD_PRIO_INHERIT = 80,
    _SC_THREAD_PRIO_PROTECT = 81,
    _SC_THREAD_PROCESS_SHARED = 82,
    _SC_NPROCESSORS_CONF = 83,
    _SC_NPROCESSORS_ONLN = 84,
    _SC_PHYS_PAGES = 85,
    _SC_AVPHYS_PAGES = 86,
    _SC_ATEXIT_MAX = 87,
    _SC_PASS_MAX = 88,
    _SC_XOPEN_VERSION = 89,
    _SC_XOPEN_XCU_VERSION = 90,
    _SC_XOPEN_UNIX = 91,
    _SC_XOPEN_CRYPT = 92,
    _SC_XOPEN_ENH_I18N = 93,
    _SC_XOPEN_SHM = 94,
    _SC_2_CHAR_TERM = 95,
    _SC_2_C_VERSION = 96,
    _SC_2_UPE = 97,
    _SC_XOPEN_XPG2 = 98,
    _SC_XOPEN_XPG3 = 99,
    _SC_XOPEN_XPG4 = 100,
    _SC_CHAR_BIT = 101,
    _SC_CHAR_MAX = 102,
    _SC_CHAR_MIN = 103,
    _SC_INT_MAX = 104,
    _SC_INT_MIN = 105,
    _SC_LONG_BIT = 106,
    _SC_WORD_BIT = 107,
    _SC_MB_LEN_MAX = 108,
    _SC_NZERO = 109,
    _SC_SSIZE_MAX = 110,
    _SC_SCHAR_MAX = 111,
    _SC_SCHAR_MIN = 112,
    _SC_SHRT_MAX = 113,
    _SC_SHRT_MIN = 114,
    _SC_UCHAR_MAX = 115,
    _SC_UINT_MAX = 116,
    _SC_ULONG_MAX = 117,
    _SC_USHRT_MAX = 118,
    _SC_NL_ARGMAX = 119,
    _SC_NL_LANGMAX = 120,
    _SC_NL_MSGMAX = 121,
    _SC_NL_NMAX = 122,
    _SC_NL_SETMAX = 123,
    _SC_NL_TEXTMAX = 124,
    _SC_XBS5_ILP32_OFF32 = 125,
    _SC_XBS5_ILP32_OFFBIG = 126,
    _SC_XBS5_LP64_OFF64 = 127,
    _SC_XBS5_LPBIG_OFFBIG = 128,
    _SC_XOPEN_LEGACY = 129,
    _SC_XOPEN_REALTIME = 130,
    _SC_XOPEN_REALTIME_THREADS = 131,
    _SC_ADVISORY_INFO = 132,
    _SC_BARRIERS = 133,
    _SC_BASE = 134,
    _SC_C_LANG_SUPPORT = 135,
    _SC_C_LANG_SUPPORT_R = 136,
    _SC_CLOCK_SELECTION = 137,
    _SC_CPUTIME = 138,
    _SC_THREAD_CPUTIME = 139,
    _SC_DEVICE_IO = 140,
    _SC_DEVICE_SPECIFIC = 141,
    _SC_DEVICE_SPECIFIC_R = 142,
    _SC_FD_MGMT = 143,
    _SC_FIFO = 144,
    _SC_PIPE = 145,
    _SC_FILE_ATTRIBUTES = 146,
    _SC_FILE_LOCKING = 147,
    _SC_FILE_SYSTEM = 148,
    _SC_MONOTONIC_CLOCK = 149,
    _SC_MULTI_PROCESS = 150,
    _SC_SINGLE_PROCESS = 151,
    _SC_NETWORKING = 152,
    _SC_READER_WRITER_LOCKS = 153,
    _SC_SPIN_LOCKS = 154,
    _SC_REGEXP = 155,
    _SC_REGEX_VERSION = 156,
    _SC_SHELL = 157,
    _SC_SIGNALS = 158,
    _SC_SPAWN = 159,
    _SC_SPORADIC_SERVER = 160,
    _SC_THREAD_SPORADIC_SERVER = 161,
    _SC_SYSTEM_DATABASE = 162,
    _SC_SYSTEM_DATABASE_R = 163,
    _SC_TIMEOUTS = 164,
    _SC_TYPED_MEMORY_OBJECTS = 165,
    _SC_USER_GROUPS = 166,
    _SC_USER_GROUPS_R = 167,
    _SC_2_PBS = 168,
    _SC_2_PBS_ACCOUNTING = 169,
    _SC_2_PBS_LOCATE = 170,
    _SC_2_PBS_MESSAGE = 171,
    _SC_2_PBS_TRACK = 172,
    _SC_SYMLOOP_MAX = 173,
    _SC_STREAMS = 174,
    _SC_2_PBS_CHECKPOINT = 175,
    _SC_V6_ILP32_OFF32 = 176,
    _SC_V6_ILP32_OFFBIG = 177,
    _SC_V6_LP64_OFF64 = 178,
    _SC_V6_LPBIG_OFFBIG = 179,
    _SC_HOST_NAME_MAX = 180,
    _SC_TRACE = 181,
    _SC_TRACE_EVENT_FILTER = 182,
    _SC_TRACE_INHERIT = 183,
    _SC_TRACE_LOG = 184,
    _SC_LEVEL1_ICACHE_SIZE = 185,
    _SC_LEVEL1_ICACHE_ASSOC = 186,
    _SC_LEVEL1_ICACHE_LINESIZE = 187,
    _SC_LEVEL1_DCACHE_SIZE = 188,
    _SC_LEVEL1_DCACHE_ASSOC = 189,
    _SC_LEVEL1_DCACHE_LINESIZE = 190,
    _SC_LEVEL2_CACHE_SIZE = 191,
    _SC_LEVEL2_CACHE_ASSOC = 192,
    _SC_LEVEL2_CACHE_LINESIZE = 193,
    _SC_LEVEL3_CACHE_SIZE = 194,
    _SC_LEVEL3_CACHE_ASSOC = 195,
    _SC_LEVEL3_CACHE_LINESIZE = 196,
    _SC_LEVEL4_CACHE_SIZE = 197,
    _SC_LEVEL4_CACHE_ASSOC = 198,
    _SC_LEVEL4_CACHE_LINESIZE = 199,
    _SC_IPV6 = 235,
    _SC_RAW_SOCKETS = 236,
    _SC_V7_ILP32_OFF32 = 237,
    _SC_V7_ILP32_OFFBIG = 238,
    _SC_V7_LP64_OFF64 = 239,
    _SC_V7_LPBIG_OFFBIG = 240,
    _SC_SS_REPL_MAX = 241,
    _SC_TRACE_EVENT_NAME_MAX = 242,
    _SC_TRACE_NAME_MAX = 243,
    _SC_TRACE_SYS_MAX = 244,
    _SC_TRACE_USER_EVENT_MAX = 245,
    _SC_XOPEN_STREAMS = 246,
    _SC_THREAD_ROBUST_PRIO_INHERIT = 247,
    _SC_THREAD_ROBUST_PRIO_PROTECT = 248,
}
pub const _CS_PATH: _bindgen_ty_5 = _bindgen_ty_5::_CS_PATH;
pub const _CS_V6_WIDTH_RESTRICTED_ENVS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_V6_WIDTH_RESTRICTED_ENVS;
pub const _CS_GNU_LIBC_VERSION: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_GNU_LIBC_VERSION;
pub const _CS_GNU_LIBPTHREAD_VERSION: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_GNU_LIBPTHREAD_VERSION;
pub const _CS_V5_WIDTH_RESTRICTED_ENVS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_V5_WIDTH_RESTRICTED_ENVS;
pub const _CS_V7_WIDTH_RESTRICTED_ENVS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_V7_WIDTH_RESTRICTED_ENVS;
pub const _CS_LFS_CFLAGS: _bindgen_ty_5 = _bindgen_ty_5::_CS_LFS_CFLAGS;
pub const _CS_LFS_LDFLAGS: _bindgen_ty_5 = _bindgen_ty_5::_CS_LFS_LDFLAGS;
pub const _CS_LFS_LIBS: _bindgen_ty_5 = _bindgen_ty_5::_CS_LFS_LIBS;
pub const _CS_LFS_LINTFLAGS: _bindgen_ty_5 = _bindgen_ty_5::_CS_LFS_LINTFLAGS;
pub const _CS_LFS64_CFLAGS: _bindgen_ty_5 = _bindgen_ty_5::_CS_LFS64_CFLAGS;
pub const _CS_LFS64_LDFLAGS: _bindgen_ty_5 = _bindgen_ty_5::_CS_LFS64_LDFLAGS;
pub const _CS_LFS64_LIBS: _bindgen_ty_5 = _bindgen_ty_5::_CS_LFS64_LIBS;
pub const _CS_LFS64_LINTFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_LFS64_LINTFLAGS;
pub const _CS_XBS5_ILP32_OFF32_CFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_XBS5_ILP32_OFF32_CFLAGS;
pub const _CS_XBS5_ILP32_OFF32_LDFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_XBS5_ILP32_OFF32_LDFLAGS;
pub const _CS_XBS5_ILP32_OFF32_LIBS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_XBS5_ILP32_OFF32_LIBS;
pub const _CS_XBS5_ILP32_OFF32_LINTFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_XBS5_ILP32_OFF32_LINTFLAGS;
pub const _CS_XBS5_ILP32_OFFBIG_CFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_XBS5_ILP32_OFFBIG_CFLAGS;
pub const _CS_XBS5_ILP32_OFFBIG_LDFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_XBS5_ILP32_OFFBIG_LDFLAGS;
pub const _CS_XBS5_ILP32_OFFBIG_LIBS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_XBS5_ILP32_OFFBIG_LIBS;
pub const _CS_XBS5_ILP32_OFFBIG_LINTFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_XBS5_ILP32_OFFBIG_LINTFLAGS;
pub const _CS_XBS5_LP64_OFF64_CFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_XBS5_LP64_OFF64_CFLAGS;
pub const _CS_XBS5_LP64_OFF64_LDFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_XBS5_LP64_OFF64_LDFLAGS;
pub const _CS_XBS5_LP64_OFF64_LIBS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_XBS5_LP64_OFF64_LIBS;
pub const _CS_XBS5_LP64_OFF64_LINTFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_XBS5_LP64_OFF64_LINTFLAGS;
pub const _CS_XBS5_LPBIG_OFFBIG_CFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_XBS5_LPBIG_OFFBIG_CFLAGS;
pub const _CS_XBS5_LPBIG_OFFBIG_LDFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_XBS5_LPBIG_OFFBIG_LDFLAGS;
pub const _CS_XBS5_LPBIG_OFFBIG_LIBS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_XBS5_LPBIG_OFFBIG_LIBS;
pub const _CS_XBS5_LPBIG_OFFBIG_LINTFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_XBS5_LPBIG_OFFBIG_LINTFLAGS;
pub const _CS_POSIX_V6_ILP32_OFF32_CFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V6_ILP32_OFF32_CFLAGS;
pub const _CS_POSIX_V6_ILP32_OFF32_LDFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V6_ILP32_OFF32_LDFLAGS;
pub const _CS_POSIX_V6_ILP32_OFF32_LIBS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V6_ILP32_OFF32_LIBS;
pub const _CS_POSIX_V6_ILP32_OFF32_LINTFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V6_ILP32_OFF32_LINTFLAGS;
pub const _CS_POSIX_V6_ILP32_OFFBIG_CFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V6_ILP32_OFFBIG_CFLAGS;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LIBS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V6_ILP32_OFFBIG_LIBS;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS;
pub const _CS_POSIX_V6_LP64_OFF64_CFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V6_LP64_OFF64_CFLAGS;
pub const _CS_POSIX_V6_LP64_OFF64_LDFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V6_LP64_OFF64_LDFLAGS;
pub const _CS_POSIX_V6_LP64_OFF64_LIBS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V6_LP64_OFF64_LIBS;
pub const _CS_POSIX_V6_LP64_OFF64_LINTFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V6_LP64_OFF64_LINTFLAGS;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LIBS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V6_LPBIG_OFFBIG_LIBS;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS;
pub const _CS_POSIX_V7_ILP32_OFF32_CFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V7_ILP32_OFF32_CFLAGS;
pub const _CS_POSIX_V7_ILP32_OFF32_LDFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V7_ILP32_OFF32_LDFLAGS;
pub const _CS_POSIX_V7_ILP32_OFF32_LIBS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V7_ILP32_OFF32_LIBS;
pub const _CS_POSIX_V7_ILP32_OFF32_LINTFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V7_ILP32_OFF32_LINTFLAGS;
pub const _CS_POSIX_V7_ILP32_OFFBIG_CFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V7_ILP32_OFFBIG_CFLAGS;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LIBS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V7_ILP32_OFFBIG_LIBS;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS;
pub const _CS_POSIX_V7_LP64_OFF64_CFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V7_LP64_OFF64_CFLAGS;
pub const _CS_POSIX_V7_LP64_OFF64_LDFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V7_LP64_OFF64_LDFLAGS;
pub const _CS_POSIX_V7_LP64_OFF64_LIBS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V7_LP64_OFF64_LIBS;
pub const _CS_POSIX_V7_LP64_OFF64_LINTFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V7_LP64_OFF64_LINTFLAGS;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LIBS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V7_LPBIG_OFFBIG_LIBS;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS: _bindgen_ty_5 =
    _bindgen_ty_5::_CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS;
pub const _CS_V6_ENV: _bindgen_ty_5 = _bindgen_ty_5::_CS_V6_ENV;
pub const _CS_V7_ENV: _bindgen_ty_5 = _bindgen_ty_5::_CS_V7_ENV;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _bindgen_ty_5 {
    _CS_PATH = 0,
    _CS_V6_WIDTH_RESTRICTED_ENVS = 1,
    _CS_GNU_LIBC_VERSION = 2,
    _CS_GNU_LIBPTHREAD_VERSION = 3,
    _CS_V5_WIDTH_RESTRICTED_ENVS = 4,
    _CS_V7_WIDTH_RESTRICTED_ENVS = 5,
    _CS_LFS_CFLAGS = 1000,
    _CS_LFS_LDFLAGS = 1001,
    _CS_LFS_LIBS = 1002,
    _CS_LFS_LINTFLAGS = 1003,
    _CS_LFS64_CFLAGS = 1004,
    _CS_LFS64_LDFLAGS = 1005,
    _CS_LFS64_LIBS = 1006,
    _CS_LFS64_LINTFLAGS = 1007,
    _CS_XBS5_ILP32_OFF32_CFLAGS = 1100,
    _CS_XBS5_ILP32_OFF32_LDFLAGS = 1101,
    _CS_XBS5_ILP32_OFF32_LIBS = 1102,
    _CS_XBS5_ILP32_OFF32_LINTFLAGS = 1103,
    _CS_XBS5_ILP32_OFFBIG_CFLAGS = 1104,
    _CS_XBS5_ILP32_OFFBIG_LDFLAGS = 1105,
    _CS_XBS5_ILP32_OFFBIG_LIBS = 1106,
    _CS_XBS5_ILP32_OFFBIG_LINTFLAGS = 1107,
    _CS_XBS5_LP64_OFF64_CFLAGS = 1108,
    _CS_XBS5_LP64_OFF64_LDFLAGS = 1109,
    _CS_XBS5_LP64_OFF64_LIBS = 1110,
    _CS_XBS5_LP64_OFF64_LINTFLAGS = 1111,
    _CS_XBS5_LPBIG_OFFBIG_CFLAGS = 1112,
    _CS_XBS5_LPBIG_OFFBIG_LDFLAGS = 1113,
    _CS_XBS5_LPBIG_OFFBIG_LIBS = 1114,
    _CS_XBS5_LPBIG_OFFBIG_LINTFLAGS = 1115,
    _CS_POSIX_V6_ILP32_OFF32_CFLAGS = 1116,
    _CS_POSIX_V6_ILP32_OFF32_LDFLAGS = 1117,
    _CS_POSIX_V6_ILP32_OFF32_LIBS = 1118,
    _CS_POSIX_V6_ILP32_OFF32_LINTFLAGS = 1119,
    _CS_POSIX_V6_ILP32_OFFBIG_CFLAGS = 1120,
    _CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS = 1121,
    _CS_POSIX_V6_ILP32_OFFBIG_LIBS = 1122,
    _CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS = 1123,
    _CS_POSIX_V6_LP64_OFF64_CFLAGS = 1124,
    _CS_POSIX_V6_LP64_OFF64_LDFLAGS = 1125,
    _CS_POSIX_V6_LP64_OFF64_LIBS = 1126,
    _CS_POSIX_V6_LP64_OFF64_LINTFLAGS = 1127,
    _CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS = 1128,
    _CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS = 1129,
    _CS_POSIX_V6_LPBIG_OFFBIG_LIBS = 1130,
    _CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS = 1131,
    _CS_POSIX_V7_ILP32_OFF32_CFLAGS = 1132,
    _CS_POSIX_V7_ILP32_OFF32_LDFLAGS = 1133,
    _CS_POSIX_V7_ILP32_OFF32_LIBS = 1134,
    _CS_POSIX_V7_ILP32_OFF32_LINTFLAGS = 1135,
    _CS_POSIX_V7_ILP32_OFFBIG_CFLAGS = 1136,
    _CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS = 1137,
    _CS_POSIX_V7_ILP32_OFFBIG_LIBS = 1138,
    _CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS = 1139,
    _CS_POSIX_V7_LP64_OFF64_CFLAGS = 1140,
    _CS_POSIX_V7_LP64_OFF64_LDFLAGS = 1141,
    _CS_POSIX_V7_LP64_OFF64_LIBS = 1142,
    _CS_POSIX_V7_LP64_OFF64_LINTFLAGS = 1143,
    _CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS = 1144,
    _CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS = 1145,
    _CS_POSIX_V7_LPBIG_OFFBIG_LIBS = 1146,
    _CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS = 1147,
    _CS_V6_ENV = 1148,
    _CS_V7_ENV = 1149,
}
extern "C" {
    pub fn pathconf(__path: *const ::std::os::raw::c_char,
                    __name: ::std::os::raw::c_int) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn fpathconf(__fd: ::std::os::raw::c_int,
                     __name: ::std::os::raw::c_int) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn sysconf(__name: ::std::os::raw::c_int) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn confstr(__name: ::std::os::raw::c_int,
                   __buf: *mut ::std::os::raw::c_char, __len: usize) -> usize;
}
extern "C" {
    pub fn getpid() -> __pid_t;
}
extern "C" {
    pub fn getppid() -> __pid_t;
}
extern "C" {
    pub fn getpgrp() -> __pid_t;
}
extern "C" {
    pub fn __getpgid(__pid: __pid_t) -> __pid_t;
}
extern "C" {
    pub fn getpgid(__pid: __pid_t) -> __pid_t;
}
extern "C" {
    pub fn setpgid(__pid: __pid_t, __pgid: __pid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setpgrp() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setsid() -> __pid_t;
}
extern "C" {
    pub fn getsid(__pid: __pid_t) -> __pid_t;
}
extern "C" {
    pub fn getuid() -> __uid_t;
}
extern "C" {
    pub fn geteuid() -> __uid_t;
}
extern "C" {
    pub fn getgid() -> __gid_t;
}
extern "C" {
    pub fn getegid() -> __gid_t;
}
extern "C" {
    pub fn getgroups(__size: ::std::os::raw::c_int, __list: *mut __gid_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setuid(__uid: __uid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setreuid(__ruid: __uid_t, __euid: __uid_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn seteuid(__uid: __uid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setgid(__gid: __gid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setregid(__rgid: __gid_t, __egid: __gid_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setegid(__gid: __gid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fork() -> __pid_t;
}
extern "C" {
    pub fn vfork() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ttyname(__fd: ::std::os::raw::c_int)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ttyname_r(__fd: ::std::os::raw::c_int,
                     __buf: *mut ::std::os::raw::c_char, __buflen: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isatty(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ttyslot() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn link(__from: *const ::std::os::raw::c_char,
                __to: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn linkat(__fromfd: ::std::os::raw::c_int,
                  __from: *const ::std::os::raw::c_char,
                  __tofd: ::std::os::raw::c_int,
                  __to: *const ::std::os::raw::c_char,
                  __flags: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn symlink(__from: *const ::std::os::raw::c_char,
                   __to: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn readlink(__path: *const ::std::os::raw::c_char,
                    __buf: *mut ::std::os::raw::c_char, __len: usize)
     -> isize;
}
extern "C" {
    pub fn symlinkat(__from: *const ::std::os::raw::c_char,
                     __tofd: ::std::os::raw::c_int,
                     __to: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn readlinkat(__fd: ::std::os::raw::c_int,
                      __path: *const ::std::os::raw::c_char,
                      __buf: *mut ::std::os::raw::c_char, __len: usize)
     -> isize;
}
extern "C" {
    pub fn unlink(__name: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn unlinkat(__fd: ::std::os::raw::c_int,
                    __name: *const ::std::os::raw::c_char,
                    __flag: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rmdir(__path: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tcgetpgrp(__fd: ::std::os::raw::c_int) -> __pid_t;
}
extern "C" {
    pub fn tcsetpgrp(__fd: ::std::os::raw::c_int, __pgrp_id: __pid_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getlogin() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn getlogin_r(__name: *mut ::std::os::raw::c_char, __name_len: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setlogin(__name: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "optarg"]
    pub static mut optarg: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "optind"]
    pub static mut optind: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "opterr"]
    pub static mut opterr: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "optopt"]
    pub static mut optopt: ::std::os::raw::c_int;
}
extern "C" {
    pub fn getopt(___argc: ::std::os::raw::c_int,
                  ___argv: *const *const ::std::os::raw::c_char,
                  __shortopts: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gethostname(__name: *mut ::std::os::raw::c_char, __len: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sethostname(__name: *const ::std::os::raw::c_char, __len: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sethostid(__id: ::std::os::raw::c_long) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getdomainname(__name: *mut ::std::os::raw::c_char, __len: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setdomainname(__name: *const ::std::os::raw::c_char, __len: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vhangup() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn revoke(__file: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn profil(__sample_buffer: *mut ::std::os::raw::c_ushort,
                  __size: usize, __offset: usize,
                  __scale: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn acct(__name: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getusershell() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn endusershell();
}
extern "C" {
    pub fn setusershell();
}
extern "C" {
    pub fn daemon(__nochdir: ::std::os::raw::c_int,
                  __noclose: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn chroot(__path: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getpass(__prompt: *const ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fsync(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gethostid() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn sync();
}
extern "C" {
    pub fn getpagesize() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getdtablesize() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn truncate(__file: *const ::std::os::raw::c_char, __length: __off_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftruncate(__fd: ::std::os::raw::c_int, __length: __off_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn brk(__addr: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sbrk(__delta: isize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn syscall(__sysno: ::std::os::raw::c_long, ...)
     -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn fdatasync(__fildes: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getentropy(__buffer: *mut ::std::os::raw::c_void, __length: usize)
     -> ::std::os::raw::c_int;
}
pub type float_t = f32;
pub type double_t = f64;
extern "C" {
    pub fn acos(__x: f64) -> f64;
}
extern "C" {
    pub fn __acos(__x: f64) -> f64;
}
extern "C" {
    pub fn asin(__x: f64) -> f64;
}
extern "C" {
    pub fn __asin(__x: f64) -> f64;
}
extern "C" {
    pub fn atan(__x: f64) -> f64;
}
extern "C" {
    pub fn __atan(__x: f64) -> f64;
}
extern "C" {
    pub fn atan2(__y: f64, __x: f64) -> f64;
}
extern "C" {
    pub fn __atan2(__y: f64, __x: f64) -> f64;
}
extern "C" {
    pub fn cos(__x: f64) -> f64;
}
extern "C" {
    pub fn __cos(__x: f64) -> f64;
}
extern "C" {
    pub fn sin(__x: f64) -> f64;
}
extern "C" {
    pub fn __sin(__x: f64) -> f64;
}
extern "C" {
    pub fn tan(__x: f64) -> f64;
}
extern "C" {
    pub fn __tan(__x: f64) -> f64;
}
extern "C" {
    pub fn cosh(__x: f64) -> f64;
}
extern "C" {
    pub fn __cosh(__x: f64) -> f64;
}
extern "C" {
    pub fn sinh(__x: f64) -> f64;
}
extern "C" {
    pub fn __sinh(__x: f64) -> f64;
}
extern "C" {
    pub fn tanh(__x: f64) -> f64;
}
extern "C" {
    pub fn __tanh(__x: f64) -> f64;
}
extern "C" {
    pub fn acosh(__x: f64) -> f64;
}
extern "C" {
    pub fn __acosh(__x: f64) -> f64;
}
extern "C" {
    pub fn asinh(__x: f64) -> f64;
}
extern "C" {
    pub fn __asinh(__x: f64) -> f64;
}
extern "C" {
    pub fn atanh(__x: f64) -> f64;
}
extern "C" {
    pub fn __atanh(__x: f64) -> f64;
}
extern "C" {
    pub fn exp(__x: f64) -> f64;
}
extern "C" {
    pub fn __exp(__x: f64) -> f64;
}
extern "C" {
    pub fn frexp(__x: f64, __exponent: *mut ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn __frexp(__x: f64, __exponent: *mut ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn ldexp(__x: f64, __exponent: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn __ldexp(__x: f64, __exponent: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn log(__x: f64) -> f64;
}
extern "C" {
    pub fn __log(__x: f64) -> f64;
}
extern "C" {
    pub fn log10(__x: f64) -> f64;
}
extern "C" {
    pub fn __log10(__x: f64) -> f64;
}
extern "C" {
    pub fn modf(__x: f64, __iptr: *mut f64) -> f64;
}
extern "C" {
    pub fn __modf(__x: f64, __iptr: *mut f64) -> f64;
}
extern "C" {
    pub fn expm1(__x: f64) -> f64;
}
extern "C" {
    pub fn __expm1(__x: f64) -> f64;
}
extern "C" {
    pub fn log1p(__x: f64) -> f64;
}
extern "C" {
    pub fn __log1p(__x: f64) -> f64;
}
extern "C" {
    pub fn logb(__x: f64) -> f64;
}
extern "C" {
    pub fn __logb(__x: f64) -> f64;
}
extern "C" {
    pub fn exp2(__x: f64) -> f64;
}
extern "C" {
    pub fn __exp2(__x: f64) -> f64;
}
extern "C" {
    pub fn log2(__x: f64) -> f64;
}
extern "C" {
    pub fn __log2(__x: f64) -> f64;
}
extern "C" {
    pub fn pow(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __pow(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn sqrt(__x: f64) -> f64;
}
extern "C" {
    pub fn __sqrt(__x: f64) -> f64;
}
extern "C" {
    pub fn hypot(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __hypot(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn cbrt(__x: f64) -> f64;
}
extern "C" {
    pub fn __cbrt(__x: f64) -> f64;
}
extern "C" {
    pub fn ceil(__x: f64) -> f64;
}
extern "C" {
    pub fn __ceil(__x: f64) -> f64;
}
extern "C" {
    pub fn fabs(__x: f64) -> f64;
}
extern "C" {
    pub fn __fabs(__x: f64) -> f64;
}
extern "C" {
    pub fn floor(__x: f64) -> f64;
}
extern "C" {
    pub fn __floor(__x: f64) -> f64;
}
extern "C" {
    pub fn fmod(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fmod(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __isinf(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __finite(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isinf(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn finite(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn drem(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __drem(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn significand(__x: f64) -> f64;
}
extern "C" {
    pub fn __significand(__x: f64) -> f64;
}
extern "C" {
    pub fn copysign(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __copysign(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn nan(__tagb: *const ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn __nan(__tagb: *const ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn __isnan(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isnan(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn j0(arg1: f64) -> f64;
}
extern "C" {
    pub fn __j0(arg1: f64) -> f64;
}
extern "C" {
    pub fn j1(arg1: f64) -> f64;
}
extern "C" {
    pub fn __j1(arg1: f64) -> f64;
}
extern "C" {
    pub fn jn(arg1: ::std::os::raw::c_int, arg2: f64) -> f64;
}
extern "C" {
    pub fn __jn(arg1: ::std::os::raw::c_int, arg2: f64) -> f64;
}
extern "C" {
    pub fn y0(arg1: f64) -> f64;
}
extern "C" {
    pub fn __y0(arg1: f64) -> f64;
}
extern "C" {
    pub fn y1(arg1: f64) -> f64;
}
extern "C" {
    pub fn __y1(arg1: f64) -> f64;
}
extern "C" {
    pub fn yn(arg1: ::std::os::raw::c_int, arg2: f64) -> f64;
}
extern "C" {
    pub fn __yn(arg1: ::std::os::raw::c_int, arg2: f64) -> f64;
}
extern "C" {
    pub fn erf(arg1: f64) -> f64;
}
extern "C" {
    pub fn __erf(arg1: f64) -> f64;
}
extern "C" {
    pub fn erfc(arg1: f64) -> f64;
}
extern "C" {
    pub fn __erfc(arg1: f64) -> f64;
}
extern "C" {
    pub fn lgamma(arg1: f64) -> f64;
}
extern "C" {
    pub fn __lgamma(arg1: f64) -> f64;
}
extern "C" {
    pub fn tgamma(arg1: f64) -> f64;
}
extern "C" {
    pub fn __tgamma(arg1: f64) -> f64;
}
extern "C" {
    pub fn gamma(arg1: f64) -> f64;
}
extern "C" {
    pub fn __gamma(arg1: f64) -> f64;
}
extern "C" {
    pub fn lgamma_r(arg1: f64, __signgamp: *mut ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn __lgamma_r(arg1: f64, __signgamp: *mut ::std::os::raw::c_int)
     -> f64;
}
extern "C" {
    pub fn rint(__x: f64) -> f64;
}
extern "C" {
    pub fn __rint(__x: f64) -> f64;
}
extern "C" {
    pub fn nextafter(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __nextafter(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn nexttoward(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __nexttoward(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn remainder(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __remainder(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn scalbn(__x: f64, __n: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn __scalbn(__x: f64, __n: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn ilogb(__x: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __ilogb(__x: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scalbln(__x: f64, __n: ::std::os::raw::c_long) -> f64;
}
extern "C" {
    pub fn __scalbln(__x: f64, __n: ::std::os::raw::c_long) -> f64;
}
extern "C" {
    pub fn nearbyint(__x: f64) -> f64;
}
extern "C" {
    pub fn __nearbyint(__x: f64) -> f64;
}
extern "C" {
    pub fn round(__x: f64) -> f64;
}
extern "C" {
    pub fn __round(__x: f64) -> f64;
}
extern "C" {
    pub fn trunc(__x: f64) -> f64;
}
extern "C" {
    pub fn __trunc(__x: f64) -> f64;
}
extern "C" {
    pub fn remquo(__x: f64, __y: f64, __quo: *mut ::std::os::raw::c_int)
     -> f64;
}
extern "C" {
    pub fn __remquo(__x: f64, __y: f64, __quo: *mut ::std::os::raw::c_int)
     -> f64;
}
extern "C" {
    pub fn lrint(__x: f64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __lrint(__x: f64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llrint(__x: f64) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn __llrint(__x: f64) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn lround(__x: f64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __lround(__x: f64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llround(__x: f64) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn __llround(__x: f64) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn fdim(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fdim(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn fmax(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fmax(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn fmin(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fmin(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fpclassify(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __signbit(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fma(__x: f64, __y: f64, __z: f64) -> f64;
}
extern "C" {
    pub fn __fma(__x: f64, __y: f64, __z: f64) -> f64;
}
extern "C" {
    pub fn scalb(__x: f64, __n: f64) -> f64;
}
extern "C" {
    pub fn __scalb(__x: f64, __n: f64) -> f64;
}
extern "C" {
    pub fn acosf(__x: f32) -> f32;
}
extern "C" {
    pub fn __acosf(__x: f32) -> f32;
}
extern "C" {
    pub fn asinf(__x: f32) -> f32;
}
extern "C" {
    pub fn __asinf(__x: f32) -> f32;
}
extern "C" {
    pub fn atanf(__x: f32) -> f32;
}
extern "C" {
    pub fn __atanf(__x: f32) -> f32;
}
extern "C" {
    pub fn atan2f(__y: f32, __x: f32) -> f32;
}
extern "C" {
    pub fn __atan2f(__y: f32, __x: f32) -> f32;
}
extern "C" {
    pub fn cosf(__x: f32) -> f32;
}
extern "C" {
    pub fn __cosf(__x: f32) -> f32;
}
extern "C" {
    pub fn sinf(__x: f32) -> f32;
}
extern "C" {
    pub fn __sinf(__x: f32) -> f32;
}
extern "C" {
    pub fn tanf(__x: f32) -> f32;
}
extern "C" {
    pub fn __tanf(__x: f32) -> f32;
}
extern "C" {
    pub fn coshf(__x: f32) -> f32;
}
extern "C" {
    pub fn __coshf(__x: f32) -> f32;
}
extern "C" {
    pub fn sinhf(__x: f32) -> f32;
}
extern "C" {
    pub fn __sinhf(__x: f32) -> f32;
}
extern "C" {
    pub fn tanhf(__x: f32) -> f32;
}
extern "C" {
    pub fn __tanhf(__x: f32) -> f32;
}
extern "C" {
    pub fn acoshf(__x: f32) -> f32;
}
extern "C" {
    pub fn __acoshf(__x: f32) -> f32;
}
extern "C" {
    pub fn asinhf(__x: f32) -> f32;
}
extern "C" {
    pub fn __asinhf(__x: f32) -> f32;
}
extern "C" {
    pub fn atanhf(__x: f32) -> f32;
}
extern "C" {
    pub fn __atanhf(__x: f32) -> f32;
}
extern "C" {
    pub fn expf(__x: f32) -> f32;
}
extern "C" {
    pub fn __expf(__x: f32) -> f32;
}
extern "C" {
    pub fn frexpf(__x: f32, __exponent: *mut ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn __frexpf(__x: f32, __exponent: *mut ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn ldexpf(__x: f32, __exponent: ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn __ldexpf(__x: f32, __exponent: ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn logf(__x: f32) -> f32;
}
extern "C" {
    pub fn __logf(__x: f32) -> f32;
}
extern "C" {
    pub fn log10f(__x: f32) -> f32;
}
extern "C" {
    pub fn __log10f(__x: f32) -> f32;
}
extern "C" {
    pub fn modff(__x: f32, __iptr: *mut f32) -> f32;
}
extern "C" {
    pub fn __modff(__x: f32, __iptr: *mut f32) -> f32;
}
extern "C" {
    pub fn expm1f(__x: f32) -> f32;
}
extern "C" {
    pub fn __expm1f(__x: f32) -> f32;
}
extern "C" {
    pub fn log1pf(__x: f32) -> f32;
}
extern "C" {
    pub fn __log1pf(__x: f32) -> f32;
}
extern "C" {
    pub fn logbf(__x: f32) -> f32;
}
extern "C" {
    pub fn __logbf(__x: f32) -> f32;
}
extern "C" {
    pub fn exp2f(__x: f32) -> f32;
}
extern "C" {
    pub fn __exp2f(__x: f32) -> f32;
}
extern "C" {
    pub fn log2f(__x: f32) -> f32;
}
extern "C" {
    pub fn __log2f(__x: f32) -> f32;
}
extern "C" {
    pub fn powf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __powf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn sqrtf(__x: f32) -> f32;
}
extern "C" {
    pub fn __sqrtf(__x: f32) -> f32;
}
extern "C" {
    pub fn hypotf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __hypotf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn cbrtf(__x: f32) -> f32;
}
extern "C" {
    pub fn __cbrtf(__x: f32) -> f32;
}
extern "C" {
    pub fn ceilf(__x: f32) -> f32;
}
extern "C" {
    pub fn __ceilf(__x: f32) -> f32;
}
extern "C" {
    pub fn fabsf(__x: f32) -> f32;
}
extern "C" {
    pub fn __fabsf(__x: f32) -> f32;
}
extern "C" {
    pub fn floorf(__x: f32) -> f32;
}
extern "C" {
    pub fn __floorf(__x: f32) -> f32;
}
extern "C" {
    pub fn fmodf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __fmodf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __isinff(__value: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __finitef(__value: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isinff(__value: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn finitef(__value: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn dremf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __dremf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn significandf(__x: f32) -> f32;
}
extern "C" {
    pub fn __significandf(__x: f32) -> f32;
}
extern "C" {
    pub fn copysignf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __copysignf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn nanf(__tagb: *const ::std::os::raw::c_char) -> f32;
}
extern "C" {
    pub fn __nanf(__tagb: *const ::std::os::raw::c_char) -> f32;
}
extern "C" {
    pub fn __isnanf(__value: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isnanf(__value: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn j0f(arg1: f32) -> f32;
}
extern "C" {
    pub fn __j0f(arg1: f32) -> f32;
}
extern "C" {
    pub fn j1f(arg1: f32) -> f32;
}
extern "C" {
    pub fn __j1f(arg1: f32) -> f32;
}
extern "C" {
    pub fn jnf(arg1: ::std::os::raw::c_int, arg2: f32) -> f32;
}
extern "C" {
    pub fn __jnf(arg1: ::std::os::raw::c_int, arg2: f32) -> f32;
}
extern "C" {
    pub fn y0f(arg1: f32) -> f32;
}
extern "C" {
    pub fn __y0f(arg1: f32) -> f32;
}
extern "C" {
    pub fn y1f(arg1: f32) -> f32;
}
extern "C" {
    pub fn __y1f(arg1: f32) -> f32;
}
extern "C" {
    pub fn ynf(arg1: ::std::os::raw::c_int, arg2: f32) -> f32;
}
extern "C" {
    pub fn __ynf(arg1: ::std::os::raw::c_int, arg2: f32) -> f32;
}
extern "C" {
    pub fn erff(arg1: f32) -> f32;
}
extern "C" {
    pub fn __erff(arg1: f32) -> f32;
}
extern "C" {
    pub fn erfcf(arg1: f32) -> f32;
}
extern "C" {
    pub fn __erfcf(arg1: f32) -> f32;
}
extern "C" {
    pub fn lgammaf(arg1: f32) -> f32;
}
extern "C" {
    pub fn __lgammaf(arg1: f32) -> f32;
}
extern "C" {
    pub fn tgammaf(arg1: f32) -> f32;
}
extern "C" {
    pub fn __tgammaf(arg1: f32) -> f32;
}
extern "C" {
    pub fn gammaf(arg1: f32) -> f32;
}
extern "C" {
    pub fn __gammaf(arg1: f32) -> f32;
}
extern "C" {
    pub fn lgammaf_r(arg1: f32, __signgamp: *mut ::std::os::raw::c_int)
     -> f32;
}
extern "C" {
    pub fn __lgammaf_r(arg1: f32, __signgamp: *mut ::std::os::raw::c_int)
     -> f32;
}
extern "C" {
    pub fn rintf(__x: f32) -> f32;
}
extern "C" {
    pub fn __rintf(__x: f32) -> f32;
}
extern "C" {
    pub fn nextafterf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __nextafterf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn nexttowardf(__x: f32, __y: f64) -> f32;
}
extern "C" {
    pub fn __nexttowardf(__x: f32, __y: f64) -> f32;
}
extern "C" {
    pub fn remainderf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __remainderf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn scalbnf(__x: f32, __n: ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn __scalbnf(__x: f32, __n: ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn ilogbf(__x: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __ilogbf(__x: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scalblnf(__x: f32, __n: ::std::os::raw::c_long) -> f32;
}
extern "C" {
    pub fn __scalblnf(__x: f32, __n: ::std::os::raw::c_long) -> f32;
}
extern "C" {
    pub fn nearbyintf(__x: f32) -> f32;
}
extern "C" {
    pub fn __nearbyintf(__x: f32) -> f32;
}
extern "C" {
    pub fn roundf(__x: f32) -> f32;
}
extern "C" {
    pub fn __roundf(__x: f32) -> f32;
}
extern "C" {
    pub fn truncf(__x: f32) -> f32;
}
extern "C" {
    pub fn __truncf(__x: f32) -> f32;
}
extern "C" {
    pub fn remquof(__x: f32, __y: f32, __quo: *mut ::std::os::raw::c_int)
     -> f32;
}
extern "C" {
    pub fn __remquof(__x: f32, __y: f32, __quo: *mut ::std::os::raw::c_int)
     -> f32;
}
extern "C" {
    pub fn lrintf(__x: f32) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __lrintf(__x: f32) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llrintf(__x: f32) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn __llrintf(__x: f32) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn lroundf(__x: f32) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __lroundf(__x: f32) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llroundf(__x: f32) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn __llroundf(__x: f32) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn fdimf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __fdimf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn fmaxf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __fmaxf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn fminf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __fminf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __fpclassifyf(__value: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __signbitf(__value: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmaf(__x: f32, __y: f32, __z: f32) -> f32;
}
extern "C" {
    pub fn __fmaf(__x: f32, __y: f32, __z: f32) -> f32;
}
extern "C" {
    pub fn scalbf(__x: f32, __n: f32) -> f32;
}
extern "C" {
    pub fn __scalbf(__x: f32, __n: f32) -> f32;
}
extern "C" {
    pub fn acosl(__x: f64) -> f64;
}
extern "C" {
    pub fn __acosl(__x: f64) -> f64;
}
extern "C" {
    pub fn asinl(__x: f64) -> f64;
}
extern "C" {
    pub fn __asinl(__x: f64) -> f64;
}
extern "C" {
    pub fn atanl(__x: f64) -> f64;
}
extern "C" {
    pub fn __atanl(__x: f64) -> f64;
}
extern "C" {
    pub fn atan2l(__y: f64, __x: f64) -> f64;
}
extern "C" {
    pub fn __atan2l(__y: f64, __x: f64) -> f64;
}
extern "C" {
    pub fn cosl(__x: f64) -> f64;
}
extern "C" {
    pub fn __cosl(__x: f64) -> f64;
}
extern "C" {
    pub fn sinl(__x: f64) -> f64;
}
extern "C" {
    pub fn __sinl(__x: f64) -> f64;
}
extern "C" {
    pub fn tanl(__x: f64) -> f64;
}
extern "C" {
    pub fn __tanl(__x: f64) -> f64;
}
extern "C" {
    pub fn coshl(__x: f64) -> f64;
}
extern "C" {
    pub fn __coshl(__x: f64) -> f64;
}
extern "C" {
    pub fn sinhl(__x: f64) -> f64;
}
extern "C" {
    pub fn __sinhl(__x: f64) -> f64;
}
extern "C" {
    pub fn tanhl(__x: f64) -> f64;
}
extern "C" {
    pub fn __tanhl(__x: f64) -> f64;
}
extern "C" {
    pub fn acoshl(__x: f64) -> f64;
}
extern "C" {
    pub fn __acoshl(__x: f64) -> f64;
}
extern "C" {
    pub fn asinhl(__x: f64) -> f64;
}
extern "C" {
    pub fn __asinhl(__x: f64) -> f64;
}
extern "C" {
    pub fn atanhl(__x: f64) -> f64;
}
extern "C" {
    pub fn __atanhl(__x: f64) -> f64;
}
extern "C" {
    pub fn expl(__x: f64) -> f64;
}
extern "C" {
    pub fn __expl(__x: f64) -> f64;
}
extern "C" {
    pub fn frexpl(__x: f64, __exponent: *mut ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn __frexpl(__x: f64, __exponent: *mut ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn ldexpl(__x: f64, __exponent: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn __ldexpl(__x: f64, __exponent: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn logl(__x: f64) -> f64;
}
extern "C" {
    pub fn __logl(__x: f64) -> f64;
}
extern "C" {
    pub fn log10l(__x: f64) -> f64;
}
extern "C" {
    pub fn __log10l(__x: f64) -> f64;
}
extern "C" {
    pub fn modfl(__x: f64, __iptr: *mut f64) -> f64;
}
extern "C" {
    pub fn __modfl(__x: f64, __iptr: *mut f64) -> f64;
}
extern "C" {
    pub fn expm1l(__x: f64) -> f64;
}
extern "C" {
    pub fn __expm1l(__x: f64) -> f64;
}
extern "C" {
    pub fn log1pl(__x: f64) -> f64;
}
extern "C" {
    pub fn __log1pl(__x: f64) -> f64;
}
extern "C" {
    pub fn logbl(__x: f64) -> f64;
}
extern "C" {
    pub fn __logbl(__x: f64) -> f64;
}
extern "C" {
    pub fn exp2l(__x: f64) -> f64;
}
extern "C" {
    pub fn __exp2l(__x: f64) -> f64;
}
extern "C" {
    pub fn log2l(__x: f64) -> f64;
}
extern "C" {
    pub fn __log2l(__x: f64) -> f64;
}
extern "C" {
    pub fn powl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __powl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn sqrtl(__x: f64) -> f64;
}
extern "C" {
    pub fn __sqrtl(__x: f64) -> f64;
}
extern "C" {
    pub fn hypotl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __hypotl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn cbrtl(__x: f64) -> f64;
}
extern "C" {
    pub fn __cbrtl(__x: f64) -> f64;
}
extern "C" {
    pub fn ceill(__x: f64) -> f64;
}
extern "C" {
    pub fn __ceill(__x: f64) -> f64;
}
extern "C" {
    pub fn fabsl(__x: f64) -> f64;
}
extern "C" {
    pub fn __fabsl(__x: f64) -> f64;
}
extern "C" {
    pub fn floorl(__x: f64) -> f64;
}
extern "C" {
    pub fn __floorl(__x: f64) -> f64;
}
extern "C" {
    pub fn fmodl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fmodl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __isinfl(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __finitel(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isinfl(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn finitel(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn dreml(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __dreml(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn significandl(__x: f64) -> f64;
}
extern "C" {
    pub fn __significandl(__x: f64) -> f64;
}
extern "C" {
    pub fn copysignl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __copysignl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn nanl(__tagb: *const ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn __nanl(__tagb: *const ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn __isnanl(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isnanl(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn j0l(arg1: f64) -> f64;
}
extern "C" {
    pub fn __j0l(arg1: f64) -> f64;
}
extern "C" {
    pub fn j1l(arg1: f64) -> f64;
}
extern "C" {
    pub fn __j1l(arg1: f64) -> f64;
}
extern "C" {
    pub fn jnl(arg1: ::std::os::raw::c_int, arg2: f64) -> f64;
}
extern "C" {
    pub fn __jnl(arg1: ::std::os::raw::c_int, arg2: f64) -> f64;
}
extern "C" {
    pub fn y0l(arg1: f64) -> f64;
}
extern "C" {
    pub fn __y0l(arg1: f64) -> f64;
}
extern "C" {
    pub fn y1l(arg1: f64) -> f64;
}
extern "C" {
    pub fn __y1l(arg1: f64) -> f64;
}
extern "C" {
    pub fn ynl(arg1: ::std::os::raw::c_int, arg2: f64) -> f64;
}
extern "C" {
    pub fn __ynl(arg1: ::std::os::raw::c_int, arg2: f64) -> f64;
}
extern "C" {
    pub fn erfl(arg1: f64) -> f64;
}
extern "C" {
    pub fn __erfl(arg1: f64) -> f64;
}
extern "C" {
    pub fn erfcl(arg1: f64) -> f64;
}
extern "C" {
    pub fn __erfcl(arg1: f64) -> f64;
}
extern "C" {
    pub fn lgammal(arg1: f64) -> f64;
}
extern "C" {
    pub fn __lgammal(arg1: f64) -> f64;
}
extern "C" {
    pub fn tgammal(arg1: f64) -> f64;
}
extern "C" {
    pub fn __tgammal(arg1: f64) -> f64;
}
extern "C" {
    pub fn gammal(arg1: f64) -> f64;
}
extern "C" {
    pub fn __gammal(arg1: f64) -> f64;
}
extern "C" {
    pub fn lgammal_r(arg1: f64, __signgamp: *mut ::std::os::raw::c_int)
     -> f64;
}
extern "C" {
    pub fn __lgammal_r(arg1: f64, __signgamp: *mut ::std::os::raw::c_int)
     -> f64;
}
extern "C" {
    pub fn rintl(__x: f64) -> f64;
}
extern "C" {
    pub fn __rintl(__x: f64) -> f64;
}
extern "C" {
    pub fn nextafterl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __nextafterl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn nexttowardl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __nexttowardl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn remainderl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __remainderl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn scalbnl(__x: f64, __n: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn __scalbnl(__x: f64, __n: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn ilogbl(__x: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __ilogbl(__x: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scalblnl(__x: f64, __n: ::std::os::raw::c_long) -> f64;
}
extern "C" {
    pub fn __scalblnl(__x: f64, __n: ::std::os::raw::c_long) -> f64;
}
extern "C" {
    pub fn nearbyintl(__x: f64) -> f64;
}
extern "C" {
    pub fn __nearbyintl(__x: f64) -> f64;
}
extern "C" {
    pub fn roundl(__x: f64) -> f64;
}
extern "C" {
    pub fn __roundl(__x: f64) -> f64;
}
extern "C" {
    pub fn truncl(__x: f64) -> f64;
}
extern "C" {
    pub fn __truncl(__x: f64) -> f64;
}
extern "C" {
    pub fn remquol(__x: f64, __y: f64, __quo: *mut ::std::os::raw::c_int)
     -> f64;
}
extern "C" {
    pub fn __remquol(__x: f64, __y: f64, __quo: *mut ::std::os::raw::c_int)
     -> f64;
}
extern "C" {
    pub fn lrintl(__x: f64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __lrintl(__x: f64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llrintl(__x: f64) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn __llrintl(__x: f64) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn lroundl(__x: f64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __lroundl(__x: f64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llroundl(__x: f64) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn __llroundl(__x: f64) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn fdiml(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fdiml(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn fmaxl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fmaxl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn fminl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fminl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fpclassifyl(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __signbitl(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmal(__x: f64, __y: f64, __z: f64) -> f64;
}
extern "C" {
    pub fn __fmal(__x: f64, __y: f64, __z: f64) -> f64;
}
extern "C" {
    pub fn scalbl(__x: f64, __n: f64) -> f64;
}
extern "C" {
    pub fn __scalbl(__x: f64, __n: f64) -> f64;
}
extern "C" {
    #[link_name = "signgam"]
    pub static mut signgam: ::std::os::raw::c_int;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _bindgen_ty_6 {
    FP_NAN = 0,
    FP_INFINITE = 1,
    FP_ZERO = 2,
    FP_SUBNORMAL = 3,
    FP_NORMAL = 4,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _LIB_VERSION_TYPE {
    _IEEE_ = -1,
    _SVID_ = 0,
    _XOPEN_ = 1,
    _POSIX_ = 2,
    _ISOC_ = 3,
}
extern "C" {
    #[link_name = "_LIB_VERSION"]
    pub static mut _LIB_VERSION: _LIB_VERSION_TYPE;
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct exception {
    pub type_: ::std::os::raw::c_int,
    pub name: *mut ::std::os::raw::c_char,
    pub arg1: f64,
    pub arg2: f64,
    pub retval: f64,
}
#[test]
fn bindgen_test_layout_exception() {
    assert_eq!(::std::mem::size_of::<exception>() , 40usize , concat ! (
               "Size of: " , stringify ! ( exception ) ));
    assert_eq! (::std::mem::align_of::<exception>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( exception ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const exception ) ) . type_ as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( exception ) , "::" ,
                stringify ! ( type_ ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const exception ) ) . name as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( exception ) , "::" ,
                stringify ! ( name ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const exception ) ) . arg1 as * const _ as
                usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( exception ) , "::" ,
                stringify ! ( arg1 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const exception ) ) . arg2 as * const _ as
                usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( exception ) , "::" ,
                stringify ! ( arg2 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const exception ) ) . retval as * const _ as
                usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( exception ) , "::" ,
                stringify ! ( retval ) ));
}
impl Clone for exception {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    pub fn matherr(__exc: *mut exception) -> ::std::os::raw::c_int;
}
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
#[test]
fn bindgen_test_layout_gravity_class_s() {
    assert_eq!(::std::mem::size_of::<gravity_class_s>() , 88usize , concat ! (
               "Size of: " , stringify ! ( gravity_class_s ) ));
    assert_eq! (::std::mem::align_of::<gravity_class_s>() , 8usize , concat !
                ( "Alignment of " , stringify ! ( gravity_class_s ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_class_s ) ) . isa as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_class_s ) ,
                "::" , stringify ! ( isa ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_class_s ) ) . gc as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_class_s ) ,
                "::" , stringify ! ( gc ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_class_s ) ) . objclass as * const
                _ as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_class_s ) ,
                "::" , stringify ! ( objclass ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_class_s ) ) . identifier as *
                const _ as usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_class_s ) ,
                "::" , stringify ! ( identifier ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_class_s ) ) . has_outer as *
                const _ as usize } , 40usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_class_s ) ,
                "::" , stringify ! ( has_outer ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_class_s ) ) . is_struct as *
                const _ as usize } , 41usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_class_s ) ,
                "::" , stringify ! ( is_struct ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_class_s ) ) . is_inited as *
                const _ as usize } , 42usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_class_s ) ,
                "::" , stringify ! ( is_inited ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_class_s ) ) . unused as * const _
                as usize } , 43usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_class_s ) ,
                "::" , stringify ! ( unused ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_class_s ) ) . xdata as * const _
                as usize } , 48usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_class_s ) ,
                "::" , stringify ! ( xdata ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_class_s ) ) . superclass as *
                const _ as usize } , 56usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_class_s ) ,
                "::" , stringify ! ( superclass ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_class_s ) ) . htable as * const _
                as usize } , 64usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_class_s ) ,
                "::" , stringify ! ( htable ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_class_s ) ) . nivars as * const _
                as usize } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_class_s ) ,
                "::" , stringify ! ( nivars ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_class_s ) ) . ivars as * const _
                as usize } , 80usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_class_s ) ,
                "::" , stringify ! ( ivars ) ));
}
impl Clone for gravity_class_s {
    fn clone(&self) -> Self { *self }
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
#[test]
fn bindgen_test_layout_gravity_value_t__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<gravity_value_t__bindgen_ty_1>() , 8usize
               , concat ! (
               "Size of: " , stringify ! ( gravity_value_t__bindgen_ty_1 ) ));
    assert_eq! (::std::mem::align_of::<gravity_value_t__bindgen_ty_1>() ,
                8usize , concat ! (
                "Alignment of " , stringify ! ( gravity_value_t__bindgen_ty_1
                ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_value_t__bindgen_ty_1 ) ) . n as
                * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                gravity_value_t__bindgen_ty_1 ) , "::" , stringify ! ( n ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_value_t__bindgen_ty_1 ) ) . f as
                * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                gravity_value_t__bindgen_ty_1 ) , "::" , stringify ! ( f ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_value_t__bindgen_ty_1 ) ) . p as
                * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                gravity_value_t__bindgen_ty_1 ) , "::" , stringify ! ( p ) ));
}
impl Clone for gravity_value_t__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_gravity_value_t() {
    assert_eq!(::std::mem::size_of::<gravity_value_t>() , 16usize , concat ! (
               "Size of: " , stringify ! ( gravity_value_t ) ));
    assert_eq! (::std::mem::align_of::<gravity_value_t>() , 8usize , concat !
                ( "Alignment of " , stringify ! ( gravity_value_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_value_t ) ) . isa as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_value_t ) ,
                "::" , stringify ! ( isa ) ));
}
impl Clone for gravity_value_t {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    #[link_name = "gravity_class_object"]
    pub static mut gravity_class_object: *mut gravity_class_t;
}
extern "C" {
    #[link_name = "gravity_class_bool"]
    pub static mut gravity_class_bool: *mut gravity_class_t;
}
extern "C" {
    #[link_name = "gravity_class_null"]
    pub static mut gravity_class_null: *mut gravity_class_t;
}
extern "C" {
    #[link_name = "gravity_class_int"]
    pub static mut gravity_class_int: *mut gravity_class_t;
}
extern "C" {
    #[link_name = "gravity_class_float"]
    pub static mut gravity_class_float: *mut gravity_class_t;
}
extern "C" {
    #[link_name = "gravity_class_function"]
    pub static mut gravity_class_function: *mut gravity_class_t;
}
extern "C" {
    #[link_name = "gravity_class_closure"]
    pub static mut gravity_class_closure: *mut gravity_class_t;
}
extern "C" {
    #[link_name = "gravity_class_fiber"]
    pub static mut gravity_class_fiber: *mut gravity_class_t;
}
extern "C" {
    #[link_name = "gravity_class_class"]
    pub static mut gravity_class_class: *mut gravity_class_t;
}
extern "C" {
    #[link_name = "gravity_class_string"]
    pub static mut gravity_class_string: *mut gravity_class_t;
}
extern "C" {
    #[link_name = "gravity_class_instance"]
    pub static mut gravity_class_instance: *mut gravity_class_t;
}
extern "C" {
    #[link_name = "gravity_class_list"]
    pub static mut gravity_class_list: *mut gravity_class_t;
}
extern "C" {
    #[link_name = "gravity_class_map"]
    pub static mut gravity_class_map: *mut gravity_class_t;
}
extern "C" {
    #[link_name = "gravity_class_module"]
    pub static mut gravity_class_module: *mut gravity_class_t;
}
extern "C" {
    #[link_name = "gravity_class_range"]
    pub static mut gravity_class_range: *mut gravity_class_t;
}
extern "C" {
    #[link_name = "gravity_class_upvalue"]
    pub static mut gravity_class_upvalue: *mut gravity_class_t;
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_value_r {
    pub n: usize,
    pub m: usize,
    pub p: *mut gravity_value_t,
}
#[test]
fn bindgen_test_layout_gravity_value_r() {
    assert_eq!(::std::mem::size_of::<gravity_value_r>() , 24usize , concat ! (
               "Size of: " , stringify ! ( gravity_value_r ) ));
    assert_eq! (::std::mem::align_of::<gravity_value_r>() , 8usize , concat !
                ( "Alignment of " , stringify ! ( gravity_value_r ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_value_r ) ) . n as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_value_r ) ,
                "::" , stringify ! ( n ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_value_r ) ) . m as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_value_r ) ,
                "::" , stringify ! ( m ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_value_r ) ) . p as * const _ as
                usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_value_r ) ,
                "::" , stringify ! ( p ) ));
}
impl Clone for gravity_value_r {
    fn clone(&self) -> Self { *self }
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
                                               nargs: u16, rindex: u32)
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
#[test]
fn bindgen_test_layout_gravity_gc_t() {
    assert_eq!(::std::mem::size_of::<gravity_gc_t>() , 16usize , concat ! (
               "Size of: " , stringify ! ( gravity_gc_t ) ));
    assert_eq! (::std::mem::align_of::<gravity_gc_t>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( gravity_gc_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_gc_t ) ) . isdark as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_gc_t ) , "::" ,
                stringify ! ( isdark ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_gc_t ) ) . next as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_gc_t ) , "::" ,
                stringify ! ( next ) ));
}
impl Clone for gravity_gc_t {
    fn clone(&self) -> Self { *self }
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
#[test]
fn bindgen_test_layout_gravity_function_t__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<gravity_function_t__bindgen_ty_1__bindgen_ty_1>()
               , 48usize , concat ! (
               "Size of: " , stringify ! (
               gravity_function_t__bindgen_ty_1__bindgen_ty_1 ) ));
    assert_eq! (::std::mem::align_of::<gravity_function_t__bindgen_ty_1__bindgen_ty_1>()
                , 8usize , concat ! (
                "Alignment of " , stringify ! (
                gravity_function_t__bindgen_ty_1__bindgen_ty_1 ) ));
    assert_eq! (unsafe {
                & (
                * (
                0 as * const gravity_function_t__bindgen_ty_1__bindgen_ty_1 )
                ) . cpool as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                gravity_function_t__bindgen_ty_1__bindgen_ty_1 ) , "::" ,
                stringify ! ( cpool ) ));
    assert_eq! (unsafe {
                & (
                * (
                0 as * const gravity_function_t__bindgen_ty_1__bindgen_ty_1 )
                ) . ninsts as * const _ as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! (
                gravity_function_t__bindgen_ty_1__bindgen_ty_1 ) , "::" ,
                stringify ! ( ninsts ) ));
    assert_eq! (unsafe {
                & (
                * (
                0 as * const gravity_function_t__bindgen_ty_1__bindgen_ty_1 )
                ) . bytecode as * const _ as usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! (
                gravity_function_t__bindgen_ty_1__bindgen_ty_1 ) , "::" ,
                stringify ! ( bytecode ) ));
    assert_eq! (unsafe {
                & (
                * (
                0 as * const gravity_function_t__bindgen_ty_1__bindgen_ty_1 )
                ) . purity as * const _ as usize } , 40usize , concat ! (
                "Alignment of field: " , stringify ! (
                gravity_function_t__bindgen_ty_1__bindgen_ty_1 ) , "::" ,
                stringify ! ( purity ) ));
    assert_eq! (unsafe {
                & (
                * (
                0 as * const gravity_function_t__bindgen_ty_1__bindgen_ty_1 )
                ) . useargs as * const _ as usize } , 44usize , concat ! (
                "Alignment of field: " , stringify ! (
                gravity_function_t__bindgen_ty_1__bindgen_ty_1 ) , "::" ,
                stringify ! ( useargs ) ));
}
impl Clone for gravity_function_t__bindgen_ty_1__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_function_t__bindgen_ty_1__bindgen_ty_2 {
    pub index: u16,
    pub special: [*mut ::std::os::raw::c_void; 2usize],
}
#[test]
fn bindgen_test_layout_gravity_function_t__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(::std::mem::size_of::<gravity_function_t__bindgen_ty_1__bindgen_ty_2>()
               , 24usize , concat ! (
               "Size of: " , stringify ! (
               gravity_function_t__bindgen_ty_1__bindgen_ty_2 ) ));
    assert_eq! (::std::mem::align_of::<gravity_function_t__bindgen_ty_1__bindgen_ty_2>()
                , 8usize , concat ! (
                "Alignment of " , stringify ! (
                gravity_function_t__bindgen_ty_1__bindgen_ty_2 ) ));
    assert_eq! (unsafe {
                & (
                * (
                0 as * const gravity_function_t__bindgen_ty_1__bindgen_ty_2 )
                ) . index as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                gravity_function_t__bindgen_ty_1__bindgen_ty_2 ) , "::" ,
                stringify ! ( index ) ));
    assert_eq! (unsafe {
                & (
                * (
                0 as * const gravity_function_t__bindgen_ty_1__bindgen_ty_2 )
                ) . special as * const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! (
                gravity_function_t__bindgen_ty_1__bindgen_ty_2 ) , "::" ,
                stringify ! ( special ) ));
}
impl Clone for gravity_function_t__bindgen_ty_1__bindgen_ty_2 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_gravity_function_t__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<gravity_function_t__bindgen_ty_1>() ,
               48usize , concat ! (
               "Size of: " , stringify ! ( gravity_function_t__bindgen_ty_1 )
               ));
    assert_eq! (::std::mem::align_of::<gravity_function_t__bindgen_ty_1>() ,
                8usize , concat ! (
                "Alignment of " , stringify ! (
                gravity_function_t__bindgen_ty_1 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_function_t__bindgen_ty_1 ) ) .
                internal as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                gravity_function_t__bindgen_ty_1 ) , "::" , stringify ! (
                internal ) ));
}
impl Clone for gravity_function_t__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_gravity_function_t() {
    assert_eq!(::std::mem::size_of::<gravity_function_t>() , 104usize , concat
               ! ( "Size of: " , stringify ! ( gravity_function_t ) ));
    assert_eq! (::std::mem::align_of::<gravity_function_t>() , 8usize , concat
                ! ( "Alignment of " , stringify ! ( gravity_function_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_function_t ) ) . isa as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_function_t ) ,
                "::" , stringify ! ( isa ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_function_t ) ) . gc as * const _
                as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_function_t ) ,
                "::" , stringify ! ( gc ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_function_t ) ) . xdata as * const
                _ as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_function_t ) ,
                "::" , stringify ! ( xdata ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_function_t ) ) . identifier as *
                const _ as usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_function_t ) ,
                "::" , stringify ! ( identifier ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_function_t ) ) . nparams as *
                const _ as usize } , 40usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_function_t ) ,
                "::" , stringify ! ( nparams ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_function_t ) ) . nlocals as *
                const _ as usize } , 42usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_function_t ) ,
                "::" , stringify ! ( nlocals ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_function_t ) ) . ntemps as *
                const _ as usize } , 44usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_function_t ) ,
                "::" , stringify ! ( ntemps ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_function_t ) ) . nupvalues as *
                const _ as usize } , 46usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_function_t ) ,
                "::" , stringify ! ( nupvalues ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_function_t ) ) . tag as * const _
                as usize } , 48usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_function_t ) ,
                "::" , stringify ! ( tag ) ));
}
impl Clone for gravity_function_t {
    fn clone(&self) -> Self { *self }
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
#[test]
fn bindgen_test_layout_upvalue_s() {
    assert_eq!(::std::mem::size_of::<upvalue_s>() , 56usize , concat ! (
               "Size of: " , stringify ! ( upvalue_s ) ));
    assert_eq! (::std::mem::align_of::<upvalue_s>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( upvalue_s ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const upvalue_s ) ) . isa as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( upvalue_s ) , "::" ,
                stringify ! ( isa ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const upvalue_s ) ) . gc as * const _ as usize
                } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( upvalue_s ) , "::" ,
                stringify ! ( gc ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const upvalue_s ) ) . value as * const _ as
                usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( upvalue_s ) , "::" ,
                stringify ! ( value ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const upvalue_s ) ) . closed as * const _ as
                usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( upvalue_s ) , "::" ,
                stringify ! ( closed ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const upvalue_s ) ) . next as * const _ as
                usize } , 48usize , concat ! (
                "Alignment of field: " , stringify ! ( upvalue_s ) , "::" ,
                stringify ! ( next ) ));
}
impl Clone for upvalue_s {
    fn clone(&self) -> Self { *self }
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
#[test]
fn bindgen_test_layout_gravity_closure_t() {
    assert_eq!(::std::mem::size_of::<gravity_closure_t>() , 40usize , concat !
               ( "Size of: " , stringify ! ( gravity_closure_t ) ));
    assert_eq! (::std::mem::align_of::<gravity_closure_t>() , 8usize , concat
                ! ( "Alignment of " , stringify ! ( gravity_closure_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_closure_t ) ) . isa as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_closure_t ) ,
                "::" , stringify ! ( isa ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_closure_t ) ) . gc as * const _
                as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_closure_t ) ,
                "::" , stringify ! ( gc ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_closure_t ) ) . f as * const _ as
                usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_closure_t ) ,
                "::" , stringify ! ( f ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_closure_t ) ) . upvalue as *
                const _ as usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_closure_t ) ,
                "::" , stringify ! ( upvalue ) ));
}
impl Clone for gravity_closure_t {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_list_t {
    pub isa: *mut gravity_class_t,
    pub gc: gravity_gc_t,
    pub array: gravity_value_r,
}
#[test]
fn bindgen_test_layout_gravity_list_t() {
    assert_eq!(::std::mem::size_of::<gravity_list_t>() , 48usize , concat ! (
               "Size of: " , stringify ! ( gravity_list_t ) ));
    assert_eq! (::std::mem::align_of::<gravity_list_t>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( gravity_list_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_list_t ) ) . isa as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_list_t ) , "::"
                , stringify ! ( isa ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_list_t ) ) . gc as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_list_t ) , "::"
                , stringify ! ( gc ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_list_t ) ) . array as * const _
                as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_list_t ) , "::"
                , stringify ! ( array ) ));
}
impl Clone for gravity_list_t {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_map_t {
    pub isa: *mut gravity_class_t,
    pub gc: gravity_gc_t,
    pub hash: *mut gravity_hash_t,
}
#[test]
fn bindgen_test_layout_gravity_map_t() {
    assert_eq!(::std::mem::size_of::<gravity_map_t>() , 32usize , concat ! (
               "Size of: " , stringify ! ( gravity_map_t ) ));
    assert_eq! (::std::mem::align_of::<gravity_map_t>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( gravity_map_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_map_t ) ) . isa as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_map_t ) , "::"
                , stringify ! ( isa ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_map_t ) ) . gc as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_map_t ) , "::"
                , stringify ! ( gc ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_map_t ) ) . hash as * const _ as
                usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_map_t ) , "::"
                , stringify ! ( hash ) ));
}
impl Clone for gravity_map_t {
    fn clone(&self) -> Self { *self }
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
#[test]
fn bindgen_test_layout_gravity_callframe_t() {
    assert_eq!(::std::mem::size_of::<gravity_callframe_t>() , 48usize , concat
               ! ( "Size of: " , stringify ! ( gravity_callframe_t ) ));
    assert_eq! (::std::mem::align_of::<gravity_callframe_t>() , 8usize ,
                concat ! (
                "Alignment of " , stringify ! ( gravity_callframe_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_callframe_t ) ) . ip as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_callframe_t ) ,
                "::" , stringify ! ( ip ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_callframe_t ) ) . dest as * const
                _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_callframe_t ) ,
                "::" , stringify ! ( dest ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_callframe_t ) ) . nargs as *
                const _ as usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_callframe_t ) ,
                "::" , stringify ! ( nargs ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_callframe_t ) ) . args as * const
                _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_callframe_t ) ,
                "::" , stringify ! ( args ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_callframe_t ) ) . closure as *
                const _ as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_callframe_t ) ,
                "::" , stringify ! ( closure ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_callframe_t ) ) . stackstart as *
                const _ as usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_callframe_t ) ,
                "::" , stringify ! ( stackstart ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_callframe_t ) ) . outloop as *
                const _ as usize } , 40usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_callframe_t ) ,
                "::" , stringify ! ( outloop ) ));
}
impl Clone for gravity_callframe_t {
    fn clone(&self) -> Self { *self }
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
#[test]
fn bindgen_test_layout_fiber_s() {
    assert_eq!(::std::mem::size_of::<fiber_s>() , 112usize , concat ! (
               "Size of: " , stringify ! ( fiber_s ) ));
    assert_eq! (::std::mem::align_of::<fiber_s>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( fiber_s ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const fiber_s ) ) . isa as * const _ as usize }
                , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( fiber_s ) , "::" ,
                stringify ! ( isa ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const fiber_s ) ) . gc as * const _ as usize }
                , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( fiber_s ) , "::" ,
                stringify ! ( gc ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const fiber_s ) ) . stack as * const _ as usize
                } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( fiber_s ) , "::" ,
                stringify ! ( stack ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const fiber_s ) ) . stacktop as * const _ as
                usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( fiber_s ) , "::" ,
                stringify ! ( stacktop ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const fiber_s ) ) . stackalloc as * const _ as
                usize } , 40usize , concat ! (
                "Alignment of field: " , stringify ! ( fiber_s ) , "::" ,
                stringify ! ( stackalloc ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const fiber_s ) ) . frames as * const _ as
                usize } , 48usize , concat ! (
                "Alignment of field: " , stringify ! ( fiber_s ) , "::" ,
                stringify ! ( frames ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const fiber_s ) ) . nframes as * const _ as
                usize } , 56usize , concat ! (
                "Alignment of field: " , stringify ! ( fiber_s ) , "::" ,
                stringify ! ( nframes ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const fiber_s ) ) . framesalloc as * const _ as
                usize } , 60usize , concat ! (
                "Alignment of field: " , stringify ! ( fiber_s ) , "::" ,
                stringify ! ( framesalloc ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const fiber_s ) ) . upvalues as * const _ as
                usize } , 64usize , concat ! (
                "Alignment of field: " , stringify ! ( fiber_s ) , "::" ,
                stringify ! ( upvalues ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const fiber_s ) ) . error as * const _ as usize
                } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( fiber_s ) , "::" ,
                stringify ! ( error ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const fiber_s ) ) . trying as * const _ as
                usize } , 80usize , concat ! (
                "Alignment of field: " , stringify ! ( fiber_s ) , "::" ,
                stringify ! ( trying ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const fiber_s ) ) . caller as * const _ as
                usize } , 88usize , concat ! (
                "Alignment of field: " , stringify ! ( fiber_s ) , "::" ,
                stringify ! ( caller ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const fiber_s ) ) . result as * const _ as
                usize } , 96usize , concat ! (
                "Alignment of field: " , stringify ! ( fiber_s ) , "::" ,
                stringify ! ( result ) ));
}
impl Clone for fiber_s {
    fn clone(&self) -> Self { *self }
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
#[test]
fn bindgen_test_layout_gravity_module_t() {
    assert_eq!(::std::mem::size_of::<gravity_module_t>() , 40usize , concat !
               ( "Size of: " , stringify ! ( gravity_module_t ) ));
    assert_eq! (::std::mem::align_of::<gravity_module_t>() , 8usize , concat !
                ( "Alignment of " , stringify ! ( gravity_module_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_module_t ) ) . isa as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_module_t ) ,
                "::" , stringify ! ( isa ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_module_t ) ) . gc as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_module_t ) ,
                "::" , stringify ! ( gc ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_module_t ) ) . identifier as *
                const _ as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_module_t ) ,
                "::" , stringify ! ( identifier ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_module_t ) ) . htable as * const
                _ as usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_module_t ) ,
                "::" , stringify ! ( htable ) ));
}
impl Clone for gravity_module_t {
    fn clone(&self) -> Self { *self }
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
#[test]
fn bindgen_test_layout_gravity_instance_t() {
    assert_eq!(::std::mem::size_of::<gravity_instance_t>() , 40usize , concat
               ! ( "Size of: " , stringify ! ( gravity_instance_t ) ));
    assert_eq! (::std::mem::align_of::<gravity_instance_t>() , 8usize , concat
                ! ( "Alignment of " , stringify ! ( gravity_instance_t ) ));
}
impl Clone for gravity_instance_t {
    fn clone(&self) -> Self { *self }
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
#[test]
fn bindgen_test_layout_gravity_string_t() {
    assert_eq!(::std::mem::size_of::<gravity_string_t>() , 48usize , concat !
               ( "Size of: " , stringify ! ( gravity_string_t ) ));
    assert_eq! (::std::mem::align_of::<gravity_string_t>() , 8usize , concat !
                ( "Alignment of " , stringify ! ( gravity_string_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_string_t ) ) . isa as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_string_t ) ,
                "::" , stringify ! ( isa ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_string_t ) ) . gc as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_string_t ) ,
                "::" , stringify ! ( gc ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_string_t ) ) . s as * const _ as
                usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_string_t ) ,
                "::" , stringify ! ( s ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_string_t ) ) . hash as * const _
                as usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_string_t ) ,
                "::" , stringify ! ( hash ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_string_t ) ) . len as * const _
                as usize } , 36usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_string_t ) ,
                "::" , stringify ! ( len ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_string_t ) ) . alloc as * const _
                as usize } , 40usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_string_t ) ,
                "::" , stringify ! ( alloc ) ));
}
impl Clone for gravity_string_t {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_range_t {
    pub isa: *mut gravity_class_t,
    pub gc: gravity_gc_t,
    pub from: gravity_int_t,
    pub to: gravity_int_t,
}
#[test]
fn bindgen_test_layout_gravity_range_t() {
    assert_eq!(::std::mem::size_of::<gravity_range_t>() , 40usize , concat ! (
               "Size of: " , stringify ! ( gravity_range_t ) ));
    assert_eq! (::std::mem::align_of::<gravity_range_t>() , 8usize , concat !
                ( "Alignment of " , stringify ! ( gravity_range_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_range_t ) ) . isa as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_range_t ) ,
                "::" , stringify ! ( isa ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_range_t ) ) . gc as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_range_t ) ,
                "::" , stringify ! ( gc ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_range_t ) ) . from as * const _
                as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_range_t ) ,
                "::" , stringify ! ( from ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_range_t ) ) . to as * const _ as
                usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_range_t ) ,
                "::" , stringify ! ( to ) ));
}
impl Clone for gravity_range_t {
    fn clone(&self) -> Self { *self }
}
pub type code_dump_function =
    ::std::option::Option<unsafe extern "C" fn(code:
                                                   *mut ::std::os::raw::c_void)>;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_function_r {
    pub n: usize,
    pub m: usize,
    pub p: *mut *mut gravity_function_t,
}
#[test]
fn bindgen_test_layout_gravity_function_r() {
    assert_eq!(::std::mem::size_of::<gravity_function_r>() , 24usize , concat
               ! ( "Size of: " , stringify ! ( gravity_function_r ) ));
    assert_eq! (::std::mem::align_of::<gravity_function_r>() , 8usize , concat
                ! ( "Alignment of " , stringify ! ( gravity_function_r ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_function_r ) ) . n as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_function_r ) ,
                "::" , stringify ! ( n ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_function_r ) ) . m as * const _
                as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_function_r ) ,
                "::" , stringify ! ( m ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_function_r ) ) . p as * const _
                as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_function_r ) ,
                "::" , stringify ! ( p ) ));
}
impl Clone for gravity_function_r {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_class_r {
    pub n: usize,
    pub m: usize,
    pub p: *mut *mut gravity_class_t,
}
#[test]
fn bindgen_test_layout_gravity_class_r() {
    assert_eq!(::std::mem::size_of::<gravity_class_r>() , 24usize , concat ! (
               "Size of: " , stringify ! ( gravity_class_r ) ));
    assert_eq! (::std::mem::align_of::<gravity_class_r>() , 8usize , concat !
                ( "Alignment of " , stringify ! ( gravity_class_r ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_class_r ) ) . n as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_class_r ) ,
                "::" , stringify ! ( n ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_class_r ) ) . m as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_class_r ) ,
                "::" , stringify ! ( m ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_class_r ) ) . p as * const _ as
                usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_class_r ) ,
                "::" , stringify ! ( p ) ));
}
impl Clone for gravity_class_r {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gravity_object_r {
    pub n: usize,
    pub m: usize,
    pub p: *mut *mut gravity_object_t,
}
#[test]
fn bindgen_test_layout_gravity_object_r() {
    assert_eq!(::std::mem::size_of::<gravity_object_r>() , 24usize , concat !
               ( "Size of: " , stringify ! ( gravity_object_r ) ));
    assert_eq! (::std::mem::align_of::<gravity_object_r>() , 8usize , concat !
                ( "Alignment of " , stringify ! ( gravity_object_r ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_object_r ) ) . n as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_object_r ) ,
                "::" , stringify ! ( n ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_object_r ) ) . m as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_object_r ) ,
                "::" , stringify ! ( m ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_object_r ) ) . p as * const _ as
                usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_object_r ) ,
                "::" , stringify ! ( p ) ));
}
impl Clone for gravity_object_r {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    pub fn gravity_module_new(vm: *mut gravity_vm,
                              identifier: *const ::std::os::raw::c_char)
     -> *mut gravity_module_t;
}
extern "C" {
    pub fn gravity_module_free(vm: *mut gravity_vm, m: *mut gravity_module_t);
}
extern "C" {
    pub fn gravity_module_blacken(vm: *mut gravity_vm,
                                  m: *mut gravity_module_t);
}
extern "C" {
    pub fn gravity_module_size(vm: *mut gravity_vm, m: *mut gravity_module_t)
     -> u32;
}
extern "C" {
    pub fn gravity_function_new(vm: *mut gravity_vm,
                                identifier: *const ::std::os::raw::c_char,
                                nparams: u16, nlocals: u16, ntemps: u16,
                                code: *mut ::std::os::raw::c_void)
     -> *mut gravity_function_t;
}
extern "C" {
    pub fn gravity_function_new_internal(vm: *mut gravity_vm,
                                         identifier:
                                             *const ::std::os::raw::c_char,
                                         exec: gravity_c_internal,
                                         nparams: u16)
     -> *mut gravity_function_t;
}
extern "C" {
    pub fn gravity_function_new_special(vm: *mut gravity_vm,
                                        identifier:
                                            *const ::std::os::raw::c_char,
                                        index: u16,
                                        getter: *mut ::std::os::raw::c_void,
                                        setter: *mut ::std::os::raw::c_void)
     -> *mut gravity_function_t;
}
extern "C" {
    pub fn gravity_function_new_bridged(vm: *mut gravity_vm,
                                        identifier:
                                            *const ::std::os::raw::c_char,
                                        xdata: *mut ::std::os::raw::c_void)
     -> *mut gravity_function_t;
}
extern "C" {
    pub fn gravity_function_cpool_add(vm: *mut gravity_vm,
                                      f: *mut gravity_function_t,
                                      v: gravity_value_t) -> u16;
}
extern "C" {
    pub fn gravity_function_cpool_get(f: *mut gravity_function_t, i: u16)
     -> gravity_value_t;
}
extern "C" {
    pub fn gravity_function_dump(f: *mut gravity_function_t,
                                 codef: code_dump_function);
}
extern "C" {
    pub fn gravity_function_setouter(f: *mut gravity_function_t,
                                     outer: *mut gravity_object_t);
}
extern "C" {
    pub fn gravity_function_setxdata(f: *mut gravity_function_t,
                                     xdata: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn gravity_function_serialize(f: *mut gravity_function_t,
                                      json: *mut json_t);
}
extern "C" {
    pub fn gravity_bytecode_deserialize(buffer: *const ::std::os::raw::c_char,
                                        len: usize, ninst: *mut u32)
     -> *mut u32;
}
extern "C" {
    pub fn gravity_function_deserialize(vm: *mut gravity_vm,
                                        json: *mut json_value)
     -> *mut gravity_function_t;
}
extern "C" {
    pub fn gravity_function_free(vm: *mut gravity_vm,
                                 f: *mut gravity_function_t);
}
extern "C" {
    pub fn gravity_function_blacken(vm: *mut gravity_vm,
                                    f: *mut gravity_function_t);
}
extern "C" {
    pub fn gravity_function_size(vm: *mut gravity_vm,
                                 f: *mut gravity_function_t) -> u32;
}
extern "C" {
    pub fn gravity_closure_new(vm: *mut gravity_vm,
                               f: *mut gravity_function_t)
     -> *mut gravity_closure_t;
}
extern "C" {
    pub fn gravity_closure_free(vm: *mut gravity_vm,
                                closure: *mut gravity_closure_t);
}
extern "C" {
    pub fn gravity_closure_size(vm: *mut gravity_vm,
                                closure: *mut gravity_closure_t) -> u32;
}
extern "C" {
    pub fn gravity_closure_blacken(vm: *mut gravity_vm,
                                   closure: *mut gravity_closure_t);
}
extern "C" {
    pub fn gravity_upvalue_new(vm: *mut gravity_vm,
                               value: *mut gravity_value_t)
     -> *mut gravity_upvalue_t;
}
extern "C" {
    pub fn gravity_upvalue_size(vm: *mut gravity_vm,
                                upvalue: *mut gravity_upvalue_t) -> u32;
}
extern "C" {
    pub fn gravity_upvalue_blacken(vm: *mut gravity_vm,
                                   upvalue: *mut gravity_upvalue_t);
}
extern "C" {
    pub fn gravity_upvalue_free(vm: *mut gravity_vm,
                                upvalue: *mut gravity_upvalue_t);
}
extern "C" {
    pub fn gravity_class_bind(c: *mut gravity_class_t,
                              key: *const ::std::os::raw::c_char,
                              value: gravity_value_t);
}
extern "C" {
    pub fn gravity_class_getsuper(c: *mut gravity_class_t)
     -> *mut gravity_class_t;
}
extern "C" {
    pub fn gravity_class_grow(c: *mut gravity_class_t, n: u32) -> bool;
}
extern "C" {
    pub fn gravity_class_setsuper(subclass: *mut gravity_class_t,
                                  superclass: *mut gravity_class_t) -> bool;
}
extern "C" {
    pub fn gravity_class_new_single(vm: *mut gravity_vm,
                                    identifier: *const ::std::os::raw::c_char,
                                    nfields: u32) -> *mut gravity_class_t;
}
extern "C" {
    pub fn gravity_class_new_pair(vm: *mut gravity_vm,
                                  identifier: *const ::std::os::raw::c_char,
                                  superclass: *mut gravity_class_t,
                                  nivar: u32, nsvar: u32)
     -> *mut gravity_class_t;
}
extern "C" {
    pub fn gravity_class_get_meta(c: *mut gravity_class_t)
     -> *mut gravity_class_t;
}
extern "C" {
    pub fn gravity_class_is_meta(c: *mut gravity_class_t) -> bool;
}
extern "C" {
    pub fn gravity_class_count_ivars(c: *mut gravity_class_t) -> u32;
}
extern "C" {
    pub fn gravity_class_dump(c: *mut gravity_class_t);
}
extern "C" {
    pub fn gravity_class_setxdata(c: *mut gravity_class_t,
                                  xdata: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn gravity_class_add_ivar(c: *mut gravity_class_t,
                                  identifier: *const ::std::os::raw::c_char)
     -> i16;
}
extern "C" {
    pub fn gravity_class_serialize(c: *mut gravity_class_t,
                                   json: *mut json_t);
}
extern "C" {
    pub fn gravity_class_deserialize(vm: *mut gravity_vm,
                                     json: *mut json_value)
     -> *mut gravity_class_t;
}
extern "C" {
    pub fn gravity_class_free(vm: *mut gravity_vm, c: *mut gravity_class_t);
}
extern "C" {
    pub fn gravity_class_free_core(vm: *mut gravity_vm,
                                   c: *mut gravity_class_t);
}
extern "C" {
    pub fn gravity_class_lookup(c: *mut gravity_class_t, key: gravity_value_t)
     -> *mut gravity_object_t;
}
extern "C" {
    pub fn gravity_class_lookup_closure(c: *mut gravity_class_t,
                                        key: gravity_value_t)
     -> *mut gravity_closure_t;
}
extern "C" {
    pub fn gravity_class_lookup_constructor(c: *mut gravity_class_t,
                                            nparams: u32)
     -> *mut gravity_closure_t;
}
extern "C" {
    pub fn gravity_class_blacken(vm: *mut gravity_vm,
                                 c: *mut gravity_class_t);
}
extern "C" {
    pub fn gravity_class_size(vm: *mut gravity_vm, c: *mut gravity_class_t)
     -> u32;
}
extern "C" {
    pub fn gravity_fiber_new(vm: *mut gravity_vm,
                             closure: *mut gravity_closure_t, nstack: u32,
                             nframes: u32) -> *mut gravity_fiber_t;
}
extern "C" {
    pub fn gravity_fiber_reassign(fiber: *mut gravity_fiber_t,
                                  closure: *mut gravity_closure_t,
                                  nargs: u16);
}
extern "C" {
    pub fn gravity_fiber_seterror(fiber: *mut gravity_fiber_t,
                                  error: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn gravity_fiber_free(vm: *mut gravity_vm,
                              fiber: *mut gravity_fiber_t);
}
extern "C" {
    pub fn gravity_fiber_blacken(vm: *mut gravity_vm,
                                 fiber: *mut gravity_fiber_t);
}
extern "C" {
    pub fn gravity_fiber_size(vm: *mut gravity_vm,
                              fiber: *mut gravity_fiber_t) -> u32;
}
extern "C" {
    pub fn gravity_instance_new(vm: *mut gravity_vm, c: *mut gravity_class_t)
     -> *mut gravity_instance_t;
}
extern "C" {
    pub fn gravity_instance_dup(vm: *mut gravity_vm,
                                src: *mut gravity_instance_t)
     -> *mut gravity_instance_t;
}
extern "C" {
    pub fn gravity_instance_setivar(instance: *mut gravity_instance_t,
                                    idx: u32, value: gravity_value_t);
}
extern "C" {
    pub fn gravity_instance_setxdata(i: *mut gravity_instance_t,
                                     xdata: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn gravity_instance_free(vm: *mut gravity_vm,
                                 i: *mut gravity_instance_t);
}
extern "C" {
    pub fn gravity_instance_lookup_event(i: *mut gravity_instance_t,
                                         name: *const ::std::os::raw::c_char)
     -> *mut gravity_closure_t;
}
extern "C" {
    pub fn gravity_instance_blacken(vm: *mut gravity_vm,
                                    i: *mut gravity_instance_t);
}
extern "C" {
    pub fn gravity_instance_size(vm: *mut gravity_vm,
                                 i: *mut gravity_instance_t) -> u32;
}
extern "C" {
    pub fn gravity_value_equals(v1: gravity_value_t, v2: gravity_value_t)
     -> bool;
}
extern "C" {
    pub fn gravity_value_hash(value: gravity_value_t) -> u32;
}
extern "C" {
    pub fn gravity_value_getclass(v: gravity_value_t) -> *mut gravity_class_t;
}
extern "C" {
    pub fn gravity_value_getsuper(v: gravity_value_t) -> *mut gravity_class_t;
}
extern "C" {
    pub fn gravity_value_free(vm: *mut gravity_vm, v: gravity_value_t);
}
extern "C" {
    pub fn gravity_value_serialize(v: gravity_value_t, json: *mut json_t);
}
extern "C" {
    pub fn gravity_value_dump(v: gravity_value_t,
                              buffer: *mut ::std::os::raw::c_char, len: u16);
}
extern "C" {
    pub fn gravity_value_isobject(v: gravity_value_t) -> bool;
}
extern "C" {
    pub fn gravity_value_xdata(value: gravity_value_t)
     -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn gravity_value_blacken(vm: *mut gravity_vm, v: gravity_value_t);
}
extern "C" {
    pub fn gravity_value_size(vm: *mut gravity_vm, v: gravity_value_t) -> u32;
}
extern "C" {
    pub fn gravity_object_serialize(obj: *mut gravity_object_t,
                                    json: *mut json_t);
}
extern "C" {
    pub fn gravity_object_deserialize(vm: *mut gravity_vm,
                                      entry: *mut json_value)
     -> *mut gravity_object_t;
}
extern "C" {
    pub fn gravity_object_free(vm: *mut gravity_vm,
                               obj: *mut gravity_object_t);
}
extern "C" {
    pub fn gravity_object_blacken(vm: *mut gravity_vm,
                                  obj: *mut gravity_object_t);
}
extern "C" {
    pub fn gravity_object_size(vm: *mut gravity_vm,
                               obj: *mut gravity_object_t) -> u32;
}
extern "C" {
    pub fn gravity_object_debug(obj: *mut gravity_object_t)
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn gravity_list_new(vm: *mut gravity_vm, n: u32)
     -> *mut gravity_list_t;
}
extern "C" {
    pub fn gravity_list_from_array(vm: *mut gravity_vm, n: u32,
                                   p: *mut gravity_value_t)
     -> *mut gravity_list_t;
}
extern "C" {
    pub fn gravity_list_free(vm: *mut gravity_vm, list: *mut gravity_list_t);
}
extern "C" {
    pub fn gravity_list_append_list(vm: *mut gravity_vm,
                                    list1: *mut gravity_list_t,
                                    list2: *mut gravity_list_t);
}
extern "C" {
    pub fn gravity_list_blacken(vm: *mut gravity_vm,
                                list: *mut gravity_list_t);
}
extern "C" {
    pub fn gravity_list_size(vm: *mut gravity_vm, list: *mut gravity_list_t)
     -> u32;
}
extern "C" {
    pub fn gravity_map_new(vm: *mut gravity_vm, n: u32) -> *mut gravity_map_t;
}
extern "C" {
    pub fn gravity_map_free(vm: *mut gravity_vm, map: *mut gravity_map_t);
}
extern "C" {
    pub fn gravity_map_append_map(vm: *mut gravity_vm,
                                  map1: *mut gravity_map_t,
                                  map2: *mut gravity_map_t);
}
extern "C" {
    pub fn gravity_map_insert(vm: *mut gravity_vm, map: *mut gravity_map_t,
                              key: gravity_value_t, value: gravity_value_t);
}
extern "C" {
    pub fn gravity_map_blacken(vm: *mut gravity_vm, map: *mut gravity_map_t);
}
extern "C" {
    pub fn gravity_map_size(vm: *mut gravity_vm, map: *mut gravity_map_t)
     -> u32;
}
extern "C" {
    pub fn gravity_range_new(vm: *mut gravity_vm, from: gravity_int_t,
                             to: gravity_int_t, inclusive: bool)
     -> *mut gravity_range_t;
}
extern "C" {
    pub fn gravity_range_free(vm: *mut gravity_vm,
                              range: *mut gravity_range_t);
}
extern "C" {
    pub fn gravity_range_blacken(vm: *mut gravity_vm,
                                 range: *mut gravity_range_t);
}
extern "C" {
    pub fn gravity_range_size(vm: *mut gravity_vm,
                              range: *mut gravity_range_t) -> u32;
}
extern "C" {
    /// MARK: - STRING -
    pub fn gravity_string_to_value(vm: *mut gravity_vm,
                                   s: *const ::std::os::raw::c_char, len: u32)
     -> gravity_value_t;
}
extern "C" {
    pub fn gravity_string_new(vm: *mut gravity_vm,
                              s: *mut ::std::os::raw::c_char, len: u32,
                              alloc: u32) -> *mut gravity_string_t;
}
extern "C" {
    pub fn gravity_string_free(vm: *mut gravity_vm,
                               value: *mut gravity_string_t);
}
extern "C" {
    pub fn gravity_string_blacken(vm: *mut gravity_vm,
                                  string: *mut gravity_string_t);
}
extern "C" {
    pub fn gravity_string_size(vm: *mut gravity_vm,
                               string: *mut gravity_string_t) -> u32;
}
extern "C" {
    pub fn gravity_hash_keyvaluefree(table: *mut gravity_hash_t,
                                     key: gravity_value_t,
                                     value: gravity_value_t,
                                     data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn gravity_hash_keyfree(table: *mut gravity_hash_t,
                                key: gravity_value_t, value: gravity_value_t,
                                data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn gravity_hash_valuefree(table: *mut gravity_hash_t,
                                  key: gravity_value_t,
                                  value: gravity_value_t,
                                  data: *mut ::std::os::raw::c_void);
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
#[test]
fn bindgen_test_layout_error_desc_t() {
    assert_eq!(::std::mem::size_of::<error_desc_t>() , 24usize , concat ! (
               "Size of: " , stringify ! ( error_desc_t ) ));
    assert_eq! (::std::mem::align_of::<error_desc_t>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( error_desc_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const error_desc_t ) ) . lineno as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( error_desc_t ) , "::" ,
                stringify ! ( lineno ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const error_desc_t ) ) . colno as * const _ as
                usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( error_desc_t ) , "::" ,
                stringify ! ( colno ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const error_desc_t ) ) . fileid as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( error_desc_t ) , "::" ,
                stringify ! ( fileid ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const error_desc_t ) ) . offset as * const _ as
                usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( error_desc_t ) , "::" ,
                stringify ! ( offset ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const error_desc_t ) ) . meta as * const _ as
                usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( error_desc_t ) , "::" ,
                stringify ! ( meta ) ));
}
impl Clone for error_desc_t {
    fn clone(&self) -> Self { *self }
}
pub type gravity_log_callback =
    ::std::option::Option<unsafe extern "C" fn(message:
                                                   *const ::std::os::raw::c_char,
                                               xdata:
                                                   *mut ::std::os::raw::c_void)>;
pub type gravity_error_callback =
    ::std::option::Option<unsafe extern "C" fn(error_type: error_type_t,
                                               description:
                                                   *const ::std::os::raw::c_char,
                                               error_desc: error_desc_t,
                                               xdata:
                                                   *mut ::std::os::raw::c_void)>;
pub type gravity_unittest_callback =
    ::std::option::Option<unsafe extern "C" fn(error_type: error_type_t,
                                               desc:
                                                   *const ::std::os::raw::c_char,
                                               note:
                                                   *const ::std::os::raw::c_char,
                                               value: gravity_value_t,
                                               row: i32, col: i32,
                                               xdata:
                                                   *mut ::std::os::raw::c_void)>;
pub type gravity_parser_callback =
    ::std::option::Option<unsafe extern "C" fn(token:
                                                   *mut ::std::os::raw::c_void,
                                               xdata:
                                                   *mut ::std::os::raw::c_void)>;
pub type gravity_precode_callback =
    ::std::option::Option<unsafe extern "C" fn(xdata:
                                                   *mut ::std::os::raw::c_void)
                              -> *const ::std::os::raw::c_char>;
pub type gravity_loadfile_callback =
    ::std::option::Option<unsafe extern "C" fn(file:
                                                   *const ::std::os::raw::c_char,
                                               size: *mut usize,
                                               fileid: *mut u32,
                                               xdata:
                                                   *mut ::std::os::raw::c_void)
                              -> *const ::std::os::raw::c_char>;
pub type gravity_filename_callback =
    ::std::option::Option<unsafe extern "C" fn(fileid: u32,
                                               xdata:
                                                   *mut ::std::os::raw::c_void)
                              -> *const ::std::os::raw::c_char>;
pub type gravity_bridge_initinstance =
    ::std::option::Option<unsafe extern "C" fn(vm: *mut gravity_vm,
                                               xdata:
                                                   *mut ::std::os::raw::c_void,
                                               instance:
                                                   *mut gravity_instance_t,
                                               args: *mut gravity_value_t,
                                               nargs: i16) -> bool>;
pub type gravity_bridge_setvalue =
    ::std::option::Option<unsafe extern "C" fn(vm: *mut gravity_vm,
                                               xdata:
                                                   *mut ::std::os::raw::c_void,
                                               target: gravity_value_t,
                                               key:
                                                   *const ::std::os::raw::c_char,
                                               value: gravity_value_t)
                              -> bool>;
pub type gravity_bridge_getvalue =
    ::std::option::Option<unsafe extern "C" fn(vm: *mut gravity_vm,
                                               xdata:
                                                   *mut ::std::os::raw::c_void,
                                               target: gravity_value_t,
                                               key:
                                                   *const ::std::os::raw::c_char,
                                               vindex: u32) -> bool>;
pub type gravity_bridge_setundef =
    ::std::option::Option<unsafe extern "C" fn(vm: *mut gravity_vm,
                                               xdata:
                                                   *mut ::std::os::raw::c_void,
                                               target: gravity_value_t,
                                               key:
                                                   *const ::std::os::raw::c_char,
                                               value: gravity_value_t)
                              -> bool>;
pub type gravity_bridge_getundef =
    ::std::option::Option<unsafe extern "C" fn(vm: *mut gravity_vm,
                                               xdata:
                                                   *mut ::std::os::raw::c_void,
                                               target: gravity_value_t,
                                               key:
                                                   *const ::std::os::raw::c_char,
                                               vindex: u32) -> bool>;
pub type gravity_bridge_execute =
    ::std::option::Option<unsafe extern "C" fn(vm: *mut gravity_vm,
                                               xdata:
                                                   *mut ::std::os::raw::c_void,
                                               args: *mut gravity_value_t,
                                               nargs: i16, vindex: u32)
                              -> bool>;
pub type gravity_bridge_size =
    ::std::option::Option<unsafe extern "C" fn(vm: *mut gravity_vm,
                                               obj: *mut gravity_object_t)
                              -> u32>;
pub type gravity_bridge_free =
    ::std::option::Option<unsafe extern "C" fn(vm: *mut gravity_vm,
                                               obj: *mut gravity_object_t)>;
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
#[test]
fn bindgen_test_layout_gravity_delegate_t() {
    assert_eq!(::std::mem::size_of::<gravity_delegate_t>() , 128usize , concat
               ! ( "Size of: " , stringify ! ( gravity_delegate_t ) ));
    assert_eq! (::std::mem::align_of::<gravity_delegate_t>() , 8usize , concat
                ! ( "Alignment of " , stringify ! ( gravity_delegate_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_delegate_t ) ) . xdata as * const
                _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_delegate_t ) ,
                "::" , stringify ! ( xdata ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_delegate_t ) ) . log_callback as
                * const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_delegate_t ) ,
                "::" , stringify ! ( log_callback ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_delegate_t ) ) . error_callback
                as * const _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_delegate_t ) ,
                "::" , stringify ! ( error_callback ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_delegate_t ) ) .
                unittest_callback as * const _ as usize } , 24usize , concat !
                (
                "Alignment of field: " , stringify ! ( gravity_delegate_t ) ,
                "::" , stringify ! ( unittest_callback ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_delegate_t ) ) . parser_callback
                as * const _ as usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_delegate_t ) ,
                "::" , stringify ! ( parser_callback ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_delegate_t ) ) . precode_callback
                as * const _ as usize } , 40usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_delegate_t ) ,
                "::" , stringify ! ( precode_callback ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_delegate_t ) ) .
                loadfile_callback as * const _ as usize } , 48usize , concat !
                (
                "Alignment of field: " , stringify ! ( gravity_delegate_t ) ,
                "::" , stringify ! ( loadfile_callback ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_delegate_t ) ) .
                filename_callback as * const _ as usize } , 56usize , concat !
                (
                "Alignment of field: " , stringify ! ( gravity_delegate_t ) ,
                "::" , stringify ! ( filename_callback ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_delegate_t ) ) .
                bridge_initinstance as * const _ as usize } , 64usize , concat
                ! (
                "Alignment of field: " , stringify ! ( gravity_delegate_t ) ,
                "::" , stringify ! ( bridge_initinstance ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_delegate_t ) ) . bridge_setvalue
                as * const _ as usize } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_delegate_t ) ,
                "::" , stringify ! ( bridge_setvalue ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_delegate_t ) ) . bridge_getvalue
                as * const _ as usize } , 80usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_delegate_t ) ,
                "::" , stringify ! ( bridge_getvalue ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_delegate_t ) ) . bridge_setundef
                as * const _ as usize } , 88usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_delegate_t ) ,
                "::" , stringify ! ( bridge_setundef ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_delegate_t ) ) . bridge_getundef
                as * const _ as usize } , 96usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_delegate_t ) ,
                "::" , stringify ! ( bridge_getundef ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_delegate_t ) ) . bridge_execute
                as * const _ as usize } , 104usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_delegate_t ) ,
                "::" , stringify ! ( bridge_execute ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_delegate_t ) ) . bridge_size as *
                const _ as usize } , 112usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_delegate_t ) ,
                "::" , stringify ! ( bridge_size ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gravity_delegate_t ) ) . bridge_free as *
                const _ as usize } , 120usize , concat ! (
                "Alignment of field: " , stringify ! ( gravity_delegate_t ) ,
                "::" , stringify ! ( bridge_free ) ));
}
impl Clone for gravity_delegate_t {
    fn clone(&self) -> Self { *self }
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
#[test]
fn bindgen_test_layout_gtoken_s() {
    assert_eq!(::std::mem::size_of::<gtoken_s>() , 40usize , concat ! (
               "Size of: " , stringify ! ( gtoken_s ) ));
    assert_eq! (::std::mem::align_of::<gtoken_s>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( gtoken_s ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gtoken_s ) ) . type_ as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gtoken_s ) , "::" ,
                stringify ! ( type_ ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gtoken_s ) ) . lineno as * const _ as
                usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( gtoken_s ) , "::" ,
                stringify ! ( lineno ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gtoken_s ) ) . colno as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( gtoken_s ) , "::" ,
                stringify ! ( colno ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gtoken_s ) ) . position as * const _ as
                usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( gtoken_s ) , "::" ,
                stringify ! ( position ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gtoken_s ) ) . bytes as * const _ as
                usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( gtoken_s ) , "::" ,
                stringify ! ( bytes ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gtoken_s ) ) . length as * const _ as
                usize } , 20usize , concat ! (
                "Alignment of field: " , stringify ! ( gtoken_s ) , "::" ,
                stringify ! ( length ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gtoken_s ) ) . fileid as * const _ as
                usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( gtoken_s ) , "::" ,
                stringify ! ( fileid ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gtoken_s ) ) . builtin as * const _ as
                usize } , 28usize , concat ! (
                "Alignment of field: " , stringify ! ( gtoken_s ) , "::" ,
                stringify ! ( builtin ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gtoken_s ) ) . value as * const _ as
                usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( gtoken_s ) , "::" ,
                stringify ! ( value ) ));
}
impl Clone for gtoken_s {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    pub fn token_string(token: gtoken_s, len: *mut u32)
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn token_name(token: gtoken_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn token_keyword(buffer: *const ::std::os::raw::c_char, len: i32)
     -> gtoken_t;
}
extern "C" {
    pub fn token_special_builtin(token: *mut gtoken_s) -> gtoken_t;
}
extern "C" {
    pub fn token_keywords_indexes(idx_start: *mut u32, idx_end: *mut u32);
}
extern "C" {
    pub fn token_literal_name(value: gliteral_t)
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn token_islabel_statement(token: gtoken_t) -> bool;
}
extern "C" {
    pub fn token_isflow_statement(token: gtoken_t) -> bool;
}
extern "C" {
    pub fn token_isloop_statement(token: gtoken_t) -> bool;
}
extern "C" {
    pub fn token_isjump_statement(token: gtoken_t) -> bool;
}
extern "C" {
    pub fn token_iscompound_statement(token: gtoken_t) -> bool;
}
extern "C" {
    pub fn token_isdeclaration_statement(token: gtoken_t) -> bool;
}
extern "C" {
    pub fn token_isempty_statement(token: gtoken_t) -> bool;
}
extern "C" {
    pub fn token_isimport_statement(token: gtoken_t) -> bool;
}
extern "C" {
    pub fn token_isspecial_statement(token: gtoken_t) -> bool;
}
extern "C" {
    pub fn token_isoperator(token: gtoken_t) -> bool;
}
extern "C" {
    pub fn token_ismacro(token: gtoken_t) -> bool;
}
extern "C" {
    pub fn token_iserror(token: gtoken_t) -> bool;
}
extern "C" {
    pub fn token_iseof(token: gtoken_t) -> bool;
}
extern "C" {
    pub fn token_isidentifier(token: gtoken_t) -> bool;
}
extern "C" {
    pub fn token_isvariable_declaration(token: gtoken_t) -> bool;
}
extern "C" {
    pub fn token_isstatement(token: gtoken_t) -> bool;
}
extern "C" {
    pub fn token_isassignment(token: gtoken_t) -> bool;
}
extern "C" {
    pub fn token_isvariable_assignment(token: gtoken_t) -> bool;
}
extern "C" {
    pub fn token_isaccess_specifier(token: gtoken_t) -> bool;
}
extern "C" {
    pub fn token_isstorage_specifier(token: gtoken_t) -> bool;
}
extern "C" {
    pub fn token_isprimary_expression(token: gtoken_t) -> bool;
}
extern "C" {
    pub fn token_isexpression_statement(token: gtoken_t) -> bool;
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
#[test]
fn bindgen_test_layout_gnode_t() {
    assert_eq!(::std::mem::size_of::<gnode_t>() , 72usize , concat ! (
               "Size of: " , stringify ! ( gnode_t ) ));
    assert_eq! (::std::mem::align_of::<gnode_t>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( gnode_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_t ) ) . tag as * const _ as usize }
                , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_t ) , "::" ,
                stringify ! ( tag ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_t ) ) . refcount as * const _ as
                usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_t ) , "::" ,
                stringify ! ( refcount ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_t ) ) . token as * const _ as usize
                } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_t ) , "::" ,
                stringify ! ( token ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_t ) ) . is_assignment as * const _
                as usize } , 48usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_t ) , "::" ,
                stringify ! ( is_assignment ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_t ) ) . meta as * const _ as usize
                } , 56usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_t ) , "::" ,
                stringify ! ( meta ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_t ) ) . decl as * const _ as usize
                } , 64usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_t ) , "::" ,
                stringify ! ( decl ) ));
}
impl Clone for gnode_t {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gupvalue_t {
    pub node: *mut gnode_t,
    pub index: u32,
    pub selfindex: u32,
    pub is_direct: bool,
}
#[test]
fn bindgen_test_layout_gupvalue_t() {
    assert_eq!(::std::mem::size_of::<gupvalue_t>() , 24usize , concat ! (
               "Size of: " , stringify ! ( gupvalue_t ) ));
    assert_eq! (::std::mem::align_of::<gupvalue_t>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( gupvalue_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gupvalue_t ) ) . node as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gupvalue_t ) , "::" ,
                stringify ! ( node ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gupvalue_t ) ) . index as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( gupvalue_t ) , "::" ,
                stringify ! ( index ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gupvalue_t ) ) . selfindex as * const _
                as usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( gupvalue_t ) , "::" ,
                stringify ! ( selfindex ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gupvalue_t ) ) . is_direct as * const _
                as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( gupvalue_t ) , "::" ,
                stringify ! ( is_direct ) ));
}
impl Clone for gupvalue_t {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_r {
    pub n: usize,
    pub m: usize,
    pub p: *mut *mut gnode_t,
}
#[test]
fn bindgen_test_layout_gnode_r() {
    assert_eq!(::std::mem::size_of::<gnode_r>() , 24usize , concat ! (
               "Size of: " , stringify ! ( gnode_r ) ));
    assert_eq! (::std::mem::align_of::<gnode_r>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( gnode_r ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_r ) ) . n as * const _ as usize } ,
                0usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_r ) , "::" ,
                stringify ! ( n ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_r ) ) . m as * const _ as usize } ,
                8usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_r ) , "::" ,
                stringify ! ( m ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_r ) ) . p as * const _ as usize } ,
                16usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_r ) , "::" ,
                stringify ! ( p ) ));
}
impl Clone for gnode_r {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gupvalue_r {
    pub n: usize,
    pub m: usize,
    pub p: *mut *mut gupvalue_t,
}
#[test]
fn bindgen_test_layout_gupvalue_r() {
    assert_eq!(::std::mem::size_of::<gupvalue_r>() , 24usize , concat ! (
               "Size of: " , stringify ! ( gupvalue_r ) ));
    assert_eq! (::std::mem::align_of::<gupvalue_r>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( gupvalue_r ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gupvalue_r ) ) . n as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gupvalue_r ) , "::" ,
                stringify ! ( n ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gupvalue_r ) ) . m as * const _ as usize
                } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( gupvalue_r ) , "::" ,
                stringify ! ( m ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gupvalue_r ) ) . p as * const _ as usize
                } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( gupvalue_r ) , "::" ,
                stringify ! ( p ) ));
}
impl Clone for gupvalue_r {
    fn clone(&self) -> Self { *self }
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
#[test]
fn bindgen_test_layout_gnode_location_t() {
    assert_eq!(::std::mem::size_of::<gnode_location_t>() , 8usize , concat ! (
               "Size of: " , stringify ! ( gnode_location_t ) ));
    assert_eq! (::std::mem::align_of::<gnode_location_t>() , 4usize , concat !
                ( "Alignment of " , stringify ! ( gnode_location_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_location_t ) ) . type_ as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_location_t ) ,
                "::" , stringify ! ( type_ ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_location_t ) ) . index as * const _
                as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_location_t ) ,
                "::" , stringify ! ( index ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_location_t ) ) . nup as * const _
                as usize } , 6usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_location_t ) ,
                "::" , stringify ! ( nup ) ));
}
impl Clone for gnode_location_t {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_compound_stmt_t {
    pub base: gnode_t,
    pub symtable: *mut symboltable_t,
    pub stmts: *mut gnode_r,
    pub nclose: u32,
}
#[test]
fn bindgen_test_layout_gnode_compound_stmt_t() {
    assert_eq!(::std::mem::size_of::<gnode_compound_stmt_t>() , 96usize ,
               concat ! ( "Size of: " , stringify ! ( gnode_compound_stmt_t )
               ));
    assert_eq! (::std::mem::align_of::<gnode_compound_stmt_t>() , 8usize ,
                concat ! (
                "Alignment of " , stringify ! ( gnode_compound_stmt_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_compound_stmt_t ) ) . base as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_compound_stmt_t )
                , "::" , stringify ! ( base ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_compound_stmt_t ) ) . symtable as *
                const _ as usize } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_compound_stmt_t )
                , "::" , stringify ! ( symtable ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_compound_stmt_t ) ) . stmts as *
                const _ as usize } , 80usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_compound_stmt_t )
                , "::" , stringify ! ( stmts ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_compound_stmt_t ) ) . nclose as *
                const _ as usize } , 88usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_compound_stmt_t )
                , "::" , stringify ! ( nclose ) ));
}
impl Clone for gnode_compound_stmt_t {
    fn clone(&self) -> Self { *self }
}
pub type gnode_list_stmt_t = gnode_compound_stmt_t;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_label_stmt_t {
    pub base: gnode_t,
    pub expr: *mut gnode_t,
    pub stmt: *mut gnode_t,
}
#[test]
fn bindgen_test_layout_gnode_label_stmt_t() {
    assert_eq!(::std::mem::size_of::<gnode_label_stmt_t>() , 88usize , concat
               ! ( "Size of: " , stringify ! ( gnode_label_stmt_t ) ));
    assert_eq! (::std::mem::align_of::<gnode_label_stmt_t>() , 8usize , concat
                ! ( "Alignment of " , stringify ! ( gnode_label_stmt_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_label_stmt_t ) ) . base as * const
                _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_label_stmt_t ) ,
                "::" , stringify ! ( base ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_label_stmt_t ) ) . expr as * const
                _ as usize } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_label_stmt_t ) ,
                "::" , stringify ! ( expr ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_label_stmt_t ) ) . stmt as * const
                _ as usize } , 80usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_label_stmt_t ) ,
                "::" , stringify ! ( stmt ) ));
}
impl Clone for gnode_label_stmt_t {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_flow_stmt_t {
    pub base: gnode_t,
    pub cond: *mut gnode_t,
    pub stmt: *mut gnode_t,
    pub elsestmt: *mut gnode_t,
}
#[test]
fn bindgen_test_layout_gnode_flow_stmt_t() {
    assert_eq!(::std::mem::size_of::<gnode_flow_stmt_t>() , 96usize , concat !
               ( "Size of: " , stringify ! ( gnode_flow_stmt_t ) ));
    assert_eq! (::std::mem::align_of::<gnode_flow_stmt_t>() , 8usize , concat
                ! ( "Alignment of " , stringify ! ( gnode_flow_stmt_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_flow_stmt_t ) ) . base as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_flow_stmt_t ) ,
                "::" , stringify ! ( base ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_flow_stmt_t ) ) . cond as * const _
                as usize } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_flow_stmt_t ) ,
                "::" , stringify ! ( cond ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_flow_stmt_t ) ) . stmt as * const _
                as usize } , 80usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_flow_stmt_t ) ,
                "::" , stringify ! ( stmt ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_flow_stmt_t ) ) . elsestmt as *
                const _ as usize } , 88usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_flow_stmt_t ) ,
                "::" , stringify ! ( elsestmt ) ));
}
impl Clone for gnode_flow_stmt_t {
    fn clone(&self) -> Self { *self }
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
#[test]
fn bindgen_test_layout_gnode_loop_stmt_t() {
    assert_eq!(::std::mem::size_of::<gnode_loop_stmt_t>() , 104usize , concat
               ! ( "Size of: " , stringify ! ( gnode_loop_stmt_t ) ));
    assert_eq! (::std::mem::align_of::<gnode_loop_stmt_t>() , 8usize , concat
                ! ( "Alignment of " , stringify ! ( gnode_loop_stmt_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_loop_stmt_t ) ) . base as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_loop_stmt_t ) ,
                "::" , stringify ! ( base ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_loop_stmt_t ) ) . cond as * const _
                as usize } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_loop_stmt_t ) ,
                "::" , stringify ! ( cond ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_loop_stmt_t ) ) . stmt as * const _
                as usize } , 80usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_loop_stmt_t ) ,
                "::" , stringify ! ( stmt ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_loop_stmt_t ) ) . expr as * const _
                as usize } , 88usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_loop_stmt_t ) ,
                "::" , stringify ! ( expr ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_loop_stmt_t ) ) . nclose as * const
                _ as usize } , 96usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_loop_stmt_t ) ,
                "::" , stringify ! ( nclose ) ));
}
impl Clone for gnode_loop_stmt_t {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_jump_stmt_t {
    pub base: gnode_t,
    pub expr: *mut gnode_t,
}
#[test]
fn bindgen_test_layout_gnode_jump_stmt_t() {
    assert_eq!(::std::mem::size_of::<gnode_jump_stmt_t>() , 80usize , concat !
               ( "Size of: " , stringify ! ( gnode_jump_stmt_t ) ));
    assert_eq! (::std::mem::align_of::<gnode_jump_stmt_t>() , 8usize , concat
                ! ( "Alignment of " , stringify ! ( gnode_jump_stmt_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_jump_stmt_t ) ) . base as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_jump_stmt_t ) ,
                "::" , stringify ! ( base ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_jump_stmt_t ) ) . expr as * const _
                as usize } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_jump_stmt_t ) ,
                "::" , stringify ! ( expr ) ));
}
impl Clone for gnode_jump_stmt_t {
    fn clone(&self) -> Self { *self }
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
#[test]
fn bindgen_test_layout_gnode_function_decl_t() {
    assert_eq!(::std::mem::size_of::<gnode_function_decl_t>() , 136usize ,
               concat ! ( "Size of: " , stringify ! ( gnode_function_decl_t )
               ));
    assert_eq! (::std::mem::align_of::<gnode_function_decl_t>() , 8usize ,
                concat ! (
                "Alignment of " , stringify ! ( gnode_function_decl_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_function_decl_t ) ) . base as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_function_decl_t )
                , "::" , stringify ! ( base ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_function_decl_t ) ) . env as *
                const _ as usize } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_function_decl_t )
                , "::" , stringify ! ( env ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_function_decl_t ) ) . access as *
                const _ as usize } , 80usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_function_decl_t )
                , "::" , stringify ! ( access ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_function_decl_t ) ) . storage as *
                const _ as usize } , 84usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_function_decl_t )
                , "::" , stringify ! ( storage ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_function_decl_t ) ) . symtable as *
                const _ as usize } , 88usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_function_decl_t )
                , "::" , stringify ! ( symtable ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_function_decl_t ) ) . identifier as
                * const _ as usize } , 96usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_function_decl_t )
                , "::" , stringify ! ( identifier ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_function_decl_t ) ) . params as *
                const _ as usize } , 104usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_function_decl_t )
                , "::" , stringify ! ( params ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_function_decl_t ) ) . block as *
                const _ as usize } , 112usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_function_decl_t )
                , "::" , stringify ! ( block ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_function_decl_t ) ) . nlocals as *
                const _ as usize } , 120usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_function_decl_t )
                , "::" , stringify ! ( nlocals ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_function_decl_t ) ) . nparams as *
                const _ as usize } , 122usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_function_decl_t )
                , "::" , stringify ! ( nparams ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_function_decl_t ) ) . uplist as *
                const _ as usize } , 128usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_function_decl_t )
                , "::" , stringify ! ( uplist ) ));
}
impl Clone for gnode_function_decl_t {
    fn clone(&self) -> Self { *self }
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
#[test]
fn bindgen_test_layout_gnode_var_t() {
    assert_eq!(::std::mem::size_of::<gnode_var_t>() , 112usize , concat ! (
               "Size of: " , stringify ! ( gnode_var_t ) ));
    assert_eq! (::std::mem::align_of::<gnode_var_t>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( gnode_var_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_var_t ) ) . base as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_var_t ) , "::" ,
                stringify ! ( base ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_var_t ) ) . env as * const _ as
                usize } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_var_t ) , "::" ,
                stringify ! ( env ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_var_t ) ) . identifier as * const _
                as usize } , 80usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_var_t ) , "::" ,
                stringify ! ( identifier ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_var_t ) ) . annotation_type as *
                const _ as usize } , 88usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_var_t ) , "::" ,
                stringify ! ( annotation_type ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_var_t ) ) . expr as * const _ as
                usize } , 96usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_var_t ) , "::" ,
                stringify ! ( expr ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_var_t ) ) . access as * const _ as
                usize } , 104usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_var_t ) , "::" ,
                stringify ! ( access ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_var_t ) ) . index as * const _ as
                usize } , 108usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_var_t ) , "::" ,
                stringify ! ( index ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_var_t ) ) . upvalue as * const _ as
                usize } , 110usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_var_t ) , "::" ,
                stringify ! ( upvalue ) ));
}
impl Clone for gnode_var_t {
    fn clone(&self) -> Self { *self }
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
#[test]
fn bindgen_test_layout_gnode_variable_decl_t() {
    assert_eq!(::std::mem::size_of::<gnode_variable_decl_t>() , 96usize ,
               concat ! ( "Size of: " , stringify ! ( gnode_variable_decl_t )
               ));
    assert_eq! (::std::mem::align_of::<gnode_variable_decl_t>() , 8usize ,
                concat ! (
                "Alignment of " , stringify ! ( gnode_variable_decl_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_variable_decl_t ) ) . base as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_variable_decl_t )
                , "::" , stringify ! ( base ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_variable_decl_t ) ) . type_ as *
                const _ as usize } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_variable_decl_t )
                , "::" , stringify ! ( type_ ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_variable_decl_t ) ) . access as *
                const _ as usize } , 76usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_variable_decl_t )
                , "::" , stringify ! ( access ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_variable_decl_t ) ) . storage as *
                const _ as usize } , 80usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_variable_decl_t )
                , "::" , stringify ! ( storage ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_variable_decl_t ) ) . decls as *
                const _ as usize } , 88usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_variable_decl_t )
                , "::" , stringify ! ( decls ) ));
}
impl Clone for gnode_variable_decl_t {
    fn clone(&self) -> Self { *self }
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
#[test]
fn bindgen_test_layout_gnode_enum_decl_t() {
    assert_eq!(::std::mem::size_of::<gnode_enum_decl_t>() , 104usize , concat
               ! ( "Size of: " , stringify ! ( gnode_enum_decl_t ) ));
    assert_eq! (::std::mem::align_of::<gnode_enum_decl_t>() , 8usize , concat
                ! ( "Alignment of " , stringify ! ( gnode_enum_decl_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_enum_decl_t ) ) . base as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_enum_decl_t ) ,
                "::" , stringify ! ( base ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_enum_decl_t ) ) . env as * const _
                as usize } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_enum_decl_t ) ,
                "::" , stringify ! ( env ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_enum_decl_t ) ) . access as * const
                _ as usize } , 80usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_enum_decl_t ) ,
                "::" , stringify ! ( access ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_enum_decl_t ) ) . storage as *
                const _ as usize } , 84usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_enum_decl_t ) ,
                "::" , stringify ! ( storage ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_enum_decl_t ) ) . symtable as *
                const _ as usize } , 88usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_enum_decl_t ) ,
                "::" , stringify ! ( symtable ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_enum_decl_t ) ) . identifier as *
                const _ as usize } , 96usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_enum_decl_t ) ,
                "::" , stringify ! ( identifier ) ));
}
impl Clone for gnode_enum_decl_t {
    fn clone(&self) -> Self { *self }
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
#[test]
fn bindgen_test_layout_gnode_class_decl_t() {
    assert_eq!(::std::mem::size_of::<gnode_class_decl_t>() , 152usize , concat
               ! ( "Size of: " , stringify ! ( gnode_class_decl_t ) ));
    assert_eq! (::std::mem::align_of::<gnode_class_decl_t>() , 8usize , concat
                ! ( "Alignment of " , stringify ! ( gnode_class_decl_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_class_decl_t ) ) . base as * const
                _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_class_decl_t ) ,
                "::" , stringify ! ( base ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_class_decl_t ) ) . bridge as *
                const _ as usize } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_class_decl_t ) ,
                "::" , stringify ! ( bridge ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_class_decl_t ) ) . is_struct as *
                const _ as usize } , 73usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_class_decl_t ) ,
                "::" , stringify ! ( is_struct ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_class_decl_t ) ) . env as * const _
                as usize } , 80usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_class_decl_t ) ,
                "::" , stringify ! ( env ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_class_decl_t ) ) . access as *
                const _ as usize } , 88usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_class_decl_t ) ,
                "::" , stringify ! ( access ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_class_decl_t ) ) . storage as *
                const _ as usize } , 92usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_class_decl_t ) ,
                "::" , stringify ! ( storage ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_class_decl_t ) ) . identifier as *
                const _ as usize } , 96usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_class_decl_t ) ,
                "::" , stringify ! ( identifier ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_class_decl_t ) ) . superclass as *
                const _ as usize } , 104usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_class_decl_t ) ,
                "::" , stringify ! ( superclass ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_class_decl_t ) ) . protocols as *
                const _ as usize } , 112usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_class_decl_t ) ,
                "::" , stringify ! ( protocols ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_class_decl_t ) ) . decls as * const
                _ as usize } , 120usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_class_decl_t ) ,
                "::" , stringify ! ( decls ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_class_decl_t ) ) . symtable as *
                const _ as usize } , 128usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_class_decl_t ) ,
                "::" , stringify ! ( symtable ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_class_decl_t ) ) . data as * const
                _ as usize } , 136usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_class_decl_t ) ,
                "::" , stringify ! ( data ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_class_decl_t ) ) . nivar as * const
                _ as usize } , 144usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_class_decl_t ) ,
                "::" , stringify ! ( nivar ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_class_decl_t ) ) . nsvar as * const
                _ as usize } , 148usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_class_decl_t ) ,
                "::" , stringify ! ( nsvar ) ));
}
impl Clone for gnode_class_decl_t {
    fn clone(&self) -> Self { *self }
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
#[test]
fn bindgen_test_layout_gnode_module_decl_t() {
    assert_eq!(::std::mem::size_of::<gnode_module_decl_t>() , 112usize ,
               concat ! ( "Size of: " , stringify ! ( gnode_module_decl_t )
               ));
    assert_eq! (::std::mem::align_of::<gnode_module_decl_t>() , 8usize ,
                concat ! (
                "Alignment of " , stringify ! ( gnode_module_decl_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_module_decl_t ) ) . base as * const
                _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_module_decl_t ) ,
                "::" , stringify ! ( base ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_module_decl_t ) ) . env as * const
                _ as usize } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_module_decl_t ) ,
                "::" , stringify ! ( env ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_module_decl_t ) ) . access as *
                const _ as usize } , 80usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_module_decl_t ) ,
                "::" , stringify ! ( access ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_module_decl_t ) ) . storage as *
                const _ as usize } , 84usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_module_decl_t ) ,
                "::" , stringify ! ( storage ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_module_decl_t ) ) . identifier as *
                const _ as usize } , 88usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_module_decl_t ) ,
                "::" , stringify ! ( identifier ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_module_decl_t ) ) . decls as *
                const _ as usize } , 96usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_module_decl_t ) ,
                "::" , stringify ! ( decls ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_module_decl_t ) ) . symtable as *
                const _ as usize } , 104usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_module_decl_t ) ,
                "::" , stringify ! ( symtable ) ));
}
impl Clone for gnode_module_decl_t {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_binary_expr_t {
    pub base: gnode_t,
    pub op: gtoken_t,
    pub left: *mut gnode_t,
    pub right: *mut gnode_t,
}
#[test]
fn bindgen_test_layout_gnode_binary_expr_t() {
    assert_eq!(::std::mem::size_of::<gnode_binary_expr_t>() , 96usize , concat
               ! ( "Size of: " , stringify ! ( gnode_binary_expr_t ) ));
    assert_eq! (::std::mem::align_of::<gnode_binary_expr_t>() , 8usize ,
                concat ! (
                "Alignment of " , stringify ! ( gnode_binary_expr_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_binary_expr_t ) ) . base as * const
                _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_binary_expr_t ) ,
                "::" , stringify ! ( base ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_binary_expr_t ) ) . op as * const _
                as usize } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_binary_expr_t ) ,
                "::" , stringify ! ( op ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_binary_expr_t ) ) . left as * const
                _ as usize } , 80usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_binary_expr_t ) ,
                "::" , stringify ! ( left ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_binary_expr_t ) ) . right as *
                const _ as usize } , 88usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_binary_expr_t ) ,
                "::" , stringify ! ( right ) ));
}
impl Clone for gnode_binary_expr_t {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_unary_expr_t {
    pub base: gnode_t,
    pub op: gtoken_t,
    pub expr: *mut gnode_t,
}
#[test]
fn bindgen_test_layout_gnode_unary_expr_t() {
    assert_eq!(::std::mem::size_of::<gnode_unary_expr_t>() , 88usize , concat
               ! ( "Size of: " , stringify ! ( gnode_unary_expr_t ) ));
    assert_eq! (::std::mem::align_of::<gnode_unary_expr_t>() , 8usize , concat
                ! ( "Alignment of " , stringify ! ( gnode_unary_expr_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_unary_expr_t ) ) . base as * const
                _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_unary_expr_t ) ,
                "::" , stringify ! ( base ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_unary_expr_t ) ) . op as * const _
                as usize } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_unary_expr_t ) ,
                "::" , stringify ! ( op ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_unary_expr_t ) ) . expr as * const
                _ as usize } , 80usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_unary_expr_t ) ,
                "::" , stringify ! ( expr ) ));
}
impl Clone for gnode_unary_expr_t {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_file_expr_t {
    pub base: gnode_t,
    pub identifiers: *mut cstring_r,
    pub location: gnode_location_t,
}
#[test]
fn bindgen_test_layout_gnode_file_expr_t() {
    assert_eq!(::std::mem::size_of::<gnode_file_expr_t>() , 88usize , concat !
               ( "Size of: " , stringify ! ( gnode_file_expr_t ) ));
    assert_eq! (::std::mem::align_of::<gnode_file_expr_t>() , 8usize , concat
                ! ( "Alignment of " , stringify ! ( gnode_file_expr_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_file_expr_t ) ) . base as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_file_expr_t ) ,
                "::" , stringify ! ( base ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_file_expr_t ) ) . identifiers as *
                const _ as usize } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_file_expr_t ) ,
                "::" , stringify ! ( identifiers ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_file_expr_t ) ) . location as *
                const _ as usize } , 80usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_file_expr_t ) ,
                "::" , stringify ! ( location ) ));
}
impl Clone for gnode_file_expr_t {
    fn clone(&self) -> Self { *self }
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
#[test]
fn bindgen_test_layout_gnode_literal_expr_t__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<gnode_literal_expr_t__bindgen_ty_1>() ,
               8usize , concat ! (
               "Size of: " , stringify ! ( gnode_literal_expr_t__bindgen_ty_1
               ) ));
    assert_eq! (::std::mem::align_of::<gnode_literal_expr_t__bindgen_ty_1>() ,
                8usize , concat ! (
                "Alignment of " , stringify ! (
                gnode_literal_expr_t__bindgen_ty_1 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_literal_expr_t__bindgen_ty_1 ) ) .
                str as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                gnode_literal_expr_t__bindgen_ty_1 ) , "::" , stringify ! (
                str ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_literal_expr_t__bindgen_ty_1 ) ) .
                d as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                gnode_literal_expr_t__bindgen_ty_1 ) , "::" , stringify ! ( d
                ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_literal_expr_t__bindgen_ty_1 ) ) .
                n64 as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                gnode_literal_expr_t__bindgen_ty_1 ) , "::" , stringify ! (
                n64 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_literal_expr_t__bindgen_ty_1 ) ) .
                r as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                gnode_literal_expr_t__bindgen_ty_1 ) , "::" , stringify ! ( r
                ) ));
}
impl Clone for gnode_literal_expr_t__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_gnode_literal_expr_t() {
    assert_eq!(::std::mem::size_of::<gnode_literal_expr_t>() , 88usize ,
               concat ! ( "Size of: " , stringify ! ( gnode_literal_expr_t )
               ));
    assert_eq! (::std::mem::align_of::<gnode_literal_expr_t>() , 8usize ,
                concat ! (
                "Alignment of " , stringify ! ( gnode_literal_expr_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_literal_expr_t ) ) . base as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_literal_expr_t )
                , "::" , stringify ! ( base ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_literal_expr_t ) ) . type_ as *
                const _ as usize } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_literal_expr_t )
                , "::" , stringify ! ( type_ ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_literal_expr_t ) ) . len as * const
                _ as usize } , 76usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_literal_expr_t )
                , "::" , stringify ! ( len ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_literal_expr_t ) ) . value as *
                const _ as usize } , 80usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_literal_expr_t )
                , "::" , stringify ! ( value ) ));
}
impl Clone for gnode_literal_expr_t {
    fn clone(&self) -> Self { *self }
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
#[test]
fn bindgen_test_layout_gnode_identifier_expr_t() {
    assert_eq!(::std::mem::size_of::<gnode_identifier_expr_t>() , 112usize ,
               concat ! (
               "Size of: " , stringify ! ( gnode_identifier_expr_t ) ));
    assert_eq! (::std::mem::align_of::<gnode_identifier_expr_t>() , 8usize ,
                concat ! (
                "Alignment of " , stringify ! ( gnode_identifier_expr_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_identifier_expr_t ) ) . base as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_identifier_expr_t
                ) , "::" , stringify ! ( base ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_identifier_expr_t ) ) . value as *
                const _ as usize } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_identifier_expr_t
                ) , "::" , stringify ! ( value ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_identifier_expr_t ) ) . value2 as *
                const _ as usize } , 80usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_identifier_expr_t
                ) , "::" , stringify ! ( value2 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_identifier_expr_t ) ) . symbol as *
                const _ as usize } , 88usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_identifier_expr_t
                ) , "::" , stringify ! ( symbol ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_identifier_expr_t ) ) . location as
                * const _ as usize } , 96usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_identifier_expr_t
                ) , "::" , stringify ! ( location ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_identifier_expr_t ) ) . upvalue as
                * const _ as usize } , 104usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_identifier_expr_t
                ) , "::" , stringify ! ( upvalue ) ));
}
impl Clone for gnode_identifier_expr_t {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_keyword_expr_t {
    pub base: gnode_t,
}
#[test]
fn bindgen_test_layout_gnode_keyword_expr_t() {
    assert_eq!(::std::mem::size_of::<gnode_keyword_expr_t>() , 72usize ,
               concat ! ( "Size of: " , stringify ! ( gnode_keyword_expr_t )
               ));
    assert_eq! (::std::mem::align_of::<gnode_keyword_expr_t>() , 8usize ,
                concat ! (
                "Alignment of " , stringify ! ( gnode_keyword_expr_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_keyword_expr_t ) ) . base as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_keyword_expr_t )
                , "::" , stringify ! ( base ) ));
}
impl Clone for gnode_keyword_expr_t {
    fn clone(&self) -> Self { *self }
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
#[test]
fn bindgen_test_layout_gnode_postfix_expr_t() {
    assert_eq!(::std::mem::size_of::<gnode_postfix_expr_t>() , 88usize ,
               concat ! ( "Size of: " , stringify ! ( gnode_postfix_expr_t )
               ));
    assert_eq! (::std::mem::align_of::<gnode_postfix_expr_t>() , 8usize ,
                concat ! (
                "Alignment of " , stringify ! ( gnode_postfix_expr_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_postfix_expr_t ) ) . base as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_postfix_expr_t )
                , "::" , stringify ! ( base ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_postfix_expr_t ) ) . id as * const
                _ as usize } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_postfix_expr_t )
                , "::" , stringify ! ( id ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_postfix_expr_t ) ) . list as *
                const _ as usize } , 80usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_postfix_expr_t )
                , "::" , stringify ! ( list ) ));
}
impl Clone for gnode_postfix_expr_t {
    fn clone(&self) -> Self { *self }
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
#[test]
fn bindgen_test_layout_gnode_postfix_subexpr_t__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<gnode_postfix_subexpr_t__bindgen_ty_1>()
               , 8usize , concat ! (
               "Size of: " , stringify ! (
               gnode_postfix_subexpr_t__bindgen_ty_1 ) ));
    assert_eq! (::std::mem::align_of::<gnode_postfix_subexpr_t__bindgen_ty_1>()
                , 8usize , concat ! (
                "Alignment of " , stringify ! (
                gnode_postfix_subexpr_t__bindgen_ty_1 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_postfix_subexpr_t__bindgen_ty_1 ) )
                . expr as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                gnode_postfix_subexpr_t__bindgen_ty_1 ) , "::" , stringify ! (
                expr ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_postfix_subexpr_t__bindgen_ty_1 ) )
                . args as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                gnode_postfix_subexpr_t__bindgen_ty_1 ) , "::" , stringify ! (
                args ) ));
}
impl Clone for gnode_postfix_subexpr_t__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_gnode_postfix_subexpr_t() {
    assert_eq!(::std::mem::size_of::<gnode_postfix_subexpr_t>() , 80usize ,
               concat ! (
               "Size of: " , stringify ! ( gnode_postfix_subexpr_t ) ));
    assert_eq! (::std::mem::align_of::<gnode_postfix_subexpr_t>() , 8usize ,
                concat ! (
                "Alignment of " , stringify ! ( gnode_postfix_subexpr_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_postfix_subexpr_t ) ) . base as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_postfix_subexpr_t
                ) , "::" , stringify ! ( base ) ));
}
impl Clone for gnode_postfix_subexpr_t {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct gnode_list_expr_t {
    pub base: gnode_t,
    pub ismap: bool,
    pub list1: *mut gnode_r,
    pub list2: *mut gnode_r,
}
#[test]
fn bindgen_test_layout_gnode_list_expr_t() {
    assert_eq!(::std::mem::size_of::<gnode_list_expr_t>() , 96usize , concat !
               ( "Size of: " , stringify ! ( gnode_list_expr_t ) ));
    assert_eq! (::std::mem::align_of::<gnode_list_expr_t>() , 8usize , concat
                ! ( "Alignment of " , stringify ! ( gnode_list_expr_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_list_expr_t ) ) . base as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_list_expr_t ) ,
                "::" , stringify ! ( base ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_list_expr_t ) ) . ismap as * const
                _ as usize } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_list_expr_t ) ,
                "::" , stringify ! ( ismap ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_list_expr_t ) ) . list1 as * const
                _ as usize } , 80usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_list_expr_t ) ,
                "::" , stringify ! ( list1 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const gnode_list_expr_t ) ) . list2 as * const
                _ as usize } , 88usize , concat ! (
                "Alignment of field: " , stringify ! ( gnode_list_expr_t ) ,
                "::" , stringify ! ( list2 ) ));
}
impl Clone for gnode_list_expr_t {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    pub fn gnode_jump_stat_create(token: gtoken_s, expr: *mut gnode_t,
                                  decl: *mut gnode_t) -> *mut gnode_t;
}
extern "C" {
    pub fn gnode_label_stat_create(token: gtoken_s, expr: *mut gnode_t,
                                   stmt: *mut gnode_t, decl: *mut gnode_t)
     -> *mut gnode_t;
}
extern "C" {
    pub fn gnode_flow_stat_create(token: gtoken_s, cond: *mut gnode_t,
                                  stmt1: *mut gnode_t, stmt2: *mut gnode_t,
                                  decl: *mut gnode_t) -> *mut gnode_t;
}
extern "C" {
    pub fn gnode_loop_stat_create(token: gtoken_s, cond: *mut gnode_t,
                                  stmt: *mut gnode_t, expr: *mut gnode_t,
                                  decl: *mut gnode_t) -> *mut gnode_t;
}
extern "C" {
    pub fn gnode_block_stat_create(type_: gnode_n, token: gtoken_s,
                                   stmts: *mut gnode_r, decl: *mut gnode_t)
     -> *mut gnode_t;
}
extern "C" {
    pub fn gnode_empty_stat_create(token: gtoken_s, decl: *mut gnode_t)
     -> *mut gnode_t;
}
extern "C" {
    pub fn gnode_enum_decl_create(token: gtoken_s,
                                  identifier: *const ::std::os::raw::c_char,
                                  access_specifier: gtoken_t,
                                  storage_specifier: gtoken_t,
                                  symtable: *mut symboltable_t,
                                  meta: *mut ::std::os::raw::c_void,
                                  decl: *mut gnode_t) -> *mut gnode_t;
}
extern "C" {
    pub fn gnode_class_decl_create(token: gtoken_s,
                                   identifier: *const ::std::os::raw::c_char,
                                   access_specifier: gtoken_t,
                                   storage_specifier: gtoken_t,
                                   superclass: *mut gnode_t,
                                   protocols: *mut gnode_r,
                                   declarations: *mut gnode_r,
                                   is_struct: bool,
                                   meta: *mut ::std::os::raw::c_void,
                                   decl: *mut gnode_t) -> *mut gnode_t;
}
extern "C" {
    pub fn gnode_module_decl_create(token: gtoken_s,
                                    identifier: *const ::std::os::raw::c_char,
                                    access_specifier: gtoken_t,
                                    storage_specifier: gtoken_t,
                                    declarations: *mut gnode_r,
                                    meta: *mut ::std::os::raw::c_void,
                                    decl: *mut gnode_t) -> *mut gnode_t;
}
extern "C" {
    pub fn gnode_variable_decl_create(token: gtoken_s, type_: gtoken_t,
                                      access_specifier: gtoken_t,
                                      storage_specifier: gtoken_t,
                                      declarations: *mut gnode_r,
                                      meta: *mut ::std::os::raw::c_void,
                                      decl: *mut gnode_t) -> *mut gnode_t;
}
extern "C" {
    pub fn gnode_variable_create(token: gtoken_s,
                                 identifier: *const ::std::os::raw::c_char,
                                 annotation_type:
                                     *const ::std::os::raw::c_char,
                                 storage_specifier: gtoken_t,
                                 expr: *mut gnode_t, decl: *mut gnode_t)
     -> *mut gnode_t;
}
extern "C" {
    pub fn gnode_function_decl_create(token: gtoken_s,
                                      identifier:
                                          *const ::std::os::raw::c_char,
                                      access_specifier: gtoken_t,
                                      storage_specifier: gtoken_t,
                                      params: *mut gnode_r,
                                      block: *mut gnode_compound_stmt_t,
                                      meta: *mut ::std::os::raw::c_void,
                                      decl: *mut gnode_t) -> *mut gnode_t;
}
extern "C" {
    pub fn gnode_binary_expr_create(op: gtoken_t, left: *mut gnode_t,
                                    right: *mut gnode_t, decl: *mut gnode_t)
     -> *mut gnode_t;
}
extern "C" {
    pub fn gnode_unary_expr_create(op: gtoken_t, expr: *mut gnode_t,
                                   decl: *mut gnode_t) -> *mut gnode_t;
}
extern "C" {
    pub fn gnode_file_expr_create(token: gtoken_s, list: *mut cstring_r,
                                  decl: *mut gnode_t) -> *mut gnode_t;
}
extern "C" {
    pub fn gnode_identifier_expr_create(token: gtoken_s,
                                        identifier:
                                            *const ::std::os::raw::c_char,
                                        identifier2:
                                            *const ::std::os::raw::c_char,
                                        decl: *mut gnode_t) -> *mut gnode_t;
}
extern "C" {
    pub fn gnode_string_interpolation_create(token: gtoken_s, r: *mut gnode_r,
                                             decl: *mut gnode_t)
     -> *mut gnode_t;
}
extern "C" {
    pub fn gnode_literal_string_expr_create(token: gtoken_s,
                                            s: *mut ::std::os::raw::c_char,
                                            len: u32, allocated: bool,
                                            decl: *mut gnode_t)
     -> *mut gnode_t;
}
extern "C" {
    pub fn gnode_literal_float_expr_create(token: gtoken_s, f: f64,
                                           decl: *mut gnode_t)
     -> *mut gnode_t;
}
extern "C" {
    pub fn gnode_literal_int_expr_create(token: gtoken_s, n: i64,
                                         decl: *mut gnode_t) -> *mut gnode_t;
}
extern "C" {
    pub fn gnode_literal_bool_expr_create(token: gtoken_s, n: i32,
                                          decl: *mut gnode_t) -> *mut gnode_t;
}
extern "C" {
    pub fn gnode_keyword_expr_create(token: gtoken_s, decl: *mut gnode_t)
     -> *mut gnode_t;
}
extern "C" {
    pub fn gnode_postfix_subexpr_create(token: gtoken_s, type_: gnode_n,
                                        expr: *mut gnode_t,
                                        list: *mut gnode_r,
                                        decl: *mut gnode_t) -> *mut gnode_t;
}
extern "C" {
    pub fn gnode_postfix_expr_create(token: gtoken_s, id: *mut gnode_t,
                                     list: *mut gnode_r, decl: *mut gnode_t)
     -> *mut gnode_t;
}
extern "C" {
    pub fn gnode_list_expr_create(token: gtoken_s, list1: *mut gnode_r,
                                  list2: *mut gnode_r, ismap: bool,
                                  decl: *mut gnode_t) -> *mut gnode_t;
}
extern "C" {
    pub fn gnode_duplicate(node: *mut gnode_t, deep: bool) -> *mut gnode_t;
}
extern "C" {
    pub fn gnode_array_create() -> *mut gnode_r;
}
extern "C" {
    pub fn gnode_array_remove_byindex(list: *mut gnode_r, index: usize)
     -> *mut gnode_r;
}
extern "C" {
    pub fn gnode_function_add_upvalue(f: *mut gnode_function_decl_t,
                                      symbol: *mut gnode_var_t, n: u16)
     -> *mut gupvalue_t;
}
extern "C" {
    pub fn cstring_array_create() -> *mut cstring_r;
}
extern "C" {
    pub fn void_array_create() -> *mut void_r;
}
extern "C" {
    pub fn gnode_array_sethead(list: *mut gnode_r, node: *mut gnode_t);
}
extern "C" {
    pub fn gnode_is_equal(node1: *mut gnode_t, node2: *mut gnode_t) -> bool;
}
extern "C" {
    pub fn gnode_is_expression(node: *mut gnode_t) -> bool;
}
extern "C" {
    pub fn gnode_is_literal(node: *mut gnode_t) -> bool;
}
extern "C" {
    pub fn gnode_is_literal_int(node: *mut gnode_t) -> bool;
}
extern "C" {
    pub fn gnode_is_literal_number(node: *mut gnode_t) -> bool;
}
extern "C" {
    pub fn gnode_is_literal_string(node: *mut gnode_t) -> bool;
}
extern "C" {
    pub fn gnode_literal_dump(node: *mut gnode_literal_expr_t,
                              buffer: *mut ::std::os::raw::c_char,
                              buffersize: ::std::os::raw::c_int);
}
extern "C" {
    pub fn gnode_free(node: *mut gnode_t);
}
extern "C" {
    pub fn meta_from_node(node: *mut gnode_t) -> *mut ::std::os::raw::c_void;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gravity_compiler_t([u8; 0]);
extern "C" {
    pub fn gravity_compiler_create(delegate: *mut gravity_delegate_t)
     -> *mut gravity_compiler_t;
}
extern "C" {
    pub fn gravity_compiler_run(compiler: *mut gravity_compiler_t,
                                source: *const ::std::os::raw::c_char,
                                len: usize, fileid: u32, is_static: bool)
     -> *mut gravity_closure_t;
}
extern "C" {
    pub fn gravity_compiler_serialize(compiler: *mut gravity_compiler_t,
                                      closure: *mut gravity_closure_t)
     -> *mut json_t;
}
extern "C" {
    pub fn gravity_compiler_serialize_infile(compiler:
                                                 *mut gravity_compiler_t,
                                             closure: *mut gravity_closure_t,
                                             path:
                                                 *const ::std::os::raw::c_char)
     -> bool;
}
extern "C" {
    pub fn gravity_compiler_transfer(compiler: *mut gravity_compiler_t,
                                     vm: *mut gravity_vm);
}
extern "C" {
    pub fn gravity_compiler_ast(compiler: *mut gravity_compiler_t)
     -> *mut gnode_t;
}
extern "C" {
    pub fn gravity_compiler_free(compiler: *mut gravity_compiler_t);
}
pub type vm_filter_cb =
    ::std::option::Option<unsafe extern "C" fn(obj: *mut gravity_object_t)
                              -> bool>;
pub type vm_transfer_cb =
    ::std::option::Option<unsafe extern "C" fn(vm: *mut gravity_vm,
                                               obj: *mut gravity_object_t)>;
pub type vm_cleanup_cb =
    ::std::option::Option<unsafe extern "C" fn(vm: *mut gravity_vm)>;
extern "C" {
    pub fn gravity_vm_new(delegate: *mut gravity_delegate_t)
     -> *mut gravity_vm;
}
extern "C" {
    pub fn gravity_vm_newmini() -> *mut gravity_vm;
}
extern "C" {
    pub fn gravity_vm_set_callbacks(vm: *mut gravity_vm,
                                    vm_transfer: vm_transfer_cb,
                                    vm_cleanup: vm_cleanup_cb);
}
extern "C" {
    pub fn gravity_vm_free(vm: *mut gravity_vm);
}
extern "C" {
    pub fn gravity_vm_reset(vm: *mut gravity_vm);
}
extern "C" {
    pub fn gravity_vm_runclosure(vm: *mut gravity_vm,
                                 closure: *mut gravity_closure_t,
                                 selfvalue: gravity_value_t,
                                 params: *mut gravity_value_t, nparams: u16)
     -> bool;
}
extern "C" {
    pub fn gravity_vm_runmain(vm: *mut gravity_vm,
                              closure: *mut gravity_closure_t) -> bool;
}
extern "C" {
    pub fn gravity_vm_loadclosure(vm: *mut gravity_vm,
                                  closure: *mut gravity_closure_t);
}
extern "C" {
    pub fn gravity_vm_setvalue(vm: *mut gravity_vm,
                               key: *const ::std::os::raw::c_char,
                               value: gravity_value_t);
}
extern "C" {
    pub fn gravity_vm_lookup(vm: *mut gravity_vm, key: gravity_value_t)
     -> gravity_value_t;
}
extern "C" {
    pub fn gravity_vm_getvalue(vm: *mut gravity_vm,
                               key: *const ::std::os::raw::c_char,
                               keylen: u32) -> gravity_value_t;
}
extern "C" {
    pub fn gravity_vm_time(vm: *mut gravity_vm) -> f64;
}
extern "C" {
    pub fn gravity_vm_result(vm: *mut gravity_vm) -> gravity_value_t;
}
extern "C" {
    pub fn gravity_vm_delegate(vm: *mut gravity_vm)
     -> *mut gravity_delegate_t;
}
extern "C" {
    pub fn gravity_vm_fiber(vm: *mut gravity_vm) -> *mut gravity_fiber_t;
}
extern "C" {
    pub fn gravity_vm_setfiber(vm: *mut gravity_vm,
                               fiber: *mut gravity_fiber_t);
}
extern "C" {
    pub fn gravity_vm_seterror(vm: *mut gravity_vm,
                               format: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    pub fn gravity_vm_seterror_string(vm: *mut gravity_vm,
                                      s: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn gravity_vm_ismini(vm: *mut gravity_vm) -> bool;
}
extern "C" {
    pub fn gravity_vm_keyindex(vm: *mut gravity_vm, index: u32)
     -> gravity_value_t;
}
extern "C" {
    pub fn gravity_vm_isaborted(vm: *mut gravity_vm) -> bool;
}
extern "C" {
    pub fn gravity_vm_setaborted(vm: *mut gravity_vm);
}
extern "C" {
    pub fn gravity_vm_getclosure(vm: *mut gravity_vm)
     -> *mut gravity_closure_t;
}
extern "C" {
    pub fn gravity_gray_value(vm: *mut gravity_vm, v: gravity_value_t);
}
extern "C" {
    pub fn gravity_gray_object(vm: *mut gravity_vm,
                               obj: *mut gravity_object_t);
}
extern "C" {
    pub fn gravity_gc_start(vm: *mut gravity_vm);
}
extern "C" {
    pub fn gravity_gc_setenabled(vm: *mut gravity_vm, enabled: bool);
}
extern "C" {
    pub fn gravity_gc_push(vm: *mut gravity_vm, obj: *mut gravity_object_t);
}
extern "C" {
    pub fn gravity_gc_pop(vm: *mut gravity_vm);
}
extern "C" {
    pub fn gravity_vm_transfer(vm: *mut gravity_vm,
                               obj: *mut gravity_object_t);
}
extern "C" {
    pub fn gravity_vm_cleanup(vm: *mut gravity_vm);
}
extern "C" {
    pub fn gravity_vm_filter(vm: *mut gravity_vm,
                             cleanup_filter: vm_filter_cb);
}
extern "C" {
    pub fn gravity_vm_loadfile(vm: *mut gravity_vm,
                               path: *const ::std::os::raw::c_char)
     -> *mut gravity_closure_t;
}
extern "C" {
    pub fn gravity_vm_loadbuffer(vm: *mut gravity_vm,
                                 buffer: *const ::std::os::raw::c_char,
                                 len: usize) -> *mut gravity_closure_t;
}
extern "C" {
    pub fn gravity_vm_initmodule(vm: *mut gravity_vm,
                                 f: *mut gravity_function_t);
}
extern "C" {
    pub fn gravity_vm_fastlookup(vm: *mut gravity_vm, c: *mut gravity_class_t,
                                 index: ::std::os::raw::c_int)
     -> *mut gravity_closure_t;
}
extern "C" {
    pub fn gravity_vm_setslot(vm: *mut gravity_vm, value: gravity_value_t,
                              index: u32);
}
extern "C" {
    pub fn gravity_vm_getslot(vm: *mut gravity_vm, index: u32)
     -> gravity_value_t;
}
extern "C" {
    pub fn gravity_vm_setdata(vm: *mut gravity_vm,
                              data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn gravity_vm_getdata(vm: *mut gravity_vm)
     -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn gravity_vm_memupdate(vm: *mut gravity_vm, value: gravity_int_t);
}
extern "C" {
    pub fn gravity_vm_get(vm: *mut gravity_vm,
                          key: *const ::std::os::raw::c_char)
     -> gravity_value_t;
}
extern "C" {
    pub fn gravity_vm_set(vm: *mut gravity_vm,
                          key: *const ::std::os::raw::c_char,
                          value: gravity_value_t) -> bool;
}
extern "C" {
    pub fn gravity_vm_anonymous(vm: *mut gravity_vm)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn gravity_core_register(vm: *mut gravity_vm);
}
extern "C" {
    pub fn gravity_iscore_class(c: *mut gravity_class_t) -> bool;
}
extern "C" {
    pub fn gravity_core_free();
}
extern "C" {
    pub fn gravity_core_identifiers(id:
                                        *mut *mut *const ::std::os::raw::c_char)
     -> u32;
}
extern "C" {
    pub fn gravity_core_class_from_name(name: *const ::std::os::raw::c_char)
     -> *mut gravity_class_t;
}
extern "C" {
    pub fn convert_value2int(vm: *mut gravity_vm, v: gravity_value_t)
     -> gravity_value_t;
}
extern "C" {
    pub fn convert_value2float(vm: *mut gravity_vm, v: gravity_value_t)
     -> gravity_value_t;
}
extern "C" {
    pub fn convert_value2bool(vm: *mut gravity_vm, v: gravity_value_t)
     -> gravity_value_t;
}
extern "C" {
    pub fn convert_value2string(vm: *mut gravity_vm, v: gravity_value_t)
     -> gravity_value_t;
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout___va_list_tag() {
    assert_eq!(::std::mem::size_of::<__va_list_tag>() , 24usize , concat ! (
               "Size of: " , stringify ! ( __va_list_tag ) ));
    assert_eq! (::std::mem::align_of::<__va_list_tag>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( __va_list_tag ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const __va_list_tag ) ) . gp_offset as * const
                _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( __va_list_tag ) , "::"
                , stringify ! ( gp_offset ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const __va_list_tag ) ) . fp_offset as * const
                _ as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( __va_list_tag ) , "::"
                , stringify ! ( fp_offset ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const __va_list_tag ) ) . overflow_arg_area as
                * const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( __va_list_tag ) , "::"
                , stringify ! ( overflow_arg_area ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const __va_list_tag ) ) . reg_save_area as *
                const _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( __va_list_tag ) , "::"
                , stringify ! ( reg_save_area ) ));
}
impl Clone for __va_list_tag {
    fn clone(&self) -> Self { *self }
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy)]
pub struct __locale_data {
    pub _address: u8,
}
impl Clone for __locale_data {
    fn clone(&self) -> Self { *self }
}
