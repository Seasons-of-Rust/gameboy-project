use sdl2;

const GAMEBOY_WIDTH: u32 = 160;
const GAMEBOY_HEIGHT: u32 = 144;

pub struct Screen {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Screen {
    pub fn new(title: &str, scale: u32) -> Screen {
        let context = sdl2::init().unwrap();
        log::debug!("SDL2 init");

        let subsystem = context.video().unwrap();
        let window = subsystem
            .window(title, GAMEBOY_WIDTH * scale, GAMEBOY_HEIGHT * scale)
            .position_centered()
            .build()
            .unwrap();
        log::debug!("Window created");

        Screen { canvas: window.into_canvas().build().unwrap() }
    }

    pub fn render(&mut self) {
        self.canvas.present();
    }
}
