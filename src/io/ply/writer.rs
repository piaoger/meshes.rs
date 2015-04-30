
//! PLY Model Writer

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

    // write header
    write!(&mut writer, "ply\n");
    write!(&mut writer, "format ascii 1.0\n");
    write!(&mut writer, "comment solidmcp generated\n");
    write!(&mut writer, "element vertex {} \n",  mesh.vertices.len()/3);
    write!(&mut writer, "property float x\n");
    write!(&mut writer, "property float y\n");
    write!(&mut writer, "property float z\n");

    match mesh.normals {
        Some(_) => {
            write!(&mut writer, "property float nx\n");
            write!(&mut writer, "property float ny\n");
            write!(&mut writer, "property float nz\n");   
        },
        None => {}
    }

    match mesh.texcoords{
        Some(_) => {
            write!(&mut writer, "property uchar red\n");
            write!(&mut writer, "property uchar green\n");
            write!(&mut writer, "property uchar blue\n");
            write!(&mut writer, "property uchar alpha\n");
        },
        None => {}
    }

    write!(&mut writer, "element face {}\n",  mesh.faces.len()/3);
    write!(&mut writer, "property list uchar int vertex_indices\n");
    write!(&mut writer, "end_header\n");


    for i in 0 ..mesh.vertices.len() / 3  {
        write!(&mut writer, "{} {} {}\n", mesh.vertices[3*i], mesh.vertices[3*i+1], mesh.vertices[3*i+2]);
    }

    for i in 0 ..mesh.faces.len() / 3  {
        let i0 = mesh.faces[3*i] as usize;
        let i1 = mesh.faces[3*i+1] as usize;
        let i2 = mesh.faces[3*i+2] as usize;

        write!(&mut writer, "3 {} {} {}\n", i0, i1, i2);
    }


    writer.flush();
}