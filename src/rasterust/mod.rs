mod raster;
mod shader;
mod samplers;

use std::ops::Add;
use std::ops::Mul;
use std::f32;

// A vector in 4-space.
pub struct Vector([f32; 4]);
// A 4x4 matrix.
pub struct Matrix([f32; 16]);

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector([x, y, z, 1.])
    }

    pub fn zero() -> Vector {
        Vector([0., 0., 0., 0.])
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
}

impl Mul for Matrix {
    type Output = Matrix;

    // Produces the matrix AB.
    fn mul(self, rhs: Matrix) -> Matrix {
        let mut out: [f32; 16] = [0.; 16];
        for j in 0..3 {
            for i in 0..3 {
                out[i * j] = self.row(j).dot(&rhs.col(i));
            }
        }
        Matrix(out)
    }
}

pub struct Rect {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

// A primitive triangle.
pub struct Triangle(Vector, Vector, Vector);

impl Triangle {
    pub fn new(a: Vector, b: Vector, c: Vector) -> Triangle {
        Triangle(a, b, c)
    }

    fn vertices(&self) -> Vec<&Vector> {
        match self {
            &Triangle(ref a, ref b, ref c) => vec![a, b, c]
        }
    }

    // Returns a bounding box encapsulating the triangle in the XY-plane.
    fn bounds(&self) -> Rect {
        let &Triangle(ref a, ref b, ref c) = self;
        let mut rect = Rect {
            top: f32::MAX,
            bottom: f32::MIN,
            left: f32::MAX,
            right: f32::MIN,
        };
        for i in [a, b, c].iter() {
            rect.top = rect.top.min(i.x());
            rect.bottom = rect.bottom.max(i.x());
            rect.left = rect.left.min(i.y());
            rect.right = rect.right.max(i.y());
        }
        rect
    }
}

pub struct Mesh(Vec<Triangle>);

impl Mesh {
    pub fn new(tris: Vec<Triangle>) -> Mesh {
        Mesh(tris)
    }
}

pub struct Model {
    mesh: Mesh,
    pos: Vector,
    scale: Vector,
    rot: Vector,
}

impl Model {
    pub fn new(mesh: Mesh) -> Model {
        Model {
            mesh: mesh,
            pos: Vector::zero(),
            scale: Vector::new(1., 1., 1.),
            rot: Vector::zero(),
        }
    }

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
    rot: Vector,
    z_near: f32, // The near z-clipping plane.
    z_far: f32, // The far z-clipping plane.
    fov: f32, // The horizontal field of view, in radians.
    ratio: f32, // Screen aspect ratio of width/height.
}

impl Camera {
    pub fn new(pos: Vector, rot: Vector, aspect: f32, fov: f32, near: f32, far: f32) -> Camera {
        Camera {
            pos: pos,
            rot: rot,
            ratio: aspect,
            fov: fov,
            z_near: near,
            z_far: far
        }
    }

    // Projects the vector into normalized screen coordinates.
    // Does not perform any clipping.
    // TODO: replace this with a simple function returning a matrix to be used
    // in a homogenous coordinate system
    fn project_vector(&self, v: &Vector) -> Vector {
        let x = v.x()/(self.ratio * (self.fov / 2.).tan() * v.z());
        let y = v.y()/v.z();
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

    fn contains_point(&self, (x, y, z): (f32, f32, f32)) -> bool {
        x >= -1. && x <= 1. &&
        y >= -1. && y <= 1. &&
        z >= -1. && z <= 1.
    }
}

pub struct Scene {
    camera: Camera,
    models: Vec<Model>,
}

impl Scene {
    pub fn new(camera: Camera) -> Scene {
        Scene {
            camera: camera,
            models: vec![]
        }
    }

    pub fn camera<'a>(&'a self) -> &'a Camera {
        &self.camera
    }

    pub fn add_model(&mut self, model: Model) {
        self.models.push(model);
    }

    pub fn render(&self, rt: &mut RenderTarget) {
        for m in &self.models {
            let model_transform = &m.get_transform();
            let &Mesh(ref triangles) = &m.mesh;
            for t in triangles {
                // FIXME(acomminos): placeholder
                let ph_shader = shader::SolidColorShader(Color::white());
                let sampler = samplers::SimpleMultiSampler(2);
                // TODO(acomminos): use model_transform
                let t_proj = self.camera.project_triangle(t);
                raster::rasterize_barycentric_ccw(&t_proj, rt, &self.camera, &sampler, &ph_shader);
            }
        }
    }
}

