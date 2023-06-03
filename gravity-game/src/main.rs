use piston_window::*;

struct Particle {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    size: f64,
}
fn main() {
    let mut window: PistonWindow = WindowSettings::new("Star Formation", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut star = Particle {
        x: 320.0,
        y: 240.0,
        vx: 0.0,
        vy: 0.0,
        size: 10.0,
    };

    let mut particles = vec![];
}
