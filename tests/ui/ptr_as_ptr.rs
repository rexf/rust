//@run-rustfix
//@aux-build:proc_macros.rs:proc-macro

#![warn(clippy::ptr_as_ptr)]

#[macro_use]
extern crate proc_macros;

mod issue_11278_a {
    #[derive(Debug)]
    pub struct T<D: std::fmt::Debug + ?Sized> {
        pub p: D,
    }
}

mod issue_11278_b {
    pub fn f(o: &mut super::issue_11278_a::T<dyn std::fmt::Debug>) -> super::issue_11278_a::T<String> {
        // Retain `super`
        *unsafe { Box::from_raw(Box::into_raw(Box::new(o)) as *mut super::issue_11278_a::T<String>) }
    }
}

#[inline_macros]
fn main() {
    let ptr: *const u32 = &42_u32;
    let mut_ptr: *mut u32 = &mut 42_u32;

    let _ = ptr as *const i32;
    let _ = mut_ptr as *mut i32;

    // Make sure the lint can handle the difference in their operator precedences.
    unsafe {
        let ptr_ptr: *const *const u32 = &ptr;
        let _ = *ptr_ptr as *const i32;
    }

    // Changes in mutability. Do not lint this.
    let _ = ptr as *mut i32;
    let _ = mut_ptr as *const i32;

    // `pointer::cast` cannot perform unsized coercions unlike `as`. Do not lint this.
    let ptr_of_array: *const [u32; 4] = &[1, 2, 3, 4];
    let _ = ptr_of_array as *const [u32];
    let _ = ptr_of_array as *const dyn std::fmt::Debug;

    // Ensure the lint doesn't produce unnecessary turbofish for inferred types.
    let _: *const i32 = ptr as *const _;
    let _: *mut i32 = mut_ptr as _;

    // Make sure the lint is triggered inside a macro
    let _ = inline!($ptr as *const i32);

    // Do not lint inside macros from external crates
    let _ = external!($ptr as *const i32);
}

#[clippy::msrv = "1.37"]
fn _msrv_1_37() {
    let ptr: *const u32 = &42_u32;
    let mut_ptr: *mut u32 = &mut 42_u32;

    // `pointer::cast` was stabilized in 1.38. Do not lint this
    let _ = ptr as *const i32;
    let _ = mut_ptr as *mut i32;
}

#[clippy::msrv = "1.38"]
fn _msrv_1_38() {
    let ptr: *const u32 = &42_u32;
    let mut_ptr: *mut u32 = &mut 42_u32;

    let _ = ptr as *const i32;
    let _ = mut_ptr as *mut i32;
}
