
//! STL Model Writer
use std::error::Error; 

use std::io::{ BufReader, BufWriter};

use std::fs::File;
use std::path::Path;

use std::str::FromStr;
use std::fmt::{self, Formatter};

use std::io::prelude::*;

use mesh::*;

pub fn save(mesh: &Mesh, name: &str) {
    let path = Path::new(name);
    let mut file =  match File::create(path) {
        // The `desc` field of `IoError` is a string that describes the error
        Err(why) => {
            let display = path.display();
            panic!("couldn't create {}: {}", display, Error::description(&why));
            //None
        },
        Ok(file) => file,
    };

    let mut writer = BufWriter::new(file);

    // solid
    //   facet normal 0 0 -1
    //     outer loop
    //       vertex -14.6722 3.11867 0
    //       vertex -13.7032 6.10105 0
    //       vertex -15 0 0
    //     endloop
    //   endfacet
    //   ...
    // endsolid
    write!(&mut writer, "solid stl.rs\n");
 
    for i in 0 ..mesh.faces.len() / 3  {
        let norm = [0f32, 0f32, 1f32];
        let i0 = 3*mesh.faces[3*i] as usize;
        let i1 = 3*mesh.faces[3*i+1] as usize;
        let i2 = 3*mesh.faces[3*i+2] as usize;

        write!(&mut writer, "    facet normal {} {} {}\n", norm[0], norm[1], norm[2]);
        write!(&mut writer, "        outer loop\n");
        write!(&mut writer, "            vertex {} {} {}\n", 
            mesh.vertices[i0], mesh.vertices[i0+1], mesh.vertices[i0+2]);
        write!(&mut writer, "            vertex {} {} {}\n", 
            mesh.vertices[i1], mesh.vertices[i1+1], mesh.vertices[i1+2]);
        write!(&mut writer, "            vertex {} {} {}\n", 
            mesh.vertices[i2], mesh.vertices[i2+1], mesh.vertices[i2+2]);

        write!(&mut writer, "        endloop\n");
        write!(&mut writer, "    endfacet\n");
    }

    write!(&mut writer, "endsolid stl.rs");
    writer.flush();
}