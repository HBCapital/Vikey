// methods/mod.rs

pub mod telex;
pub mod telex_v2;
pub mod viqr;
pub mod vni;

pub use telex::TelexMethod;
pub use telex_v2::TelexMethodV2;
pub use viqr::VIQRMethod;
pub use vni::VNIMethod;
