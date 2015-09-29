
// //! Stan Melax's progressive mesh algorithm
// //! 
// //! - Homepage: http://www.melax.com/polychop/
// //! - Article:  http://www.melax.com/gdmag.pdf
// //! - updated code: https://github.com/melax/sandbox/tree/master/bunnylod
// //! 
// //! A Javascript port:
// //!
// //!    - http://jabtunes.com/labs/3d/bunny/SimplifyModifier.js
// //!    - http://jabtunes.com/labs/3d/bunny/
// //! 
// //!  The algorithm is also used by [OGRE](http://sourceforge.net/p/ogre/code/HEAD/tree/trunk/OgreMain/src/OgreProgressiveMesh.cpp)
// //!  
// //!     - http://jabtunes.com/labs/3d/bunny/SimplifyModifier.js
// //!     - http://jabtunes.com/labs/3d/bunny/
// //! 
// //!  The algorithm is also used by [OGRE](http://sourceforge.net/p/ogre/code/HEAD/tree/trunk/OgreMain/src/OgreProgressiveMesh.cpp)
// //!     

// use std;

// use geom::Vector3;
// use utils::*;


// pub struct Mesh {
//     pub vertices: Vec<Vertex>,
//     pub triangles: Vec<Triangle>
// }

// impl Mesh {
//     pub fn new() -> Mesh {
//         Mesh {
//             vertices: vec![],
//             triangles: vec![],
//         }
//     }

//     pub fn build(&mut self, vertices: Vec<f32>, faces: Vec<u32>) {

//         let mut verts = vec![];
//         for i in 0 ..vertices.len() / 3  {
//             let vert = Box::new(Vertex::new(vertices[3*i], vertices[3*i+1], vertices[3*i+2], i));
//             verts.push(vert);
//         }

//         let mut triangles = vec![];
//         for i in 0 ..faces.len() / 3  {
//             let i0 = faces[3*i] as usize;
//             let i1 = faces[3*i+1] as usize;
//             let i2 = faces[3*i+2] as usize;
//             triangles.push(Box::new(Triangle::new(
//                 Box::new((*(verts[i0])).clone()), Box::new((*(verts[i0])).clone()), Box::new((*(verts[i0])).clone())
//             )));
//         }

//         self.compute_all_edge_collapse_costs();

//         let mut permutation = Vec::<usize>::new();
//         let mut map = Vec::<usize>::new();

//         let invalid = vertices.len() + 1000usize;

//         // reduce the object down to nothing:
//         while vertices.len() > 0 {
//             // get the next vertex to collapse
//             let ref mut vert = verts.as_mut()[self.minimum_cost_edge()];

            

//             {
//                  let mut collapse: Box<Vertex>;
//                 let id = vert.id;
        
//                 permutation[id] = vertices.len() - 1;

//                 //let mut vc : Box<Vertex>;
//                 // keep track of vertex to which we collapse to
//                 map[vertices.len() - 1] = match vert.collapse {
//                     Some(ref c) => {
//                          let id = c.id;
//                          collapse = Box::new((*c).clone());
//                          id
//                     },
//                     None => {
//                         collapse = Box::new(Vertex::new(0f32, 0f32, 0f32, 1000));
//                         invalid 
//                     },
//                 };

//                 if  (map[vertices.len() - 1] == invalid) {
//                     self.collapse(vert, &mut collapse);
//                 }
                
//             }
//         }

//         // reorder the map Array based on the collapse ordering
//         for i in 0..map.len() {
//             map[i] = if (map[i] == invalid) { 0 } else { permutation[map[i]]};
//         }
//     }

//    fn compute_all_edge_collapse_costs(&mut self) {
//         // For all the edges, compute the difference it would make
//         // to the model if it was collapsed.  The least of these
//         // per vertex is cached in each vertex object.
//         for i in 0..self.vertices.len() {
//             self.compute_edge_cost_at_vertex(i);
//         }
//     }

//     fn compute_edge_cost_at_vertex(&mut self, idx: usize) {
//         let mut verts = self.vertices.as_mut();
//         let ref mut v = verts[idx];
//         // compute the edge collapse cost for all edges that start
//         // from vertex v.  Since we are only interested in reducing
//         // the object by selecting the min cost edge at each step, we
//         // only cache the cost of the least cost edge at this vertex
//         // (in member variable collapse) as well as the value of the 
//         // cost (in member variable objdist).
//         if v.neighbors.len() == 0 {
//             // v doesn't have neighbors so it costs nothing to collapse
//             v.collapse = None;
//             v.objdist = -0.01f32;
//             return;
//         }

