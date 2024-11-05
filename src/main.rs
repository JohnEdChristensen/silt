use wgpu_boiler::{world::World, State};
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};
fn main() {
    pollster::block_on(run());
}

//const VERTICES: &[Vertex] = &[
//    Vertex {
//        position: [-0.0868241, 0.49240386, 0.0],
//        tex_coords: [0.4131759, 0.00759614],
//    }, // A
//    Vertex {
//        position: [-0.49513406, 0.06958647, 0.0],
//        tex_coords: [0.0048659444, 0.43041354],
//    }, // B
//    Vertex {
//        position: [-0.21918549, -0.44939706, 0.0],
//        tex_coords: [0.28081453, 0.949397],
//    }, // C
//    Vertex {
//        position: [0.35966998, -0.3473291, 0.0],
//        tex_coords: [0.85967, 0.84732914],
//    }, // D
//    Vertex {
//        position: [0.44147372, 0.2347359, 0.0],
//        tex_coords: [0.9414737, 0.2652641],
//    }, // E
//];
//
//const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4, /* padding */ 0];

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
async fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    //#[cfg(target_arch = "wasm32")]
    //{
    //    // Winit prevents sizing with CSS, so we have to set
    //    // the size manually when on web.
    //    use winit::dpi::PhysicalSize;
    //    let _ = window.request_inner_size(PhysicalSize::new(450, 400));
    //
    //    use winit::platform::web::WindowExtWebSys;
    //    web_sys::window()
    //        .and_then(|win| win.document())
    //        .and_then(|doc| {
    //            let dst = doc.get_element_by_id("wasm-example")?;
    //            let canvas = web_sys::Element::from(window.canvas()?);
    //            dst.append_child(&canvas).ok()?;
    //            Some(())
    //        })
    //        .expect("Couldn't append canvas to document body.");
    //}

    // State::new uses async code, so we're going to wait for it to finish
    let mut state = State::new(&window).await;

    let world = World::default();
    let (verts, indices) = world.height_map.create_triangles();
    state.update_verts(&verts, &indices);
    let mut surface_configured = false;

    event_loop
        .run(move |event, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == state.window().id() => {
                    if !state.input(event) {
                        match event {
                            WindowEvent::CloseRequested
                            | WindowEvent::KeyboardInput {
                                event:
                                    KeyEvent {
                                        state: ElementState::Pressed,
                                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => control_flow.exit(),
                            WindowEvent::Resized(physical_size) => {
                                surface_configured = true;
                                state.resize(*physical_size);
                            }
                            WindowEvent::RedrawRequested => {
                                // This tells winit that we want another frame after this one
                                state.window().request_redraw();

                                if !surface_configured {
                                    return;
                                }

                                state.update();
                                match state.render() {
                                    Ok(_) => {}
                                    // Reconfigure the surface if it's lost or outdated
                                    Err(
                                        wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                                    ) => state.resize(state.size),
                                    // The system is out of memory, we should probably quit
                                    Err(wgpu::SurfaceError::OutOfMemory) => {
                                        log::error!("OutOfMemory");
                                        control_flow.exit();
                                    }

                                    // This happens when the a frame takes too long to present
                                    Err(wgpu::SurfaceError::Timeout) => {
                                        log::warn!("Surface timeout")
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        })
        .unwrap();
}
