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
pub mod modal;
pub mod optimization;
pub mod optimization_commands;
pub mod topology_optimization; // SIMP + OC method with STL export
pub mod coupling;
pub mod thermal_coupling;
pub mod contact;
pub mod fatigue;
pub mod transient_dynamics;
pub mod cfd;
pub mod explicit_dynamics;
pub mod electronics;
pub mod biomechanics;
pub mod code_exec;