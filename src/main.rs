extern crate sdl2;
extern crate gl;

use advection::Advection;
use externalForce::ExternalForce;
use postProcessPass::PostProcessPass;
use renderTarget::RenderTarget;
use sdl2::event::Event;
use gl::{types::{GLfloat, GLenum, GLuint, GLint, GLchar, GLsizeiptr}, Uniform2f};
use std::{ffi::CString, io::Read, thread::current, collections::HashMap};
use crate::shader::ShaderProgram;

pub mod shader;
pub mod math;
pub mod renderTarget;
pub mod postProcessPass;
pub mod advection;
pub mod externalForce;



const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

// mod gargen {
//     pub struct GardenTest {
//         test: u32
//     }
// }


static VERTICES: [f32; 12] = [
    -1.0,  1.0,
    -1.0, -1.0, 
     1.0,  1.0, 

     1.0,  1.0, 
    1.0, -1.0, 
    -1.0,  -1.0,
];


// #[derive(Default, Copy, Clone)]
// struct RenderTarget {
//     framebuffer: u32,
//     texture: u32,
// }

// impl RenderTarget {
//     fn new() -> Self {
//         let mut render_target = RenderTarget::default();

//         unsafe {
//             gl::GenFramebuffers(1, &mut render_target.framebuffer);
//             gl::GenTextures(1, &mut render_target.texture);

//             gl::BindFramebuffer(gl::FRAMEBUFFER, render_target.framebuffer);
//             gl::BindTexture(gl::TEXTURE_2D, render_target.texture);

//             gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA16F as i32, WIDTH as i32, HEIGHT as i32, 0, gl::RGBA, gl::FLOAT, std::ptr::null());
//             gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
//             gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
//             gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
//             gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
//             gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, render_target.texture, 0);
//         }

//         render_target
//     }
// }

// struct Simulation {
//     current_render_target_index: usize,
//     render_targets: [RenderTarget; 2],
// }

// impl Simulation {
//     fn new() -> Self {
//         Simulation { render_targets: [RenderTarget::new(), RenderTarget::new()], current_render_target_index: 0 }
//     }

//     fn render(&self) {
//         let current_rt = self.render_targets[self.current_render_target_index];

//         unsafe {
//             gl::BindFramebuffer(gl::FRAMEBUFFER, current_rt.framebuffer);
//         }
//     }
// }




fn create_rect() {
    let mut vbo = 0;
    let mut ibo = 0;

    let mesh = generate_sphere_mesh();

    println!("MESH LEN: {}", mesh.indices.len());

    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    //     gl::BufferData(
    //         gl::ARRAY_BUFFER,
    //         (VERTICES.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
    //         std::mem::transmute(&VERTICES[0]),
    //         gl::STATIC_DRAW
    // );
    gl::BufferData(
        gl::ARRAY_BUFFER,
         (mesh.vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
         std::mem::transmute(mesh.vertices.as_ptr()),
         gl::STATIC_DRAW
    );

    gl::GenBuffers(1, &mut ibo);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        (mesh.indices.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
            std::mem::transmute(mesh.indices.as_ptr()),
            gl::STATIC_DRAW
    );

    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());

    }

}

struct Mesh {
    vertices: Vec<f32>,
    indices: Vec<i32>,
}

fn generate_sphere_mesh() -> Mesh {
    let M = 8;
    let N = 16;
    let mut sph_vtx: Vec<f32> = Vec::with_capacity(3 * 16 * 8);
    let mut sph_idx: Vec<i32> = Vec::with_capacity(M * (N * 2 + 2) * (M - 1) * 2);

    for i in 0..M + 1 {
        let p = std::f32::consts::PI * i as f32 / M as f32;
        let r = f32::sin(p);
        let y = f32::cos(p);

        for j in 0..N {
            let th = j as f32 * 2. * std::f32::consts::PI / N as f32;
            sph_vtx.push(r * f32::cos(th));    
            sph_vtx.push(y);
            sph_vtx.push(r * f32::sin(th));
        }
    }

    for i in 0..M {
        for j in 0..N {
            sph_idx.push((i * N  + j) as i32);
            sph_idx.push(((i + 1) * N + j) as i32);
        }

        sph_idx.push((i * N) as i32);
        sph_idx.push(((i + 1) * N) as i32);

        if i < M - 1 {
            sph_idx.push(((i + 1) * N * (N - 1)) as i32);
            sph_idx.push((i + 1) as i32);
        }
    }

    Mesh {
        vertices: sph_vtx,
        indices: sph_idx
    }

}


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut window = video_subsystem.window("fluids-simulation", WIDTH, HEIGHT).opengl().resizable().build().unwrap();
    let gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    let program = ShaderProgram::new( "shaders/vertex.glsl", "shaders/fragment.glsl");

    // *** LOG ***
    unsafe {
        let version = gl::GetString(gl::VERSION);
        let dataStr = std::ffi::CStr::from_ptr(version as *const i8).to_str().unwrap();
        println!("{}", dataStr);
    }
    // *** LOG END ***

    create_rect();

    unsafe {
        program.enable();
        gl::ClearColor(0.1, 0.2, 0.3, 1.0);
    }

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut render_count: u64 = 0;

    let resolution = math::Vec2{x: WIDTH as f32, y:HEIGHT as f32};
    let mut advection_pass = Advection::new();
    let mut external_force = ExternalForce::new();
    let mut velocity_rt = RenderTarget::new(2, &resolution);

    let mut mouse = math::Vec2::default();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::MouseButtonDown { timestamp, window_id, which, mouse_btn, clicks, x, y } => {
                        let position = math::Vec2 { x: x as f32 / WIDTH as f32, y: (HEIGHT as f32 -  y as f32) / HEIGHT as f32 };
                        println!("CLICK: {:?}", position);
                }
                Event::MouseMotion { timestamp: _, window_id: _, which: _, mousestate: _, x, y, xrel: _, yrel: _ } => {
                        let position = math::Vec2 { x: x as f32 / WIDTH as f32, y: (HEIGHT as f32 -  y as f32) / HEIGHT as f32 };
                        mouse = position;
                        println!("X: {}, Y: {}", x, y);
                }
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            let current_render_target = (render_count % 2) as usize;
            let next_render_target  = ((render_count + 1) % 2) as usize;



            // program_compute.enable();
            // program_compute.set_uniform_1f("devergenceSampler", 0.);

            // gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            // gl::BindFramebuffer(gl::FRAMEBUFFER, render_targets[current_render_target].framebuffer);
            // gl::BindTexture(gl::TEXTURE_2D, render_targets[next_render_target].texture);
            // gl::ActiveTexture(gl::TEXTURE0);
            // gl::DrawElements(gl::TRIANGLE_STRIP, 286, gl::UNSIGNED_INT, std::ptr::null());
            // program_compute.set_uniform_vec2("u_mouse", &math::Vec2{x: 0., y: 0.});


            velocity_rt.bind();
            advection_pass.render();
            
            velocity_rt.bind();
            external_force.set_force(&mouse);
            external_force.render();

            program.enable();
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            // gl::BindTexture(gl::TEXTURE_2D, render_targets[current_render_target].texture);
            gl::BindTexture(gl::TEXTURE_2D, velocity_rt.get_texture());
            gl::ActiveTexture(gl::TEXTURE0);
            gl::DrawElements(gl::TRIANGLE_STRIP, 286, gl::UNSIGNED_INT, std::ptr::null());
        }

        window.gl_swap_window();
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
        render_count += 1;
    }
}
