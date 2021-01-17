use gl::types::{GLboolean, GLenum, GLfloat, GLsizeiptr, GLubyte, GLuint, GLvoid};
use std::ffi::CString;
use std::mem::{self, size_of};
use std::ptr;

mod shader;

static VERTEX_DATA: [GLfloat; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];

pub struct Renderer {
    program: GLuint,
    vert_shader: GLuint,
    frag_shader: GLuint,
    vao: GLuint,
    vbo: GLuint,
}

impl Renderer {
    pub fn new() -> Self {
        let mut renderer = Renderer {
            program: 0,
            vert_shader: 0,
            frag_shader: 0,
            vao: 0,
            vbo: 0,
        };

        renderer.vert_shader =
            shader::compile_shader(include_str!("../shaders/text.vert.glsl"), gl::VERTEX_SHADER);
        renderer.frag_shader = shader::compile_shader(
            include_str!("../shaders/text.frag.glsl"),
            gl::FRAGMENT_SHADER,
        );
        renderer.program = shader::link_program(renderer.vert_shader, renderer.frag_shader);

        unsafe {
            // Create Vertex Array Object
            gl::GenVertexArrays(1, &mut renderer.vao);
            gl::BindVertexArray(renderer.vao);

            // Create a Vertex Buffer Object and copy the vertex data to it
            gl::GenBuffers(1, &mut renderer.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, renderer.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                VERTEX_DATA.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            // Use shader program
            gl::UseProgram(renderer.program);
            let name = CString::new("out_color").unwrap();
            gl::BindFragDataLocation(renderer.program, 0, name.as_ptr());

            // Specify the layout of the vertex data
            let name = CString::new("position").unwrap();
            let pos_attr = gl::GetAttribLocation(renderer.program, name.as_ptr());
            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(
                pos_attr as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                0,
                ptr::null(),
            );

            // Unbind buffers and program
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }

        renderer
    }

    pub fn render(&mut self) {
        unsafe {
            // Bind program and buffers
            gl::UseProgram(self.program);
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            // Draw a triangle from the 3 vertices
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            if self.program != 0 {
                gl::DeleteProgram(self.program);
            }
            if self.vert_shader != 0 {
                gl::DeleteShader(self.vert_shader);
            }
            if self.frag_shader != 0 {
                gl::DeleteShader(self.frag_shader);
            }
            if self.vbo != 0 {
                gl::DeleteBuffers(1, &self.vbo);
            }
            if self.vao != 0 {
                gl::DeleteVertexArrays(1, &self.vao);
            }
        }
    }
}
