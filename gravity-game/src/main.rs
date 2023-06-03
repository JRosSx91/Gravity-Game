use piston_window::*;
use rand::*;

enum ParticleType {
    Hydrogen,
    Helium,
}
struct Particle {
    x: f64,
    y: f64,
    speed_x: f64,
    speed_y: f64,
    mass: f64,
    particle_type: ParticleType,
    color: [f64; 4],
}
fn main() {
    let mut window: PistonWindow = WindowSettings::new("Star Formation", [1248, 1024])
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
        particles.retain(|particle| {
            if (star.x - particle.x).powi(2) + (star.y - particle.y).powi(2) < star.size.powi(2) {
                star.size += particle.size;
                false
            } else {
                true
            }
        });
        window.draw_2d(&e, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            // Draw the star
            ellipse(
                [1.0, 1.0, 0.0, 1.0],
                [
                    star.x - star.size,
                    star.y - star.size,
                    star.size * 2.0,
                    star.size * 2.0,
                ],
                c.transform,
                g,
            );

            // Draw the particles
            for particle in &particles {
                ellipse(
                    [0.0, 0.0, 0.0, 1.0],
                    [
                        particle.x - particle.size,
                        particle.y - particle.size,
                        particle.size * 2.0,
                        particle.size * 2.0,
                    ],
                    c.transform,
                    g,
                );
            }
        });
    }
}
