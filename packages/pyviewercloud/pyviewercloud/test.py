import numpy as np
import pyviewercloud as pcviewer


def test_viewer_small_pointcloud():
    viewer = pcviewer.PointcloudViewer(120, 120, 15)
    small_point_cloud = np.random.randn(5000, 3).astype(np.float32)
    # viewer.plot_3d_points([small_point_cloud], [[255, 255, 255]])
    # assert True


def test_viewer_small_red_small_blue():
    viewer = pcviewer.PointcloudViewer(120, 120, 15)
    small_red = np.random.randn(5000, 3).astype(np.float32)
    small_blue = np.random.randn(5000, 3).astype(np.float32)
    viewer.plot_3d_points([small_red, small_blue], [[255, 0, 0], [0, 0, 255]])
    assert True


def test_viewer_medium_pointcloud():
    viewer = pcviewer.PointcloudViewer(120, 120, 15)
    medium_point_cloud = np.random.randn(5000, 3).astype(np.float32)
    # viewer.plot_3d_points(medium_point_cloud, [255, 255, 255])


def test_viewer_large_pointcloud():
    viewer = pcviewer.PointcloudViewer(120, 120, 15)
    large_point_cloud = np.random.randn(5000, 3).astype(np.float32)
    # viewer.plot_3d_points(large_point_cloud, [255, 255, 255])
