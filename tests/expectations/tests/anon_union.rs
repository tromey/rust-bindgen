/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
pub struct TErrorResult {
    pub mResult: ::std::os::raw::c_int,
    pub __bindgen_anon_1: TErrorResult__bindgen_ty_1,
    pub mMightHaveUnreported: bool,
    pub mUnionState: TErrorResult_UnionState,
}
pub const TErrorResult_UnionState_HasException: TErrorResult_UnionState =
    TErrorResult_UnionState::HasMessage;
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TErrorResult_UnionState { HasMessage = 0, }
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash)]
pub struct TErrorResult_Message {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash)]
pub struct TErrorResult_DOMExceptionInfo {
    pub _address: u8,
}
#[repr(C)]
pub union TErrorResult__bindgen_ty_1 {
    pub mMessage: *mut TErrorResult_Message,
    pub mDOMExceptionInfo: *mut TErrorResult_DOMExceptionInfo,
}
impl Default for TErrorResult__bindgen_ty_1 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
impl Default for TErrorResult {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
pub struct ErrorResult {
    pub _base: TErrorResult,
}
#[test]
fn bindgen_test_layout_ErrorResult() {
    assert_eq!(::std::mem::size_of::<ErrorResult>() , 24usize , concat ! (
               "Size of: " , stringify ! ( ErrorResult ) ));
    assert_eq! (::std::mem::align_of::<ErrorResult>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( ErrorResult ) ));
}
impl Default for ErrorResult {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[test]
fn __bindgen_test_layout_TErrorResult_open0_int_close0_instantiation() {
    assert_eq!(::std::mem::size_of::<TErrorResult>() , 24usize , concat ! (
               "Size of template specialization: " , stringify ! (
               TErrorResult ) ));
    assert_eq!(::std::mem::align_of::<TErrorResult>() , 8usize , concat ! (
               "Alignment of template specialization: " , stringify ! (
               TErrorResult ) ));
}