//         v.objdist = 1000000.0f32;
//         v.collapse = None;
//         // search all neighboring edges for "least cost" edge
//         for i in 0..v.neighbors.len() {    
//             let dist = 0.0f32; //compute_edge_collapse_cost(v, v.neighbors.get_mut(0[i]);
//             if dist < v.objdist {
//                  // candidate for edge collapse
//                 v.collapse = Some(Box::new((*(v.neighbors[i])).clone()));
//                 v.objdist= dist;             // cost of the collapse
//             }
//         }
//     }

//     fn compute_edge_collapse_cost(&mut self, u: &Box<Vertex>, v: &Box<Vertex>) -> f32 {
//         // if we collapse edge uv by moving u to v then how 
//         // much different will the model change, i.e. how much "error".
//         // Texture, vertex normal, and border vertex code was removed
//         // to keep this demo as simple as possible.
//         // The method of determining cost was designed in order 
//         // to exploit small and coplanar regions for
//         // effective polygon reduction.
//         // Is is possible to add some checks here to see if "folds"
//         // would be generated.  i.e. normal of a remaining face gets
//         // flipped.  I never seemed to run into this problem and
//         // therefore never added code to detect this case.
//         let v0_pos = (*u).position;
//         let v0 = Vector3::new(v0_pos[0], v0_pos[1], v0_pos[2]);
//         let v1_pos = (*u).position;
//         let v1 = Vector3::new(v1_pos[0], v1_pos[1], v1_pos[2]);
//         let v01 = v0.minus(&v1);
//         let edge_length = v01.length();

//         let mut curvature = 0.0f32;
//         let mut sides = Vec::<Box<Triangle>>::new();
//         for i in 0..u.faces.len() {
//             if u.faces[i].has_vertex(v) {
//                 sides.push(Box::new((*(u.faces[i])).clone()));
//             }
//         }
        
//         for i in 0..u.faces.len() {
//            let mut mincurv = 1.0f32; // curve for face i and closer side to it
//             for j in 0..sides.len() {  
//                 let dotprod : f32 = (*((*u).faces[i])).normal.dot(&(sides[j].normal));      // use dot product of face normals. 
//                 mincurv = mincurv.min((1.0f32-dotprod)/2.0f32);
//             }
//             curvature = curvature.max(mincurv);
//         }

//         edge_length * curvature
//     }

//     fn minimum_cost_edge(&self) -> usize {
//         // Find the edge that when collapsed will affect model the least.
//         // This funtion actually returns a Vertex, the second vertex
//         // of the edge (collapse candidate) is stored in the vertex data.
//         // Serious optimization opportunity here: this function currently
//         // does a sequential search through an unsorted Array :-(
//         // Our algorithm could be O(n*lg(n)) instead of O(n*n)
//         let ref mn = self.vertices[0];
//         let mut idx = 0usize;
//         for i in 0..self.vertices.len() {
//             if (self.vertices[i]).objdist < mn.objdist {
//                 idx = i;
//             }
//         }

//         idx
//     }

//     fn collapse(&mut self, ub: &mut Box<Vertex>, vb: &mut Box<Vertex>) {

//         // make tmp a Array of all the neighbors of u
//         let mut tmp = Vec::<Box<Vertex>>::new();
//         {
//             let ref u = **ub;
//             for i in 0..u.neighbors.len() {
//                 tmp.push(Box::new((*(u.neighbors[i])).clone()));
//             }
//         }

//         // delete triangles on edge uv:
//         {
//             let ref mut faces = ub.faces;
//             let mut i = faces.len() -1;
//             let mut finish = false;
//             while !finish {
//                 if faces[i].has_vertex(vb) {
//                     faces.remove(i);
                    
//                     if i > 0 {
//                         i = i - 1;
//                     } else {
//                         finish = true;
//                     }
//                 }
//             }
//         }

//         {
//             // update remaining triangles to have v instead of u
//             // TODO
//             let ubv = (**ub).clone();
//             let vbv = (**vb).clone();
//             let ref mut faces = ub.faces;
//             let mut i = faces.len() -1;
//             let mut finish = false;
//             while !finish {
//                 let ref mut face = faces.as_mut()[i];
//                 face.replace_vertex(&ubv, &vbv);
//                 if i > 0 {
//                     i = i - 1;
//                 } else {
//                     finish = true;
//                 }
//             }
//         }

//         {
//         	for i in 0..tmp.len() {
//         		self.compute_edge_cost_at_vertex(i);
//         	}
//         }

 
//         // // recompute the edge collapse costs for neighboring vertices
//         // for (unsigned int i = 0; i<tmp.size(); i++) {
//         //     ComputeEdgeCostAtVertex(tmp[i]);
//         // }
//     }
// }


