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
    color: [f32; 4],
}

const G: f64 = 6.67430e-11;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Star Formation", [1248, 1024])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut star = Particle {
        x: 320.0,
        y: 240.0,
        speed_x: 0.0,
        speed_y: 0.0,
        mass: 10.0,
        particle_type: ParticleType::Hydrogen,
        color: [1.0, 1.0, 0.0, 1.0], // Amarillo para la estrella
    };

    let mut particles: Vec<Particle> = (0..1000)
        .map(|_| {
            let particle_type = if rand::random::<f64>() < 0.75 {
                ParticleType::Hydrogen
            } else {
                ParticleType::Helium
            };
            let color = match particle_type {
                ParticleType::Hydrogen => [0.0, 1.0, 0.0, 1.0], // Verde para el hidrógeno
                ParticleType::Helium => [0.0, 0.0, 1.0, 1.0],   // Azul para el helio
            };
            Particle {
                x: rand::random::<f64>() * 800.0,
                y: rand::random::<f64>() * 800.0,
                speed_x: rand::random::<f64>() - 0.5,
                speed_y: rand::random::<f64>() - 0.5,
                mass: match particle_type {
                    ParticleType::Hydrogen => 1.0,
                    ParticleType::Helium => 4.0,
                },
                particle_type,
                color,
            }
        })
        .collect();

    while let Some(e) = window.next() {
        if rand::random::<f64>() < 0.01 {
            particles.push(new_particle());
        }

        // Actualiza las partículas
        star = update_particles(&mut particles);

        // Elimina las partículas que se encuentran dentro de la estrella
        particles.retain(|particle| {
            let dx = star.x - particle.x;
            let dy = star.y - particle.y;
            let distance = (dx.powi(2) + dy.powi(2)).sqrt();
            distance >= star.mass / 2.0
        });
        window.draw_2d(&e, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            // Draw the star
            ellipse(
                [1.0, 1.0, 0.0, 1.0],
                [
                    star.x - star.mass,
                    star.y - star.mass,
                    star.mass * 2.0,
                    star.mass * 2.0,
                ],
                c.transform,
                g,
            );

            // Draw the particles
            for particle in &particles {
                ellipse(
                    particle.color,
                    [particle.x, particle.y, particle.mass, particle.mass],
                    c.transform,
                    g,
                );
            }
        });
    }
}
fn gravity(star: &Particle, particle: &mut Particle) {
    let dx = star.x - particle.x;
    let dy = star.y - particle.y;
    let distance = (dx.powi(2) + dy.powi(2)).sqrt();
    let force = G * star.mass * particle.mass / distance.powi(2);
    let force_x = force * dx / distance;
    let force_y = force * dy / distance;

    particle.speed_x += force_x / particle.mass;
    particle.speed_y += force_y / particle.mass;
}

fn update_particles(particles: &mut Vec<Particle>) -> Particle {
    let mut star = Particle {
        x: 320.0,
        y: 240.0,
        speed_x: 0.0,
        speed_y: 0.0,
        mass: 10.0,
        particle_type: ParticleType::Hydrogen,
        color: [1.0, 1.0, 0.0, 1.0],
    };

    for particle in particles.iter_mut() {
        gravity(&star, particle);
        particle.x += particle.speed_x;
        particle.y += particle.speed_y;
        let dx = star.x - particle.x;
        let dy = star.y - particle.y;
        let distance = (dx.powi(2) + dy.powi(2)).sqrt();
        if distance < star.mass / 2.0 {
            star.mass += particle.mass;
            *particle = new_particle();
        }
    }

    star
}

fn new_particle() -> Particle {
    let particle_type: ParticleType = if rand::random::<f64>() < 0.75 {
        ParticleType::Hydrogen
    } else {
        ParticleType::Helium
    };
    let color: [f32; 4] = match particle_type {
        ParticleType::Hydrogen => [0.0, 1.0, 0.0, 1.0], // Verde para el hidrógeno
        ParticleType::Helium => [0.0, 0.0, 1.0, 1.0],   // Azul para el helio
    };
    Particle {
        x: rand::random::<f64>() * 800.0,
        y: rand::random::<f64>() * 800.0,
        speed_x: rand::random::<f64>() - 0.5,
        speed_y: rand::random::<f64>() - 0.5,
        mass: match particle_type {
            ParticleType::Hydrogen => 1.0,
            ParticleType::Helium => 4.0,
        },
        particle_type,
        color,
    }
}
