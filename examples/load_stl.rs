
extern crate meshes;

use meshes::io as meshio;

fn my_test_load_stl() 
{
    let result = meshio::stl::load("./assets/models/stl/hello.stl", true); 
    match result {
        Ok(mesh) => {
            meshio::stl::save(&mesh, "rsult.stl");
            meshio::obj::save(&mesh, "result.obj");
            meshio::ply::save(&mesh, "result.ply");
        },
        _=>{}
   }
}

fn main() {
    my_test_load_stl();
}