pub struct Buffer<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl <T> Buffer<T> where T: Clone {
    pub fn new(width: usize, height: usize, initial: T) -> Buffer<T> {
        let mut data: Vec<T> = Vec::with_capacity(width * height);
        // FIXME(acomminos): find more idiomatic way to do this
        for i in 0..(width * height) {
            data.push(initial.clone());
        }
        Buffer {
            width: width,
            height: height,
            data: data,
        }
    }

    pub fn put(&mut self, (x, y): (usize, usize), val: T) {
        self.data[x + (y * self.width)] = val;
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.data[x + (y * self.width)]
    }
}

// Pixel blend modes.
pub enum CompositeMode {
    SourceOver,
}

// A 32-bit ARGB colour.
// Use premultiplied alpha for consistency.
#[derive(Copy, Clone)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32
}

impl Color {
    fn white() -> Color {
        Color::new(1., 1., 1., 1.)
    }

    fn zero() -> Color {
        Color::new(0., 0., 0., 0.)
    }

    // Create
    fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r: r, g: g, b: b, a: a }
    }

    fn from_rgba32(rgba: &u32) -> Color {
        let max = u8::max_value() as f32;
        Color::new((((rgba >> 24) & 0xFFu32) as f32)/max,
                   (((rgba >> 16) & 0xFFu32) as f32)/max,
                   (((rgba >> 8) & 0xFFu32) as f32)/max,
                   (((rgba >> 0) & 0xFFu32) as f32)/max)

    }

    fn to_rgba32(&self) -> (u8, u8, u8, u8) {
        ((self.r * (u8::max_value() as f32)) as u8,
         (self.g * (u8::max_value() as f32)) as u8,
         (self.b * (u8::max_value() as f32)) as u8,
         (self.a * (u8::max_value() as f32)) as u8)
    }

    fn unpremultiply(&self) -> Color {
        Color {
            r: self.r / self.a,
            g: self.g / self.a,
            b: self.b / self.a,
            a: self.a,
        }
    }

    fn multiply(&self, val: f32) -> Color {
        Color::new(self.r * val, self.g * val, self.b * val, self.a * val)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b, self.a + rhs.a)
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
    pub fn new(width: usize, height: usize) -> RenderTarget {
        RenderTarget {
            width: width,
            height: height,
            color: Buffer::<u32>::new(width, height, 0u32),
            depth: Buffer::<f32>::new(width, height, 1.),
        }
    }

    // Toy painting function to paint the pixel at (x, y) with the 32-bit RGBA
    // colour provided.
    pub fn paint(&mut self, (x, y): (usize, usize), src: &Color, op: CompositeMode) {
        let dest = Color::from_rgba32(self.color.get(x, y));
        let color = match op {
            // note: colors here are premultiplied
            SourceOver => dest.multiply(1. - src.a) + *src
        };
        let (r, g, b, a) = color.to_rgba32();
        self.color.put((x, y), ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | a as u32)
    }

    // Checks to see if depth is less than the value stored in the depth buffer.
    // If so, returns true and stores the depth value.
    // The depth buffer stores floating-point values in the range [0, 1]. By
    // default, it is initialized to 1.
    pub fn check_depth(&mut self, (x, y): (usize, usize), depth: f32) -> bool {
        if depth < *self.depth.get(x, y) {
            self.depth.put((x, y), depth);
            return true;
        }
        return false;
    }

    // Returns the ratio of width:height.
    pub fn aspect(&self) -> f32 {
        (self.width as f32) / (self.height as f32)
    }

    pub fn print_ascii(&self) {
        print!["┌──"];
        for _ in 1..(self.color.width - 1) {
            print!["──"];
        }
        println!["──┐"];

        for y in 0..self.color.height {
            print!["│"];
            for x in 0..self.color.width {
                let color = Color::from_rgba32(self.color.get(x, y));
                let a = color.a;
                let block = if a == 0. {
                    "  "
                } else if a <= 0.25 {
                    "░░"
                } else if a <= 0.5 {
                    "▒▒"
                } else if a <= 0.75 {
                    "▓▓"
                } else {
                    "██"
                };
                print!["{}", block];
            }
            println!["│"];
        }

        print!["└──"];
        for _ in 1..(self.color.width - 1) {
            print!["──"];
        }
        println!["──┘"];
    }
}


