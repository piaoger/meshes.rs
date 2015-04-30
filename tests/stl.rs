
extern crate meshes;

use meshes::io as meshio;

#[test]
fn test_load_stl() 
{
    meshio::stl::load("./assets/models/stl/hello.stl", true).is_ok();
}