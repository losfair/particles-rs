extern "C" {
    fn i_rand01() -> f64;
}

pub fn rand01() -> f64 {
    unsafe {
        i_rand01()
    }
}
