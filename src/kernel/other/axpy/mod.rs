pub unsafe fn daxpy(n:usize, a:f64, x:&Vec<f64>, icx:usize, y:&mut Vec<f64>, icy:usize){
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
