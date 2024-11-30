use std::mem;

/// Quake algorithm
#[allow(non_upper_case_globals)]
pub unsafe fn q_rsqrt(number: f32) -> f32 {
    let mut i: i32;
    let (x2, mut y): (f32, f32);
    const threehalfs: f32 = 1.5;

    x2 = number * 0.5;
    y  = number;
    i  = mem::transmute::<f32, i32>(y);      // evil floating point bit level hacking
    i  = 0x5f3759df - ( i >> 1 );            // what the fuck?
    y  = mem::transmute::<i32, f32>(i);
    y  = y * ( threehalfs - ( x2 * y * y )); // 1st iteration
//  y  = y * ( threehalfs - ( x2 * y * y )); // 2nd iteration, this can be removed

    return y;
}
