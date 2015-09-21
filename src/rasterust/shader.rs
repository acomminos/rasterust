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
