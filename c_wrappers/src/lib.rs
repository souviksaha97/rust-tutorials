
#[no_mangle]
pub extern "C" fn adder(x: u32, y: u32) -> u32{
    x + y
}

#[no_mangle]
pub extern "C" fn add_pointers(a: *const i32, b: *const i32) -> i32 {
    let a_val: i32 = unsafe { *a };
    let b_val: i32 = unsafe { *b };
    a_val + b_val
}


#[no_mangle]
pub extern "C" fn sum_of_array(array: *const u32, len: usize) -> u32 {
    let arr: &[u32] = unsafe { std::slice::from_raw_parts(array, len) };
    arr.iter().sum()
}

#[no_mangle]
pub extern "C" fn sum_of_arrays(array1: *const u32, array2: *const u32, len: usize) -> *mut u32{
    let arr1: &[u32] = unsafe { std::slice::from_raw_parts(array1, len) };
    let arr2: &[u32] = unsafe { std::slice::from_raw_parts(array2, len) };
    let mut result: Vec<u32> = vec![0; len];
    for i in 0..len {
        result[i] = arr1[i] + arr2[i];
    }
    let result_ptr: *mut u32 = result.as_mut_ptr();
    std::mem::forget(result);
    result_ptr
}
