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
pub mod auth;
pub mod collaboration;
pub mod fsi;
pub mod molecular_dynamics;
pub mod md_postprocess;
pub mod trajectory_viewer;
pub mod atom_builder;
pub mod phase_field;
pub mod phase_field_postprocess;
pub mod phase_field_bridge;

// ============================================================================
// V1.7 - DFT 第一性原理计算 (DFT First-Principles Calculation)
// ============================================================================
pub mod dft_input;
pub mod dft_task;
pub mod dft_postprocess;
pub mod dft_bridge;

// ============================================================================
// V2.0 - 多尺度工作流编排器 (Multiscale Workflow Orchestrator)
// ============================================================================
pub mod workflow_editor;
pub mod data_transfer;
pub mod workflow_scheduler;
pub mod param_mapping;
pub mod workflow_presets;
pub mod result_comparison;
pub mod report_generator;
pub mod adaptive_precision;

// ============================================================================
// V1.8 - 多尺度数据管理与验证框架 (Multiscale Data Management & Validation)
// ============================================================================
pub mod ontology;
pub mod coordinate_mapping;
pub mod coarse_graining;
pub mod error_tracking;
pub mod benchmark;
pub mod regression_ci;
pub mod audit_log;
pub mod cross_scale_viz;

// ============================================================================
// V1.9 - 多尺度集成与高级工作流 (Multiscale Integration & Advanced Workflow)
// ============================================================================
pub mod multiscale_integration;
pub mod workflow_template;
pub mod high_throughput;
pub mod ai_recommend;
pub mod nightly_ci;
pub mod multiscale_workspace;
pub mod solver_manager;