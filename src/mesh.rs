

/// Triangle Mesh
#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<f32>,
    pub faces: Vec<u32>,
    pub normals: Option<Vec<f32>>,
    pub texcoords: Option<Vec<f32>>,
    pub bounds: Option<Vec<f32>>,
}

impl Mesh {
    /// Create a new mesh specifying the geometry for the mesh
    pub fn new(verts: Vec<f32>, norm: Option<Vec<f32>>, tex: Option<Vec<f32>>, faces: Vec<u32>) -> Mesh {
        Mesh { vertices: verts, normals: norm, texcoords: tex, faces: faces, bounds: None }
    }
}
