extern crate sdl2;
extern crate gl;

use sdl2::event::Event;
use gl::types::{GLfloat, GLenum, GLuint, GLint, GLchar, GLsizeiptr};

use std::{ffi::CString, io::Read};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

static VERTICES: [f32; 12] = [
    -1.0,  1.0,
    -1.0, -1.0, 
     1.0,  1.0, 

     1.0,  1.0, 
    1.0, -1.0, 
    -1.0,  -1.0,
];

fn compile_shader(src: &str, shader_type: GLenum) -> GLuint {
    let shader;

    unsafe {
        shader = gl::CreateShader(shader_type);
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        let mut compile_status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut compile_status);

        if compile_status != gl::TRUE as GLint {
            let mut len = 0;
            let mut buffer: *mut GLchar = std::ptr::null_mut();

            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

            buffer = vec![0 as GLchar; len as usize].as_mut_ptr() as *mut GLchar;

            gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), buffer);

            panic!("{}", std::ffi::CStr::from_ptr(buffer).to_str().ok().expect("ShaderInfoLog failed"));
        }
    }

    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        program
    }

}

fn create_shader(path: &str, shader_type: GLenum) -> GLuint {
    let mut file = std::fs::File::open(path).unwrap();
    let mut file_data = String::new();
    file.read_to_string(&mut file_data).unwrap();

    compile_shader(file_data.as_str(), shader_type)
}

fn create_rect() {
    let mut vbo = 0;

    unsafe {

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTICES.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            std::mem::transmute(&VERTICES[0]),
            gl::STATIC_DRAW
    );
    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());

    }

}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut window = video_subsystem.window("fluids-simulation", WIDTH, HEIGHT).opengl().resizable().build().unwrap();
    let gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    let program = link_program(
        create_shader("/Users/rpg59/dev/rust/fluids-simulation/shaders/vertex.glsl", gl::VERTEX_SHADER),
         create_shader("/Users/rpg59/dev/rust/fluids-simulation/shaders/fragment.glsl", gl::FRAGMENT_SHADER));

    unsafe {
        let version = gl::GetString(gl::VERSION);
        let dataStr = std::ffi::CStr::from_ptr(version as *const i8).to_str().unwrap();
        println!("{}", dataStr);
    }

    create_rect();

    unsafe {
        gl::UseProgram(program);
        gl::ClearColor(0.1, 0.2, 0.3, 1.0);
    }

    let mut event_pump = sdl_context.event_pump().unwrap();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }

        window.gl_swap_window();
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
