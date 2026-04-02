use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tauri::Manager;

mod commands;
pub use commands::{cae_api, file, input_gen, output_parser, postprocess, project, settings, ai, materials, parametric, transient_dynamics, contact, cfd, topology_optimization, optimization_commands, electronics, biomechanics, explicit_dynamics, code_exec, step_import, auth};
pub mod solver;
mod db;
mod models;

use db::Database;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    tracing::info!("Starting CAELab application");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            tracing::info!("Application setup complete");
            
            // Initialize database
            let app_data_dir = app.path().app_data_dir().expect("Failed to get app data dir");
            std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");
            
            let db_path = app_data_dir.join("caelab.db");
            tracing::info!("Database path: {:?}", db_path);
            
            let database = Database::new(db_path).expect("Failed to initialize database");
            app.manage(database);

            // Initialize global cancel flag for solver
            let cancel_flag = Arc::new(AtomicBool::new(false));
            app.manage(commands::cae_api::GlobalCancelFlag(cancel_flag));
            
            tracing::info!("Database initialized successfully");
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Project commands
            commands::project::create_project,
            commands::project::list_projects,
            commands::project::get_project,
            commands::project::update_project,
            commands::project::delete_project,
            // File commands
            commands::file::create_file,
            commands::file::list_files,
            commands::file::get_file,
            commands::file::update_file,
            commands::file::delete_file,
            commands::file::read_file_content,
            commands::file::write_file_content,
            // Version history commands
            commands::file::save_note_version,
            commands::file::get_note_versions,
            commands::file::get_note_version,
            commands::file::restore_note_version,
            commands::file::delete_note_version,
            // Note link commands
            commands::file::create_note_link,
            commands::file::get_note_links,
            commands::file::get_note_backlinks,
            commands::file::delete_note_link,
            // Search commands
            commands::file::search_notes,
            // Embed record commands
            commands::file::add_embed_record,
            commands::file::get_embed_records,
            commands::file::delete_embed_record,
            commands::file::update_embed_record,
            // Category commands
            commands::file::update_file_category,
            commands::file::get_file_categories,
            // Settings commands
            commands::settings::save_settings,
            commands::settings::get_settings,
            commands::settings::delete_settings,
            // CAE API commands
            commands::cae_api::check_solver,
            commands::cae_api::generate_input,
            commands::cae_api::run_solver,
            commands::cae_api::run_solver_with_progress,
            commands::cae_api::cancel_solver,
            commands::cae_api::parse_results,
            commands::cae_api::get_mesh_data,
            commands::cae_api::create_beam_model,
            commands::cae_api::get_color_map,
            commands::cae_api::generate_2d_mesh,
            commands::cae_api::generate_3d_mesh,
            commands::cae_api::generate_structured_mesh,
            commands::cae_api::export_mesh_to_inp,
            commands::cae_api::create_fixed_bc,
            commands::cae_api::create_custom_fixed_bc,
            commands::cae_api::create_point_load,
            commands::cae_api::create_vector_point_load,
            commands::cae_api::create_pressure_load,
            commands::cae_api::create_traction_load,
            commands::cae_api::create_heat_flux_load,
            commands::cae_api::create_bc_container,
            commands::cae_api::generate_bc_inp,
            commands::cae_api::get_bc_types,
            commands::cae_api::get_load_directions,
            commands::cae_api::get_uniform_load_types,
            commands::cae_api::find_nodes_at_face,
            commands::cae_api::create_cantilever_fixed_bc,
            commands::cae_api::create_cantilever_point_load,
            commands::cae_api::generate_complete_inp,
            // AI commands
            commands::ai::ai_chat,
            commands::ai::ai_chat_stream,
            commands::ai::test_ai_connection,
            // Material commands
            commands::materials::list_materials,
            commands::materials::get_material,
            commands::materials::create_material,
            commands::materials::update_material,
            commands::materials::delete_material,
            commands::materials::get_builtin_material_names,
            // Parametric analysis commands
            commands::parametric::run_parametric_scan,
            commands::parametric::run_parametric_scan_async,
            // Transient dynamics commands
            commands::transient_dynamics::get_tutorial_examples,
            commands::transient_dynamics::generate_transient_inp_file,
            commands::transient_dynamics::run_transient_simulation,
            commands::transient_dynamics::calculate_rayleigh_coefficients,
            commands::transient_dynamics::get_load_curve_template,
            // Thermal coupling commands
            commands::thermal_coupling::generate_thermal_coupling_inp,
            commands::thermal_coupling::get_thermal_coupling_templates,
            commands::thermal_coupling::generate_sequential_coupling_inp_files,
            commands::thermal_coupling::parse_thermal_result_file,
            commands::thermal_coupling::get_face_nodes,
            // Contact analysis commands
            commands::contact::create_contact_config,
            commands::contact::generate_contact_inp,
            commands::contact::diagnose_contact_pairs,
            commands::contact::get_contact_template_inp,
            commands::contact::generate_surface_def,
            commands::contact::get_contact_algorithm_recommendations,
            commands::contact::get_convergence_suggestions,
            // CFD commands
            commands::cfd::generate_openfoam_case,
            commands::cfd::parse_openfoam_log,
            commands::cfd::download_openfoam_case,
            commands::cfd::import_cfd_geometry,
            commands::cfd::generate_cfd_report,
            commands::cfd::generate_cfd_sample_results,
            // Topology optimization commands
            commands::optimization_commands::run_topology_optimization_full,
            commands::optimization_commands::run_topology_optimization,
            commands::optimization_commands::run_shape_optimization,
            commands::optimization_commands::run_size_optimization,
            commands::optimization_commands::export_topology_to_stl,
            commands::optimization_commands::export_stl_ascii,
            commands::optimization_commands::export_optimization_results,
            commands::optimization_commands::load_optimization_template,
            commands::optimization_commands::get_optimization_templates,
            commands::optimization_commands::get_topology_tutorial_examples,
            commands::optimization_commands::get_iteration_density_field,
            commands::optimization_commands::reset_optimizer,
            commands::optimization_commands::calculate_simp_stiffness,
            commands::optimization_commands::calculate_oc_sensitivity,
            commands::optimization_commands::generate_optimization_inp_file,
            commands::optimization_commands::generate_optimization_inp,
            // Fatigue analysis commands
            commands::fatigue::fatigue_analysis,
            commands::fatigue::rainflow_analysis,
            commands::fatigue::fit_sn_curve,
            commands::fatigue::calculate_node_damage,
            commands::fatigue::generate_fatigue_inp_file,
            commands::fatigue::run_fatigue_simulation,
            commands::fatigue::get_fatigue_templates,
            // Electronics analysis commands
            commands::electronics::get_material_library,
            commands::electronics::get_analysis_templates,
            commands::electronics::run_electronics_analysis,
            // Biomechanics analysis commands
            commands::biomechanics::get_bio_material_library,
            commands::biomechanics::get_bio_templates,
            commands::biomechanics::run_biomechanics_analysis,
            commands::biomechanics::import_stl_geometry,
            // Explicit dynamics commands
            commands::explicit_dynamics::get_explicit_dynamics_templates,
            commands::explicit_dynamics::get_explicit_dynamics_template,
            commands::explicit_dynamics::generate_explicit_dynamics_inp_cmd,
            commands::explicit_dynamics::generate_explicit_animation_inp_cmd,
            commands::explicit_dynamics::calculate_critical_time_step,
            commands::explicit_dynamics::calculate_mass_scaling_suggestion,
            // Buckling analysis commands
            commands::cae_api::generate_buckling_inp,
            commands::cae_api::generate_nonlinear_buckling_inp,
            commands::cae_api::run_buckling_solver,
            commands::cae_api::parse_buckling_result,
            commands::cae_api::calculate_buckling_safety,
            commands::cae_api::generate_buckling_inp_compat,
            // Contact analysis commands (cae_api)
            commands::cae_api::generate_inp_with_contact,
            commands::cae_api::get_contact_settings,
            commands::cae_api::validate_contact_surfaces,
            commands::cae_api::get_contact_result_fields,
            // Frequency response commands
            commands::cae_api::generate_frequency_response_inp,
            commands::cae_api::run_frequency_response_solver,
            commands::cae_api::parse_frequency_response_result,
            // Thermal coupling commands (cae_api)
            commands::cae_api::run_sequential_coupling,
            commands::cae_api::run_fully_coupled,
            commands::cae_api::parse_thermal_stress_result,
            commands::cae_api::get_coupling_templates,
            // Mesh quality check command
            commands::cae_api::check_mesh_quality,
            // Code execution commands
            commands::code_exec::execute_code,
            // Mesh refinement commands
            commands::cae_api::refine_mesh,
            // STEP/IGES file import commands
            commands::step_import::import_step_file,
            commands::step_import::check_step_import_available,
            // Auth commands
            commands::auth::register,
            commands::auth::login,
            commands::auth::refresh_token,
            commands::auth::get_profile,
            commands::auth::update_profile,
            commands::auth::change_password,
            commands::auth::list_devices,
            commands::auth::logout_device,
            commands::auth::get_membership_status,
            commands::auth::update_membership,
            commands::auth::send_verification_code,
            commands::auth::verify_code,
            commands::auth::verify_access_token_cmd,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
