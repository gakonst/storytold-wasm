//! Useful functions for checking features.
// workerd has no CPU thread pool.
pub fn get_num_threads() -> usize { 1 }

pub fn has_accelerate() -> bool {
    cfg!(feature = "accelerate")
}

pub fn has_mkl() -> bool {
    cfg!(feature = "mkl")
}

pub fn cuda_is_available() -> bool {
    cfg!(feature = "cuda")
}

pub fn metal_is_available() -> bool {
    cfg!(feature = "metal")
}

pub fn with_avx() -> bool {
    cfg!(target_feature = "avx")
}

pub fn with_neon() -> bool {
    cfg!(target_feature = "neon")
}

pub fn with_simd128() -> bool {
    cfg!(target_feature = "simd128")
}

pub fn with_f16c() -> bool {
    cfg!(target_feature = "f16c")
}
