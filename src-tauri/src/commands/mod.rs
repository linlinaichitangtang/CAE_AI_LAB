pub mod file;
pub mod project;
pub mod settings;
pub mod cae_api;
pub mod input_gen;
pub mod output_parser;
pub mod postprocess;
pub mod solver;

pub mod ai;
pub mod materials;
pub mod parametric;
#[allow(dead_code)]
pub mod modal;
#[allow(dead_code)]
pub mod optimization;
pub mod optimization_commands;
pub mod topology_optimization; // SIMP + OC method with STL export
#[allow(dead_code)]
pub mod coupling;
#[allow(dead_code)]
pub mod thermal_coupling;
pub mod contact;
#[allow(dead_code)]
pub mod fatigue;
pub mod transient_dynamics;
pub mod cfd;
pub mod explicit_dynamics;
pub mod electronics;
pub mod biomechanics;
pub mod code_exec;
pub mod step_import;