// Fiat-crypto generates some unused type aliases, but we don't want to edit the generated code at all.
#![allow(dead_code)]

pub mod fp;
pub mod fq;
pub mod fr;

pub use fp::wrapper::Fp as Fp32;
pub use fq::wrapper::Fq as Fq32;
pub use fr::wrapper::Fr as Fr32;
