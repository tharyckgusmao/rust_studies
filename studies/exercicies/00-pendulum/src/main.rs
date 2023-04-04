// reference manual do codigo
use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

struct Pendulum {
    origin: vector::Vector,
    position: vector::Vector,
    angle: f32,
    angular_velocity: f32,
    angular_acceleration: f32,

    r: f32, //radius
    g: f32, // gravity
    f: f32, //friction
}

impl Pendulum {
    fn new(x: f32, y: f32, r: f32) -> Pendulum {
        Pendulum {
            origin: vector::Vector::new(x, y),
            position: vector::Vector::new(0.0, 0.0),
            angle: 1.0,
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            r,
            g: 0.2,
            f: 0.98,
        }
    }
    fn update(&mut self) {
        self.angular_acceleration = (-1.0 * self.g * self.angle.sin()) / self.r;
        self.angular_velocity += self.angular_acceleration;
        self.angle += self.angular_velocity;
        self.angular_velocity *= self.f;

        self.position
            .set(self.r * self.angle.sin(), self.r * self.angle.cos());

        self.position.add(&self.origin);
    }
    fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_line(
            (self.origin.x, self.origin.y),
            (self.position.x, self.position.y),
            3.0,
            Color::RED,
        );

        graphics.draw_circle((self.position.x, self.position.y), 15.0, Color::RED);
    }
    fn restart(&mut self) {
        self.angular_velocity = 0.0;
        self.angle = 1.0;
        self.angular_velocity = 0.0;

        self.position.set(0.0, 0.0);

        self.position.add(&self.origin);
    }
}

mod vector {
    pub struct Vector {
        pub x: f32,
        pub y: f32,
    }

    impl Vector {
        pub fn new(x: f32, y: f32) -> Vector {
            Vector { x, y }
        }
        pub fn add(&mut self, other: &Vector) -> &Vector {
            self.x += other.x;
            self.y += other.y;
            self
        }
        pub fn set(&mut self, x: f32, y: f32) -> &Vector {
            self.x = x;
            self.y = y;
            self
        }
    }
}

struct MyWindowHandler {
    p: Vec<Pendulum>,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));

        for pendulum in &mut self.p {
            pendulum.update();
            pendulum.draw(graphics);
        }

        helper.request_redraw();
    }
}

fn main() {
    let window = Window::new_centered("Pendulum Test", (800, 480)).unwrap();
    let win = MyWindowHandler {
        p: vec![
            Pendulum::new(400.0, 0.0, 120.0),
            Pendulum::new(400.0, 0.0, 30.0),
        ],
    };
    window.run_loop(win);
}
