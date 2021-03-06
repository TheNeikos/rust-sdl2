extern crate sdl2;

pub fn main() {
    sdl2::init(sdl2::INIT_VIDEO);

    let window = match sdl2::video::Window::new("rust-sdl2 demo: Video", sdl2::video::WindowPos::PosCentered, sdl2::video::WindowPos::PosCentered, 800, 600, sdl2::video::OPENGL) {
        Ok(window) => window,
        Err(err) => panic!(format!("failed to create window: {}", err))
    };

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!(format!("failed to create renderer: {}", err))
    };

    let _ = renderer.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
    let _ = renderer.clear();
    renderer.present();

    loop {
        match sdl2::event::poll_event() {
            sdl2::event::Event::Quit(_) => break,
            sdl2::event::Event::KeyDown(_, _, key, _, _, _) => {
                if key == sdl2::keycode::KeyCode::Escape {
                    break
                }
            },
            _ => {}
        }
    }

    sdl2::quit();
}
