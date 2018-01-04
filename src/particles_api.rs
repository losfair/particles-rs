use particles;
use particles::{ParticlesState, ParticlesConfig};

#[no_mangle]
pub extern "C" fn particles_config_create(
    height: usize,
    width: usize,
    n_particles: usize,
    max_edge_len: f64,
    velocity_factor: f64,
    particle_radius: f64
) -> *mut ParticlesConfig {
    let config = Box::new(ParticlesConfig {
        height: height,
        width: width,
        n_particles: n_particles,
        max_edge_len: max_edge_len,
        velocity_factor: velocity_factor,
        particle_radius: particle_radius,
        collision_enabled: false,
        edges_enabled: true
    });
    Box::into_raw(config)
}

#[no_mangle]
pub unsafe extern "C" fn particles_config_destroy(config: *mut ParticlesConfig) {
    Box::from_raw(config);
}

#[no_mangle]
pub unsafe extern "C" fn particles_config_enable_collision(config: &mut ParticlesConfig) {
    config.collision_enabled = true;
}

#[no_mangle]
pub unsafe extern "C" fn particles_config_disable_collision(config: &mut ParticlesConfig) {
    config.collision_enabled = false;
}

#[no_mangle]
pub unsafe extern "C" fn particles_config_enable_edges(config: &mut ParticlesConfig) {
    config.edges_enabled = true;
}

#[no_mangle]
pub unsafe extern "C" fn particles_config_disable_edges(config: &mut ParticlesConfig) {
    config.edges_enabled = false;
}

#[no_mangle]
pub extern "C" fn particles_state_create(
    config: *mut ParticlesConfig
) -> *mut ParticlesState {
    let config = unsafe { Box::from_raw(config) };
    let state = Box::new(ParticlesState::new(*config));
    Box::into_raw(state)
}

#[no_mangle]
pub unsafe extern "C" fn particles_state_destroy(
    state: *mut ParticlesState
) {
    Box::from_raw(state);
}

#[no_mangle]
pub unsafe extern "C" fn particles_state_borrow_config(
    state: &mut ParticlesState
) -> &mut ParticlesConfig {
    state.get_config_mut()
}

#[no_mangle]
pub extern "C" fn particles_state_set_size(
    state: &mut ParticlesState,
    height: usize,
    width: usize
) {
    state.set_height(height);
    state.set_width(width);
}

#[no_mangle]
pub extern "C" fn particles_state_update(state: &mut ParticlesState) {
    state.update_all();
}

#[no_mangle]
pub extern "C" fn particles_state_render(
    state: &mut ParticlesState
) -> *mut particles::RenderedParticles {
    Box::into_raw(Box::new(state.render()))
}

#[no_mangle]
pub unsafe extern "C" fn particles_rendered_destroy(
    rendered: *mut particles::RenderedParticles
) {
    Box::from_raw(rendered);
}

#[no_mangle]
pub extern "C" fn particles_rendered_get_n_nodes(rendered: &particles::RenderedParticles) -> usize {
    rendered.nodes.len()
}

#[no_mangle]
pub extern "C" fn particles_rendered_get_n_edges(rendered: &particles::RenderedParticles) -> usize {
    rendered.edges.len()
}

#[no_mangle]
pub extern "C" fn particles_rendered_get_nodes_ref(
    rendered: &particles::RenderedParticles
) -> *const particles::RenderedNode {
    if rendered.nodes.len() == 0 {
        ::std::ptr::null()
    } else {
        &rendered.nodes[0]
    }
}

#[no_mangle]
pub extern "C" fn particles_rendered_get_edges_ref(
    rendered: &particles::RenderedParticles
) -> *const particles::RenderedEdge {
    if rendered.edges.len() == 0 {
        ::std::ptr::null()
    } else {
        &rendered.edges[0]
    }
}
