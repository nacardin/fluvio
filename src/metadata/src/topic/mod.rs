mod spec;
mod status;

pub use self::spec::*;
pub use self::status::*;


pub const PENDING_REASON: &'static str = "waiting for live spus";



#[cfg(feature = "k8")]
mod k8;
#[cfg(feature = "k8")]
pub use k8::*;
