use rasterust::*;

pub trait Shader {
    fn shade(&self, bary: (f32, f32, f32)) -> Color;
}

// A basic color shader, outputting a constant colour.
pub struct SolidColorShader(pub Color);

impl Shader for SolidColorShader {
    fn shade(&self, bary: (f32, f32, f32)) -> Color {
        match self {
            &SolidColorShader(ref color) => color.clone()
        }
    }
}

pub struct TriColorShader(Color, Color, Color);

impl Shader for TriColorShader {
    fn shade(&self, (wa, wb, wc): (f32, f32, f32)) -> Color {
        let &TriColorShader(ref a, ref b, ref c): &TriColorShader = self;
        Color {
            r: a.r * wa + b.r * wb + c.r * wc,
            g: a.g * wa + b.g * wb + c.g * wc,
            b: a.b * wa + b.b * wb + c.b * wc,
            a: a.a * wa + b.a * wb + c.a * wc,
        }
    }
}
