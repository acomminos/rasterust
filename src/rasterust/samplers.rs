use rasterust::*;

// Given a pixel, provides subpixel sampling points.
pub trait Sampler {
    // Returns a list of weighted samples as (x, y, weight).
    fn sample(&self, x: u32, y: u32) -> Vec<(f32, f32, f32)>;
}

// where the field is the power of 2 to be raised to get the number of samples.
pub struct SimpleMultiSampler(pub u32);

impl Sampler for SimpleMultiSampler {
    fn sample(&self, x: u32, y: u32) -> Vec<(f32, f32, f32)> {
        let &SimpleMultiSampler(samples) = self;
        let dimen_samples: u32 = 2u32.pow(samples - 1); // sample uniformly across (x,y) (divide by 2)
        let interval: f32 = 1f32 / (dimen_samples as f32);
        // offset sampling points by half a tick.
        let x_offset: f32 = (x as f32) + (interval / 2f32);
        let y_offset: f32 = (y as f32) + (interval / 2f32);

        let mut points: Vec<(f32, f32, f32)> = vec![];
        for xp in 0..dimen_samples {
            for yp in 0..dimen_samples {
                points.push((x_offset + ((xp as f32) * interval),
                             y_offset + ((yp as f32) * interval),
                             1f32 / (samples as f32)));
            }
        }
        points
    }
}
