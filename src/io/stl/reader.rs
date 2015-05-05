
//! STL Model Reader
use std::fs::File;
use std::path::Path;
use std::error::Error; 

use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom; 

use std::str::FromStr; 
// HashMap or BTreeMap ?
use std::collections::HashMap; 
//use std::collections::BTreeMap;


// internal mods
use utils::*;
use mesh::Mesh;
use vertex::Vertex;
use io::*;

enum StlFileType {
    Ascii = 0,
    Binary = 1
}


fn process_raw_vertex(v2i_map: &mut HashMap<Vertex,u32>, vertex : & Vec<f32>) ->u32 {

    let scale = 10000.0;
    let vertex = Vertex { pos: [scale*vertex[0], scale*vertex[1], scale*vertex[2]]};
    let index = match v2i_map.get(&vertex) {
        None => {
            let index = v2i_map.len() as u32;
            v2i_map.insert(vertex,index);
            index
        },
        Some(&index) => {
            index
        }
    };

    index
}


fn parse_vertex(vertex : &mut Vec<f32>, n0: Option<&str>, n1: Option<&str>, n2: Option<&str>) {

    let (n0, n1, n2) = match (n0, n1, n2) {
        (Some(n0), Some(n1), Some(n2)) => (n0, n1, n2),
        _ => {
                //panic!("could not parse line {} {} {}", n0, n1, n2);
                panic!("could not parse vertex");
        }
    };
    let v = match (FromStr::from_str(n0), FromStr::from_str(n1), FromStr::from_str(n2)) {
        (Ok(n0), Ok(n1), Ok(n2)) => (n0, n1, n2),
        _ => {
            panic!("could not parse vertex");
        }
    };

    vec_push_all(vertex, &[v.0, v.1, v.2]);
}

fn parse_normal(normals : &mut Vec<f32>, n0: Option<&str>, n1: Option<&str>, n2: Option<&str>) {

    let (n0, n1, n2) = match (n0, n1, n2) {
        (Some(n0), Some(n1), Some(n2)) => (n0, n1, n2),
        _ => {
                //print!("could not parse line {} {} {}", n0, n1, n2);
                panic!("could not parse normal");
            }
    };

    let normal = match (FromStr::from_str(n0), FromStr::from_str(n1), FromStr::from_str(n2)) {
        (Ok(n0), Ok(n1), Ok(n2)) => (n0, n1, n2),
        _ => {
            panic!("could not parse normal");
        }
    };

    vec_push_all(normals, &[normal.0, normal.1, normal.2]);
}



fn guess_filetype<B: BufRead + Seek>(reader: &mut B) -> Result<StlFileType, LoadError> {
    let size = reader.seek(SeekFrom::End(0)).unwrap();
    reader.seek( SeekFrom::Start(0)).unwrap();;

    let mut is_binary = true;
    if size < 84 {
        is_binary = false;
    }

    let mut header = [0;80];
    reader.read(&mut header).unwrap();;

    if is_binary {
        let ntris;
        {
            let mut ntris_bytes = [0;4];
            reader.read(&mut ntris_bytes).unwrap();;
            ntris = get4byte(&mut ntris_bytes) as u64;
        } 

        // binary stl file size
        // 1) 80 byte header
        // 2) 4 byte face count
        // 3) 50 bytes per face
        if (ntris * 50 + 84) != size  {
            is_binary = false;
        }
    }

    reader.seek(SeekFrom::Start(0)).unwrap();;

    let result = if is_binary {
        Ok(StlFileType::Binary)
    } else {

        let mut start = 0usize;
        for i in 0..80 {
            if header[i] == b's' {
                start = i;
                break;
            }
        }

        if header[start..5+start] == b"solid"[..] {
            Ok(StlFileType::Ascii)
        } else {
            Err(LoadError::ReadErr)
        }
    };

    result
}

