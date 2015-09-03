use rasterust::*;

fn render_to_scene(scene: &Scene, target: &RenderTarget) {
    let camera = &scene.camera;
    for m in &scene.models {
        let mat = &m.get_transform();
        let mesh = &m.mesh;
        //let x = -v.x/v.z;
        //let y = v.y/v.z;
    }
}
