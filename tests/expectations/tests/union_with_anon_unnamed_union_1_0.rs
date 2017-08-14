/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


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
impl <T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) { }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash)]
pub struct foo {
    pub a: __BindgenUnionField<::std::os::raw::c_uint>,
    pub __bindgen_anon_1: __BindgenUnionField<foo__bindgen_ty_1>,
    pub bindgen_union_field: u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash)]
pub struct foo__bindgen_ty_1 {
    pub b: __BindgenUnionField<::std::os::raw::c_ushort>,
    pub c: __BindgenUnionField<::std::os::raw::c_uchar>,
    pub bindgen_union_field: u16,
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<foo__bindgen_ty_1>() , 2usize , concat !
               ( "Size of: " , stringify ! ( foo__bindgen_ty_1 ) ));
    assert_eq! (::std::mem::align_of::<foo__bindgen_ty_1>() , 2usize , concat
                ! ( "Alignment of " , stringify ! ( foo__bindgen_ty_1 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const foo__bindgen_ty_1 ) ) . b as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( foo__bindgen_ty_1 ) ,
                "::" , stringify ! ( b ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const foo__bindgen_ty_1 ) ) . c as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( foo__bindgen_ty_1 ) ,
                "::" , stringify ! ( c ) ));
}
impl Clone for foo__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(::std::mem::size_of::<foo>() , 4usize , concat ! (
               "Size of: " , stringify ! ( foo ) ));
    assert_eq! (::std::mem::align_of::<foo>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( foo ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const foo ) ) . a as * const _ as usize } ,
                0usize , concat ! (
                "Alignment of field: " , stringify ! ( foo ) , "::" ,
                stringify ! ( a ) ));
}
impl Clone for foo {
    fn clone(&self) -> Self { *self }
}
