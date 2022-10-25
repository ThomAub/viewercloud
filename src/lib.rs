//! # Documentation
//!
//!

#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![warn(unused_qualifications)]

use kiss3d::resource::GPUVec;
use na::Point3;
use nalgebra as na;
/// A 2D BoundingBox
#[derive(Debug)]
pub struct BBox2D {
    _xmin: Point3<f32>,
}
/// A 3D BoundingBox
#[derive(Debug)]
pub struct BBox3D {
    _xmin: Point3<f32>,
}
/// A PointCloud
//pub type PointCloud = Vec<Point3<f32>>;
#[derive(Debug, Clone, Default)]
pub struct PointCloud {
    /// field to store the multiple pointcloud to be passed to the viewer
    pub data: Vec<Point3<f32>>,
}

impl PointCloud {
    /// Create a new empty PointCloud
    pub fn new() -> Self {
        Self { data: vec![] }
    }
    /// Extend the current PointCloud
    pub fn append(&mut self, other: &mut Vec<Point3<f32>>) {
        self.data.append(other);
    }
    /// Extend the current PointCloud
    pub fn push(&mut self, point: Point3<f32>) {
        self.data.push(point);
    }
}

/// A PointCloud on GPU
pub type PointCloudGPU = GPUVec<Point3<f32>>;

#[macro_use]
pub mod viewer;
pub mod annotation;
pub mod pclparser;
pub mod renderer;
