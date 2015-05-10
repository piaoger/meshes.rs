
//! # A mesh processing library written in Rust.
//!
//! This crate provides Mesh readers and writers and basic model processing functions.
//!
//! Supported File Formats: STL, OBJ, PLY
//! 
//! Mesh Processing: None

#![crate_name = "meshes"]
#![crate_type = "rlib"]

pub mod io;
pub mod utils;
pub mod simplification;

mod mesh;
mod vertex;

pub use vertex::Vertex;
pub use mesh::Mesh;