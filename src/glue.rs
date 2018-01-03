use std::os::raw::c_char;
use std::ffi::CString;

#[no_mangle]
pub unsafe extern "C" fn g_destroy_cstring(s: *mut c_char) {
    CString::from_raw(s);
}

#[no_mangle]
pub unsafe extern "C" fn g_alloc(len: usize) -> *mut u8 {
    let ptr_size = ::std::mem::size_of::<usize>();
    let mut v: Vec<u8> = Vec::with_capacity(len + ptr_size);
    v.set_len(len + ptr_size);

    //eprintln!("alloc: {:?} {}+{}", &v[0] as *const u8, ptr_size, len);

    *(&mut v[0] as *mut u8 as *mut usize) = len;
    let mut v = v.into_boxed_slice();
    let addr = &mut v[ptr_size] as *mut u8;
    Box::into_raw(v);

    addr
}

#[no_mangle]
pub unsafe extern "C" fn g_free(mem: *mut u8) {
    let ptr_size = ::std::mem::size_of::<usize>();
    let mem = mem.offset(-(ptr_size as isize));
    let len = *(mem as *const u8 as *const usize);

    //eprintln!("free: {:?} {}+{}", mem, ptr_size, len);

    Box::from_raw(::std::slice::from_raw_parts_mut(mem, len + ptr_size));
}

#[test]
fn test_alloc_free() {
    for i in 0..10000000 {
        unsafe {
            let m = g_alloc(i % 1024 + 1);
            *m = 42;
            g_free(m);
        }
    }
}
