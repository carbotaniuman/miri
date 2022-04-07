// compile-flags: -Zmiri-disable-stacked-borrows -Zmiri-seed=0

fn ptr_int_guess() {
    let x: i32 = 3;
    // we fix the seed so that this pointer's value is 0x23080
    let _x_ptr = &x as *const i32;

    // try and get lucky with a guess
    let x_usize = 0x23080 as usize;
    let ptr = x_usize as *const i32;
    assert_eq!(unsafe { *ptr }, 3); //~ ERROR 0x23080 is not a valid pointer
}

fn main() {
    ptr_int_guess();
}
