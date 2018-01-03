use imports;
use std::collections::BTreeSet;

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
    right: RenderedNode
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
    pub velocity_factor: f64
}

#[derive(Clone, Debug)]
pub struct Particle {
    x: f64, // vertical
    y: f64, // horizontal
    velocity_x: f64,
    velocity_y: f64
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

    pub fn set_height(&mut self, height: usize) {
        self.config.height = height;
    }

    pub fn set_width(&mut self, width: usize) {
        self.config.width = width;
    }

    pub fn update_all(&mut self) {
        self.update_particles();
        self.update_edges();
    }

    pub fn update_particles(&mut self) {
        for particle in &mut self.particles {
            particle.x += particle.velocity_x * self.config.velocity_factor;
            particle.y += particle.velocity_y * self.config.velocity_factor;
            if particle.out_of_view(&self.config) {
                *particle = Particle::random(&self.config);
            }
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
                    if imports::rand01() > 0.5 {
                        edges.insert(Edge::new(left, right));
                    }
                }
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
            edges.push(RenderedEdge {
                left: RenderedNode {
                    x: self.particles[edge.left].x,
                    y: self.particles[edge.left].y
                },
                right: RenderedNode {
                    x: self.particles[edge.right].x,
                    y: self.particles[edge.right].y
                }
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
        let x = imports::rand01() * (config.height as f64);
        let y = imports::rand01() * (config.width as f64);
        Particle {
            x: x,
            y: y,
            velocity_x: (imports::rand01() - 0.5) * 2.0, // (-1, 1)
            velocity_y: (imports::rand01() - 0.5) * 2.0 // (-1, 1)
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