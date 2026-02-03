pub mod client;
pub mod error;
pub mod resources;
pub mod types;

pub use client::BlindPay;
pub use error::{BlindPayError, Result};
pub use types::*;

// Re-export commonly used types
pub mod prelude {
    pub use crate::client::BlindPay;
    pub use crate::error::{BlindPayError, Result};
    pub use crate::types::*;
}
