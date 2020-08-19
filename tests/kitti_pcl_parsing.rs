/// >>> import numpy as np
/// >>> np.arange(0,12, dtype=np.float32).reshape(-1,4).tofile("small_vec.bin")
/// >>> np.arange(0,12000,
/// dtype=np.float32).reshape(-1,4).tofile("bigger_vec.bin") >>> np.savetxt("
/// bigger_vec.txt", np.arange(0,12000, dtype=np.float32).reshape(-1,4))
/// >>> np.savetxt("small_vec.txt", np.arange(0,12,
/// dtype=np.float32).reshape(-1,4))
use kitti_viewer::pclparser::{parse_pointcloud_txt, read_kitti_bin_file};
use nalgebra::Point3;
#[test]
fn test_read_small_bin_file() {
    assert_eq!(
        read_kitti_bin_file("tests/data/kitti/velodyne/small_vec.bin".to_string()).unwrap(),
        vec![
            Point3::new(2.0, 0.0, 1.0),
            Point3::new(1.0, 1.0, 1.0),
            Point3::new(6.0, 4.0, 5.0),
            Point3::new(1.0, 1.0, 1.0),
            Point3::new(10.0, 8.0, 9.0),
            Point3::new(1.0, 1.0, 1.0),
        ]
    )
}
#[test]
fn test_read_bigger_bin_file() {
    assert_eq!(
        read_kitti_bin_file("tests/data/kitti/velodyne/bigger_vec.bin".to_string())
            .unwrap()
            .len(),
        6000
    )
}
#[test]
fn test_read_small_txt_file() {
    assert_eq!(
        parse_pointcloud_txt("tests/data/kitti/velodyne/small_vec.txt".to_string()).unwrap(),
        vec![
            Point3::new(2.0, 0.0, 1.0),
            Point3::new(1.0, 1.0, 1.0),
            Point3::new(6.0, 4.0, 5.0),
            Point3::new(1.0, 1.0, 1.0),
            Point3::new(10.0, 8.0, 9.0),
            Point3::new(1.0, 1.0, 1.0),
        ]
    )
}
#[test]
fn test_read_bigger_txt_file() {
    assert_eq!(
        parse_pointcloud_txt("tests/data/kitti/velodyne/bigger_vec.txt".to_string())
            .unwrap()
            .len(),
        6000
    )
}
#[test]
fn test_read_000000_bin_file() {
    assert_eq!(
        read_kitti_bin_file("tests/data/kitti/velodyne/000000.bin".to_string())
            .unwrap()
            .len(),
        230768
    )
}
#[test]
fn test_read_000001_bin_file() {
    assert_eq!(
        read_kitti_bin_file("tests/data/kitti/velodyne/000001.bin".to_string())
            .unwrap()
            .len(),
        240536
    )
}
