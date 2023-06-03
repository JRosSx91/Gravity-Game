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

    while let Some(e) = window.next() {
        if rand::random::<f64>() < 0.01 {
            particles.push(Particle {
                x: rand::random::<f64>() * 640.0,
                y: rand::random::<f64>() * 480.0,
                vx: 0.0,
                vy: 0.0,
                size: 1.0,
            });
        }
        for particle in &mut particles {
            let dx = star.x - particle.x;
            let dy = star.y - particle.y;
            let dist2 = dx * dx + dy * dy;
            let force = 0.1 * star.size / (1.0 + dist2);
            particle.vx += force * dx;
            particle.vy += force * dy;
        }
        for particle in &mut particles {
            particle.x += particle.vx;
            particle.y += particle.vy;
        }
    }
}
