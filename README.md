# raytracer

This project is a simple ray tracing project written in Rust, based on the [Ray Tracing in One Weekend](https://raytracing.github.io/) book series. The code was adapted by me from C++ to Rust.

## Features

* Rendering of spheres and 3D models (STL files)
* 5 different materials : Diffuse, Metal, DiffuseMetal, Dielectric, and DiffuseLight
* Setting a color for the Skybox
* Setting up the Camera where you want in the Scene and render a Scene from different Cameras
* Configuration of the rendering through the `Config` struct
* Optimizations: Multithreading and Bounding Volume Hierarchy (BVH)

## How to use

Clone this repository, and run `cargo run --release` in the `raytracer` directory.
Check the `src/bin/main.rs` file for example scenes to render.

## Render examples

![Big scene](/examples/big_scene.png?raw=true "Big")
![Light](/examples/light.png?raw=true "Light")
![Teapot](/examples/teapot.png?raw=true "Teapot")
![Close up](/examples/close_up.png?raw=true "Close")