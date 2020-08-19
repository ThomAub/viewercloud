# Kitti Viewer

`kitti_viewer` is a library and also a cli to read and display [KITTI](http://www.cvlibs.net/datasets/kitti/) pointcloud.
It will also be able to display the 3D annotations and the 3D BoundingBox computed by your favorite algorithm.

## Usage

```sh
> kitti-viewer --help
kitti-viewer 0.1.0
Thomaub <github.thomaub@gmail.com>
Display KITTI 3D Pointcloud with annotations and your model inferences

USAGE:
    kitti-viewer <pointcloud-file> [ARGS]

ARGS:
    <pointcloud-file>     Path to the kitti Pointcloud .bin or .txt file
    <annotations-file>    Path to the kitti 3D annotation .txt file
    <inferences-file>     Path to your model inferences .json file

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```

## Build

```sh
cargo build --release
```

It will be possible to download from github release and brew.
