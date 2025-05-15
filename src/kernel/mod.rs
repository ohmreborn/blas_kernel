
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
pub mod aarch64;

#[cfg(not(target_arch = "aarch64"))]
pub mod other;
