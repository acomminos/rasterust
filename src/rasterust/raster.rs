use rasterust::*;
use rasterust::shader;
use rasterust::samplers;

// Rasterizes the given triangle onto the RenderTarget.
// The given triangle is expected to be wound counter-clockwise and be in
// normalized device coordinates.
pub fn rasterize_barycentric_ccw<T, U>(tri: &Triangle, target: &mut RenderTarget, camera: &Camera, sampler: &U, shader: &T) where T: shader::Shader, U: samplers::Sampler {
    let &Triangle(ref a, ref b, ref c) = tri;
    let (ab, bc, ca) = (b.sub(&a), c.sub(&b), a.sub(&c));
    let area: f32 = (ab.x() * bc.y()) - (ab.y() * bc.x());
    let sw = ((target.width as f32) - 1.)/2.;
    let sh = ((target.height as f32) - 1.)/2.;
    println!["ab: ({}, {}), bc: ({}, {}), ca: ({}, {})", ab.x(), ab.y(), bc.x(), bc.y(), ca.x(), ca.y()];
    // TODO(acomminos): only rasterize within triangle bounds
    for y in 0..target.height {
        for x in 0..target.width {
            let mut color = Color::zero();
            let mut depth = 0.;
            let mut depth_entries = 0;
            for (sx, sy, weight) in sampler.sample(x as u32, y as u32) {
                //println!["sx: {}, sy: {}, weight: {}", sx, sy, weight];
                let px = (sx - sw)/sw;
                let py = -(sy - sh)/sh;
                // fetch barycentric triple
                let (w0, w1, w2) = (((bc.x() * (py - b.y())) - (bc.y() * (px - b.x()))) / area,
                ((ca.x() * (py - c.y())) - (ca.y() * (px - c.x()))) / area,
                ((ab.x() * (py - a.y())) - (ab.y() * (px - a.x()))) / area);
                if w0 >= 0. && w1 >= 0. && w2 >= 0. {
                    // point is in the left half-space of all 3 vectors, thus interior
                    // interpolate z using barycentric parameters
                    let pz = (a.z() * w0) + (b.z() * w1) + (c.z() * w2);
                    if camera.contains_point((px, py, pz)) {
                        let shader_color = shader.shade((w0, w1, w2));
                        color = color + shader_color.multiply(weight);
                        // FIXME: don't check depth for each multisample! things fade out.
                        depth += pz;
                        depth_entries += 1;
                    }
                }
            }
            if depth_entries > 0 && target.check_depth((x, y), depth / (depth_entries as f32)) {
                target.paint((x, y), &color, CompositeMode::SourceOver);
            }
        }
    }
}
