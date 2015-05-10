
//! AMF Model Writer

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

    let amf_unit = "millimeter"; // inch
    let amf_version = "1.1";
    let object_id = "1";


    write!(&mut writer, "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n").unwrap();
    write!(&mut writer, "<amf unit='{}' version='{}'>\n", amf_unit, amf_version).unwrap();
    write!(&mut writer,"  <object id='{}'>\n", object_id).unwrap();
    write!(&mut writer, "    <mesh>\n").unwrap();

    write!(&mut writer, "        <vertices>\n").unwrap();
    for i in 0 ..mesh.vertices.len() / 3  {
        write!(&mut writer, "            <vertex>\n").unwrap();
        write!(&mut writer, "                <coordinates>\n").unwrap();
        write!(&mut writer, "                    <x>{}</x> <y>{}</y> <z>{}</z>\n", 
        											mesh.vertices[3*i], mesh.vertices[3*i + 1], mesh.vertices[3*i + 2]).unwrap();
        write!(&mut writer, "                </coordinates>\n").unwrap();
        write!(&mut writer, "            </vertex>\n").unwrap();
    }
    write!(&mut writer, "\n        </vertices>\n").unwrap();

    write!(&mut writer, "        <volume>\n").unwrap();
    for i in 0 ..mesh.faces.len() / 3  {
        let i0 = mesh.faces[3*i] as usize;
        let i1 = mesh.faces[3*i+1] as usize;
        let i2 = mesh.faces[3*i+2] as usize;
        write!(&mut writer, "            <triangle>\n");
        write!(&mut writer, "                <v1>{}</v1> <v2>{}</v2> <v3>{}</v3>\n", i0, i1,i2).unwrap();
        write!(&mut writer, "            </triangle>\n").unwrap();
    }
    write!(&mut writer, "        </volume>\n").unwrap();

    write!(&mut writer, "    </mesh>\n").unwrap();
    write!(&mut writer, "  </object>\n").unwrap();
    write!(&mut writer, "</amf>\n").unwrap();
}
