extern crate glutin;

pub struct GameBuilder {
    window: WindowBuilder,
}

impl GameBuilder {
    pub fn new() -> GameBuilder {
        GameBuilder {
            window: WindowBuilder::new(),
        }
    }

    pub fn with_window(mut self, window: WindowBuilder) -> GameBuilder {
        self.window = window;
        self
    }

    pub fn build(self) -> Game {
        Game {
            window: self.window.build(),
        }
    }
}

pub struct WindowBuilder {
    width: u32,
    height: u32,
    title: String,
}

impl WindowBuilder {
    pub fn new() -> WindowBuilder {
        WindowBuilder {
            width: 800,
            height: 600,
            title: "Sword".to_string(),
        }
    }

    pub fn with_title<S: Into<String>>(mut self, title: S) -> WindowBuilder {
        self.title = title.into();
        self
    }

    pub fn with_width(mut self, width: u32) -> WindowBuilder {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: u32) -> WindowBuilder {
        self.height = height;
        self
    }

    pub fn build(self) -> Window {
        Window {
            width: self.width,
            height: self.height,
            title: self.title,
        }
    }
}

pub struct Window {
    width: u32,
    height: u32,
    title: String,
}

pub struct Game {
    window: Window,
}

pub struct Input {
    pub dt: f32,
}

impl Input {
    pub fn new() -> Input {
        Input {
            dt: 0.016,
        }
    }
}

impl Game {
    pub fn run<S, U, R>(self, mut state: S, update: U, render: R) where U: Fn(&Input, &mut S), R: Fn(&S) {
        let window = glutin::WindowBuilder::new()
            .with_title(self.window.title)
            .with_dimensions(self.window.width, self.window.height)
            .build().unwrap();

        let input = Input::new();

        'gameloop: loop {
            for event in window.poll_events() {
                match event {
                    glutin::Event::Closed => break 'gameloop,
                    _ => ()
                }
            }

            update(&input, &mut state);
            render(&state);

            let _ = window.swap_buffers();

            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }
}
