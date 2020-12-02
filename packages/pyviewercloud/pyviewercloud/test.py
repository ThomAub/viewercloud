import numpy as np
import pyviewercloud as pcviewer

def test_viewer_kitti_pointcloud():
    # viewer = pcviewer.PointcloudViewer(1200, 1800, 15000)
    # kitti_point_cloud = np.fromfile("../../tests/data/kitti/velodyne/000001.bin", dtype=np.float32).reshape((-1, 4))[:,:3]
    # viewer.add_pointcloud(kitti_point_cloud, [255, 255, 255])
    # viewer.show()
    assert True

def test_viewer_lyft_pointcloud():
    # # New viewer
    # viewer = pcviewer.PointcloudViewer(1200, 1800, 15000)
    # # Load some pointcloud from the lyft perception dataset
    # lyft_point_cloud_1 = np.fromfile("../../tests/data/lyft/host-a101_lidar0_1241893239502712366.bin", dtype=np.float32).reshape((-1, 5))[:,:3]
    # lyft_point_cloud_2= np.fromfile("../../tests/data/lyft/host-a101_lidar1_1241893239502712366.bin", dtype=np.float32).reshape((-1, 5))[:,:3]
    # lyft_point_cloud_3= np.fromfile("../../tests/data/lyft/host-a101_lidar2_1241893239502712366.bin", dtype=np.float32).reshape((-1, 5))[:,:3]
    # # Add them one by one to the viewer to have different color
    # viewer.add_pointcloud(lyft_point_cloud_1, [255, 0, 0])
    # viewer.add_pointcloud(lyft_point_cloud_2, [0, 0, 255])
    # viewer.add_pointcloud(lyft_point_cloud_3, [0, 255, 0])
    # # You can now display the window
    # viewer.show()
    assert True

def test_viewer_kitti_with_centroid():
    # viewer = pcviewer.PointcloudViewer(1200, 1800, 15000)
    # kitti_point_cloud = np.fromfile("../../tests/data/kitti/velodyne/000001.bin", dtype=np.float32).reshape((-1, 4))[:,:3]
    # viewer.add_pointcloud(kitti_point_cloud, [255, 255, 255])
    # #centroids = np.random.randn(3, 3).astype(np.float32)
    # centroids = np.array([[-11.5,0,-0.8]]).astype(np.float32)
    # viewer.add_centroid(centroids, [255, 0, 0])
    # viewer.show()
    assert True