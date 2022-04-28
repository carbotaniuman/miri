// compile-flags: -Zmiri-permissive-provenance -Zmiri-disable-stacked-borrows
#![feature(strict_provenance)]

use std::mem::transmute;
use std::ptr::from_exposed_addr;

fn ptr_roundtrip_out_of_bounds() {
    let x: i32 = 3;
    let x_ptr = &x as *const i32;

    let x_usize = x_ptr.wrapping_offset(128).expose_addr();

    let ptr = from_exposed_addr::<i32>(x_usize).wrapping_offset(-128);
    assert_eq!(unsafe { *ptr }, 3);
}

fn ptr_roundtrip_out_of_bounds_with_addr() {
    let x: i32 = 0;
    let y: i32 = 1;

    let x_ptr = &x as *const i32;
    let y_ptr = &y as *const i32;

    let x_usize = x_ptr.expose_addr();
    let y_usize = y_ptr.expose_addr();

    let ptr = from_exposed_addr::<i32>(y_usize).with_addr(x_usize);
    assert_eq!(unsafe { *ptr }, 0);
}

fn ptr_roundtrip_non_perfect() {
    let arr = [0, 1];
    let first_ptr = &arr[0] as *const i32;

    let ptr = from_exposed_addr::<i32>(first_ptr.expose_addr());
    let ptr = unsafe { ptr.add(1) };
    assert_eq!(unsafe { *ptr }, 1);
}

fn fn_ptr_roundtrip() {
    fn foo() -> i32 {
        5
    }

    let func: fn() -> i32 = foo;
    let func_usize = (func as *const ()).expose_addr();

    let ptr: fn() -> i32 = unsafe {
        transmute::<_, _>(from_exposed_addr::<()>(func_usize))
    };
    assert_eq!(ptr(), 5);
}

fn main() {
    ptr_roundtrip_out_of_bounds();
    ptr_roundtrip_out_of_bounds_with_addr();
    ptr_roundtrip_non_perfect();
    fn_ptr_roundtrip();
}
