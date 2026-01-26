pub mod f64 {
    type T = f64;
    qlora_gemm_common::gemm_def!(f64, 1);
}
