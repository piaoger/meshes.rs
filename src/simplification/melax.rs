
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




/*

THREE.SimplifyModifier = function() {

};

(function() {
    var cb = new THREE.Vector3(), ab = new THREE.Vector3();

    function pushIfUnique( array, object ) {

        if ( array.indexOf( object ) === -1 ) array.push( object );

    }

    function computeEdgeCollapseCost( u, v ) {

        // if we collapse edge uv by moving u to v then how
        // much different will the model change, i.e. the "error".

        var edgelength = v.position.distanceTo( u.position );

        var curvature = 0;


        var sideFaces = [];
        var i, uFaces = u.faces, il = u.faces.length, face, sideFace;

        // find the "sides" triangles that are on the edge uv
        for ( i = 0 ; i < il; i ++ ) {

            face = u.faces[ i ];

            if ( face.hasVertex(v) ) {

                sideFaces.push( face );

            }

        }

        // use the triangle facing most away from the sides
        // to determine our curvature term
        for ( i = 0 ; i < il; i ++ ) {

            var minCurvature = 1;
            face = u.faces[ i ];

            for( var j = 0; j < sideFaces.length; j ++ ) {

                sideFace = sideFaces[ j ];
                // use dot product of face normals.
                var dotProd = face.normal.dot( sideFace.normal );
                minCurvature = Math.min( minCurvature, ( 1 - dotProd ) / 2);
            }

            curvature = Math.max( curvature, minCurvature );
        }

        if (u.isBorder() && sideFaces.length > 1) {
            curvature = 1;
        }

        return edgelength * curvature;

    }

    function computeEdgeCostAtVertex( v ) {

        if ( v.neighbors.length === 0 ) {

            // collpase if no neighbors.
            v.collapse = null;
            v.cost = - 0.01;

            return;

        }

        v.cost = 1000000;
        v.collapse = null;

        // search all neighboring edges for â€œleast costâ€ edge
        for( var i = 0; i < v.neighbors.length; i ++ ) {

            var c;
            c = computeEdgeCollapseCost( v, v.neighbors[ i ] );

            if ( c < v.cost ) {

                v.collapse = v.neighbors[i];
                v.cost = c;

            }

        }

    }

    function removeVertex( v, vertices ) {

        console.assert( v.faces.length === 0 );

        while ( v.neighbors.length ) {

            var n = v.neighbors.pop();
            n.neighbors.splice( n.neighbors.indexOf( v ), 1 );

        }

        vertices.splice( vertices.indexOf( v ), 1 );

    }

    function removeFace( f, faces ) {

        faces.splice( faces.indexOf( f ), 1 );

        if ( f.v1 ) f.v1.faces.splice( f.v1.faces.indexOf( f ), 1 );
        if ( f.v2 ) f.v2.faces.splice( f.v2.faces.indexOf( f ), 1 );
        if ( f.v3 ) f.v3.faces.splice( f.v3.faces.indexOf( f ), 1 );

        var vs = [this.v1, this.v2, this.v3];
        var v1, v2;
        for(var i=0;i<3;i++) {
            v1 = vs[i];
            v2 = vs[(i+1)%3]
            if(!v1 || !v2) continue;
            v1.removeIfNonNeighbor(v2);
            v2.removeIfNonNeighbor(v1);
        }

    }

    function collapse( vertices, faces, u, v ) { // u and v are pointers to vertices of an edge

        // Collapse the edge uv by moving vertex u onto v

        if ( !v ) {

            // u is a vertex all by itself so just delete it..
            removeVertex( u, vertices );
            return;

        }

        var i;
        var tmpVertices = [];

        for( i = 0 ; i < u.neighbors.length; i ++ ) {

            tmpVertices.push( u.neighbors[ i ] );

        }


        // delete triangles on edge uv:
        for( i = u.faces.length - 1; i >= 0; i -- ) {

            if ( u.faces[ i ].hasVertex( v ) ) {

                removeFace( u.faces[ i ], faces );

            }

        }

        // update remaining triangles to have v instead of u
        for( i = u.faces.length -1 ; i >= 0; i -- ) {

            u.faces[i].replaceVertex( u, v );

        }


        removeVertex( u, vertices );

        // recompute the edge collapse costs in neighborhood
        for( i = 0; i < tmpVertices.length; i ++ ) {

            computeEdgeCostAtVertex( tmpVertices[ i ] );

        }

    }



    function minimumCostEdge( vertices ) {

        // O(n * n) approach. TODO optimize this

        var least = vertices[ 0 ];

        for (var i = 0; i < vertices.length; i ++ ) {

            if ( vertices[ i ].cost < least.cost ) {

                least = vertices[ i ];

            }
        }

        return least;

    }

    // we use a triangle class to represent structure of face slightly differently

    function Triangle( v1, v2, v3 ) {

        this.v1 = v1;
        this.v2 = v2;
        this.v3 = v3;

        this.normal = new THREE.Vector3();

        this.computeNormal();

        v1.faces.push( this );
        v1.addUniqueNeighbor( v2 );
        v1.addUniqueNeighbor( v3 );

        v2.faces.push( this );
        v2.addUniqueNeighbor( v1 );
        v2.addUniqueNeighbor( v3 );


        v3.faces.push( this );
        v3.addUniqueNeighbor( v1 );
        v3.addUniqueNeighbor( v2 );

    }

    Triangle.prototype.computeNormal = function() {

        var vA = this.v1.position;
        var vB = this.v2.position;
        var vC = this.v3.position;

        cb.subVectors( vC, vB );
        ab.subVectors( vA, vB );
        cb.cross( ab ).normalize();

        this.normal.copy( cb );

    };

    Triangle.prototype.hasVertex = function( v ) {

        return v === this.v1 || v === this.v2 || v === this.v3;

    };

    Triangle.prototype.replaceVertex = function( oldv, newv ) {

        if ( oldv === this.v1 ) this.v1 = newv;
        else if ( oldv === this.v2 ) this.v2 = newv;
        else if ( oldv === this.v3 ) this.v3 = newv;

        oldv.faces.splice( oldv.faces.indexOf( this ), 1 );

        newv.faces.push( this );


        oldv.removeIfNonNeighbor( this.v1 );
        this.v1.removeIfNonNeighbor( oldv );

        oldv.removeIfNonNeighbor( this.v2 );
        this.v2.removeIfNonNeighbor( oldv );

        oldv.removeIfNonNeighbor( this.v3 );
        this.v3.removeIfNonNeighbor( oldv );

        this.v1.addUniqueNeighbor( this.v2 );
        this.v1.addUniqueNeighbor( this.v3 );

        this.v2.addUniqueNeighbor( this.v1 );
        this.v2.addUniqueNeighbor( this.v3 );

        this.v3.addUniqueNeighbor( this.v1 );
        this.v3.addUniqueNeighbor( this.v2 );

        this.computeNormal();

    };

    function Vertex( v, id ) {

        this.position = v;

        this.id = id; // old index id

        this.faces = []; // faces vertex is connected
        this.neighbors = []; // neighbouring vertices

        // these will be computed in computeEdgeCostAtVertex()
        this.cost = 0; // cost of collapsing this vertex, the less the better
        this.collapse = null; // best candinate for collapsing

    }

    Vertex.prototype.addUniqueNeighbor = function( vertex ) {
        pushIfUnique(this.neighbors, vertex);
    }


    Vertex.prototype.removeIfNonNeighbor = function( n ) {

        var neighbors = this.neighbors;
        var faces = this.faces;

        var offset = neighbors.indexOf( n );
        if ( offset === -1 ) return;
        for ( var i = 0; i < faces.length; i ++ ) {

            if ( faces[ i ].hasVertex( n ) ) return;

        }

        neighbors.splice( offset, 1 );
    }

    Vertex.prototype.isBorder = function() {
        var len = this.neighbors.length;
        for (var i = 0; i < len; i++) {
            var count = 0;
            var face_len = this.faces.length;
            for (var j = 0;j < face_len; j++) {
                if (this.faces[j].hasVertex(this.neighbors[i])) {
                    count++;
                }
            }
            if (count === 1) return true;
        }
        return false;
    };


    THREE.SimplifyModifier.prototype.modify = function( geometry ) {

        var oldVertices = geometry.vertices;
        var oldFaces = geometry.faces;

        var newGeometry = new THREE.Geometry();

        var vertices = new Array( oldVertices.length );
        var faces = new Array( oldFaces.length );

        var i, il, face;

        //
        // put data of original geometry in different data structures
        //

        // add vertices
        for ( i = 0, il = oldVertices.length; i < il; i ++ ) {

            vertices[ i ] = new Vertex( oldVertices[ i ], i );

        }

        // add faces
        for ( i = 0, il = oldFaces.length; i < il; i ++ ) {

            face = oldFaces[ i ];
            faces[ i ] = new Triangle( vertices[ face.a ], vertices[ face.b ], vertices[ face.c ] );

        }

        // compute all edge collapse costs
        for ( i = 0, il = vertices.length; i < il; i ++ ) {

            computeEdgeCostAtVertex( vertices[ i ] );

        }

        var permutation = new Array( vertices.length );
        var map = new Array( vertices.length );

        var nextVertex;

        // reduce the object down to nothing:
        var z = 0;
        z = vertices.length * 0.25 | 0;
        z = 300;

        // while( z-- ) {
        // nextVertex = minimumCostEdge( vertices );
        // collapse( vertices, faces, nextVertex, nextVertex.collapse );
        // }

        while( vertices.length > 0 ) {

            // get the next vertex to collapse
            nextVertex = minimumCostEdge( vertices );

            // keep track of this vertex, i.e. the collapse ordering
            permutation[ nextVertex.id ] = vertices.length - 1;

            // keep track of vertex to which we collapse to
            map[ vertices.length - 1 ] = nextVertex.collapse ? nextVertex.collapse.id : -1;

            // console.log('b4 vertices', vertices.length, 'faces', faces.length);
            // console.log( nextVertex.id, '>', nextVertex.collapse.id)

            // Collapse this edge (nextVertex will go into nextVertex.collapse)
            collapse( vertices, faces, nextVertex, nextVertex.collapse );

            // console.log('after vertices', vertices.length, 'faces', faces.length);
            // console.log('.', map);
            // console.log('*', permutation);
            // permutation [7, 6, 5, 2, 1, 4, 3, 0, 11, 10, 9, 8]
        }


        var sortedVertices = new Array(vertices.length);

        for (i = 0; i < map.length; i ++ ) {

            map[i] = ( map[i] === - 1 ) ? 0 : permutation[ map[ i ] ];

            sortedVertices[ permutation[ i ] ] = oldVertices[ i ];

        }

        // console.log('after vertices', vertices.length, 'faces', faces.length);

        // console.log('map', map);
        // console.log('permutation', permutation);

        var sortedGeometry = new THREE.Geometry();

        for (i=0; i < sortedVertices.length; i++) {

            sortedGeometry.vertices.push( sortedVertices[ i ] );

        }


        for (i = 0; i < oldFaces.length; i++) {

            face = oldFaces[ i ];

            var a = permutation[ face.a ];
            var b = permutation[ face.b ];
            var c = permutation[ face.c ];

            sortedGeometry.faces.push( new THREE.Face3( a, b, c ) );


        }

        // geometry.vertices = [];
        // geometry.faces = [];

        geometry.vertices = sortedGeometry.vertices.concat();
        sortedGeometry.map = map;
        sortedGeometry.permutation = permutation;

        return sortedGeometry;


    };
})()
*/
