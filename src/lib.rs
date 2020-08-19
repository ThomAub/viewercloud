//! # Documentation
//!
//! WOW such a nice documentation of the lib

#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![warn(unused_qualifications)]

use kiss3d::resource::GPUVec;
use nalgebra::Point3;
/// A 2D BoundingBox
pub type BBox2D = Vec<Point3<f32>>;
/// A 3D BoundingBox
pub type BBox3D = Vec<Point3<f32>>;
/// A PointCloud
pub type PointCloud = Vec<Point3<f32>>;
/// A PointCloud on GPU
pub type PointCloudGPU = GPUVec<Point3<f32>>;

#[macro_use]
pub mod appstate;
pub mod annotation;
pub mod pclparser;
pub mod pclrenderer;
