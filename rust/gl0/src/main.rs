use euclid::default::Size2D;
use gl::types::GLvoid;
use surfman::{
    Connection, ContextAttributeFlags, ContextAttributes, GLVersion, SurfaceAccess, SurfaceType,
};

fn main() {
    let connection = Connection::new().unwrap();
    let adapter = connection.create_hardware_adapter().unwrap();
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
