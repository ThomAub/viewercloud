mod pclparser;
mod renderer;
mod viewer;

use anyhow::Result;
use clap::{crate_authors, crate_version, Clap};
use kiss3d::resource::{AllocationType, BufferType, GPUVec};
use kiss3d::window::Window;

use pclparser::read_kitti_bin_file;
use renderer::PointCloudRenderer;
use viewer::AppState;
use viewercloud::{PointCloud, PointCloudGPU};
/// Display KITTI 3D Pointcloud with annotations and your model inferences
#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
struct Opts {
    ///Path to the kitti Pointcloud .bin or .txt file
    pointcloud_file: String,
    ///Path to the kitti 3D annotation .txt file
    annotations_file: Option<String>,
    ///Path to your model inferences .json file
    inferences_file: Option<String>,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let pointcloud_file = opts.pointcloud_file;
    println!("Value for pointcloud_file: {}", pointcloud_file);

    if let Some(annotations_file) = opts.annotations_file {
        println!("Using input annotations_file: {}", annotations_file);
    }
    if let Some(inferences_file) = opts.inferences_file {
        println!("Using input inferences_file: {}", inferences_file);
    }
    // Parsing Data files into GPU Points
    let pcl_data: PointCloud = read_kitti_bin_file(pointcloud_file)?;
    let point_cloud_data: PointCloudGPU = GPUVec::new(pcl_data, BufferType::Array, AllocationType::StreamDraw);

    // 3D Rendering
    let window = Window::new_with_size("KITII velodyne point_cloud", 1500, 1000);
    let app = AppState {
        point_cloud_renderer: PointCloudRenderer::new(2.0, point_cloud_data),
    };

    window.render_loop(app);
    Ok(())
}
