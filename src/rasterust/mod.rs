mod raster;

// A vector in 4-space.
pub struct Vector([f32; 4]);
// A 4x4 matrix.
pub struct Matrix([f32; 16]);

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector([x, y, z, 1.])
    }

    fn nth(&self, idx: usize) -> Option<f32> {
        match (self, idx)  {
            (&Vector(ref data), 0...3) => Some(data[idx]),
            _ => None
        }
    }

    fn x(&self) -> f32 {
        match self.nth(0) {
            Some(s) => s,
            _ => panic!()
        }
    }

    fn y(&self) -> f32 {
        match self.nth(1) {
            Some(s) => s,
            _ => panic!()
        }
    }

    fn z(&self) -> f32 {
        match self.nth(2) {
            Some(s) => s,
            _ => panic!()
        }
    }

    fn w(&self) -> f32 {
        match self.nth(3) {
            Some(s) => s,
            _ => panic!()
        }
    }

    fn dot(&self, vec: &Vector) -> f32 {
        match (self, vec) {
            (&Vector(a), &Vector(b)) => {
                a.iter().zip(b.iter()).fold(0., |sum, (i, j)| sum + (i * j))
            }
        }
    }

    fn sub(&self, vec: &Vector) -> Vector {
        Vector([self.x() - vec.x(),
                self.y() - vec.y(),
                self.z() - vec.z(),
                self.w() - vec.w()])
    }
}

impl Matrix {
    fn identity() -> Matrix {
        Matrix([1., 0., 0., 0.,
                0., 1., 0., 0.,
                0., 0., 1., 0.,
                0., 0., 0., 1.])
    }

    fn translate(pos: &Vector) -> Matrix {
        Matrix([1., 0., 0., pos.x(),
                0., 1., 0., pos.y(),
                0., 0., 1., pos.z(),
                0., 0., 0., pos.w()])
    }

    fn scale(scale: &Vector) -> Matrix {
        Matrix([scale.x(), 0., 0., 0.,
                0., scale.y(), 0., 0.,
                0., 0., scale.z(), 0.,
                0., 0., 0., scale.w()])
    }

    fn apply(&self, vec: &Vector) -> Vector {
        let mut data: [f32; 4] = [0.; 4];
        for i in 0..3 {
            data[i] = self.row(i).dot(vec);
        }
        Vector(data)
    }

    fn row(&self, row: usize) -> Vector {
        match self {
            &Matrix(ref data) => {
                Vector([data[row * 4],
                        data[1 + (row * 4)],
                        data[2 + (row * 4)],
                        data[3 + (row * 4)]])
            }
        }
    }

    fn col(&self, col: usize) -> Vector {
        match (self) {
            &Matrix(ref data) => {
                Vector([data[col],
                        data[col + 4],
                        data[col + 8],
                        data[col + 12]])
            }
        }
    }

    // Produces the matrix AB, where mat is A and self is B
    fn compose(&self, mat: &Matrix) -> Matrix {
        let mut out: [f32; 16] = [0.; 16];
        for j in 0..3 {
            for i in 0..3 {
                out[i * j] = mat.row(j).dot(&self.col(i));
            }
        }
        Matrix(out)
    }
}

pub struct Rect {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

// A primitive triangle.
pub struct Triangle(Vector, Vector, Vector);

impl Triangle {
    fn vertices(&self) -> Vec<&Vector> {
        match self {
            &Triangle(ref a, ref b, ref c) => vec![a, b, c]
        }
    }
}

pub struct Mesh(Vec<Triangle>);

pub struct Model {
    mesh: Mesh,
    pos: Vector,
    scale: Vector,
    rot: Vector,
}

impl Model {
    fn rotate(&mut self, rotation: &Vector) {
        // TODO
    }

    fn translate(&mut self, translation: &Vector) {
        // TODO
    }

    fn scale(&mut self, scale: &Vector) {
        // TODO
    }

    fn get_transform(&self) -> Matrix {
        let translate: Matrix = Matrix::translate(&self.pos);
        // TODO(acomminos): other transforms
        translate
    }
}

// A perspective camera.
pub struct Camera {
    pos: Vector,
    z_near: f32,
    z_far: f32,
    fov: f32,
    ratio: f32,
}

impl Camera {
    // Projects the vector into normalized screen coordinates.
    // Does not perform any clipping.
    // TODO(acomminos): support fov
    fn project_vector(&self, v: &Vector) -> Vector {
        let x = -v.x()/(self.ratio * v.z());
        let y = -v.y()/v.z();
        let z = (v.z() - self.z_near)/(self.z_far - self.z_near);
        Vector([x, y, z, 1.])
    }

    fn project_triangle(&self, tri: &Triangle) -> Triangle {
        match tri {
            &Triangle(ref a, ref b, ref c) => {
                Triangle(self.project_vector(a),
                         self.project_vector(b),
                         self.project_vector(c))
            }
        }
    }
}

pub struct Scene {
    camera: Camera,
    models: Vec<Model>,
}

impl Scene {
    fn render(&self, rt: &mut RenderTarget) {
        for m in &self.models {
            let mat = &m.get_transform();
            match &m.mesh {
                &Mesh(ref triangles) => {
                    for t in triangles {
                        raster::rasterize_barycentric_ccw(t, rt, &self.camera);
                        // TODO
                    }
                }
            }
        }
    }
}

pub struct Buffer<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl <T> Buffer<T> {
    pub fn create(width: usize, height: usize) -> Buffer<T> {
        let mut data = Vec::new();
        data.reserve(width * height);
        Buffer {
            width: width,
            height: height,
            data: data,
        }
    }
}

// A standard render target with a ARGB color buffer and floating point depth
// buffer.
pub struct RenderTarget {
    width: usize,
    height: usize,
    color: Buffer<u32>,
    depth: Buffer<f32>,
}

impl RenderTarget {
    pub fn create(width: usize, height: usize) -> RenderTarget {
        RenderTarget {
            width: width,
            height: height,
            color: Buffer::<u32>::create(width, height),
            depth: Buffer::<f32>::create(width, height),
        }
    }
}
