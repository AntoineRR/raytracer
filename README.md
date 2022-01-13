# raytracer

This project is a simple ray tracing project written in Rust, based on the [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) book. The code was adapted by me from C++ to Rust.

## Features

* Multithreaded rendering of spheres
* 3 different materials : Diffuse, Metal, and Dielectric
* Setting up the Camera where you want in the Scene and render a Scene from different Cameras
* Configuration of the rendering through the `Config` struct

## How to use

Check the `src/bin/main.rs` file for an example scene to render.

## Render examples

![Front view](/examples/front_view.png?raw=true "Front")
![Close up](/examples/close_up.png?raw=true "Close")
![Wide view](/examples/wide_view.png?raw=true "Wide")