use rasterust::*;

// Rasterizes the given triangle onto the RenderTarget.
// The points of the given triangle assume that the front face is counter-clockwise.
pub fn rasterize_barycentric_ccw<T>(tri: &Triangle, target: &mut RenderTarget, camera: &Camera, shader: &T) where T: Shader {
    match camera.project_triangle(tri) {
        Triangle(a, b, c) => {
            let ab = b.sub(&a);
            let bc = c.sub(&b);
            let ca = a.sub(&c);
            let area: f32 = ((ab.x() * bc.y()) - (ab.y() * bc.x()))/2.;
            println!["ab: ({}, {}), bc: ({}, {}), ca: ({}, {})", ab.x(), ab.y(), bc.x(), bc.y(), ca.x(), ca.y()];
            for y in 0..target.height {
                for x in 0..target.width {
                    let sw = ((target.width as f32) - 1.)/2.;
                    let sh = ((target.height as f32) - 1.)/2.;
                    let px = ((x as f32) - sw)/sw;
                    let py = -((y as f32) - sh)/sh;
                    // TODO(acomminos): only rasterize within triangle bounds
                    // fetch barycentric triple
                    let (w0, w1, w2) = (((bc.x() * (py - b.y())) - (bc.y() * (px - b.x()))) / area,
                                        ((ca.x() * (py - c.y())) - (ca.y() * (px - c.x()))) / area,
                                        ((ab.x() * (py - a.y())) - (ab.y() * (px - a.x()))) / area);
                    println!["pixel hit: ({}, {} [NDC: {}, {}]), area: {}, coords: ({}, {}, {})", x, y, px, py, area, w0, w1, w2];
                    if w0 >= 0. && w1 >= 0. && w2 >= 0. {
                        // point is in the left half-space of all 3 vectors, thus interior
                        // TODO(acomminos): depth testing
                        target.paint((x, y), shader.shade((w0, w1, w2)).to_argb32());
                    }
                }
            }
        }
    }
}
