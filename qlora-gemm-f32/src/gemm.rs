pub mod f32 {
    type T = f32;
    qlora_gemm_common::gemm_def!(f32, 2);
}
