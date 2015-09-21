mod rasterust;

use rasterust::*;

fn main() {
    let mut target = RenderTarget::new(20, 15);
    let camera = Camera::new(Vector::zero(), Vector::zero(), 1., 45., 0., 1.);
    let mut scene = Scene::new(camera);

    let tri2 = Triangle::new(Vector::new(0., 0., 0.1),
                             Vector::new(1., 1., 0.1),
                             Vector::new(0., 1., 0.1));
    let tri1 = Triangle::new(Vector::new(1., 1., 0.1),
                             Vector::new(1., -1., 0.1),
                             Vector::new(-1., -1., 0.1));
    let mesh = Mesh::new(vec![tri1, tri2]);
    let model = Model::new(mesh);
    scene.add_model(model);
    scene.render(&mut target);
    target.print_ascii();
}
