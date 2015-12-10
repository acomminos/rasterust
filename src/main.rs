mod rasterust;

use rasterust::*;
use std::f32;

fn main() {
    let mut target = RenderTarget::new(40, 20);
    let camera = Camera::new(Vector::zero(), Vector::zero(), target.aspect(), f32::consts::PI/2., 0., 20.);
    let mut scene = Scene::new(camera);

    let tri2 = Triangle::new(Vector::new(-10., 0., 10.),
                             Vector::new(700., 500., 10.),
                             Vector::new(0., 500., 5.));
    let tri1 = Triangle::new(Vector::new(1., 1., 2.),
                             Vector::new(1., -1., 2.),
                             Vector::new(-1., -1., 2.));
    let mesh = Mesh::new(vec![tri1, tri2]);
    let model = Model::new(mesh);
    scene.add_model(model);
    scene.render(&mut target);
    target.print_ascii();
}
