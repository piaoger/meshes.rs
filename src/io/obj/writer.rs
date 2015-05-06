
//! OBJ Model Writer

use std::error::Error; 

use std::io::BufWriter;
use std::io::prelude::*;

use std::fs::File;
use std::path::Path;

// internal
use mesh::*;


pub fn save(mesh: &Mesh, name: &str) {
    let path = Path::new(name);
    let file =  match File::create(path) {
        // The `desc` field of `IoError` is a string that describes the error
        Err(why) => {
            panic!("couldn't create {}: {}", path.display(), Error::description(&why));
        },
        Ok(file) => file,
    };

    let mut writer = BufWriter::new(file);

    // v -10.037000 11.147200 0.000000
    // v -7.500000 12.990400 0.000000
    // v -15.000000 0.000000 0.000000
    // f 42 43 3
    write!(&mut writer, "#solid solidmcp\n").unwrap();
 
    for i in 0 ..mesh.vertices.len() / 3  {
        write!(&mut writer, "v {} {} {}\n", mesh.vertices[3*i], mesh.vertices[3*i+1], mesh.vertices[3*i+2]).unwrap();
    }

    for i in 0 ..mesh.faces.len() / 3  {
        let i0 = mesh.faces[3*i] as usize;
        let i1 = mesh.faces[3*i+1] as usize;
        let i2 = mesh.faces[3*i+2] as usize;

        // index of obj starts from 1
        write!(&mut writer, "f {} {} {}\n", i0+1, i1+1, i2+1).unwrap();
    }

    write!(&mut writer, "#endsolid solidmcp").unwrap();

    writer.flush().unwrap();
}