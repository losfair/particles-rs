extern crate rand;

mod imports;
mod collision;
pub mod particles;
pub mod rng;

#[cfg(feature = "glue-api")]
pub mod glue;

#[cfg(feature = "particles-api")]
pub mod particles_api;

pub static PRS_BUILD_ID: &'static str = env!("PRS_BUILD_ID");
