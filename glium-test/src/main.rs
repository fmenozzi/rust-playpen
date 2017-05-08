#[macro_use]
extern crate glium;

use glium::{DisplayBuild, Surface};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn main() {
    let display = glium::glutin::WindowBuilder::new()
                    .build_glium()
                    .unwrap();

    let triangle = vec![
        Vertex {position: [-0.50, -0.50]},
        Vertex {position: [ 0.00,  0.50]},
        Vertex {position: [ 0.50, -0.25]},
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &triangle).unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vs_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fs_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vs_src, fs_src, None).unwrap();

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        for event in display.poll_events() {
            match event {
                glium::glutin::Event::Closed => {
                    return;
                }

                _ => ()
            }
        }
    }
}
