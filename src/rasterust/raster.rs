use rasterust::*;

// Rasterizes the given triangle onto the RenderTarget.
// The points of the given triangle assume that the front face is counter-clockwise.
pub fn rasterize_barycentric_ccw(tri: &Triangle, target: &mut RenderTarget, camera: &Camera) {
    match camera.project_triangle(tri) {
        Triangle(a, b, c) => {
            let ab = b.sub(&a);
            let bc = c.sub(&b);
            let ca = a.sub(&c);
            let area = ((ab.x() * bc.y()) - (ab.y() * bc.x()))/2.;
            for y in 0..target.height {
                for x in 0..target.width {
                    let (px, py) = (x as f32, y as f32);
                    // fetch barycentric triple
                    let (w0, w1, w2) = (((bc.x() * (py - b.y())) - (bc.y() * (px - b.x()))) / area,
                                        ((ca.x() * (py - c.y())) - (ca.y() * (px - c.x()))) / area,
                                        ((ab.x() * (py - a.y())) - (ab.y() * (px - a.x()))) / area);
                    if w0 > 0. && w1 > 0. && w2 > 0. {
                        // point is in the left half-space of all 3 vectors, thus interior
                        // TODO(acomminos): depth testing
                        //shade(target, (w0, w1, w2))
                    }
                }
            }
        }
    }
}

fn render_to_scene(scene: &Scene, target: &RenderTarget) {
    let camera = &scene.camera;
    for m in &scene.models {
        let mat = &m.get_transform();
        let mesh = &m.mesh;
        //let x = -v.x/v.z;
        //let y = v.y/v.z;
    }
}
