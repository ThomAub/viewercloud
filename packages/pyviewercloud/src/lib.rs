use kiss3d::light::Light;
use kiss3d::resource::{AllocationType, BufferType, GPUVec};
use kiss3d::window::Window;
use na::Point3;
use nalgebra as na;
use ndarray::{stack, Array, Array2, Axis};
use numpy::PyReadonlyArray2;
use pyo3::prelude::{pyclass, pymethods, pymodule, PyModule, PyResult, Python};
use pyo3::types::PyList;

use viewercloud::renderer::PointCloudRenderer;
use viewercloud::viewer::AppState;
/// PointcloudViewer Class
#[pyclass(unsendable)]
struct PointcloudViewer {
    window_height: u32,
    window_width: u32,
    max_points: usize,
    debug: bool,
}
#[pymethods]
impl PointcloudViewer {
    #[new]
    fn new(window_height: u32, window_width: u32, max_points: usize) -> Self {
        PointcloudViewer {
            window_height,
            window_width,
            max_points,
            debug: false,
        }
    }
    fn plot_3d_points(&self, pointclouds: &PyList, pointclouds_color: &PyList) {
        let pointclouds: Vec<PyReadonlyArray2<f32>> = pointclouds.extract().expect("Cannot extract pointcloud list");
        let pointclouds_color: Vec<Vec<f32>> = pointclouds_color.extract().expect("Cannot extract color list list");
        let mut data: Vec<Point3<f32>> = vec![];
        if pointclouds.len() != pointclouds_color.len() {
            panic!(
                "The number of pointcloud and colors should be the same. Here found {} pointclouds and {} colors",
                pointclouds.len(),
                pointclouds_color.len()
            )
        }
        let pts_colored = pointclouds.iter().zip(pointclouds_color.iter());

        for (pts_3d, color) in pts_colored {
            // looking at the number of points in the pointcloud
            let pts_3d = pts_3d.as_array();
            let n_points = pts_3d.raw_dim()[0];

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
                    "Display 1 Pointcloud is the color ({},{},{})",
                    color[0], //.get_item(0),
                    color[1], //.get_item(1),
                    color[2]  //.get_item(2)
                );
            }
            let color_vec: Vec<f32> = color.to_vec();
            let color_base: Array2<f32> = ndarray::ArrayBase::ones((n_points, 3));
            let mul_color = Array::from(color_vec);
            let final_color = color_base * mul_color.broadcast((n_points, 3)).expect("Cannot brodcast color");

            let points = stack!(Axis(1), pts_3d, final_color)
                .into_shape((n_points * 2, 3))
                .expect("Cannot stack data together");

            let mut points_vec: Vec<Point3<f32>> = points
                .axis_chunks_iter(Axis(0), 1)
                .map(|vec_own| vec_own.to_slice().unwrap())
                .map(|slice| Point3::from_slice(slice))
                .collect();

            data.append(&mut points_vec);
        }
        let mut window = Window::new_with_size("Viewercloud", self.window_height, self.window_width);
        window.set_light(Light::StickToCamera);

        let point_cloud_data: GPUVec<Point3<f32>> = GPUVec::new(data, BufferType::Array, AllocationType::StreamDraw);
        let app = AppState {
            point_cloud_renderer: PointCloudRenderer::new(2.0, point_cloud_data),
        };
        window.render_loop(app);
    }
}

/// Main entry of the python module
#[pymodule]
fn pyviewercloud(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PointcloudViewer>()?;
    Ok(())
}
