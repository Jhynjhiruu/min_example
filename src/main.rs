use std::ffi::c_int;

extern "C" {
    fn build2_function() -> c_int;
}

fn main() {
    println!("build2_function: {}", unsafe { build2_function() });
}
