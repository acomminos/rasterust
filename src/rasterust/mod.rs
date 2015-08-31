mod raster;

// A vector in 4-space.
pub struct Vector(f32, f32, f32, f32);

// A 4x4 matrix.
pub struct Matrix {
    a: f32, b: f32, c: f32, tx: f32,
    d: f32, e: f32, f: f32, ty: f32,
    g: f32, h: f32, i: f32, tz: f32,
    j: f32, k: f32, l: f32, tw: f32,
}

impl Matrix {
    fn identity() -> Matrix {
        Matrix {
            a: 1., b: 0., c: 0., tx: 0.,
            d: 0., e: 1., f: 0., ty: 0.,
            g: 0., h: 0., i: 1., tz: 0.,
            j: 0., k: 0., l: 0., tw: 1.,
        }
    }

    fn translate(pos: &Vector) -> Matrix {
        let mut mat = Matrix::identity();
        match *pos {
            Vector(x, y, z, _) => {
                mat.tx = x;
                mat.ty = y;
                mat.tz = z;
            }
        };
        mat
    }

    fn scale(scale: &Vector) -> Matrix {
        let mut mat = Matrix::identity();
        match *scale {
            Vector(x, y, z, _) => {
                mat.a = x;
                mat.e = y;
                mat.i = z;
            }
        };
        mat
    }
}

pub struct Quaternion(f32, f32, f32, f32);

// A primitive triangle.
pub struct Triangle(Vector, Vector, Vector);

pub struct Mesh(Vec<Triangle>);

pub struct Model {
    mesh: Mesh,
    pos: Vector,
    scale: Vector,
    rot: Quaternion,
}

impl Model {
    fn get_matrix(&self) -> Matrix {
        let translate: Matrix = Matrix::translate(&self.pos);
        // TODO(acomminos): other transforms
        translate
    }
}

pub struct Camera {
    pos: Vector,
    zNear: f32,
    zFar: f32,
    fov: f32,
}

pub struct Scene {
    camera: Camera,
    models: Vec<Model>,
}
