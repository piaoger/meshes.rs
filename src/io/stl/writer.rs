
//! STL Model Writer
use std::error::Error;

use std::io::BufWriter;
use std::io::prelude::*;

use std::fs::File;
use std::path::Path;

use mesh::*;
use utils::*;

/// Save Mesh to STL File
pub fn save(mesh: &Mesh, name: &str) {
    save_ascii(mesh, name);
}

#[allow(dead_code)]
fn save_ascii(mesh: &Mesh, name: &str) {
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
    write!(&mut writer, "solid stl.rs\n").unwrap();

    for i in 0 ..mesh.faces.len() / 3  {
        let norm = [0f32, 0f32, 1f32];
        let i0 = 3*mesh.faces[3*i] as usize;
        let i1 = 3*mesh.faces[3*i+1] as usize;
        let i2 = 3*mesh.faces[3*i+2] as usize;

        write!(&mut writer, "    facet normal {} {} {}\n", norm[0], norm[1], norm[2]).unwrap();
        write!(&mut writer, "        outer loop\n").unwrap();
        write!(&mut writer, "            vertex {} {} {}\n",
            mesh.vertices[i0], mesh.vertices[i0+1], mesh.vertices[i0+2]).unwrap();
        write!(&mut writer, "            vertex {} {} {}\n",
            mesh.vertices[i1], mesh.vertices[i1+1], mesh.vertices[i1+2]).unwrap();
        write!(&mut writer, "            vertex {} {} {}\n",
            mesh.vertices[i2], mesh.vertices[i2+1], mesh.vertices[i2+2]).unwrap();

        write!(&mut writer, "        endloop\n").unwrap();
        write!(&mut writer, "    endfacet\n").unwrap();
    }

    write!(&mut writer, "endsolid stl.rs").unwrap();
    writer.flush().unwrap();
}

#[allow(dead_code)]
fn save_binary(mesh: &Mesh, name: &str) {
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

    let header : [u8; 80]= [b's'; 80];
    writer.write(&header).unwrap();

    write_u32(&mut writer, (mesh.faces.len()/3) as u32);

    for i in 0..mesh.faces.len()/3 {

        write_float32(&mut writer, 1.0);
        write_float32(&mut writer, 0.0);
        write_float32(&mut writer, 0.0);

        for j in 0..3 {

            let idx = 3*mesh.faces[3*i + j] as usize;

            write_float32(&mut writer, mesh.vertices[idx]);
            write_float32(&mut writer, mesh.vertices[idx+1]);
            write_float32(&mut writer, mesh.vertices[idx+2]);
        }

        // Some vendors use it for color
        write_u16(&mut writer, 0x0000);
    }

    writer.flush().unwrap();
}