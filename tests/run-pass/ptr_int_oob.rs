// compile-flags: -Zmiri-disable-stacked-borrows

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
    let ptr = with_addr(ptr, x_usize);
    assert_eq!(unsafe { *ptr }, 0);
}

fn main() {
    ptr_roundtrip_out_of_bounds();
    ptr_roundtrip_out_of_bounds_with_addr();
}

pub fn with_addr<T>(ptr: *const T, addr: usize) -> *const T
    where
        T: Sized,
{
    // FIXME(strict_provenance_magic): I am magic and should be a compiler intrinsic.
    //
    // In the mean-time, this operation is defined to be "as if" it was
    // a wrapping_offset, so we can emulate it as such. This should properly
    // restore pointer provenance even under today's compiler.
    let self_addr = ptr as isize;
    let dest_addr = addr as isize;
    let offset = dest_addr.wrapping_sub(self_addr);

    // This is the canonical desugarring of this operation
    ptr.cast::<u8>().wrapping_offset(offset).cast::<T>()
}
