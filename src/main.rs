//
// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// main
//

use renderer::Renderer;

mod ppm_interface;
mod vectors;
mod matrix;
mod renderer;

use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut ppm = ppm_interface::PPMInterface::new(String::from(args[1].clone()));
    let height = 540;
    let width = 960;
    let mut renderer : Renderer = Renderer::get_renderer_from_file(String::from(args[2].clone()));
    ppm.write(width, height, renderer.render());
    Ok(())
}