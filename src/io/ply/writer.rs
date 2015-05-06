
//! PLY Model Writer

use std::error::Error; 

use std::io::BufWriter;
use std::io::prelude::*;

use std::fs::File;
use std::path::Path;

use mesh::*;


pub fn save(mesh: &Mesh, name: &str) {
    let path = Path::new(name);
    let file =  match File::create(path) {
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
    write!(&mut writer, "ply\n").unwrap();
    write!(&mut writer, "format ascii 1.0\n").unwrap();
    write!(&mut writer, "comment solidmcp generated\n").unwrap();
    write!(&mut writer, "element vertex {} \n",  mesh.vertices.len()/3).unwrap();
    write!(&mut writer, "property float x\n").unwrap();
    write!(&mut writer, "property float y\n").unwrap();
    write!(&mut writer, "property float z\n").unwrap();

    match mesh.normals {
        Some(_) => {
            write!(&mut writer, "property float nx\n").unwrap();
            write!(&mut writer, "property float ny\n").unwrap();
            write!(&mut writer, "property float nz\n").unwrap();   
        },
        None => {}
    }

    match mesh.texcoords{
        Some(_) => {
            write!(&mut writer, "property uchar red\n").unwrap();
            write!(&mut writer, "property uchar green\n").unwrap();
            write!(&mut writer, "property uchar blue\n").unwrap();
            write!(&mut writer, "property uchar alpha\n").unwrap();
        },
        None => {}
    }

    write!(&mut writer, "element face {}\n",  mesh.faces.len()/3).unwrap();
    write!(&mut writer, "property list uchar int vertex_indices\n").unwrap();
    write!(&mut writer, "end_header\n").unwrap();

    for i in 0 ..mesh.vertices.len() / 3  {
        write!(&mut writer, "{} {} {}\n", mesh.vertices[3*i], mesh.vertices[3*i+1], mesh.vertices[3*i+2]).unwrap();
    }

    for i in 0 ..mesh.faces.len() / 3  {
        let i0 = mesh.faces[3*i] as usize;
        let i1 = mesh.faces[3*i+1] as usize;
        let i2 = mesh.faces[3*i+2] as usize;

        write!(&mut writer, "3 {} {} {}\n", i0, i1, i2).unwrap();
    }


    writer.flush().unwrap();;
}