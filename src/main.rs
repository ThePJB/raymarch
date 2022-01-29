use glow::*;


fn main() {
    let mut window_x = 1024.0f32;
    let mut window_y = 768.0f32;

    unsafe {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("Raymarch")
            .with_inner_size(glutin::dpi::PhysicalSize::new(window_x, window_y));
        let window = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap();

        let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);

        gl.clear_color(0.0, 1.0, 0.0, 1.0);


        gl.debug_message_callback(|a, b, c, d, msg| {
            println!("{} {} {} {} msg: {}", a, b, c, d, msg);
        });

        let program = gl.create_program().expect("Cannot create program");

        {   // Shader stuff
            let shader_version = "#version 410";
            let shader_sources = [
                (glow::VERTEX_SHADER, std::fs::read_to_string("src/raymarch.vert").unwrap()),
                (glow::FRAGMENT_SHADER, std::fs::read_to_string("src/raymarch.frag").unwrap()),
            ];
            let mut shaders = Vec::with_capacity(shader_sources.len());
            for (shader_type, shader_source) in shader_sources.iter() {
                let shader = gl
                    .create_shader(*shader_type)
                    .expect("Cannot create shader");
                gl.shader_source(shader, &format!("{}\n{}", shader_version, shader_source));
                gl.compile_shader(shader);
                if !gl.get_shader_compile_status(shader) {
                    panic!("{}", gl.get_shader_info_log(shader));
                }
                gl.attach_shader(program, shader);
                shaders.push(shader);
            }
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }
            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }
        }

        let vao = gl.create_vertex_array().unwrap();
        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.bind_vertex_array(Some(vao));
        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 4*3, 0);
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 4*3, 4*3);
        gl.enable_vertex_attrib_array(1);

        let z = 0.0;
        let vert_buffer: [f32; 18] = [
            -1.0, -1.0, z,
            1.0, -1.0, z,
            1.0, 1.0, z,

            -1.0, -1.0, z,
            -1.0, 1.0, z,
            1.0, 1.0, z,
        ];
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytemuck::bytes_of(&vert_buffer), glow::DYNAMIC_DRAW);


        // make vao, vbo or whatever and upload this


        use glutin::event::{Event, WindowEvent};
        use glutin::event_loop::ControlFlow;



        event_loop.run(move |event, _, control_flow| {


            *control_flow = ControlFlow::Poll;

            let mut cleanup = || {
                gl.delete_program(program);
                *control_flow = ControlFlow::Exit;
            };

            match event {
                Event::LoopDestroyed |
                Event::WindowEvent {event: WindowEvent::CloseRequested, ..} |
                Event::WindowEvent {event: WindowEvent::KeyboardInput {
                    input: glutin::event::KeyboardInput { virtual_keycode: Some(glutin::event::VirtualKeyCode::Escape), ..}, ..}, ..}
                => {
                    cleanup();
                },

                Event::MainEventsCleared => {
                    // draw
                    gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
                    gl.use_program(Some(program));

                    gl.draw_arrays(glow::TRIANGLES, 0, 6);

                    window.swap_buffers().unwrap();
                }

                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::Resized(physical_size) => {
                        window.resize(*physical_size);
                        window_x = physical_size.width as f32;
                        window_y = physical_size.height as f32;
                        gl.viewport(0, 0, physical_size.width as i32, physical_size.height as i32);
                        println!("aspect ratio: {:?}", window_x / window_y);
                        // prob update aspect  ratio in camera

                    }
                    WindowEvent::CloseRequested => {
                        cleanup();
                    }
                    WindowEvent::KeyboardInput {
                        input: glutin::event::KeyboardInput { virtual_keycode: Some(virtual_code), state, .. },
                        ..
                    } => {
                        match (virtual_code, state) {
                            (glutin::event::VirtualKeyCode::Escape, _) => {
                                cleanup();
                            },
                        _ => (),
                    }},
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
