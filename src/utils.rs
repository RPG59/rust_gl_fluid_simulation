macro_rules! gl_call {
    () => {
        check_gl_error(file!(), line()!);
    };
}

fn check_gl_error(function: &str, line: u32) {
    unsafe {
        loop {
            let code = gl::GetError();

            if code == gl::NO_ERROR {
                return;
            }

            let message = match code {
                gl::INVALID_ENUM => "INVALID_ENUM",
                gl::INVALID_VALUE => "INVALID_VALUE",
                gl::INVALID_OPERATION => "INVALID_OPERATION",
                gl::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
                gl::STACK_OVERFLOW => "STACK_OVERFLOW",
                gl::STACK_UNDERFLOW => "STACK_UNDERFLOW",
                gl::OUT_OF_MEMORY => "OUT_OF_MEMORY",
                _ => "UNDEFINED_ERROR",
            };

            println!("GL error in {}, line: {}, message: {}", function, line, message);
        }
    }
}