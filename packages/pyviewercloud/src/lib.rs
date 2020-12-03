use kiss3d::light::Light;
use kiss3d::resource::{AllocationType, BufferType, GPUVec};
use kiss3d::window::Window;
use na::geometry::Translation3;
use na::Point3;
use nalgebra as na;
use ndarray::{stack, Array, Array2, Axis};
use numpy::PyReadonlyArray2;
use pyo3::prelude::{pyclass, pymethods, pymodule, PyModule, PyResult, Python};
use pyo3::types::PyList;

use viewercloud::renderer::PointCloudRenderer;
use viewercloud::viewer::AppState;
use viewercloud::PointCloud;
/// PointcloudViewer Class
/// window_height: u32,
/// window_width: u32,
/// max_points: usize,
/// colored_pointcloud: PointCloud,
/// centroids: Vec<(Point3<f32>, Point3<f32>)>,
/// debug: bool,
#[pyclass(unsendable)]
struct PointcloudViewer {
    window_height: u32,
    window_width: u32,
    max_points: usize,
    colored_pointcloud: PointCloud,
    centroids: Vec<(Point3<f32>, Point3<f32>)>,
    debug: bool,
}
#[pymethods]
impl PointcloudViewer {
    /// Create an new instance of viewercloud
    #[new]
    pub fn new(window_height: u32, window_width: u32, max_points: usize) -> Self {
        PointcloudViewer {
            window_height,
            window_width,
            max_points,
            colored_pointcloud: PointCloud::new(),
            centroids: vec![],
            debug: false,
        }
    }
    /// Add a new pointcloud and an associated color to the current viewer.
    ///
    /// Args: pointcloud: numpy.ndarray [NX3] in float32 each row represent a point.
    ///       color: List[int, int, int] List storing the RGB representation of the color.
    pub fn add_pointcloud(&mut self, pointcloud: PyReadonlyArray2<f32>, color: &PyList) {
        let pointcloud = pointcloud.as_array();
        let color: Vec<f32> = color.extract().expect("Cannot extract list to plot the pointcloud");
        let n_points = pointcloud.raw_dim()[0];
        if self.debug {
            if n_points < self.max_points {
                println!("Creating window with {} points", n_points);
            } else {
                println!(
                    "Creating window with {} points instead of {} for performance",
                    self.max_points, n_points
                );
            }
        }
        if self.debug {
            println!(
                "Display 1 Pointcloud with the color ({},{},{})",
                color[0], //.get_item(0),
                color[1], //.get_item(1),
                color[2]  //.get_item(2)
            );
        }
        let color_base: Array2<f32> = ndarray::ArrayBase::ones((n_points, 3));
        let mul_color = Array::from(color);
        let final_color = color_base * mul_color.broadcast((n_points, 3)).expect("Cannot brodcast color");

        let points = stack!(Axis(1), pointcloud, final_color)
            .into_shape((n_points * 2, 3))
            .expect("Cannot stack data together");

        let mut points_vec: Vec<Point3<f32>> = points
            .axis_chunks_iter(Axis(0), 1)
            .map(|vec_own| vec_own.to_slice().unwrap())
            .map(|slice| Point3::from_slice(slice))
            .collect();

        self.colored_pointcloud.append(&mut points_vec);
    }
    /// Add a new centroid with the associated color to the current viewer.
    /// The centroid is represented as an sphere of radius 0.1 m
    ///
    /// Args: centroids: numpy.ndarray [NX3] in float32 each row represent a centroid.
    ///       color: List[int, int, int] List storing the RGB representation of the color.
    pub fn add_centroid(&mut self, centroids: PyReadonlyArray2<f32>, color: &PyList) {
        let centroids = centroids.as_array();
        let color: Vec<f32> = color.extract().expect("Cannot extract color list");
        let n_points = centroids.raw_dim()[0];
        if self.debug {
            if n_points < self.max_points {
                println!("Creating window with {} points", n_points);
            } else {
                println!(
                    "Creating window with {} points instead of {} for performance",
                    self.max_points, n_points
                );
            }
        }
        if self.debug {
            println!(
                "Display 1 centroid with the color ({},{},{})",
                color[0], //.get_item(0),
                color[1], //.get_item(1),
                color[2]  //.get_item(2)
            );
        }
        let mut points_vec: Vec<(Point3<f32>, Point3<f32>)> = centroids
            .axis_chunks_iter(Axis(0), 1)
            .map(|vec_own| vec_own.to_slice().unwrap())
            .inspect(|x| {
                dbg!(x);
            })
            .map(|slice| (Point3::from_slice(slice), Point3::from_slice(&color)))
            .collect();
        self.centroids.append(&mut points_vec);
    }
    pub fn show(&self) {
        let mut window = Window::new_with_size("Viewercloud", self.window_height, self.window_width);
        window.set_light(Light::StickToCamera);
        let pointcloud_gpu: GPUVec<Point3<f32>> = GPUVec::new(
            self.colored_pointcloud.data.clone(),
            BufferType::Array,
            AllocationType::StreamDraw,
        );
        for (centroid, color) in self.centroids.iter() {
            let mut c = window.add_sphere(0.1);
            c.set_color(color.x, color.y, color.z);
            c.prepend_to_local_translation(&Translation3::new(centroid.x, centroid.y, centroid.z));
        }
        if self.debug {
            println!("The points are loaded in GPU {}", pointcloud_gpu.is_on_gpu());
        }
        let app = AppState {
            point_cloud_renderer: PointCloudRenderer::new(2.0, pointcloud_gpu),
        };
        window.render_loop(app);
    }
}

/// Main entry of the python module pyviewercloud
/// Contains the class PointcloudViewer
///
/// ```python
/// import pyviewercloud
/// viewer = pyviewercloud.PointcloudViewer(120, 120, 15)
/// pointcloud = np.random.randn(50000, 3).astype(np.float32)
/// viewer.add_pointcloud(pointcloud, [255, 255, 255])
/// centroids = np.random.randn(3, 3).astype(np.float32)
/// viewer.add_centroid(centroids, [255, 0, 0])
/// ```
#[pymodule]
fn pyviewercloud(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PointcloudViewer>()?;
    Ok(())
}
