//! Reader and writer for OBJ model
//! 
//! The OBJ file format is a simple data-format that represents 3D geometry alone
//!  — namely, the position of each vertex, the UV position of each texture coordinate vertex, vertex normals, 
//! and the faces that make each polygon defined as a list of vertices, and texture vertices.
//!
//! - cube.obj
//! 
//! ```
//! # 8 vertices, 12 faces
//! mtllib cube.mtl
//! v 1 -1 1
//! vn 0.57735 -0.57735 0.57735
//! v 1 1 1
//! vn 0.57735 0.57735 0.57735
//! v -1 1 1
//! vn -0.333333 0.666667 0.666667
//! v -1 -1 1
//! vn -0.816497 -0.408248 0.408248
//! v 1 -1 -1
//! vn 0.408248 -0.408248 -0.816497
//! v 1 1 -1
//! vn 0.666667 0.666667 -0.333333
//! v -1 1 -1
//! vn -0.666667 0.333333 -0.666667
//! v -1 -1 -1
//! vn -0.408248 -0.816497 -0.408248
//! usemtl mat000
//! f 1//1 2//2 3//3
//! f 1//1 3//3 4//4
//! f 1//1 6//6 2//2
//! f 6//6 1//1 5//5
//! f 5//5 1//1 8//8
//! f 8//8 1//1 4//4
//! f 8//8 4//4 7//7
//! f 7//7 4//4 3//3
//! f 7//7 3//3 6//6
//! f 6//6 3//3 2//2
//! f 7//7 6//6 5//5
//! f 7//7 5//5 8//8
//! ```
//! 
//! - cube.mtl
//!
//! ```
//! newmtl mat000
//! Ka 0.5000 0.5000 0.5000
//! Kd 0 0 0
//! Tr 0
//! illum 1



mod writer;

pub use self::writer::save;