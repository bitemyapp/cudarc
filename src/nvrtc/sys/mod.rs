#[cfg(feature = "cuda-11070")]
mod sys_11070;
#[cfg(feature = "cuda-11070")]
pub use sys_11070::*;

#[cfg(feature = "cuda-11080")]
mod sys_11080;
#[cfg(feature = "cuda-11080")]
pub use sys_11080::*;

#[cfg(feature = "cuda-12000")]
mod sys_12000;
#[cfg(feature = "cuda-12000")]
pub use sys_12000::*;

#[cfg(feature = "cuda-12010")]
mod sys_12010;
#[cfg(feature = "cuda-12010")]
pub use sys_12010::*;

#[cfg(feature = "cuda-12020")]
mod sys_12020;
#[cfg(feature = "cuda-12020")]
pub use sys_12020::*;

#[cfg(feature = "cuda-12030")]
mod sys_12030;
#[cfg(feature = "cuda-12030")]
pub use sys_12030::*;

pub unsafe fn lib() -> &'static Lib {
    static LIB: std::sync::OnceLock<Lib> = std::sync::OnceLock::new();
    LIB.get_or_init(|| Lib::new(libloading::library_filename("nvrtc")).unwrap())
}
