//! This crate provides Mesh readers and writers and basic model processing functions.

#![crate_name = "meshes"]
#![crate_type = "rlib"]

pub mod io;
pub mod utils;

mod mesh;
mod vertex;

pub use vertex::Vertex;
pub use mesh::Mesh;