fn read_ascii_stl<B: BufRead>(reader: &mut B) -> IoResult<Mesh> {

    let mut verts : Vec<f32>= vec![];
    let mut normals : Vec<f32>= vec![];
    let mut faces = vec![];

    let mut vi_map :HashMap<Vertex,u32>= HashMap::new();

    let mut cur_tri : Vec<u32> = Vec::with_capacity(3);
    let mut cur_vt = -1;

    for (_, line) in reader.lines().enumerate() {
        let mut words = match line {
            Ok(ref line) => {
                // Split a string on whitespace, don't include empty strings
                // Some STL files contains 1+ spaces between keywords or values
                let ws = line[..]
                         .trim()
                         .split(|c: char| -> bool {
                             c == ' '
                         })
                         .filter(|s: &&str| -> bool {
                             *s != ""
                         });

                ws
            },
            Err(e) => {
                println!("Failed to read line due to {}", e);
                return Err(LoadError::ReadErr);
            },
        };
        match words.next() {
            Some("solid") => {
                continue;
            },

            Some("facet") => {
                match words.next() {
                    Some("normal") => {
                            let (n0, n1, n2) = (words.next(), words.next(), words.next());
                            parse_normal(&mut normals, n0, n1, n2);
                        },
                    None => { continue; },
                    Some(_) => { return Err(LoadError::UnrecognizedCharacter) },
                }
            },

            Some("outer") => {

            },
            Some("vertex") => {
                    cur_vt += 1;
                    let (v0, v1, v2) = (words.next(), words.next(), words.next());
                    let mut xyz : Vec<f32> = Vec::with_capacity(3);
                    parse_vertex(&mut xyz, v0, v1, v2);

                    let len = vi_map.len() as u32;
                    let index = process_raw_vertex(&mut vi_map, &xyz);
                    if index == len {
                        vec_push_all(&mut verts, &xyz);
                    }

                    cur_tri.push(index);
                },
            Some("endloop") => {

                if cur_tri.len() != 3 {
                    panic!("Facet {} is invalid for vertex count is {}.", cur_vt, cur_tri.len() / 3);
                }

                vec_push_all(&mut faces, &cur_tri);

                cur_vt = -1;
                cur_tri.clear();
            },

            Some("endfacet") => {

            },
            Some("endsolid") => { 
                continue; 
            },

            None => { continue; },
            Some(m) => { 
                println!("Skipping empty line");
                println!("First Unrecognized character {}", m); 
                return Err(LoadError::UnrecognizedCharacter) 
            },
        }

    }


    let mesh = Mesh::new(verts, None, None, faces);
    Ok(mesh)
}


fn read_binary_stl<B: BufRead + Seek>(reader: &mut B) -> IoResult<Mesh> {

    let mut header =   [0;80];
    reader.read(&mut header).unwrap();;

    // TODO: add binary/ascii check here 
    if header[0..5] == b"solid"[..] {
        print!("is solid");
    } else {
        print!("!solid");
    }

    let mut ntris;
    {
        let mut ntris_bytes = [0;4];
        reader.read(&mut ntris_bytes).unwrap();;
        ntris = get4byte(&mut ntris_bytes);
    }

    let mut verts : Vec<f32>= vec![];
    //let mut normals : Vec<f32>= vec![];
    let mut faces = vec![];
    let mut vi_map :HashMap<Vertex,u32>= HashMap::new();

    let mut cur_tri : Vec<u32> = Vec::with_capacity(3);
    for _ in 0..ntris {

        let _ = read_float32(reader);
        let _ = read_float32(reader);
        let _ = read_float32(reader);

        for _ in 0..3 {
            let x = read_float32(reader);
            let y = read_float32(reader);
            let z = read_float32(reader); 

            let len = vi_map.len() as u32;
            let xyz : Vec<f32> = vec![x, y, z];
            let index = process_raw_vertex(&mut vi_map, &xyz);
            if index == len {
                vec_push_all(&mut verts, &xyz);
            }

            cur_tri.push(index);
        }

        vec_push_all(&mut faces, &cur_tri);
        cur_tri.clear();
 
        // Some vendors use it for color
        read_u16(reader);
    }

    let mesh = Mesh::new(verts, None, None, faces);

    Ok(mesh)
}

/// Load stl file
pub fn load(name: &str) -> IoResult<Mesh> {
    // Create a path to the desired file
    let path = Path::new(name);
    let display = path.display();

    // Open the path in read-only mode, returns `IoResult<File>`
    let file = match File::open(path) {
        // The `desc` field of `IoError` is a string that describes the error
        Err(why) => {
            println!("couldn't open {}: {}", display, Error::description(&why));
            return Err(LoadError::OpenFileErr);
        },
        Ok(file) => file,
    };

    let mut reader = BufReader::new(file);

    let mesh = match guess_filetype(&mut reader) {
        Ok(StlFileType::Ascii) => {
            read_ascii_stl(&mut reader)
        },
        Ok(StlFileType::Binary) => {
            read_binary_stl(&mut reader)
        }
        Err(_) => {
            Err(LoadError::ReadErr)
        }
    };

    mesh
}
