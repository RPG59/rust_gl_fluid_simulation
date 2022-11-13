use gl::{types::{GLfloat, GLenum, GLuint, GLint, GLchar, GLsizeiptr}, Uniform2f};

use crate::math;

pub struct ShaderProgram {
  program: u32,
}

impl ShaderProgram {
    pub fn new(vs_path: &str, fs_path: &str) -> Self {
        ShaderProgram { program: ShaderProgram::link_program(
            ShaderProgram::create_shader(vs_path, gl::VERTEX_SHADER),
            ShaderProgram::create_shader(fs_path, gl::FRAGMENT_SHADER)
        )}
    }

    pub fn enable(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }

    pub fn set_uniform_1f(&self, name: &str, data: f32) {
        unsafe {
            let location = ShaderProgram::get_uniform_location(self, name);
            gl::Uniform1f(location, data);
        }
    }

    pub fn set_uniform_vec2(&self, name: &str, vec: &math::Vec2) {
        unsafe {
            let location = ShaderProgram::get_uniform_location(self, name);
            gl::Uniform2f(location, vec.x, vec.y);
        }
    }

    // pub fn get_active_uniform(&self) {
    //     let bufflen = 1024;
    //     let mut buffer: Vec<u8> = Vec::with_capacity(bufflen);
    //     let mut uniform_size = 999;
    //     let ptr = buffer.as_mut_ptr() as *mut i8;

    //     unsafe {
    //         gl::GetActiveUniform(self.program, 0, bufflen as i32, &mut uniform_size, std::ptr::null_mut(), std::ptr::null_mut(), ptr);
    //         let res = String::from_utf8_unchecked(buffer);
    //         println!("test getUniform: {}", res);
    //         println!("test size: {}", uniform_size);
    //     }
    // }
    
    unsafe fn get_uniform_location(&self, name: &str) -> i32 {
        let cname = std::ffi::CString::new(name.as_bytes()).unwrap();
        let location = gl::GetUniformLocation(self.program, cname.as_ptr());

        if location == -1 {
            println!("\x1b[93m GetUniformLocation ERROR! name: `{}` \x1b[0m", name);
        }

        location
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
        let mut file_data = std::fs::read_to_string(path).unwrap();

        ShaderProgram::compile_shader(file_data.as_str(), shader_type)
    }

    fn compile_shader(src: &str, shader_type: GLenum) -> GLuint {
        let shader;

        unsafe {
            shader = gl::CreateShader(shader_type);
            let c_str = std::ffi::CString::new(src.as_bytes()).unwrap();
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
            gl::CompileShader(shader);

            let mut compile_status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut compile_status);

            if compile_status != gl::TRUE as GLint {
                let mut len = 0;

                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

                let mut buffer = Vec::<u8>::with_capacity(len as usize);
                gl::GetShaderInfoLog(shader, len, &mut len, buffer.as_mut_ptr() as *mut i8);

                buffer.set_len(len as usize);

                panic!("Compiles shader error! src: {}, message: {}", src, std::str::from_utf8(&buffer).ok().expect("ShaderInfoLog failed"));

            }
        }

        shader
    }

}