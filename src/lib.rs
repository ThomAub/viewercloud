//! # Documentation
//!
//! WOW such a nice documentation of the lib

#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![warn(unused_qualifications)]

use kiss3d::resource::GPUVec;
use na::Point3;
use nalgebra as na;
/// A 2D BoundingBox
#[derive(Debug)]
pub struct BBox2D {
    xmin: Point3<f32>,
}
/// A 3D BoundingBox
#[derive(Debug)]
pub struct BBox3D {
    xmin: Point3<f32>,
}
/// A PointCloud
pub type PointCloud = Vec<Point3<f32>>;
// #[derive(Debug)]
// pub struct PointCloud {
//     data: Vec<Point3<f32>>,
// }

// impl PointCloud {
//     /// Create a new PointCloud
//     pub fn new() -> Self {
//         Self { data: vec![] }
//     }
//     /// Appends an Point to the PointCloud
//     pub fn push(&mut self, point: Point3<f32>) -> () {
//         self.data.push(point);
//     }
// }

/// A PointCloud on GPU
pub type PointCloudGPU = GPUVec<Point3<f32>>;

#[macro_use]
pub mod viewer;
pub mod annotation;
pub mod pclparser;
pub mod renderer;
