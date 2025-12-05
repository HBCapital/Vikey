// methods/mod.rs

pub mod telex;
pub mod vni;
pub mod viqr;
pub mod telex_v2;

pub use telex::TelexMethod;
pub use vni::VNIMethod;
pub use viqr::VIQRMethod;
pub use telex_v2::TelexMethodV2;
