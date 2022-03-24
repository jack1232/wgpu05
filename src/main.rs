mod common;

use winit::{
    event_loop::{EventLoop},
};
use std::borrow::Cow;

fn main() {
    let mut primitive_type = "triangle-list";
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        primitive_type = &args[1];
    }

    let mut topology = wgpu::PrimitiveTopology::TriangleList;
    let mut index_format = None;
    if  primitive_type == "triangle-strip" {
        topology = wgpu::PrimitiveTopology::TriangleStrip;
        index_format = Some(wgpu::IndexFormat::Uint32);
    }

    let inputs = common::Inputs{
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        topology: topology,
        strip_index_format: index_format
    };
    let event_loop = EventLoop::new();    
    let window = winit::window::Window::new(&event_loop).unwrap(); 
    
    window.set_title(&*format!("{}: {}", "Primitive", primitive_type));
    env_logger::init();   
    pollster::block_on( common::run(event_loop, window, inputs, 9));    
}