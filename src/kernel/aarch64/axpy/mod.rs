

pub unsafe fn daxpy(n:usize, a:f64, x:&Vec<f64>, icx:usize, y:&mut Vec<f64>, icy:usize){
    assert!(x.len() >= n * icx);
    assert!(y.len() >= n * icy);
    use std::arch::aarch64::{float64x2_t, vfmaq_f64, vld1q_dup_f64, vld1q_f64, vst1q_f64};

    let n1: i128 = (n as i128) & -2;
    let n1: usize = n1 as usize;
    let mut i:usize = 0;
    unsafe {
        let raw_a: float64x2_t = vld1q_dup_f64(&a);
        while i < n1{
            let arr_x: float64x2_t = vld1q_f64(&x[i]);
            let arr_y: float64x2_t = vld1q_f64(&y[i]);
            let arr_z: float64x2_t = vfmaq_f64(arr_y, arr_x, raw_a);
            vst1q_f64(&mut y[i], arr_z);
            i += 2;
        }
    }
    while i < n{
        y[i] += a * x[i];
        i += 1;
    }
}

pub unsafe fn daxpy_non(n:usize, a:f64, x:&Vec<f64>, icx:usize, y:&mut Vec<f64>, icy:usize){
    assert!(x.len() >= n * icx);
    assert!(y.len() >= n * icy);
    let mut iy: usize = 0;
    let mut ix: usize = 0;
    for _ in 0..n{
        y[iy] += a * x[ix];
        iy += icy;
        ix += icx;
    }
}
