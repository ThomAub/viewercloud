use kitti_viewer::annotation::{read_annotation_file, KittiAnnotation};
#[test]
fn test_read_annotation_file_000000() {
    assert_eq!(
        read_annotation_file("tests/data/kitti/label/000000.txt".to_string()).unwrap(),
        vec![KittiAnnotation {
            category: "Pedestrian".to_string(),
            truncation: 0.00,
            occlusion: 0,
            alpha: -0.20,

            xmin: 712.40,
            ymin: 143.00,
            xmax: 810.73,
            ymax: 307.92,

            h: 1.89,
            w: 0.48,
            l: 1.20,

            x: 1.84,
            y: 1.47,
            z: 8.41,
            ry: 0.01,

            score: 1.0,
        }]
    )
}
#[test]
fn test_read_annotation_file_000001() {
    let ann_vec = read_annotation_file("tests/data/kitti/label/000001.txt".to_string()).unwrap();
    assert_eq!(ann_vec.len(), 3);
    assert_eq!(ann_vec[0].category, "Truck");
    assert_eq!(ann_vec[1].category, "Car");
    assert_eq!(ann_vec[2].category, "Cyclist");
}
#[test]
#[should_panic]
fn test_read_annotation_fake_file() {
    read_annotation_file("tests/data/kitti/label/fake.txt".to_string()).unwrap();
}
#[test]
#[should_panic]
fn test_read_annotation_wrong_extension() {
    read_annotation_file("tests/data/kitti/label/fake.json".to_string()).unwrap();
}
