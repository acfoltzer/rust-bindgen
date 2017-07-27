/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod JS {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Rooted {
            pub _address: u8,
        }
        pub type Rooted_ElementType<T> = T;
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy)]
    pub struct c {
        pub b: u8,
    }
    #[test]
    fn bindgen_test_layout_c() {
        assert_eq!(::std::mem::size_of::<c>() , 1usize , concat ! (
                   "Size of: " , stringify ! ( c ) ));
        assert_eq! (::std::mem::align_of::<c>() , 1usize , concat ! (
                    "Alignment of " , stringify ! ( c ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const c ) ) . b as * const _ as usize } ,
                    0usize , concat ! (
                    "Alignment of field: " , stringify ! ( c ) , "::" ,
                    stringify ! ( b ) ));
    }
    impl Clone for c {
        fn clone(&self) -> Self { *self }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy)]
    pub struct B {
        pub a: root::c,
    }
    #[test]
    fn bindgen_test_layout_B() {
        assert_eq!(::std::mem::size_of::<B>() , 1usize , concat ! (
                   "Size of: " , stringify ! ( B ) ));
        assert_eq! (::std::mem::align_of::<B>() , 1usize , concat ! (
                    "Alignment of " , stringify ! ( B ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const B ) ) . a as * const _ as usize } ,
                    0usize , concat ! (
                    "Alignment of field: " , stringify ! ( B ) , "::" ,
                    stringify ! ( a ) ));
    }
    impl Clone for B {
        fn clone(&self) -> Self { *self }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct StaticRefPtr {
        pub _address: u8,
    }
    #[test]
    fn __bindgen_test_layout_StaticRefPtr_open0_B_close0_instantiation() {
        assert_eq!(::std::mem::size_of::<root::StaticRefPtr>() , 1usize ,
                   concat ! (
                   "Size of template specialization: " , stringify ! (
                   root::StaticRefPtr ) ));
        assert_eq!(::std::mem::align_of::<root::StaticRefPtr>() , 1usize ,
                   concat ! (
                   "Alignment of template specialization: " , stringify ! (
                   root::StaticRefPtr ) ));
    }
}
