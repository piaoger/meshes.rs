//! Reader and writer for PLY model
//!
//! PLY is a computer file format known as the Polygon File Format or the Stanford Triangle Format.
//! It was principally designed to store three-dimensional data from 3D scanners.
//!
//! - cube.ply
//! 
//! ```
//! ply
//! format ascii 1.0
//! comment solidmcp generated
//! element vertex 8 
//! property float x
//! property float y
//! property float z
//! element face 12
//! property list uchar int vertex_indices
//! end_header
//! 5 -5 10
//! 5 -5 0
//! 5 5 10
//! 5 5 0
//! -5 -5 10
//! -5 -5 0
//! -5 5 10
//! -5 5 0
//! 3 0 1 2
//! 3 2 1 3
//! 3 4 5 0
//! 3 0 5 1
//! 3 6 7 4
//! 3 4 7 5
//! 3 2 3 6
//! 3 6 3 7
//! 3 2 6 0
//! 3 0 6 4
//! 3 7 3 5
//! 3 5 3 1
//! ```

mod writer;

pub use self::writer::save;