#![allow(non_upper_case_globals,
         non_camel_case_types,
         non_snake_case)]
extern crate gravity_sys;
use gravity_sys::*;

#[test]
fn bindgen_test_layout___fsid_t() {
    assert_eq!(::std::mem::size_of::<__fsid_t>(),
               8usize,
               concat!("Size of: ", stringify!(__fsid_t)));
    assert_eq!(::std::mem::align_of::<__fsid_t>(),
               4usize,
               concat!("Alignment of ", stringify!(__fsid_t)));
    assert_eq!(unsafe { &(*(0 as *const __fsid_t)).__val as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(__fsid_t),
                       "::",
                       stringify!(__val)));
}
#[test]
fn bindgen_test_layout_dirent() {
    assert_eq!(::std::mem::size_of::<dirent>(),
               280usize,
               concat!("Size of: ", stringify!(dirent)));
    assert_eq!(::std::mem::align_of::<dirent>(),
               8usize,
               concat!("Alignment of ", stringify!(dirent)));
    assert_eq!(unsafe { &(*(0 as *const dirent)).d_ino as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(dirent),
                       "::",
                       stringify!(d_ino)));
    assert_eq!(unsafe { &(*(0 as *const dirent)).d_off as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(dirent),
                       "::",
                       stringify!(d_off)));
    assert_eq!(unsafe { &(*(0 as *const dirent)).d_reclen as *const _ as usize },
               16usize,
               concat!("Alignment of field: ",
                       stringify!(dirent),
                       "::",
                       stringify!(d_reclen)));
    assert_eq!(unsafe { &(*(0 as *const dirent)).d_type as *const _ as usize },
               18usize,
               concat!("Alignment of field: ",
                       stringify!(dirent),
                       "::",
                       stringify!(d_type)));
    assert_eq!(unsafe { &(*(0 as *const dirent)).d_name as *const _ as usize },
               19usize,
               concat!("Alignment of field: ",
                       stringify!(dirent),
                       "::",
                       stringify!(d_name)));
}
#[test]
fn bindgen_test_layout_uint16_r() {
    assert_eq!(::std::mem::size_of::<uint16_r>(),
               24usize,
               concat!("Size of: ", stringify!(uint16_r)));
    assert_eq!(::std::mem::align_of::<uint16_r>(),
               8usize,
               concat!("Alignment of ", stringify!(uint16_r)));
    assert_eq!(unsafe { &(*(0 as *const uint16_r)).n as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(uint16_r),
                       "::",
                       stringify!(n)));
    assert_eq!(unsafe { &(*(0 as *const uint16_r)).m as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(uint16_r),
                       "::",
                       stringify!(m)));
    assert_eq!(unsafe { &(*(0 as *const uint16_r)).p as *const _ as usize },
               16usize,
               concat!("Alignment of field: ",
                       stringify!(uint16_r),
                       "::",
                       stringify!(p)));
}
#[test]
fn bindgen_test_layout_uint32_r() {
    assert_eq!(::std::mem::size_of::<uint32_r>(),
               24usize,
               concat!("Size of: ", stringify!(uint32_r)));
    assert_eq!(::std::mem::align_of::<uint32_r>(),
               8usize,
               concat!("Alignment of ", stringify!(uint32_r)));
    assert_eq!(unsafe { &(*(0 as *const uint32_r)).n as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(uint32_r),
                       "::",
                       stringify!(n)));
    assert_eq!(unsafe { &(*(0 as *const uint32_r)).m as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(uint32_r),
                       "::",
                       stringify!(m)));
    assert_eq!(unsafe { &(*(0 as *const uint32_r)).p as *const _ as usize },
               16usize,
               concat!("Alignment of field: ",
                       stringify!(uint32_r),
                       "::",
                       stringify!(p)));
}
#[test]
fn bindgen_test_layout_void_r() {
    assert_eq!(::std::mem::size_of::<void_r>(),
               24usize,
               concat!("Size of: ", stringify!(void_r)));
    assert_eq!(::std::mem::align_of::<void_r>(),
               8usize,
               concat!("Alignment of ", stringify!(void_r)));
    assert_eq!(unsafe { &(*(0 as *const void_r)).n as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(void_r),
                       "::",
                       stringify!(n)));
    assert_eq!(unsafe { &(*(0 as *const void_r)).m as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(void_r),
                       "::",
                       stringify!(m)));
    assert_eq!(unsafe { &(*(0 as *const void_r)).p as *const _ as usize },
               16usize,
               concat!("Alignment of field: ",
                       stringify!(void_r),
                       "::",
                       stringify!(p)));
}
#[test]
fn bindgen_test_layout_cstring_r() {
    assert_eq!(::std::mem::size_of::<cstring_r>(),
               24usize,
               concat!("Size of: ", stringify!(cstring_r)));
    assert_eq!(::std::mem::align_of::<cstring_r>(),
               8usize,
               concat!("Alignment of ", stringify!(cstring_r)));
    assert_eq!(unsafe { &(*(0 as *const cstring_r)).n as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(cstring_r),
                       "::",
                       stringify!(n)));
    assert_eq!(unsafe { &(*(0 as *const cstring_r)).m as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(cstring_r),
                       "::",
                       stringify!(m)));
    assert_eq!(unsafe { &(*(0 as *const cstring_r)).p as *const _ as usize },
               16usize,
               concat!("Alignment of field: ",
                       stringify!(cstring_r),
                       "::",
                       stringify!(p)));
}
#[test]
fn bindgen_test_layout_imaxdiv_t() {
    assert_eq!(::std::mem::size_of::<imaxdiv_t>(),
               16usize,
               concat!("Size of: ", stringify!(imaxdiv_t)));
    assert_eq!(::std::mem::align_of::<imaxdiv_t>(),
               8usize,
               concat!("Alignment of ", stringify!(imaxdiv_t)));
    assert_eq!(unsafe { &(*(0 as *const imaxdiv_t)).quot as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(imaxdiv_t),
                       "::",
                       stringify!(quot)));
    assert_eq!(unsafe { &(*(0 as *const imaxdiv_t)).rem as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(imaxdiv_t),
                       "::",
                       stringify!(rem)));
}
#[test]
fn bindgen_test_layout_json_settings() {
    assert_eq!(::std::mem::size_of::<json_settings>(),
               48usize,
               concat!("Size of: ", stringify!(json_settings)));
    assert_eq!(::std::mem::align_of::<json_settings>(),
               8usize,
               concat!("Alignment of ", stringify!(json_settings)));
    assert_eq!(unsafe { &(*(0 as *const json_settings)).max_memory as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(json_settings),
                       "::",
                       stringify!(max_memory)));
    assert_eq!(unsafe { &(*(0 as *const json_settings)).settings as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(json_settings),
                       "::",
                       stringify!(settings)));
    assert_eq!(unsafe { &(*(0 as *const json_settings)).memory_alloc as *const _ as usize },
               16usize,
               concat!("Alignment of field: ",
                       stringify!(json_settings),
                       "::",
                       stringify!(memory_alloc)));
    assert_eq!(unsafe { &(*(0 as *const json_settings)).memory_free as *const _ as usize },
               24usize,
               concat!("Alignment of field: ",
                       stringify!(json_settings),
                       "::",
                       stringify!(memory_free)));
    assert_eq!(unsafe { &(*(0 as *const json_settings)).user_data as *const _ as usize },
               32usize,
               concat!("Alignment of field: ",
                       stringify!(json_settings),
                       "::",
                       stringify!(user_data)));
    assert_eq!(unsafe { &(*(0 as *const json_settings)).value_extra as *const _ as usize },
               40usize,
               concat!("Alignment of field: ",
                       stringify!(json_settings),
                       "::",
                       stringify!(value_extra)));
}
#[test]
fn bindgen_test_layout__json_value__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<_json_value__bindgen_ty_1__bindgen_ty_1>(),
               16usize,
               concat!("Size of: ",
                       stringify!(_json_value__bindgen_ty_1__bindgen_ty_1)));
    assert_eq!(::std::mem::align_of::<_json_value__bindgen_ty_1__bindgen_ty_1>(),
               8usize,
               concat!("Alignment of ",
                       stringify!(_json_value__bindgen_ty_1__bindgen_ty_1)));
    assert_eq!(unsafe {
                   &(*(0 as *const _json_value__bindgen_ty_1__bindgen_ty_1)).length as *const _ as
                   usize
               },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(_json_value__bindgen_ty_1__bindgen_ty_1),
                       "::",
                       stringify!(length)));
    assert_eq!(unsafe {
                   &(*(0 as *const _json_value__bindgen_ty_1__bindgen_ty_1)).ptr as *const _ as
                   usize
               },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(_json_value__bindgen_ty_1__bindgen_ty_1),
                       "::",
                       stringify!(ptr)));
}
#[test]
fn bindgen_test_layout__json_value__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(::std::mem::size_of::<_json_value__bindgen_ty_1__bindgen_ty_2>(),
               16usize,
               concat!("Size of: ",
                       stringify!(_json_value__bindgen_ty_1__bindgen_ty_2)));
    assert_eq!(::std::mem::align_of::<_json_value__bindgen_ty_1__bindgen_ty_2>(),
               8usize,
               concat!("Alignment of ",
                       stringify!(_json_value__bindgen_ty_1__bindgen_ty_2)));
    assert_eq!(unsafe {
                   &(*(0 as *const _json_value__bindgen_ty_1__bindgen_ty_2)).length as *const _ as
                   usize
               },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(_json_value__bindgen_ty_1__bindgen_ty_2),
                       "::",
                       stringify!(length)));
    assert_eq!(unsafe {
                   &(*(0 as *const _json_value__bindgen_ty_1__bindgen_ty_2)).values as *const _ as
                   usize
               },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(_json_value__bindgen_ty_1__bindgen_ty_2),
                       "::",
                       stringify!(values)));
}
#[test]
fn bindgen_test_layout__json_value__bindgen_ty_1__bindgen_ty_3() {
    assert_eq!(::std::mem::size_of::<_json_value__bindgen_ty_1__bindgen_ty_3>(),
               16usize,
               concat!("Size of: ",
                       stringify!(_json_value__bindgen_ty_1__bindgen_ty_3)));
    assert_eq!(::std::mem::align_of::<_json_value__bindgen_ty_1__bindgen_ty_3>(),
               8usize,
               concat!("Alignment of ",
                       stringify!(_json_value__bindgen_ty_1__bindgen_ty_3)));
    assert_eq!(unsafe {
                   &(*(0 as *const _json_value__bindgen_ty_1__bindgen_ty_3)).length as *const _ as
                   usize
               },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(_json_value__bindgen_ty_1__bindgen_ty_3),
                       "::",
                       stringify!(length)));
    assert_eq!(unsafe {
                   &(*(0 as *const _json_value__bindgen_ty_1__bindgen_ty_3)).values as *const _ as
                   usize
               },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(_json_value__bindgen_ty_1__bindgen_ty_3),
                       "::",
                       stringify!(values)));
}
#[test]
fn bindgen_test_layout__json_value__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<_json_value__bindgen_ty_1>(),
               16usize,
               concat!("Size of: ", stringify!(_json_value__bindgen_ty_1)));
    assert_eq!(::std::mem::align_of::<_json_value__bindgen_ty_1>(),
               8usize,
               concat!("Alignment of ", stringify!(_json_value__bindgen_ty_1)));
    assert_eq!(unsafe { &(*(0 as *const _json_value__bindgen_ty_1)).boolean as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(_json_value__bindgen_ty_1),
                       "::",
                       stringify!(boolean)));
    assert_eq!(unsafe { &(*(0 as *const _json_value__bindgen_ty_1)).integer as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(_json_value__bindgen_ty_1),
                       "::",
                       stringify!(integer)));
    assert_eq!(unsafe { &(*(0 as *const _json_value__bindgen_ty_1)).dbl as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(_json_value__bindgen_ty_1),
                       "::",
                       stringify!(dbl)));
    assert_eq!(unsafe { &(*(0 as *const _json_value__bindgen_ty_1)).string as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(_json_value__bindgen_ty_1),
                       "::",
                       stringify!(string)));
    assert_eq!(unsafe { &(*(0 as *const _json_value__bindgen_ty_1)).object as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(_json_value__bindgen_ty_1),
                       "::",
                       stringify!(object)));
    assert_eq!(unsafe { &(*(0 as *const _json_value__bindgen_ty_1)).array as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(_json_value__bindgen_ty_1),
                       "::",
                       stringify!(array)));
}
#[test]
fn bindgen_test_layout__json_value__bindgen_ty_2() {
    assert_eq!(::std::mem::size_of::<_json_value__bindgen_ty_2>(),
               8usize,
               concat!("Size of: ", stringify!(_json_value__bindgen_ty_2)));
    assert_eq!(::std::mem::align_of::<_json_value__bindgen_ty_2>(),
               8usize,
               concat!("Alignment of ", stringify!(_json_value__bindgen_ty_2)));
    assert_eq!(unsafe {
                   &(*(0 as *const _json_value__bindgen_ty_2)).next_alloc as *const _ as usize
               },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(_json_value__bindgen_ty_2),
                       "::",
                       stringify!(next_alloc)));
    assert_eq!(unsafe {
                   &(*(0 as *const _json_value__bindgen_ty_2)).object_mem as *const _ as usize
               },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(_json_value__bindgen_ty_2),
                       "::",
                       stringify!(object_mem)));
}
#[test]
fn bindgen_test_layout__json_value() {
    assert_eq!(::std::mem::size_of::<_json_value>(),
               40usize,
               concat!("Size of: ", stringify!(_json_value)));
    assert_eq!(::std::mem::align_of::<_json_value>(),
               8usize,
               concat!("Alignment of ", stringify!(_json_value)));
    assert_eq!(unsafe { &(*(0 as *const _json_value)).parent as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(_json_value),
                       "::",
                       stringify!(parent)));
    assert_eq!(unsafe { &(*(0 as *const _json_value)).type_ as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(_json_value),
                       "::",
                       stringify!(type_)));
    assert_eq!(unsafe { &(*(0 as *const _json_value)).u as *const _ as usize },
               16usize,
               concat!("Alignment of field: ",
                       stringify!(_json_value),
                       "::",
                       stringify!(u)));
    assert_eq!(unsafe { &(*(0 as *const _json_value))._reserved as *const _ as usize },
               32usize,
               concat!("Alignment of field: ",
                       stringify!(_json_value),
                       "::",
                       stringify!(_reserved)));
}
#[test]
fn bindgen_test_layout__json_object_entry() {
    assert_eq!(::std::mem::size_of::<_json_object_entry>(),
               24usize,
               concat!("Size of: ", stringify!(_json_object_entry)));
    assert_eq!(::std::mem::align_of::<_json_object_entry>(),
               8usize,
               concat!("Alignment of ", stringify!(_json_object_entry)));
    assert_eq!(unsafe { &(*(0 as *const _json_object_entry)).name as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(_json_object_entry),
                       "::",
                       stringify!(name)));
    assert_eq!(unsafe { &(*(0 as *const _json_object_entry)).name_length as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(_json_object_entry),
                       "::",
                       stringify!(name_length)));
    assert_eq!(unsafe { &(*(0 as *const _json_object_entry)).value as *const _ as usize },
               16usize,
               concat!("Alignment of field: ",
                       stringify!(_json_object_entry),
                       "::",
                       stringify!(value)));
}
#[test]
fn bindgen_test_layout_gravity_class_s() {
    assert_eq!(::std::mem::size_of::<gravity_class_s>(),
               88usize,
               concat!("Size of: ", stringify!(gravity_class_s)));
    assert_eq!(::std::mem::align_of::<gravity_class_s>(),
               8usize,
               concat!("Alignment of ", stringify!(gravity_class_s)));
    assert_eq!(unsafe { &(*(0 as *const gravity_class_s)).isa as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_class_s),
                       "::",
                       stringify!(isa)));
    assert_eq!(unsafe { &(*(0 as *const gravity_class_s)).gc as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_class_s),
                       "::",
                       stringify!(gc)));
    assert_eq!(unsafe { &(*(0 as *const gravity_class_s)).objclass as *const _ as usize },
               24usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_class_s),
                       "::",
                       stringify!(objclass)));
    assert_eq!(unsafe { &(*(0 as *const gravity_class_s)).identifier as *const _ as usize },
               32usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_class_s),
                       "::",
                       stringify!(identifier)));
    assert_eq!(unsafe { &(*(0 as *const gravity_class_s)).has_outer as *const _ as usize },
               40usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_class_s),
                       "::",
                       stringify!(has_outer)));
    assert_eq!(unsafe { &(*(0 as *const gravity_class_s)).is_struct as *const _ as usize },
               41usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_class_s),
                       "::",
                       stringify!(is_struct)));
    assert_eq!(unsafe { &(*(0 as *const gravity_class_s)).is_inited as *const _ as usize },
               42usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_class_s),
                       "::",
                       stringify!(is_inited)));
    assert_eq!(unsafe { &(*(0 as *const gravity_class_s)).unused as *const _ as usize },
               43usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_class_s),
                       "::",
                       stringify!(unused)));
    assert_eq!(unsafe { &(*(0 as *const gravity_class_s)).xdata as *const _ as usize },
               48usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_class_s),
                       "::",
                       stringify!(xdata)));
    assert_eq!(unsafe { &(*(0 as *const gravity_class_s)).superclass as *const _ as usize },
               56usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_class_s),
                       "::",
                       stringify!(superclass)));
    assert_eq!(unsafe { &(*(0 as *const gravity_class_s)).htable as *const _ as usize },
               64usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_class_s),
                       "::",
                       stringify!(htable)));
    assert_eq!(unsafe { &(*(0 as *const gravity_class_s)).nivars as *const _ as usize },
               72usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_class_s),
                       "::",
                       stringify!(nivars)));
    assert_eq!(unsafe { &(*(0 as *const gravity_class_s)).ivars as *const _ as usize },
               80usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_class_s),
                       "::",
                       stringify!(ivars)));
}
#[test]
fn bindgen_test_layout_gravity_value_t__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<gravity_value_t__bindgen_ty_1>(),
               8usize,
               concat!("Size of: ", stringify!(gravity_value_t__bindgen_ty_1)));
    assert_eq!(::std::mem::align_of::<gravity_value_t__bindgen_ty_1>(),
               8usize,
               concat!("Alignment of ", stringify!(gravity_value_t__bindgen_ty_1)));
    assert_eq!(unsafe { &(*(0 as *const gravity_value_t__bindgen_ty_1)).n as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_value_t__bindgen_ty_1),
                       "::",
                       stringify!(n)));
    assert_eq!(unsafe { &(*(0 as *const gravity_value_t__bindgen_ty_1)).f as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_value_t__bindgen_ty_1),
                       "::",
                       stringify!(f)));
    assert_eq!(unsafe { &(*(0 as *const gravity_value_t__bindgen_ty_1)).p as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_value_t__bindgen_ty_1),
                       "::",
                       stringify!(p)));
}
#[test]
fn bindgen_test_layout_gravity_value_t() {
    assert_eq!(::std::mem::size_of::<gravity_value_t>(),
               16usize,
               concat!("Size of: ", stringify!(gravity_value_t)));
    assert_eq!(::std::mem::align_of::<gravity_value_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gravity_value_t)));
    assert_eq!(unsafe { &(*(0 as *const gravity_value_t)).isa as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_value_t),
                       "::",
                       stringify!(isa)));
}
#[test]
fn bindgen_test_layout_gravity_value_r() {
    assert_eq!(::std::mem::size_of::<gravity_value_r>(),
               24usize,
               concat!("Size of: ", stringify!(gravity_value_r)));
    assert_eq!(::std::mem::align_of::<gravity_value_r>(),
               8usize,
               concat!("Alignment of ", stringify!(gravity_value_r)));
    assert_eq!(unsafe { &(*(0 as *const gravity_value_r)).n as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_value_r),
                       "::",
                       stringify!(n)));
    assert_eq!(unsafe { &(*(0 as *const gravity_value_r)).m as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_value_r),
                       "::",
                       stringify!(m)));
    assert_eq!(unsafe { &(*(0 as *const gravity_value_r)).p as *const _ as usize },
               16usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_value_r),
                       "::",
                       stringify!(p)));
}
#[test]
fn bindgen_test_layout_gravity_gc_t() {
    assert_eq!(::std::mem::size_of::<gravity_gc_t>(),
               16usize,
               concat!("Size of: ", stringify!(gravity_gc_t)));
    assert_eq!(::std::mem::align_of::<gravity_gc_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gravity_gc_t)));
    assert_eq!(unsafe { &(*(0 as *const gravity_gc_t)).isdark as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_gc_t),
                       "::",
                       stringify!(isdark)));
    assert_eq!(unsafe { &(*(0 as *const gravity_gc_t)).next as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_gc_t),
                       "::",
                       stringify!(next)));
}
#[test]
fn bindgen_test_layout_gravity_function_t__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<gravity_function_t__bindgen_ty_1__bindgen_ty_1>(),
               48usize,
               concat!("Size of: ",
                       stringify!(gravity_function_t__bindgen_ty_1__bindgen_ty_1)));
    assert_eq!(::std::mem::align_of::<gravity_function_t__bindgen_ty_1__bindgen_ty_1>(),
               8usize,
               concat!("Alignment of ",
                       stringify!(gravity_function_t__bindgen_ty_1__bindgen_ty_1)));
    assert_eq!(unsafe {
                   &(*(0 as *const gravity_function_t__bindgen_ty_1__bindgen_ty_1)).cpool as
                   *const _ as usize
               },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_function_t__bindgen_ty_1__bindgen_ty_1),
                       "::",
                       stringify!(cpool)));
    assert_eq!(unsafe {
                   &(*(0 as *const gravity_function_t__bindgen_ty_1__bindgen_ty_1)).ninsts as
                   *const _ as usize
               },
               24usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_function_t__bindgen_ty_1__bindgen_ty_1),
                       "::",
                       stringify!(ninsts)));
    assert_eq!(unsafe {
                   &(*(0 as *const gravity_function_t__bindgen_ty_1__bindgen_ty_1)).bytecode as
                   *const _ as usize
               },
               32usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_function_t__bindgen_ty_1__bindgen_ty_1),
                       "::",
                       stringify!(bytecode)));
    assert_eq!(unsafe {
                   &(*(0 as *const gravity_function_t__bindgen_ty_1__bindgen_ty_1)).purity as
                   *const _ as usize
               },
               40usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_function_t__bindgen_ty_1__bindgen_ty_1),
                       "::",
                       stringify!(purity)));
    assert_eq!(unsafe {
                   &(*(0 as *const gravity_function_t__bindgen_ty_1__bindgen_ty_1)).useargs as
                   *const _ as usize
               },
               44usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_function_t__bindgen_ty_1__bindgen_ty_1),
                       "::",
                       stringify!(useargs)));
}
#[test]
fn bindgen_test_layout_gravity_function_t__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(::std::mem::size_of::<gravity_function_t__bindgen_ty_1__bindgen_ty_2>(),
               24usize,
               concat!("Size of: ",
                       stringify!(gravity_function_t__bindgen_ty_1__bindgen_ty_2)));
    assert_eq!(::std::mem::align_of::<gravity_function_t__bindgen_ty_1__bindgen_ty_2>(),
               8usize,
               concat!("Alignment of ",
                       stringify!(gravity_function_t__bindgen_ty_1__bindgen_ty_2)));
    assert_eq!(unsafe {
                   &(*(0 as *const gravity_function_t__bindgen_ty_1__bindgen_ty_2)).index as
                   *const _ as usize
               },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_function_t__bindgen_ty_1__bindgen_ty_2),
                       "::",
                       stringify!(index)));
    assert_eq!(unsafe {
                   &(*(0 as *const gravity_function_t__bindgen_ty_1__bindgen_ty_2)).special as
                   *const _ as usize
               },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_function_t__bindgen_ty_1__bindgen_ty_2),
                       "::",
                       stringify!(special)));
}
#[test]
fn bindgen_test_layout_gravity_function_t__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<gravity_function_t__bindgen_ty_1>(),
               48usize,
               concat!("Size of: ", stringify!(gravity_function_t__bindgen_ty_1)));
    assert_eq!(::std::mem::align_of::<gravity_function_t__bindgen_ty_1>(),
               8usize,
               concat!("Alignment of ",
                       stringify!(gravity_function_t__bindgen_ty_1)));
    assert_eq!(unsafe {
                   &(*(0 as *const gravity_function_t__bindgen_ty_1)).internal as *const _ as usize
               },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_function_t__bindgen_ty_1),
                       "::",
                       stringify!(internal)));
}
#[test]
fn bindgen_test_layout_gravity_function_t() {
    assert_eq!(::std::mem::size_of::<gravity_function_t>(),
               104usize,
               concat!("Size of: ", stringify!(gravity_function_t)));
    assert_eq!(::std::mem::align_of::<gravity_function_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gravity_function_t)));
    assert_eq!(unsafe { &(*(0 as *const gravity_function_t)).isa as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_function_t),
                       "::",
                       stringify!(isa)));
    assert_eq!(unsafe { &(*(0 as *const gravity_function_t)).gc as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_function_t),
                       "::",
                       stringify!(gc)));
    assert_eq!(unsafe { &(*(0 as *const gravity_function_t)).xdata as *const _ as usize },
               24usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_function_t),
                       "::",
                       stringify!(xdata)));
    assert_eq!(unsafe { &(*(0 as *const gravity_function_t)).identifier as *const _ as usize },
               32usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_function_t),
                       "::",
                       stringify!(identifier)));
    assert_eq!(unsafe { &(*(0 as *const gravity_function_t)).nparams as *const _ as usize },
               40usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_function_t),
                       "::",
                       stringify!(nparams)));
    assert_eq!(unsafe { &(*(0 as *const gravity_function_t)).nlocals as *const _ as usize },
               42usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_function_t),
                       "::",
                       stringify!(nlocals)));
    assert_eq!(unsafe { &(*(0 as *const gravity_function_t)).ntemps as *const _ as usize },
               44usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_function_t),
                       "::",
                       stringify!(ntemps)));
    assert_eq!(unsafe { &(*(0 as *const gravity_function_t)).nupvalues as *const _ as usize },
               46usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_function_t),
                       "::",
                       stringify!(nupvalues)));
    assert_eq!(unsafe { &(*(0 as *const gravity_function_t)).tag as *const _ as usize },
               48usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_function_t),
                       "::",
                       stringify!(tag)));
}
#[test]
fn bindgen_test_layout_upvalue_s() {
    assert_eq!(::std::mem::size_of::<upvalue_s>(),
               56usize,
               concat!("Size of: ", stringify!(upvalue_s)));
    assert_eq!(::std::mem::align_of::<upvalue_s>(),
               8usize,
               concat!("Alignment of ", stringify!(upvalue_s)));
    assert_eq!(unsafe { &(*(0 as *const upvalue_s)).isa as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(upvalue_s),
                       "::",
                       stringify!(isa)));
    assert_eq!(unsafe { &(*(0 as *const upvalue_s)).gc as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(upvalue_s),
                       "::",
                       stringify!(gc)));
    assert_eq!(unsafe { &(*(0 as *const upvalue_s)).value as *const _ as usize },
               24usize,
               concat!("Alignment of field: ",
                       stringify!(upvalue_s),
                       "::",
                       stringify!(value)));
    assert_eq!(unsafe { &(*(0 as *const upvalue_s)).closed as *const _ as usize },
               32usize,
               concat!("Alignment of field: ",
                       stringify!(upvalue_s),
                       "::",
                       stringify!(closed)));
    assert_eq!(unsafe { &(*(0 as *const upvalue_s)).next as *const _ as usize },
               48usize,
               concat!("Alignment of field: ",
                       stringify!(upvalue_s),
                       "::",
                       stringify!(next)));
}
#[test]
fn bindgen_test_layout_gravity_closure_t() {
    assert_eq!(::std::mem::size_of::<gravity_closure_t>(),
               40usize,
               concat!("Size of: ", stringify!(gravity_closure_t)));
    assert_eq!(::std::mem::align_of::<gravity_closure_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gravity_closure_t)));
    assert_eq!(unsafe { &(*(0 as *const gravity_closure_t)).isa as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_closure_t),
                       "::",
                       stringify!(isa)));
    assert_eq!(unsafe { &(*(0 as *const gravity_closure_t)).gc as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_closure_t),
                       "::",
                       stringify!(gc)));
    assert_eq!(unsafe { &(*(0 as *const gravity_closure_t)).f as *const _ as usize },
               24usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_closure_t),
                       "::",
                       stringify!(f)));
    assert_eq!(unsafe { &(*(0 as *const gravity_closure_t)).upvalue as *const _ as usize },
               32usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_closure_t),
                       "::",
                       stringify!(upvalue)));
}
#[test]
fn bindgen_test_layout_gravity_list_t() {
    assert_eq!(::std::mem::size_of::<gravity_list_t>(),
               48usize,
               concat!("Size of: ", stringify!(gravity_list_t)));
    assert_eq!(::std::mem::align_of::<gravity_list_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gravity_list_t)));
    assert_eq!(unsafe { &(*(0 as *const gravity_list_t)).isa as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_list_t),
                       "::",
                       stringify!(isa)));
    assert_eq!(unsafe { &(*(0 as *const gravity_list_t)).gc as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_list_t),
                       "::",
                       stringify!(gc)));
    assert_eq!(unsafe { &(*(0 as *const gravity_list_t)).array as *const _ as usize },
               24usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_list_t),
                       "::",
                       stringify!(array)));
}
#[test]
fn bindgen_test_layout_gravity_map_t() {
    assert_eq!(::std::mem::size_of::<gravity_map_t>(),
               32usize,
               concat!("Size of: ", stringify!(gravity_map_t)));
    assert_eq!(::std::mem::align_of::<gravity_map_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gravity_map_t)));
    assert_eq!(unsafe { &(*(0 as *const gravity_map_t)).isa as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_map_t),
                       "::",
                       stringify!(isa)));
    assert_eq!(unsafe { &(*(0 as *const gravity_map_t)).gc as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_map_t),
                       "::",
                       stringify!(gc)));
    assert_eq!(unsafe { &(*(0 as *const gravity_map_t)).hash as *const _ as usize },
               24usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_map_t),
                       "::",
                       stringify!(hash)));
}
#[test]
fn bindgen_test_layout_gravity_callframe_t() {
    assert_eq!(::std::mem::size_of::<gravity_callframe_t>(),
               48usize,
               concat!("Size of: ", stringify!(gravity_callframe_t)));
    assert_eq!(::std::mem::align_of::<gravity_callframe_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gravity_callframe_t)));
    assert_eq!(unsafe { &(*(0 as *const gravity_callframe_t)).ip as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_callframe_t),
                       "::",
                       stringify!(ip)));
    assert_eq!(unsafe { &(*(0 as *const gravity_callframe_t)).dest as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_callframe_t),
                       "::",
                       stringify!(dest)));
    assert_eq!(unsafe { &(*(0 as *const gravity_callframe_t)).nargs as *const _ as usize },
               12usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_callframe_t),
                       "::",
                       stringify!(nargs)));
    assert_eq!(unsafe { &(*(0 as *const gravity_callframe_t)).args as *const _ as usize },
               16usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_callframe_t),
                       "::",
                       stringify!(args)));
    assert_eq!(unsafe { &(*(0 as *const gravity_callframe_t)).closure as *const _ as usize },
               24usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_callframe_t),
                       "::",
                       stringify!(closure)));
    assert_eq!(unsafe { &(*(0 as *const gravity_callframe_t)).stackstart as *const _ as usize },
               32usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_callframe_t),
                       "::",
                       stringify!(stackstart)));
    assert_eq!(unsafe { &(*(0 as *const gravity_callframe_t)).outloop as *const _ as usize },
               40usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_callframe_t),
                       "::",
                       stringify!(outloop)));
}
#[test]
fn bindgen_test_layout_fiber_s() {
    assert_eq!(::std::mem::size_of::<fiber_s>(),
               112usize,
               concat!("Size of: ", stringify!(fiber_s)));
    assert_eq!(::std::mem::align_of::<fiber_s>(),
               8usize,
               concat!("Alignment of ", stringify!(fiber_s)));
    assert_eq!(unsafe { &(*(0 as *const fiber_s)).isa as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(fiber_s),
                       "::",
                       stringify!(isa)));
    assert_eq!(unsafe { &(*(0 as *const fiber_s)).gc as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(fiber_s),
                       "::",
                       stringify!(gc)));
    assert_eq!(unsafe { &(*(0 as *const fiber_s)).stack as *const _ as usize },
               24usize,
               concat!("Alignment of field: ",
                       stringify!(fiber_s),
                       "::",
                       stringify!(stack)));
    assert_eq!(unsafe { &(*(0 as *const fiber_s)).stacktop as *const _ as usize },
               32usize,
               concat!("Alignment of field: ",
                       stringify!(fiber_s),
                       "::",
                       stringify!(stacktop)));
    assert_eq!(unsafe { &(*(0 as *const fiber_s)).stackalloc as *const _ as usize },
               40usize,
               concat!("Alignment of field: ",
                       stringify!(fiber_s),
                       "::",
                       stringify!(stackalloc)));
    assert_eq!(unsafe { &(*(0 as *const fiber_s)).frames as *const _ as usize },
               48usize,
               concat!("Alignment of field: ",
                       stringify!(fiber_s),
                       "::",
                       stringify!(frames)));
    assert_eq!(unsafe { &(*(0 as *const fiber_s)).nframes as *const _ as usize },
               56usize,
               concat!("Alignment of field: ",
                       stringify!(fiber_s),
                       "::",
                       stringify!(nframes)));
    assert_eq!(unsafe { &(*(0 as *const fiber_s)).framesalloc as *const _ as usize },
               60usize,
               concat!("Alignment of field: ",
                       stringify!(fiber_s),
                       "::",
                       stringify!(framesalloc)));
    assert_eq!(unsafe { &(*(0 as *const fiber_s)).upvalues as *const _ as usize },
               64usize,
               concat!("Alignment of field: ",
                       stringify!(fiber_s),
                       "::",
                       stringify!(upvalues)));
    assert_eq!(unsafe { &(*(0 as *const fiber_s)).error as *const _ as usize },
               72usize,
               concat!("Alignment of field: ",
                       stringify!(fiber_s),
                       "::",
                       stringify!(error)));
    assert_eq!(unsafe { &(*(0 as *const fiber_s)).trying as *const _ as usize },
               80usize,
               concat!("Alignment of field: ",
                       stringify!(fiber_s),
                       "::",
                       stringify!(trying)));
    assert_eq!(unsafe { &(*(0 as *const fiber_s)).caller as *const _ as usize },
               88usize,
               concat!("Alignment of field: ",
                       stringify!(fiber_s),
                       "::",
                       stringify!(caller)));
    assert_eq!(unsafe { &(*(0 as *const fiber_s)).result as *const _ as usize },
               96usize,
               concat!("Alignment of field: ",
                       stringify!(fiber_s),
                       "::",
                       stringify!(result)));
}
#[test]
fn bindgen_test_layout_gravity_module_t() {
    assert_eq!(::std::mem::size_of::<gravity_module_t>(),
               40usize,
               concat!("Size of: ", stringify!(gravity_module_t)));
    assert_eq!(::std::mem::align_of::<gravity_module_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gravity_module_t)));
    assert_eq!(unsafe { &(*(0 as *const gravity_module_t)).isa as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_module_t),
                       "::",
                       stringify!(isa)));
    assert_eq!(unsafe { &(*(0 as *const gravity_module_t)).gc as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_module_t),
                       "::",
                       stringify!(gc)));
    assert_eq!(unsafe { &(*(0 as *const gravity_module_t)).identifier as *const _ as usize },
               24usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_module_t),
                       "::",
                       stringify!(identifier)));
    assert_eq!(unsafe { &(*(0 as *const gravity_module_t)).htable as *const _ as usize },
               32usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_module_t),
                       "::",
                       stringify!(htable)));
}
#[test]
fn bindgen_test_layout_gravity_instance_t() {
    assert_eq!(::std::mem::size_of::<gravity_instance_t>(),
               40usize,
               concat!("Size of: ", stringify!(gravity_instance_t)));
    assert_eq!(::std::mem::align_of::<gravity_instance_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gravity_instance_t)));
}
#[test]
fn bindgen_test_layout_gravity_string_t() {
    assert_eq!(::std::mem::size_of::<gravity_string_t>(),
               48usize,
               concat!("Size of: ", stringify!(gravity_string_t)));
    assert_eq!(::std::mem::align_of::<gravity_string_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gravity_string_t)));
    assert_eq!(unsafe { &(*(0 as *const gravity_string_t)).isa as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_string_t),
                       "::",
                       stringify!(isa)));
    assert_eq!(unsafe { &(*(0 as *const gravity_string_t)).gc as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_string_t),
                       "::",
                       stringify!(gc)));
    assert_eq!(unsafe { &(*(0 as *const gravity_string_t)).s as *const _ as usize },
               24usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_string_t),
                       "::",
                       stringify!(s)));
    assert_eq!(unsafe { &(*(0 as *const gravity_string_t)).hash as *const _ as usize },
               32usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_string_t),
                       "::",
                       stringify!(hash)));
    assert_eq!(unsafe { &(*(0 as *const gravity_string_t)).len as *const _ as usize },
               36usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_string_t),
                       "::",
                       stringify!(len)));
    assert_eq!(unsafe { &(*(0 as *const gravity_string_t)).alloc as *const _ as usize },
               40usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_string_t),
                       "::",
                       stringify!(alloc)));
}
#[test]
fn bindgen_test_layout_gravity_range_t() {
    assert_eq!(::std::mem::size_of::<gravity_range_t>(),
               40usize,
               concat!("Size of: ", stringify!(gravity_range_t)));
    assert_eq!(::std::mem::align_of::<gravity_range_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gravity_range_t)));
    assert_eq!(unsafe { &(*(0 as *const gravity_range_t)).isa as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_range_t),
                       "::",
                       stringify!(isa)));
    assert_eq!(unsafe { &(*(0 as *const gravity_range_t)).gc as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_range_t),
                       "::",
                       stringify!(gc)));
    assert_eq!(unsafe { &(*(0 as *const gravity_range_t)).from as *const _ as usize },
               24usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_range_t),
                       "::",
                       stringify!(from)));
    assert_eq!(unsafe { &(*(0 as *const gravity_range_t)).to as *const _ as usize },
               32usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_range_t),
                       "::",
                       stringify!(to)));
}
#[test]
fn bindgen_test_layout_gravity_function_r() {
    assert_eq!(::std::mem::size_of::<gravity_function_r>(),
               24usize,
               concat!("Size of: ", stringify!(gravity_function_r)));
    assert_eq!(::std::mem::align_of::<gravity_function_r>(),
               8usize,
               concat!("Alignment of ", stringify!(gravity_function_r)));
    assert_eq!(unsafe { &(*(0 as *const gravity_function_r)).n as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_function_r),
                       "::",
                       stringify!(n)));
    assert_eq!(unsafe { &(*(0 as *const gravity_function_r)).m as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_function_r),
                       "::",
                       stringify!(m)));
    assert_eq!(unsafe { &(*(0 as *const gravity_function_r)).p as *const _ as usize },
               16usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_function_r),
                       "::",
                       stringify!(p)));
}
#[test]
fn bindgen_test_layout_gravity_class_r() {
    assert_eq!(::std::mem::size_of::<gravity_class_r>(),
               24usize,
               concat!("Size of: ", stringify!(gravity_class_r)));
    assert_eq!(::std::mem::align_of::<gravity_class_r>(),
               8usize,
               concat!("Alignment of ", stringify!(gravity_class_r)));
    assert_eq!(unsafe { &(*(0 as *const gravity_class_r)).n as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_class_r),
                       "::",
                       stringify!(n)));
    assert_eq!(unsafe { &(*(0 as *const gravity_class_r)).m as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_class_r),
                       "::",
                       stringify!(m)));
    assert_eq!(unsafe { &(*(0 as *const gravity_class_r)).p as *const _ as usize },
               16usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_class_r),
                       "::",
                       stringify!(p)));
}
#[test]
fn bindgen_test_layout_gravity_object_r() {
    assert_eq!(::std::mem::size_of::<gravity_object_r>(),
               24usize,
               concat!("Size of: ", stringify!(gravity_object_r)));
    assert_eq!(::std::mem::align_of::<gravity_object_r>(),
               8usize,
               concat!("Alignment of ", stringify!(gravity_object_r)));
    assert_eq!(unsafe { &(*(0 as *const gravity_object_r)).n as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_object_r),
                       "::",
                       stringify!(n)));
    assert_eq!(unsafe { &(*(0 as *const gravity_object_r)).m as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_object_r),
                       "::",
                       stringify!(m)));
    assert_eq!(unsafe { &(*(0 as *const gravity_object_r)).p as *const _ as usize },
               16usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_object_r),
                       "::",
                       stringify!(p)));
}
#[test]
fn bindgen_test_layout_error_desc_t() {
    assert_eq!(::std::mem::size_of::<error_desc_t>(),
               24usize,
               concat!("Size of: ", stringify!(error_desc_t)));
    assert_eq!(::std::mem::align_of::<error_desc_t>(),
               8usize,
               concat!("Alignment of ", stringify!(error_desc_t)));
    assert_eq!(unsafe { &(*(0 as *const error_desc_t)).lineno as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(error_desc_t),
                       "::",
                       stringify!(lineno)));
    assert_eq!(unsafe { &(*(0 as *const error_desc_t)).colno as *const _ as usize },
               4usize,
               concat!("Alignment of field: ",
                       stringify!(error_desc_t),
                       "::",
                       stringify!(colno)));
    assert_eq!(unsafe { &(*(0 as *const error_desc_t)).fileid as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(error_desc_t),
                       "::",
                       stringify!(fileid)));
    assert_eq!(unsafe { &(*(0 as *const error_desc_t)).offset as *const _ as usize },
               12usize,
               concat!("Alignment of field: ",
                       stringify!(error_desc_t),
                       "::",
                       stringify!(offset)));
    assert_eq!(unsafe { &(*(0 as *const error_desc_t)).meta as *const _ as usize },
               16usize,
               concat!("Alignment of field: ",
                       stringify!(error_desc_t),
                       "::",
                       stringify!(meta)));
}
#[test]
fn bindgen_test_layout_gravity_delegate_t() {
    assert_eq!(::std::mem::size_of::<gravity_delegate_t>(),
               128usize,
               concat!("Size of: ", stringify!(gravity_delegate_t)));
    assert_eq!(::std::mem::align_of::<gravity_delegate_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gravity_delegate_t)));
    assert_eq!(unsafe { &(*(0 as *const gravity_delegate_t)).xdata as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_delegate_t),
                       "::",
                       stringify!(xdata)));
    assert_eq!(unsafe { &(*(0 as *const gravity_delegate_t)).log_callback as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_delegate_t),
                       "::",
                       stringify!(log_callback)));
    assert_eq!(unsafe { &(*(0 as *const gravity_delegate_t)).error_callback as *const _ as usize },
               16usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_delegate_t),
                       "::",
                       stringify!(error_callback)));
    assert_eq!(unsafe {
                   &(*(0 as *const gravity_delegate_t)).unittest_callback as *const _ as usize
               },
               24usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_delegate_t),
                       "::",
                       stringify!(unittest_callback)));
    assert_eq!(unsafe {
                   &(*(0 as *const gravity_delegate_t)).parser_callback as *const _ as usize
               },
               32usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_delegate_t),
                       "::",
                       stringify!(parser_callback)));
    assert_eq!(unsafe {
                   &(*(0 as *const gravity_delegate_t)).precode_callback as *const _ as usize
               },
               40usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_delegate_t),
                       "::",
                       stringify!(precode_callback)));
    assert_eq!(unsafe {
                   &(*(0 as *const gravity_delegate_t)).loadfile_callback as *const _ as usize
               },
               48usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_delegate_t),
                       "::",
                       stringify!(loadfile_callback)));
    assert_eq!(unsafe {
                   &(*(0 as *const gravity_delegate_t)).filename_callback as *const _ as usize
               },
               56usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_delegate_t),
                       "::",
                       stringify!(filename_callback)));
    assert_eq!(unsafe {
                   &(*(0 as *const gravity_delegate_t)).bridge_initinstance as *const _ as usize
               },
               64usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_delegate_t),
                       "::",
                       stringify!(bridge_initinstance)));
    assert_eq!(unsafe {
                   &(*(0 as *const gravity_delegate_t)).bridge_setvalue as *const _ as usize
               },
               72usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_delegate_t),
                       "::",
                       stringify!(bridge_setvalue)));
    assert_eq!(unsafe {
                   &(*(0 as *const gravity_delegate_t)).bridge_getvalue as *const _ as usize
               },
               80usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_delegate_t),
                       "::",
                       stringify!(bridge_getvalue)));
    assert_eq!(unsafe {
                   &(*(0 as *const gravity_delegate_t)).bridge_setundef as *const _ as usize
               },
               88usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_delegate_t),
                       "::",
                       stringify!(bridge_setundef)));
    assert_eq!(unsafe {
                   &(*(0 as *const gravity_delegate_t)).bridge_getundef as *const _ as usize
               },
               96usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_delegate_t),
                       "::",
                       stringify!(bridge_getundef)));
    assert_eq!(unsafe { &(*(0 as *const gravity_delegate_t)).bridge_execute as *const _ as usize },
               104usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_delegate_t),
                       "::",
                       stringify!(bridge_execute)));
    assert_eq!(unsafe { &(*(0 as *const gravity_delegate_t)).bridge_size as *const _ as usize },
               112usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_delegate_t),
                       "::",
                       stringify!(bridge_size)));
    assert_eq!(unsafe { &(*(0 as *const gravity_delegate_t)).bridge_free as *const _ as usize },
               120usize,
               concat!("Alignment of field: ",
                       stringify!(gravity_delegate_t),
                       "::",
                       stringify!(bridge_free)));
}
#[test]
fn bindgen_test_layout_gtoken_s() {
    assert_eq!(::std::mem::size_of::<gtoken_s>(),
               40usize,
               concat!("Size of: ", stringify!(gtoken_s)));
    assert_eq!(::std::mem::align_of::<gtoken_s>(),
               8usize,
               concat!("Alignment of ", stringify!(gtoken_s)));
    assert_eq!(unsafe { &(*(0 as *const gtoken_s)).type_ as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gtoken_s),
                       "::",
                       stringify!(type_)));
    assert_eq!(unsafe { &(*(0 as *const gtoken_s)).lineno as *const _ as usize },
               4usize,
               concat!("Alignment of field: ",
                       stringify!(gtoken_s),
                       "::",
                       stringify!(lineno)));
    assert_eq!(unsafe { &(*(0 as *const gtoken_s)).colno as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(gtoken_s),
                       "::",
                       stringify!(colno)));
    assert_eq!(unsafe { &(*(0 as *const gtoken_s)).position as *const _ as usize },
               12usize,
               concat!("Alignment of field: ",
                       stringify!(gtoken_s),
                       "::",
                       stringify!(position)));
    assert_eq!(unsafe { &(*(0 as *const gtoken_s)).bytes as *const _ as usize },
               16usize,
               concat!("Alignment of field: ",
                       stringify!(gtoken_s),
                       "::",
                       stringify!(bytes)));
    assert_eq!(unsafe { &(*(0 as *const gtoken_s)).length as *const _ as usize },
               20usize,
               concat!("Alignment of field: ",
                       stringify!(gtoken_s),
                       "::",
                       stringify!(length)));
    assert_eq!(unsafe { &(*(0 as *const gtoken_s)).fileid as *const _ as usize },
               24usize,
               concat!("Alignment of field: ",
                       stringify!(gtoken_s),
                       "::",
                       stringify!(fileid)));
    assert_eq!(unsafe { &(*(0 as *const gtoken_s)).builtin as *const _ as usize },
               28usize,
               concat!("Alignment of field: ",
                       stringify!(gtoken_s),
                       "::",
                       stringify!(builtin)));
    assert_eq!(unsafe { &(*(0 as *const gtoken_s)).value as *const _ as usize },
               32usize,
               concat!("Alignment of field: ",
                       stringify!(gtoken_s),
                       "::",
                       stringify!(value)));
}
#[test]
fn bindgen_test_layout_gnode_t() {
    assert_eq!(::std::mem::size_of::<gnode_t>(),
               72usize,
               concat!("Size of: ", stringify!(gnode_t)));
    assert_eq!(::std::mem::align_of::<gnode_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gnode_t)));
    assert_eq!(unsafe { &(*(0 as *const gnode_t)).tag as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_t),
                       "::",
                       stringify!(tag)));
    assert_eq!(unsafe { &(*(0 as *const gnode_t)).refcount as *const _ as usize },
               4usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_t),
                       "::",
                       stringify!(refcount)));
    assert_eq!(unsafe { &(*(0 as *const gnode_t)).token as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_t),
                       "::",
                       stringify!(token)));
    assert_eq!(unsafe { &(*(0 as *const gnode_t)).is_assignment as *const _ as usize },
               48usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_t),
                       "::",
                       stringify!(is_assignment)));
    assert_eq!(unsafe { &(*(0 as *const gnode_t)).meta as *const _ as usize },
               56usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_t),
                       "::",
                       stringify!(meta)));
    assert_eq!(unsafe { &(*(0 as *const gnode_t)).decl as *const _ as usize },
               64usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_t),
                       "::",
                       stringify!(decl)));
}
#[test]
fn bindgen_test_layout_gupvalue_t() {
    assert_eq!(::std::mem::size_of::<gupvalue_t>(),
               24usize,
               concat!("Size of: ", stringify!(gupvalue_t)));
    assert_eq!(::std::mem::align_of::<gupvalue_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gupvalue_t)));
    assert_eq!(unsafe { &(*(0 as *const gupvalue_t)).node as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gupvalue_t),
                       "::",
                       stringify!(node)));
    assert_eq!(unsafe { &(*(0 as *const gupvalue_t)).index as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(gupvalue_t),
                       "::",
                       stringify!(index)));
    assert_eq!(unsafe { &(*(0 as *const gupvalue_t)).selfindex as *const _ as usize },
               12usize,
               concat!("Alignment of field: ",
                       stringify!(gupvalue_t),
                       "::",
                       stringify!(selfindex)));
    assert_eq!(unsafe { &(*(0 as *const gupvalue_t)).is_direct as *const _ as usize },
               16usize,
               concat!("Alignment of field: ",
                       stringify!(gupvalue_t),
                       "::",
                       stringify!(is_direct)));
}
#[test]
fn bindgen_test_layout_gnode_r() {
    assert_eq!(::std::mem::size_of::<gnode_r>(),
               24usize,
               concat!("Size of: ", stringify!(gnode_r)));
    assert_eq!(::std::mem::align_of::<gnode_r>(),
               8usize,
               concat!("Alignment of ", stringify!(gnode_r)));
    assert_eq!(unsafe { &(*(0 as *const gnode_r)).n as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_r),
                       "::",
                       stringify!(n)));
    assert_eq!(unsafe { &(*(0 as *const gnode_r)).m as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_r),
                       "::",
                       stringify!(m)));
    assert_eq!(unsafe { &(*(0 as *const gnode_r)).p as *const _ as usize },
               16usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_r),
                       "::",
                       stringify!(p)));
}
#[test]
fn bindgen_test_layout_gupvalue_r() {
    assert_eq!(::std::mem::size_of::<gupvalue_r>(),
               24usize,
               concat!("Size of: ", stringify!(gupvalue_r)));
    assert_eq!(::std::mem::align_of::<gupvalue_r>(),
               8usize,
               concat!("Alignment of ", stringify!(gupvalue_r)));
    assert_eq!(unsafe { &(*(0 as *const gupvalue_r)).n as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gupvalue_r),
                       "::",
                       stringify!(n)));
    assert_eq!(unsafe { &(*(0 as *const gupvalue_r)).m as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(gupvalue_r),
                       "::",
                       stringify!(m)));
    assert_eq!(unsafe { &(*(0 as *const gupvalue_r)).p as *const _ as usize },
               16usize,
               concat!("Alignment of field: ",
                       stringify!(gupvalue_r),
                       "::",
                       stringify!(p)));
}
#[test]
fn bindgen_test_layout_gnode_location_t() {
    assert_eq!(::std::mem::size_of::<gnode_location_t>(),
               8usize,
               concat!("Size of: ", stringify!(gnode_location_t)));
    assert_eq!(::std::mem::align_of::<gnode_location_t>(),
               4usize,
               concat!("Alignment of ", stringify!(gnode_location_t)));
    assert_eq!(unsafe { &(*(0 as *const gnode_location_t)).type_ as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_location_t),
                       "::",
                       stringify!(type_)));
    assert_eq!(unsafe { &(*(0 as *const gnode_location_t)).index as *const _ as usize },
               4usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_location_t),
                       "::",
                       stringify!(index)));
    assert_eq!(unsafe { &(*(0 as *const gnode_location_t)).nup as *const _ as usize },
               6usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_location_t),
                       "::",
                       stringify!(nup)));
}
#[test]
fn bindgen_test_layout_gnode_compound_stmt_t() {
    assert_eq!(::std::mem::size_of::<gnode_compound_stmt_t>(),
               96usize,
               concat!("Size of: ", stringify!(gnode_compound_stmt_t)));
    assert_eq!(::std::mem::align_of::<gnode_compound_stmt_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gnode_compound_stmt_t)));
    assert_eq!(unsafe { &(*(0 as *const gnode_compound_stmt_t)).base as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_compound_stmt_t),
                       "::",
                       stringify!(base)));
    assert_eq!(unsafe { &(*(0 as *const gnode_compound_stmt_t)).symtable as *const _ as usize },
               72usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_compound_stmt_t),
                       "::",
                       stringify!(symtable)));
    assert_eq!(unsafe { &(*(0 as *const gnode_compound_stmt_t)).stmts as *const _ as usize },
               80usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_compound_stmt_t),
                       "::",
                       stringify!(stmts)));
    assert_eq!(unsafe { &(*(0 as *const gnode_compound_stmt_t)).nclose as *const _ as usize },
               88usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_compound_stmt_t),
                       "::",
                       stringify!(nclose)));
}
#[test]
fn bindgen_test_layout_gnode_label_stmt_t() {
    assert_eq!(::std::mem::size_of::<gnode_label_stmt_t>(),
               88usize,
               concat!("Size of: ", stringify!(gnode_label_stmt_t)));
    assert_eq!(::std::mem::align_of::<gnode_label_stmt_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gnode_label_stmt_t)));
    assert_eq!(unsafe { &(*(0 as *const gnode_label_stmt_t)).base as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_label_stmt_t),
                       "::",
                       stringify!(base)));
    assert_eq!(unsafe { &(*(0 as *const gnode_label_stmt_t)).expr as *const _ as usize },
               72usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_label_stmt_t),
                       "::",
                       stringify!(expr)));
    assert_eq!(unsafe { &(*(0 as *const gnode_label_stmt_t)).stmt as *const _ as usize },
               80usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_label_stmt_t),
                       "::",
                       stringify!(stmt)));
}
#[test]
fn bindgen_test_layout_gnode_flow_stmt_t() {
    assert_eq!(::std::mem::size_of::<gnode_flow_stmt_t>(),
               96usize,
               concat!("Size of: ", stringify!(gnode_flow_stmt_t)));
    assert_eq!(::std::mem::align_of::<gnode_flow_stmt_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gnode_flow_stmt_t)));
    assert_eq!(unsafe { &(*(0 as *const gnode_flow_stmt_t)).base as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_flow_stmt_t),
                       "::",
                       stringify!(base)));
    assert_eq!(unsafe { &(*(0 as *const gnode_flow_stmt_t)).cond as *const _ as usize },
               72usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_flow_stmt_t),
                       "::",
                       stringify!(cond)));
    assert_eq!(unsafe { &(*(0 as *const gnode_flow_stmt_t)).stmt as *const _ as usize },
               80usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_flow_stmt_t),
                       "::",
                       stringify!(stmt)));
    assert_eq!(unsafe { &(*(0 as *const gnode_flow_stmt_t)).elsestmt as *const _ as usize },
               88usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_flow_stmt_t),
                       "::",
                       stringify!(elsestmt)));
}
#[test]
fn bindgen_test_layout_gnode_loop_stmt_t() {
    assert_eq!(::std::mem::size_of::<gnode_loop_stmt_t>(),
               104usize,
               concat!("Size of: ", stringify!(gnode_loop_stmt_t)));
    assert_eq!(::std::mem::align_of::<gnode_loop_stmt_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gnode_loop_stmt_t)));
    assert_eq!(unsafe { &(*(0 as *const gnode_loop_stmt_t)).base as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_loop_stmt_t),
                       "::",
                       stringify!(base)));
    assert_eq!(unsafe { &(*(0 as *const gnode_loop_stmt_t)).cond as *const _ as usize },
               72usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_loop_stmt_t),
                       "::",
                       stringify!(cond)));
    assert_eq!(unsafe { &(*(0 as *const gnode_loop_stmt_t)).stmt as *const _ as usize },
               80usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_loop_stmt_t),
                       "::",
                       stringify!(stmt)));
    assert_eq!(unsafe { &(*(0 as *const gnode_loop_stmt_t)).expr as *const _ as usize },
               88usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_loop_stmt_t),
                       "::",
                       stringify!(expr)));
    assert_eq!(unsafe { &(*(0 as *const gnode_loop_stmt_t)).nclose as *const _ as usize },
               96usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_loop_stmt_t),
                       "::",
                       stringify!(nclose)));
}
#[test]
fn bindgen_test_layout_gnode_jump_stmt_t() {
    assert_eq!(::std::mem::size_of::<gnode_jump_stmt_t>(),
               80usize,
               concat!("Size of: ", stringify!(gnode_jump_stmt_t)));
    assert_eq!(::std::mem::align_of::<gnode_jump_stmt_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gnode_jump_stmt_t)));
    assert_eq!(unsafe { &(*(0 as *const gnode_jump_stmt_t)).base as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_jump_stmt_t),
                       "::",
                       stringify!(base)));
    assert_eq!(unsafe { &(*(0 as *const gnode_jump_stmt_t)).expr as *const _ as usize },
               72usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_jump_stmt_t),
                       "::",
                       stringify!(expr)));
}
#[test]
fn bindgen_test_layout_gnode_function_decl_t() {
    assert_eq!(::std::mem::size_of::<gnode_function_decl_t>(),
               136usize,
               concat!("Size of: ", stringify!(gnode_function_decl_t)));
    assert_eq!(::std::mem::align_of::<gnode_function_decl_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gnode_function_decl_t)));
    assert_eq!(unsafe { &(*(0 as *const gnode_function_decl_t)).base as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_function_decl_t),
                       "::",
                       stringify!(base)));
    assert_eq!(unsafe { &(*(0 as *const gnode_function_decl_t)).env as *const _ as usize },
               72usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_function_decl_t),
                       "::",
                       stringify!(env)));
    assert_eq!(unsafe { &(*(0 as *const gnode_function_decl_t)).access as *const _ as usize },
               80usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_function_decl_t),
                       "::",
                       stringify!(access)));
    assert_eq!(unsafe { &(*(0 as *const gnode_function_decl_t)).storage as *const _ as usize },
               84usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_function_decl_t),
                       "::",
                       stringify!(storage)));
    assert_eq!(unsafe { &(*(0 as *const gnode_function_decl_t)).symtable as *const _ as usize },
               88usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_function_decl_t),
                       "::",
                       stringify!(symtable)));
    assert_eq!(unsafe { &(*(0 as *const gnode_function_decl_t)).identifier as *const _ as usize },
               96usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_function_decl_t),
                       "::",
                       stringify!(identifier)));
    assert_eq!(unsafe { &(*(0 as *const gnode_function_decl_t)).params as *const _ as usize },
               104usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_function_decl_t),
                       "::",
                       stringify!(params)));
    assert_eq!(unsafe { &(*(0 as *const gnode_function_decl_t)).block as *const _ as usize },
               112usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_function_decl_t),
                       "::",
                       stringify!(block)));
    assert_eq!(unsafe { &(*(0 as *const gnode_function_decl_t)).nlocals as *const _ as usize },
               120usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_function_decl_t),
                       "::",
                       stringify!(nlocals)));
    assert_eq!(unsafe { &(*(0 as *const gnode_function_decl_t)).nparams as *const _ as usize },
               122usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_function_decl_t),
                       "::",
                       stringify!(nparams)));
    assert_eq!(unsafe { &(*(0 as *const gnode_function_decl_t)).uplist as *const _ as usize },
               128usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_function_decl_t),
                       "::",
                       stringify!(uplist)));
}
#[test]
fn bindgen_test_layout_gnode_var_t() {
    assert_eq!(::std::mem::size_of::<gnode_var_t>(),
               112usize,
               concat!("Size of: ", stringify!(gnode_var_t)));
    assert_eq!(::std::mem::align_of::<gnode_var_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gnode_var_t)));
    assert_eq!(unsafe { &(*(0 as *const gnode_var_t)).base as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_var_t),
                       "::",
                       stringify!(base)));
    assert_eq!(unsafe { &(*(0 as *const gnode_var_t)).env as *const _ as usize },
               72usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_var_t),
                       "::",
                       stringify!(env)));
    assert_eq!(unsafe { &(*(0 as *const gnode_var_t)).identifier as *const _ as usize },
               80usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_var_t),
                       "::",
                       stringify!(identifier)));
    assert_eq!(unsafe { &(*(0 as *const gnode_var_t)).annotation_type as *const _ as usize },
               88usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_var_t),
                       "::",
                       stringify!(annotation_type)));
    assert_eq!(unsafe { &(*(0 as *const gnode_var_t)).expr as *const _ as usize },
               96usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_var_t),
                       "::",
                       stringify!(expr)));
    assert_eq!(unsafe { &(*(0 as *const gnode_var_t)).access as *const _ as usize },
               104usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_var_t),
                       "::",
                       stringify!(access)));
    assert_eq!(unsafe { &(*(0 as *const gnode_var_t)).index as *const _ as usize },
               108usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_var_t),
                       "::",
                       stringify!(index)));
    assert_eq!(unsafe { &(*(0 as *const gnode_var_t)).upvalue as *const _ as usize },
               110usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_var_t),
                       "::",
                       stringify!(upvalue)));
}
#[test]
fn bindgen_test_layout_gnode_variable_decl_t() {
    assert_eq!(::std::mem::size_of::<gnode_variable_decl_t>(),
               96usize,
               concat!("Size of: ", stringify!(gnode_variable_decl_t)));
    assert_eq!(::std::mem::align_of::<gnode_variable_decl_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gnode_variable_decl_t)));
    assert_eq!(unsafe { &(*(0 as *const gnode_variable_decl_t)).base as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_variable_decl_t),
                       "::",
                       stringify!(base)));
    assert_eq!(unsafe { &(*(0 as *const gnode_variable_decl_t)).type_ as *const _ as usize },
               72usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_variable_decl_t),
                       "::",
                       stringify!(type_)));
    assert_eq!(unsafe { &(*(0 as *const gnode_variable_decl_t)).access as *const _ as usize },
               76usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_variable_decl_t),
                       "::",
                       stringify!(access)));
    assert_eq!(unsafe { &(*(0 as *const gnode_variable_decl_t)).storage as *const _ as usize },
               80usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_variable_decl_t),
                       "::",
                       stringify!(storage)));
    assert_eq!(unsafe { &(*(0 as *const gnode_variable_decl_t)).decls as *const _ as usize },
               88usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_variable_decl_t),
                       "::",
                       stringify!(decls)));
}
#[test]
fn bindgen_test_layout_gnode_enum_decl_t() {
    assert_eq!(::std::mem::size_of::<gnode_enum_decl_t>(),
               104usize,
               concat!("Size of: ", stringify!(gnode_enum_decl_t)));
    assert_eq!(::std::mem::align_of::<gnode_enum_decl_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gnode_enum_decl_t)));
    assert_eq!(unsafe { &(*(0 as *const gnode_enum_decl_t)).base as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_enum_decl_t),
                       "::",
                       stringify!(base)));
    assert_eq!(unsafe { &(*(0 as *const gnode_enum_decl_t)).env as *const _ as usize },
               72usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_enum_decl_t),
                       "::",
                       stringify!(env)));
    assert_eq!(unsafe { &(*(0 as *const gnode_enum_decl_t)).access as *const _ as usize },
               80usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_enum_decl_t),
                       "::",
                       stringify!(access)));
    assert_eq!(unsafe { &(*(0 as *const gnode_enum_decl_t)).storage as *const _ as usize },
               84usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_enum_decl_t),
                       "::",
                       stringify!(storage)));
    assert_eq!(unsafe { &(*(0 as *const gnode_enum_decl_t)).symtable as *const _ as usize },
               88usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_enum_decl_t),
                       "::",
                       stringify!(symtable)));
    assert_eq!(unsafe { &(*(0 as *const gnode_enum_decl_t)).identifier as *const _ as usize },
               96usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_enum_decl_t),
                       "::",
                       stringify!(identifier)));
}
#[test]
fn bindgen_test_layout_gnode_class_decl_t() {
    assert_eq!(::std::mem::size_of::<gnode_class_decl_t>(),
               152usize,
               concat!("Size of: ", stringify!(gnode_class_decl_t)));
    assert_eq!(::std::mem::align_of::<gnode_class_decl_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gnode_class_decl_t)));
    assert_eq!(unsafe { &(*(0 as *const gnode_class_decl_t)).base as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_class_decl_t),
                       "::",
                       stringify!(base)));
    assert_eq!(unsafe { &(*(0 as *const gnode_class_decl_t)).bridge as *const _ as usize },
               72usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_class_decl_t),
                       "::",
                       stringify!(bridge)));
    assert_eq!(unsafe { &(*(0 as *const gnode_class_decl_t)).is_struct as *const _ as usize },
               73usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_class_decl_t),
                       "::",
                       stringify!(is_struct)));
    assert_eq!(unsafe { &(*(0 as *const gnode_class_decl_t)).env as *const _ as usize },
               80usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_class_decl_t),
                       "::",
                       stringify!(env)));
    assert_eq!(unsafe { &(*(0 as *const gnode_class_decl_t)).access as *const _ as usize },
               88usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_class_decl_t),
                       "::",
                       stringify!(access)));
    assert_eq!(unsafe { &(*(0 as *const gnode_class_decl_t)).storage as *const _ as usize },
               92usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_class_decl_t),
                       "::",
                       stringify!(storage)));
    assert_eq!(unsafe { &(*(0 as *const gnode_class_decl_t)).identifier as *const _ as usize },
               96usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_class_decl_t),
                       "::",
                       stringify!(identifier)));
    assert_eq!(unsafe { &(*(0 as *const gnode_class_decl_t)).superclass as *const _ as usize },
               104usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_class_decl_t),
                       "::",
                       stringify!(superclass)));
    assert_eq!(unsafe { &(*(0 as *const gnode_class_decl_t)).protocols as *const _ as usize },
               112usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_class_decl_t),
                       "::",
                       stringify!(protocols)));
    assert_eq!(unsafe { &(*(0 as *const gnode_class_decl_t)).decls as *const _ as usize },
               120usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_class_decl_t),
                       "::",
                       stringify!(decls)));
    assert_eq!(unsafe { &(*(0 as *const gnode_class_decl_t)).symtable as *const _ as usize },
               128usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_class_decl_t),
                       "::",
                       stringify!(symtable)));
    assert_eq!(unsafe { &(*(0 as *const gnode_class_decl_t)).data as *const _ as usize },
               136usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_class_decl_t),
                       "::",
                       stringify!(data)));
    assert_eq!(unsafe { &(*(0 as *const gnode_class_decl_t)).nivar as *const _ as usize },
               144usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_class_decl_t),
                       "::",
                       stringify!(nivar)));
    assert_eq!(unsafe { &(*(0 as *const gnode_class_decl_t)).nsvar as *const _ as usize },
               148usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_class_decl_t),
                       "::",
                       stringify!(nsvar)));
}
#[test]
fn bindgen_test_layout_gnode_module_decl_t() {
    assert_eq!(::std::mem::size_of::<gnode_module_decl_t>(),
               112usize,
               concat!("Size of: ", stringify!(gnode_module_decl_t)));
    assert_eq!(::std::mem::align_of::<gnode_module_decl_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gnode_module_decl_t)));
    assert_eq!(unsafe { &(*(0 as *const gnode_module_decl_t)).base as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_module_decl_t),
                       "::",
                       stringify!(base)));
    assert_eq!(unsafe { &(*(0 as *const gnode_module_decl_t)).env as *const _ as usize },
               72usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_module_decl_t),
                       "::",
                       stringify!(env)));
    assert_eq!(unsafe { &(*(0 as *const gnode_module_decl_t)).access as *const _ as usize },
               80usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_module_decl_t),
                       "::",
                       stringify!(access)));
    assert_eq!(unsafe { &(*(0 as *const gnode_module_decl_t)).storage as *const _ as usize },
               84usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_module_decl_t),
                       "::",
                       stringify!(storage)));
    assert_eq!(unsafe { &(*(0 as *const gnode_module_decl_t)).identifier as *const _ as usize },
               88usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_module_decl_t),
                       "::",
                       stringify!(identifier)));
    assert_eq!(unsafe { &(*(0 as *const gnode_module_decl_t)).decls as *const _ as usize },
               96usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_module_decl_t),
                       "::",
                       stringify!(decls)));
    assert_eq!(unsafe { &(*(0 as *const gnode_module_decl_t)).symtable as *const _ as usize },
               104usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_module_decl_t),
                       "::",
                       stringify!(symtable)));
}
#[test]
fn bindgen_test_layout_gnode_binary_expr_t() {
    assert_eq!(::std::mem::size_of::<gnode_binary_expr_t>(),
               96usize,
               concat!("Size of: ", stringify!(gnode_binary_expr_t)));
    assert_eq!(::std::mem::align_of::<gnode_binary_expr_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gnode_binary_expr_t)));
    assert_eq!(unsafe { &(*(0 as *const gnode_binary_expr_t)).base as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_binary_expr_t),
                       "::",
                       stringify!(base)));
    assert_eq!(unsafe { &(*(0 as *const gnode_binary_expr_t)).op as *const _ as usize },
               72usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_binary_expr_t),
                       "::",
                       stringify!(op)));
    assert_eq!(unsafe { &(*(0 as *const gnode_binary_expr_t)).left as *const _ as usize },
               80usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_binary_expr_t),
                       "::",
                       stringify!(left)));
    assert_eq!(unsafe { &(*(0 as *const gnode_binary_expr_t)).right as *const _ as usize },
               88usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_binary_expr_t),
                       "::",
                       stringify!(right)));
}
#[test]
fn bindgen_test_layout_gnode_unary_expr_t() {
    assert_eq!(::std::mem::size_of::<gnode_unary_expr_t>(),
               88usize,
               concat!("Size of: ", stringify!(gnode_unary_expr_t)));
    assert_eq!(::std::mem::align_of::<gnode_unary_expr_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gnode_unary_expr_t)));
    assert_eq!(unsafe { &(*(0 as *const gnode_unary_expr_t)).base as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_unary_expr_t),
                       "::",
                       stringify!(base)));
    assert_eq!(unsafe { &(*(0 as *const gnode_unary_expr_t)).op as *const _ as usize },
               72usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_unary_expr_t),
                       "::",
                       stringify!(op)));
    assert_eq!(unsafe { &(*(0 as *const gnode_unary_expr_t)).expr as *const _ as usize },
               80usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_unary_expr_t),
                       "::",
                       stringify!(expr)));
}
#[test]
fn bindgen_test_layout_gnode_file_expr_t() {
    assert_eq!(::std::mem::size_of::<gnode_file_expr_t>(),
               88usize,
               concat!("Size of: ", stringify!(gnode_file_expr_t)));
    assert_eq!(::std::mem::align_of::<gnode_file_expr_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gnode_file_expr_t)));
    assert_eq!(unsafe { &(*(0 as *const gnode_file_expr_t)).base as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_file_expr_t),
                       "::",
                       stringify!(base)));
    assert_eq!(unsafe { &(*(0 as *const gnode_file_expr_t)).identifiers as *const _ as usize },
               72usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_file_expr_t),
                       "::",
                       stringify!(identifiers)));
    assert_eq!(unsafe { &(*(0 as *const gnode_file_expr_t)).location as *const _ as usize },
               80usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_file_expr_t),
                       "::",
                       stringify!(location)));
}
#[test]
fn bindgen_test_layout_gnode_literal_expr_t__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<gnode_literal_expr_t__bindgen_ty_1>(),
               8usize,
               concat!("Size of: ", stringify!(gnode_literal_expr_t__bindgen_ty_1)));
    assert_eq!(::std::mem::align_of::<gnode_literal_expr_t__bindgen_ty_1>(),
               8usize,
               concat!("Alignment of ",
                       stringify!(gnode_literal_expr_t__bindgen_ty_1)));
    assert_eq!(unsafe {
                   &(*(0 as *const gnode_literal_expr_t__bindgen_ty_1)).str as *const _ as usize
               },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_literal_expr_t__bindgen_ty_1),
                       "::",
                       stringify!(str)));
    assert_eq!(unsafe {
                   &(*(0 as *const gnode_literal_expr_t__bindgen_ty_1)).d as *const _ as usize
               },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_literal_expr_t__bindgen_ty_1),
                       "::",
                       stringify!(d)));
    assert_eq!(unsafe {
                   &(*(0 as *const gnode_literal_expr_t__bindgen_ty_1)).n64 as *const _ as usize
               },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_literal_expr_t__bindgen_ty_1),
                       "::",
                       stringify!(n64)));
    assert_eq!(unsafe {
                   &(*(0 as *const gnode_literal_expr_t__bindgen_ty_1)).r as *const _ as usize
               },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_literal_expr_t__bindgen_ty_1),
                       "::",
                       stringify!(r)));
}
#[test]
fn bindgen_test_layout_gnode_literal_expr_t() {
    assert_eq!(::std::mem::size_of::<gnode_literal_expr_t>(),
               88usize,
               concat!("Size of: ", stringify!(gnode_literal_expr_t)));
    assert_eq!(::std::mem::align_of::<gnode_literal_expr_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gnode_literal_expr_t)));
    assert_eq!(unsafe { &(*(0 as *const gnode_literal_expr_t)).base as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_literal_expr_t),
                       "::",
                       stringify!(base)));
    assert_eq!(unsafe { &(*(0 as *const gnode_literal_expr_t)).type_ as *const _ as usize },
               72usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_literal_expr_t),
                       "::",
                       stringify!(type_)));
    assert_eq!(unsafe { &(*(0 as *const gnode_literal_expr_t)).len as *const _ as usize },
               76usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_literal_expr_t),
                       "::",
                       stringify!(len)));
    assert_eq!(unsafe { &(*(0 as *const gnode_literal_expr_t)).value as *const _ as usize },
               80usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_literal_expr_t),
                       "::",
                       stringify!(value)));
}
#[test]
fn bindgen_test_layout_gnode_identifier_expr_t() {
    assert_eq!(::std::mem::size_of::<gnode_identifier_expr_t>(),
               112usize,
               concat!("Size of: ", stringify!(gnode_identifier_expr_t)));
    assert_eq!(::std::mem::align_of::<gnode_identifier_expr_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gnode_identifier_expr_t)));
    assert_eq!(unsafe { &(*(0 as *const gnode_identifier_expr_t)).base as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_identifier_expr_t),
                       "::",
                       stringify!(base)));
    assert_eq!(unsafe { &(*(0 as *const gnode_identifier_expr_t)).value as *const _ as usize },
               72usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_identifier_expr_t),
                       "::",
                       stringify!(value)));
    assert_eq!(unsafe { &(*(0 as *const gnode_identifier_expr_t)).value2 as *const _ as usize },
               80usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_identifier_expr_t),
                       "::",
                       stringify!(value2)));
    assert_eq!(unsafe { &(*(0 as *const gnode_identifier_expr_t)).symbol as *const _ as usize },
               88usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_identifier_expr_t),
                       "::",
                       stringify!(symbol)));
    assert_eq!(unsafe { &(*(0 as *const gnode_identifier_expr_t)).location as *const _ as usize },
               96usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_identifier_expr_t),
                       "::",
                       stringify!(location)));
    assert_eq!(unsafe { &(*(0 as *const gnode_identifier_expr_t)).upvalue as *const _ as usize },
               104usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_identifier_expr_t),
                       "::",
                       stringify!(upvalue)));
}
#[test]
fn bindgen_test_layout_gnode_keyword_expr_t() {
    assert_eq!(::std::mem::size_of::<gnode_keyword_expr_t>(),
               72usize,
               concat!("Size of: ", stringify!(gnode_keyword_expr_t)));
    assert_eq!(::std::mem::align_of::<gnode_keyword_expr_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gnode_keyword_expr_t)));
    assert_eq!(unsafe { &(*(0 as *const gnode_keyword_expr_t)).base as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_keyword_expr_t),
                       "::",
                       stringify!(base)));
}
#[test]
fn bindgen_test_layout_gnode_postfix_expr_t() {
    assert_eq!(::std::mem::size_of::<gnode_postfix_expr_t>(),
               88usize,
               concat!("Size of: ", stringify!(gnode_postfix_expr_t)));
    assert_eq!(::std::mem::align_of::<gnode_postfix_expr_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gnode_postfix_expr_t)));
    assert_eq!(unsafe { &(*(0 as *const gnode_postfix_expr_t)).base as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_postfix_expr_t),
                       "::",
                       stringify!(base)));
    assert_eq!(unsafe { &(*(0 as *const gnode_postfix_expr_t)).id as *const _ as usize },
               72usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_postfix_expr_t),
                       "::",
                       stringify!(id)));
    assert_eq!(unsafe { &(*(0 as *const gnode_postfix_expr_t)).list as *const _ as usize },
               80usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_postfix_expr_t),
                       "::",
                       stringify!(list)));
}
#[test]
fn bindgen_test_layout_gnode_postfix_subexpr_t__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<gnode_postfix_subexpr_t__bindgen_ty_1>(),
               8usize,
               concat!("Size of: ",
                       stringify!(gnode_postfix_subexpr_t__bindgen_ty_1)));
    assert_eq!(::std::mem::align_of::<gnode_postfix_subexpr_t__bindgen_ty_1>(),
               8usize,
               concat!("Alignment of ",
                       stringify!(gnode_postfix_subexpr_t__bindgen_ty_1)));
    assert_eq!(unsafe {
                   &(*(0 as *const gnode_postfix_subexpr_t__bindgen_ty_1)).expr as *const _ as usize
               },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_postfix_subexpr_t__bindgen_ty_1),
                       "::",
                       stringify!(expr)));
    assert_eq!(unsafe {
                   &(*(0 as *const gnode_postfix_subexpr_t__bindgen_ty_1)).args as *const _ as usize
               },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_postfix_subexpr_t__bindgen_ty_1),
                       "::",
                       stringify!(args)));
}
#[test]
fn bindgen_test_layout_gnode_postfix_subexpr_t() {
    assert_eq!(::std::mem::size_of::<gnode_postfix_subexpr_t>(),
               80usize,
               concat!("Size of: ", stringify!(gnode_postfix_subexpr_t)));
    assert_eq!(::std::mem::align_of::<gnode_postfix_subexpr_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gnode_postfix_subexpr_t)));
    assert_eq!(unsafe { &(*(0 as *const gnode_postfix_subexpr_t)).base as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_postfix_subexpr_t),
                       "::",
                       stringify!(base)));
}
#[test]
fn bindgen_test_layout_gnode_list_expr_t() {
    assert_eq!(::std::mem::size_of::<gnode_list_expr_t>(),
               96usize,
               concat!("Size of: ", stringify!(gnode_list_expr_t)));
    assert_eq!(::std::mem::align_of::<gnode_list_expr_t>(),
               8usize,
               concat!("Alignment of ", stringify!(gnode_list_expr_t)));
    assert_eq!(unsafe { &(*(0 as *const gnode_list_expr_t)).base as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_list_expr_t),
                       "::",
                       stringify!(base)));
    assert_eq!(unsafe { &(*(0 as *const gnode_list_expr_t)).ismap as *const _ as usize },
               72usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_list_expr_t),
                       "::",
                       stringify!(ismap)));
    assert_eq!(unsafe { &(*(0 as *const gnode_list_expr_t)).list1 as *const _ as usize },
               80usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_list_expr_t),
                       "::",
                       stringify!(list1)));
    assert_eq!(unsafe { &(*(0 as *const gnode_list_expr_t)).list2 as *const _ as usize },
               88usize,
               concat!("Alignment of field: ",
                       stringify!(gnode_list_expr_t),
                       "::",
                       stringify!(list2)));
}
