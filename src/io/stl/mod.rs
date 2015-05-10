
//! Reader and writer for STL model
//! 
//! STL (STereoLithography) is a file format native to the stereolithography CAD software created by 3D Systems.
//! 
//! The STL format specifies both ASCII and binary representations:
//!
//! - binary format
//! 
//! ```
//! UINT8[80] : Header
//! UINT32 : Number of triangles
//! foreach triangle
//!     REAL32[3] : Normal vector
//!     REAL32[3] : Vertex 1
//!     REAL32[3] : Vertex 2
//!     REAL32[3] : Vertex 3
//!     UINT16 : Attribute byte count
//! end
//! ```
//! 
//! - ascii format
//! 
//! ```
//! solid name
//! facet normal ni nj nk
//!     outer loop
//!         vertex v1x v1y v1z
//!         vertex v2x v2y v2z
//!         vertex v3x v3y v3z
//!     endloop
//! endfacet
//! endsolid name
//! ```
//! 
//! cube.stl
//!
//! ```
//! solid ascii
//!   facet normal 10.000000 2.220446e-016 0.000000e+000
//!     outer loop
//!       vertex   5.000000 -5.000000 10.000000
//!       vertex   5.000000 -5.000000 0.000000e+000
//!       vertex   5.000000 5.000000 10.000000
//!     endloop
//!   endfacet
//!   facet normal 10.000000 2.220446e-016 0.000000e+000
//!     outer loop
//!       vertex   5.000000 5.000000 10.000000
//!       vertex   5.000000 -5.000000 0.000000e+000
//!       vertex   5.000000 5.000000 0.000000e+000
//!     endloop
//!   endfacet
//!   facet normal 2.220446e-016 -10.000000 0.000000e+000
//!     outer loop
//!       vertex   -5.000000 -5.000000 10.000000
//!       vertex   -5.000000 -5.000000 0.000000e+000
//!       vertex   5.000000 -5.000000 10.000000
//!     endloop
//!   endfacet
//!   facet normal 2.220446e-016 -10.000000 0.000000e+000
//!     outer loop
//!       vertex   5.000000 -5.000000 10.000000
//!       vertex   -5.000000 -5.000000 0.000000e+000
//!       vertex   5.000000 -5.000000 0.000000e+000
//!     endloop
//!   endfacet
//!   facet normal -10.000000 -6.661338e-016 -0.000000e+000
//!     outer loop
//!       vertex   -5.000000 5.000000 10.000000
//!       vertex   -5.000000 5.000000 0.000000e+000
//!       vertex   -5.000000 -5.000000 10.000000
//!     endloop
//!   endfacet
//!   facet normal -10.000000 -6.661338e-016 0.000000e+000
//!     outer loop
//!       vertex   -5.000000 -5.000000 10.000000
//!       vertex   -5.000000 5.000000 0.000000e+000
//!       vertex   -5.000000 -5.000000 0.000000e+000
//!     endloop
//!   endfacet
//!   facet normal -5.551115e-017 10.000000 0.000000e+000
//!     outer loop
//!       vertex   5.000000 5.000000 10.000000
//!       vertex   5.000000 5.000000 0.000000e+000
//!       vertex   -5.000000 5.000000 10.000000
//!     endloop
//!   endfacet
//!   facet normal -5.551115e-017 10.000000 0.000000e+000
//!     outer loop
//!       vertex   -5.000000 5.000000 10.000000
//!       vertex   5.000000 5.000000 0.000000e+000
//!       vertex   -5.000000 5.000000 0.000000e+000
//!     endloop
//!   endfacet
//!   facet normal 0.000000e+000 0.000000e+000 10.000000
//!     outer loop
//!       vertex   5.000000 5.000000 10.000000
//!       vertex   -5.000000 5.000000 10.000000
//!       vertex   5.000000 -5.000000 10.000000
//!     endloop
//!   endfacet
//!   facet normal 0.000000e+000 0.000000e+000 10.000000
//!     outer loop
//!       vertex   5.000000 -5.000000 10.000000
//!       vertex   -5.000000 5.000000 10.000000
//!       vertex   -5.000000 -5.000000 10.000000
//!     endloop
//!   endfacet
//!   facet normal 0.000000e+000 -0.000000e+000 -10.000000
//!     outer loop
//!       vertex   -5.000000 5.000000 0.000000e+000
//!       vertex   5.000000 5.000000 0.000000e+000
//!       vertex   -5.000000 -5.000000 0.000000e+000
//!     endloop
//!   endfacet
//!   facet normal 0.000000e+000 0.000000e+000 -10.000000
//!     outer loop
//!       vertex   -5.000000 -5.000000 0.000000e+000
//!       vertex   5.000000 5.000000 0.000000e+000
//!       vertex   5.000000 -5.000000 0.000000e+000
//!     endloop
//!   endfacet
//! endsolid
//! ```
//!
//! - references
//!
//! [The STL Format](http://www.ennex.com/~fabbers/StL.asp)
//!
//! [wikipedia](http://en.wikipedia.org/wiki/STL_%28file_format%29)

mod reader;
mod writer;

pub use self::reader::load;
pub use self::writer::save;