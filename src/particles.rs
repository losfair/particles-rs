use rng;
use std::collections::BTreeSet;
use collision;

#[derive(Clone, Debug)]
pub struct ParticlesState {
    config: ParticlesConfig,
    particles: Vec<Particle>,
    edges: BTreeSet<Edge>
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct RenderedNode {
    x: f64,
    y: f64
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct RenderedEdge {
    left: RenderedNode,
    right: RenderedNode,
    opacity: f64
}

#[derive(Clone, Debug)]
pub struct RenderedParticles {
    pub nodes: Vec<RenderedNode>,
    pub edges: Vec<RenderedEdge>
}

#[derive(Clone, Debug)]
pub struct ParticlesConfig {
    pub height: usize,
    pub width: usize,
    pub n_particles: usize,
    pub max_edge_len: f64,
    pub velocity_factor: f64,
    pub particle_radius: f64,
    pub collision_enabled: bool,
    pub edges_enabled: bool,
    pub magnetic_strength: f64,
    pub electric_strength: (f64, f64)
}

#[derive(Clone, Debug)]
pub struct Particle {
    x: f64, // vertical
    y: f64, // horizontal
    velocity_x: f64,
    velocity_y: f64,
    charge: f64
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Edge {
    // left < right must hold.
    left: usize, // particle id
    right: usize
}

impl ParticlesState {
    pub fn new(config: ParticlesConfig) -> ParticlesState {
        let mut initial_particles = Vec::with_capacity(config.n_particles);
        for _ in 0..config.n_particles {
            initial_particles.push(Particle::random(&config));
        }
        ParticlesState {
            config: config,
            particles: initial_particles,
            edges: BTreeSet::new()
        }
    }

    pub fn get_config(&self) -> &ParticlesConfig {
        &self.config
    }

    pub fn get_config_mut(&mut self) -> &mut ParticlesConfig {
        &mut self.config
    }

    pub fn set_height(&mut self, height: usize) {
        self.config.height = height;
    }

    pub fn set_width(&mut self, width: usize) {
        self.config.width = width;
    }

    pub fn update_all(&mut self) {
        if self.config.magnetic_strength != 0.0 {
            self.evaluate_magnetic_field();
        }
        if self.config.electric_strength.0 != 0.0 || self.config.electric_strength.1 != 0.0 {
            self.evaluate_electric_field();
        }

        if self.config.collision_enabled {
            self.evaluate_collisions();
        }

        self.update_particles();

        if self.config.edges_enabled {
            self.update_edges();
        }
    }

    pub fn update_particles(&mut self) {
        let mut out_of_view_particle_ids: BTreeSet<usize> = BTreeSet::new();

        for i in 0..self.particles.len() {
            let particle = &mut self.particles[i];
            particle.x += particle.velocity_x * self.config.velocity_factor;
            particle.y += particle.velocity_y * self.config.velocity_factor;
            if particle.out_of_view(&self.config) {
                out_of_view_particle_ids.insert(i);
            }
        }

        for id in &out_of_view_particle_ids {
            let mut new_particle = Particle::random(&self.config);
            let position_particle_id = (rng::open01() * (self.particles.len() as f64)) as usize;

            if out_of_view_particle_ids.get(&position_particle_id).is_none() {
                let position_particle = &self.particles[position_particle_id];
                new_particle.x = position_particle.x;
                new_particle.y = position_particle.y;
            }

            self.particles[*id] = new_particle;
        }
    }

    pub fn update_edges(&mut self) {
        let edges = &mut self.edges;
        {
            let mut edges_to_remove = Vec::new();
            for it in edges.iter() {
                if self.particles[it.left].euclidean_distance(&self.particles[it.right]) > self.config.max_edge_len {
                    // the Edge struct is light so clone it
                    edges_to_remove.push(it.clone());
                }
            }
            for e in edges_to_remove {
                edges.remove(&e);
            }
        }

        for left in 0..self.particles.len() {
            for right in (left + 1)..self.particles.len() {
                let left_particle = &self.particles[left];
                let right_particle = &self.particles[right];
                if left_particle.euclidean_distance(right_particle) < self.config.max_edge_len {
                    if rng::open01() > 0.5 {
                        edges.insert(Edge::new(left, right));
                    }
                }
            }
        }
    }

    pub fn evaluate_magnetic_field(&mut self) {
        for particle in &mut self.particles {
            let (nv_x, nv_y) = (particle.velocity_y, -particle.velocity_x);

            let dt_x = particle.charge * nv_x * self.config.magnetic_strength;
            let dt_y = particle.charge * nv_y * self.config.magnetic_strength;

            particle.velocity_x += dt_x;
            particle.velocity_y += dt_y;
        }
    }

    pub fn evaluate_electric_field(&mut self) {
        for particle in &mut self.particles {
            let dt_x = particle.charge * self.config.electric_strength.0;
            let dt_y = particle.charge * self.config.electric_strength.1;

            particle.velocity_x += dt_x;
            particle.velocity_y += dt_y;
        }
    }

    pub fn evaluate_collisions(&mut self) {
        for i in 0..self.particles.len() {
            for j in (i + 1)..self.particles.len() {
                if self.particles[i].euclidean_distance(&self.particles[j]) > self.config.particle_radius * 2.0 {
                    continue;
                }

                let (
                    (left_vx, left_vy),
                    (right_vx, right_vy)
                ) = {
                    let left = &self.particles[i];
                    let right = &self.particles[j];

                    collision::collision_2d(
                        1.0,
                        1.0,
                        1.0,
                        (left.x, left.y),
                        (right.x, right.y),
                        (left.velocity_x, left.velocity_y),
                        (right.velocity_x, right.velocity_y)
                    )
                };
                self.particles[i].velocity_x = left_vx;
                self.particles[i].velocity_y = left_vy;
                self.particles[j].velocity_x = right_vx;
                self.particles[j].velocity_y = right_vy;
            }
        }
    }

    pub fn render(&self) -> RenderedParticles {
        let mut nodes: Vec<RenderedNode> = Vec::with_capacity(self.particles.len());
        let mut edges: Vec<RenderedEdge> = Vec::with_capacity(self.edges.len());

        for particle in &self.particles {
            nodes.push(RenderedNode {
                x: particle.x,
                y: particle.y
            });
        }
        for edge in &self.edges {
            let mut opacity = 1.0 - self.particles[edge.left].euclidean_distance(&self.particles[edge.right]) / self.config.max_edge_len;
            if opacity < 0.0 {
                opacity = 0.0;
            }

            edges.push(RenderedEdge {
                left: RenderedNode {
                    x: self.particles[edge.left].x,
                    y: self.particles[edge.left].y
                },
                right: RenderedNode {
                    x: self.particles[edge.right].x,
                    y: self.particles[edge.right].y
                },
                opacity: opacity
            });
        }

        RenderedParticles {
            nodes: nodes,
            edges: edges
        }
    }
}

impl Particle {
    pub fn random(config: &ParticlesConfig) -> Particle {
        let x = rng::open01() * (config.height as f64);
        let y = rng::open01() * (config.width as f64);
        Particle {
            x: x,
            y: y,
            velocity_x: (rng::open01() - 0.5) * 2.0, // (-1, 1)
            velocity_y: (rng::open01() - 0.5) * 2.0, // (-1, 1)
            charge: (rng::open01() - 0.5) * 2.0 // (-1, 1)
        }
    }

    pub fn out_of_view(&self, config: &ParticlesConfig) -> bool {
        if self.x < 0.0 - config.max_edge_len || self.x > config.height as f64 + config.max_edge_len {
            true
        } else if self.y < 0.0 - config.max_edge_len || self.y > config.width as f64 + config.max_edge_len {
            true
        } else {
            false
        }
    }

    pub fn euclidean_distance(&self, other: &Particle) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl Edge {
    pub fn new(left: usize, right: usize) -> Edge {
        if left == right {
            panic!("Edge::new: left == right");
        } else if left > right {
            Edge {
                left: right,
                right: left
            }
        } else {
            Edge {
                left: left,
                right: right
            }
        }
    }
}
