//! KITTI Annotation file parsing.
use anyhow::Result;
use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
/// Struct to reprensent every KITTI annotation.
#[derive(Debug, Default, PartialEq)]
pub struct KittiAnnotation {
    /// Describes the type of object:
    /// 'Car', 'Van', 'Truck',
    /// 'Pedestrian', 'Person_sitting',
    /// 'Cyclist', 'Tram',
    /// 'Misc' or 'DontCare'.
    pub category: String,
    /// Float from 0 (non-truncated)
    /// to 1 (truncated), where truncated refers
    /// to the object leaving image boundaries
    pub truncation: f32,
    /// Integer (0,1,2,3) indicating occlusion state:
    /// 0 = fully visible,
    /// 1 = partly occluded,
    /// 2 = largely occluded,
    /// 3 = unknown
    pub occlusion: i16,
    /// Observation angle of object, ranging [-pi..pi]
    pub alpha: f32,
    /// 2D bounding box xmin of object in the image
    pub xmin: f32,
    /// 2D bounding box ymin of object in the image
    pub ymin: f32,
    /// 2D bounding box xmax of object in the image
    pub xmax: f32,
    /// 2D bounding box ymax of object in the image
    pub ymax: f32,
    /// 3D object dimensions  height(in meters)
    pub h: f32,
    /// 3D object dimensions width (in meters)
    pub w: f32,
    /// 3D object dimensions length (in meters)
    pub l: f32,
    /// 3D object location x in camera coordinates (in meters)
    pub x: f32,
    /// 3D object location y in camera coordinates (in meters)
    pub y: f32,
    /// 3D object location z in camera coordinates (in meters)
    pub z: f32,
    /// Rotation ry around Y-axis in camera coordinates [-pi..pi]
    pub ry: f32,
    /// Score indicating confidence in detection [0..1]
    /// If coming from gound truth score is 1.0
    pub score: f32,
}

impl KittiAnnotation {
    /// Create a KittiAnnotation
    pub fn new(
        category: String,
        truncation: f32,
        occlusion: i16,
        alpha: f32,
        xmin: f32,
        ymin: f32,
        xmax: f32,
        ymax: f32,
        h: f32,
        w: f32,
        l: f32,
        x: f32,
        y: f32,
        z: f32,
        ry: f32,
        score: f32,
    ) -> Self {
        Self {
            category,
            truncation,
            occlusion,
            alpha,
            xmin,
            ymin,
            xmax,
            ymax,
            h,
            w,
            l,
            x,
            y,
            z,
            ry,
            score,
        }
    }
    /// Return the 2D BoundingBox in image coordinates system
    pub fn get_2d_bounding_box(self) -> [f32; 4] {
        [self.xmin, self.ymin, self.xmax, self.ymax]
    }
    /// Return the 3D BoundingBox in the Lidar coordinates system
    pub fn get_3d_bounding_box(self) -> [f32; 7] {
        [self.x, self.y, self.z, self.h, self.w, self.l, self.ry]
    }
}

/// Parse a KITTI annotation file describe in the DevKit
pub fn read_annotation_file(kitti_annotations_path: String) -> Result<Vec<KittiAnnotation>> {
    let extension = Path::new(&kitti_annotations_path).extension();
    if extension != Some(OsStr::new("txt")) {
        panic!(
            "KITTI annotation file are in txt format and it received an got {:?}.",
            extension
        );
    }
    let file = File::open(kitti_annotations_path).expect("This file does not exist");
    let file = BufReader::new(file);
    let mut annotation: Vec<KittiAnnotation> = vec![];
    for line in file.lines() {
        let line = line?;
        let data: Vec<&str> = line.split_whitespace().collect();
        if data[0].to_string() != "DontCare" {
            let anno = KittiAnnotation::new(
                data[0].to_string(),
                data[1].parse()?,
                data[2].parse()?,
                data[3].parse()?,
                data[4].parse()?,
                data[5].parse()?,
                data[6].parse()?,
                data[7].parse()?,
                data[8].parse()?,
                data[9].parse()?,
                data[10].parse()?,
                data[11].parse()?,
                data[12].parse()?,
                data[13].parse()?,
                data[14].parse()?,
                1.0,
            );
            annotation.push(anno);
        }
    }
    Ok(annotation)
}
