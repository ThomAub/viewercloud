//! KITTI PointCloud file parsing.
use crate::PointCloud;
use anyhow::Result;
use na::Point3;
use nalgebra as na;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
/// Read KITTI pointcloud data store in binary file format.
/// It's the same format from numpy to_file method.
pub fn read_kitti_bin_file(pointcloud_file: String) -> Result<PointCloud> {
    let mut file = File::open(pointcloud_file)?;
    let mut raw_data = vec![];
    file.read_to_end(&mut raw_data)?;

    let mut pcl_data: PointCloud = PointCloud::new();
    for num in raw_data.chunks_exact(16) {
        let x = f32::from_ne_bytes(
            num[0..4]
                .try_into()
                .expect("not the good size for the x coordinate"),
        );
        let y = f32::from_ne_bytes(
            num[4..8]
                .try_into()
                .expect("not the good size for the y coordinate"),
        );
        let z = f32::from_ne_bytes(
            num[8..12]
                .try_into()
                .expect("not the good size for the z coordinate"),
        );
        let _ = f32::from_ne_bytes(
            num[12..16]
                .try_into()
                .expect("not the good size for intensity"),
        );

        // To fit the view from kiss3D window we change axis
        let point = Point3::from_slice(&[z, x, y]);

        // The PointCloudRenderer is using 1 point for 3D point and 1 point for color
        let color = Point3::new(1.0, 1.0, 1.0);
        pcl_data.push(point);
        pcl_data.push(color);
    }
    Ok(pcl_data)
}
#[allow(dead_code)]
/// Read pointcloud data store in a txt file
pub fn parse_pointcloud_txt(file_path: String) -> Result<PointCloud> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(&file);
    let mut pcl_data: PointCloud = PointCloud::new();

    for line in reader.lines() {
        let data = line?;
        let data_point: Vec<f32> = data
            .split_whitespace()
            .map(|word| word.parse::<f32>().unwrap())
            .collect();

        // To fit the view from kiss3D window we change axis
        let point = Point3::from_slice(&[data_point[2], data_point[0], data_point[1]]);

        // The PointCloudRenderer is using 1 point for 3D point and 1 point for color
        let color = Point3::new(1.0, 1.0, 1.0);
        pcl_data.push(point);
        pcl_data.push(color);
    }
    Ok(pcl_data)
}
