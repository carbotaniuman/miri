// compile-flags: -Zmiri-permissive-provenance -Zmiri-disable-stacked-borrows
#![feature(strict_provenance)]

fn ptr_roundtrip_out_of_bounds() {
    let x: i32 = 3;
    let x_ptr = &x as *const i32;

    let x_usize = x_ptr.wrapping_offset(128) as usize;

    let ptr = (x_usize as *const i32).wrapping_offset(-128);
    assert_eq!(unsafe { *ptr }, 3);
}

fn ptr_roundtrip_out_of_bounds_with_addr() {
    let x: i32 = 0;
    let y: i32 = 1;

    let x_ptr = &x as *const _;
    let y_ptr = &y as *const _;

    let x_usize = x_ptr as usize;
    let y_usize = y_ptr as usize;

    let ptr = y_usize as *const i32;
    let ptr = ptr.with_addr(x_usize);
    assert_eq!(unsafe { *ptr }, 0);
}

fn main() {
    ptr_roundtrip_out_of_bounds();
    ptr_roundtrip_out_of_bounds_with_addr();
}
