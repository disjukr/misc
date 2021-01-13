use euclid::default::Size2D;
use gl::types::{GLchar, GLfloat, GLint, GLsizeiptr, GLubyte, GLuint, GLvoid};
use surfman::{
    Connection, ContextAttributeFlags, ContextAttributes, GLVersion, SurfaceAccess, SurfaceType,
};

static VERT_SHADER: &str = include_str!("./vert.glsl");
static FRAG_SHADER: &str = include_str!("./frag.glsl");

static VERTICES: [GLfloat; 8] = [-1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0];
static INDICES: [GLubyte; 6] = [0, 1, 2, 0, 2, 3];

fn main() {
    let connection = Connection::new().unwrap();
    let adapter = connection.create_adapter().unwrap();
    let mut device = connection.create_device(&adapter).unwrap();
    let mut context = {
        let attributes = ContextAttributes {
            version: GLVersion::new(3, 3),
            flags: ContextAttributeFlags::empty(),
        };
        let descriptor = device.create_context_descriptor(&attributes).unwrap();
        device.create_context(&descriptor, None).unwrap()
    };
    let surface = device
        .create_surface(
            &context,
            SurfaceAccess::GPUOnly,
            SurfaceType::Generic {
                size: Size2D::new(640, 480),
            },
        )
        .unwrap();
    device
        .bind_surface_to_context(&mut context, surface)
        .unwrap();
    device.make_context_current(&context).unwrap();
    gl::load_with(|symbol_name| device.get_proc_address(&context, symbol_name));
    let mut pixels: Vec<u8> = vec![0; 640 as usize * 480 as usize * 4];
    let surface_info = device.context_surface_info(&context).unwrap().unwrap();
    unsafe {
        gl::BindFramebuffer(gl::FRAMEBUFFER, surface_info.framebuffer_object);
        gl::Viewport(0, 0, 640, 480);
        gl::ClearColor(0.3, 0.4, 0.5, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(
                vertex_shader,
                1,
                &(VERT_SHADER.as_ptr() as *const GLchar),
                &(VERT_SHADER.len() as GLint),
            );
            gl::CompileShader(vertex_shader);
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(
                fragment_shader,
                1,
                &(FRAG_SHADER.as_ptr() as *const GLchar),
                &(FRAG_SHADER.len() as GLint),
            );
            gl::CompileShader(fragment_shader);
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);
        }
        {
            let mut vertex_array: GLuint = 0;
            gl::GenVertexArrays(1, &mut vertex_array);
            gl::BindVertexArray(vertex_array);
        }
        {
            let mut vertex_buffer: GLuint = 0;
            gl::GenBuffers(1, &mut vertex_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (VERTICES.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                VERTICES.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
        }
        {
            let mut index_buffer: GLuint = 0;
            gl::GenBuffers(1, &mut index_buffer);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (INDICES.len() * std::mem::size_of::<GLubyte>()) as GLsizeiptr,
                INDICES.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
        }
        {
            let pos_attr_index = 0;
            gl::EnableVertexAttribArray(pos_attr_index);
            gl::VertexAttribPointer(pos_attr_index, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        }
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_BYTE, std::ptr::null());
        gl::Flush();
        gl::ReadPixels(
            0,
            0,
            640,
            480,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            pixels.as_mut_ptr() as *mut GLvoid,
        );
    }
    device.destroy_context(&mut context).unwrap();
    image::save_buffer("test.png", &pixels, 640, 480, image::ColorType::Rgba8).unwrap();
    std::process::exit(0);
}