// /// triangle of progressive mesh
// pub struct Triangle {
//   pub vertices: [Box<Vertex>; 3],
//   pub normal: Vector3,
// }

// impl Triangle {

//     pub fn new(v0: Box<Vertex>, v1: Box<Vertex>, v2: Box<Vertex>) -> Triangle {
//         let mut triangle = Triangle{
//             vertices: [v0, v1, v2],
//             normal: Vector3::new(0f32, 0f32, 0f32)
//         };
//         triangle.compute_normal();

//         triangle
//     }

//     pub fn clone(&self) -> Triangle  {

//         let mut triangle = Triangle {
//             vertices: [
//                 Box::new((*(self.vertices[0])).clone()), 
//                 Box::new((*(self.vertices[1])).clone()), 
//                 Box::new((*(self.vertices[2])).clone()),
//             ],
//             normal: self.normal.clone(),
//         };

//          triangle
//     }

//     /// calcuate normal 
//     pub fn compute_normal(&mut self) {
//         let p0 = self.vertices[0].position;
//         let p1 = self.vertices[1].position;
//         let p2 = self.vertices[2].position;

//         let v01 = Vector3::new(p1[0]-p0[0], p1[1]-p0[1], p1[2]-p0[2]);
//         let v12 = Vector3::new(p2[0]-p1[0], p2[1]-p1[1], p2[2]-p1[2]);

//         let normal = v01.cross(&v12);
//         if(normal.length() == 0f32) {
//             return;
//         }

//         self.normal = normal;
//     }

//     fn replace_vertex(&mut self, old: &Vertex, new: &Vertex) {
//         //let pos = self.vertices.position_elem(&old);
//         for i in 0..self.vertices.len() {
//             if *(self.vertices[i]) == *old {
//                 self.vertices[i] = Box::new(new.clone());
//                  return;
//             }
//         }
//     }

//     fn has_vertex(& self, v: &Box<Vertex>) ->bool {
//         let mut found = false;
//         for i in 0..self.vertices.len() {
//             if self.vertices[i] == *v {
//                 found = true;
//                 break;
//             }
//         }

//         found
//     }
// }

// /// vertex of progressive mesh
// pub struct Vertex {
//     pub position: [f32; 3],
//     pub id: usize,
//     pub neighbors: Vec<Box<Vertex>>,
//     pub faces: Vec<Box<Triangle>>,
//     pub objdist: f32,
//     pub collapse: Option<Box<Vertex>>,
// }

// impl Vertex {
//     fn new(x: f32, y: f32, z: f32, id: usize) -> Vertex {
//         Vertex {
//             position: [x, y, z],
//             id: id,
//             neighbors: vec![],
//             faces: vec![],
//             objdist: -100000f32,
//             collapse: None
//         }
//     }

//     pub fn clone(&self) -> Vertex  {

//         let mut collapse = match self.collapse {
//             Some(ref v) => {
//                 Some(Box::new((*v).clone()))
//             },
//             None => None
//         };
//         let mut vert = Vertex {
//             position: self.position.clone(),
//             id: self.id,
//             neighbors: vec![],
//             faces: vec![],
//             objdist: self.objdist,
//             collapse: collapse,
//         };


//         for i in 0..self.neighbors.len() {
//             let ref boxed = self.neighbors[i];
//             let ref neighbor = *boxed;
//             vert.neighbors.push(Box::new((*neighbor).clone()));
//         }

//         for i in 0..self.faces.len() {
//             let ref boxed = self.faces[i];
//             let ref face = *boxed;
//             vert.faces.push(Box::new((*face).clone()));
//         }

//         vert
//     } 

//     fn remove_if_no_neighbor(&mut self, n: &Box<Vertex>) {
//         if !(self.neighbors.contains(n)) {
//             return;
//         }

//         for i in 0..self.faces.len() {
//             if self.faces[i].has_vertex(n) {
//                 return;
//             }
//         }

//         for i in 0..self.neighbors.len() {
//              if self.neighbors[i] == *n {
//                 self.neighbors.remove(i);
//             }
//         }
//     }
// }

// impl PartialEq for Vertex {
//     fn eq(&self, v: &Vertex) -> bool { 
//         (self.position[0] - v.position[0]).abs()  <= std::f32::EPSILON && 
//         (self.position[1] - v.position[1]).abs()  <= std::f32::EPSILON && 
//         (self.position[2] - v.position[2]).abs()  <= std::f32::EPSILON && 
//         (self.id == v.id)
//     } 
// }

// impl Eq for Vertex {
// }

