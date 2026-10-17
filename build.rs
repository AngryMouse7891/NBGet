extern crate embed_resource;

fn main(){
    embed_resource::compile("data/nbget.rc", embed_resource::NONE).manifest_optional().unwrap();
}