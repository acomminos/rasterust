mod rasterust;

use rasterust::*;

fn main() {
    let target = RenderTarget::create(1024, 768);
    let tri = Triangle::new(Vector::new(1., -1., 0.),
                            Vector::new(1., 1., 0.),
                            Vector::new(-1., -1., 0.));
}
