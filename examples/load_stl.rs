
extern crate meshes;

use meshes::io as meshio;

fn my_test_load_stl() 
{
    //let result = meshio::stl::load("/Users/piaoger/storage/Dropbox/assets/stl/dragon-binary.stl"); 
    let result = meshio::stl::load("./assets/models/stl/hello.stl"); 
    match result {
        Ok(mesh) => {
            meshio::stl::save(&mesh, "result.stl");
            meshio::obj::save(&mesh, "result.obj");
            meshio::ply::save(&mesh, "result.ply");
        },
        _ => {}
   }
}

fn main() {
    my_test_load_stl();
}
