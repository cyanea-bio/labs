//! Shared primitives, traits, and utilities for the Cyanea bioinformatics ecosystem.
//!
//! `cyanea-core` provides the foundation that all other Cyanea crates build on:
//!
//! - **Error types** — [`CyaneaError`] and [`Result`] for structured error handling
//! - **Traits** — Core abstractions like [`Sequence`], [`ContentAddressable`], [`Compressible`]
//! - **Hashing** — SHA-256 content addressing for data integrity
//! - **Compression** — zstd and gzip with algorithm auto-detection
//! - **Memory mapping** — Zero-copy file access (std feature only)
//!
//! The `io` feature (part of the default `std`) links the standard library for
//! [`CyaneaError::Io`] and [`hash::sha256_file`]. With neither feature the crate
//! is `no_std` and needs only `alloc`.

#![cfg_attr(not(feature = "io"), no_std)]

extern crate alloc;

pub mod bitvec;
pub mod error;
pub mod fenwick;
pub mod hash;
pub mod prob;
pub mod traits;

#[cfg(feature = "std")]
pub mod compress;

#[cfg(feature = "std")]
pub mod mmap;

pub use bitvec::{RankSelectBitVec, WaveletMatrix};
pub use error::{CyaneaError, Result};
pub use fenwick::FenwickTree;
pub use prob::{LogProb, PhredProb};
pub use traits::*;
