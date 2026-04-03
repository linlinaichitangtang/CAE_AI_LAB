use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::Mutex as StdMutex;
use tauri::Manager;

mod commands;
pub use commands::{cae_api, file, input_gen, output_parser, postprocess, project, settings, ai, materials, parametric, transient_dynamics, contact, cfd, topology_optimization, optimization_commands, electronics, biomechanics, explicit_dynamics, code_exec, step_import, auth, collaboration, fsi, molecular_dynamics, atom_builder, md_postprocess, trajectory_viewer, phase_field, phase_field_postprocess, phase_field_bridge, dft_input, dft_task, dft_postprocess, dft_bridge, ontology, coordinate_mapping, coarse_graining, error_tracking, benchmark, regression_ci, audit_log, cross_scale_viz, multiscale_integration, workflow_template, high_throughput, ai_recommend, nightly_ci, multiscale_workspace, workflow_editor, data_transfer, workflow_scheduler, param_mapping, workflow_presets, result_comparison, report_generator, adaptive_precision, solver_manager};
pub mod solver;
pub mod plugin;
mod db;
mod models;
pub mod api_server;

use db::Database;

/// Shared state for the API server
struct ApiServerState {
    port: StdMutex<u16>,
    is_running: Arc<AtomicBool>,
    shutdown_tx: StdMutex<Option<tokio::sync::watch::Sender<bool>>>,
}

/// Start the REST API server
#[tauri::command]
async fn start_api_server_cmd(
    app: tauri::AppHandle,
    port: Option<u16>,
    api_state: tauri::State<'_, ApiServerState>,
    _db: tauri::State<'_, Database>,
) -> Result<serde_json::Value, String> {
    // Check if already running
    if api_state.is_running.load(std::sync::atomic::Ordering::Relaxed) {
        let current_port = *api_state.port.lock().map_err(|e| e.to_string())?;
        return Err(format!("API server is already running on port {}", current_port));
    }

    let actual_port = port.unwrap_or_else(|| *api_state.port.lock().unwrap_or_else(|e| e.into_inner()));
    *api_state.port.lock().map_err(|e| e.to_string())? = actual_port;

    // Create shutdown channel
    let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
    *api_state.shutdown_tx.lock().map_err(|e| e.to_string())? = Some(shutdown_tx);

    // Get database reference for the API server
    let db_path = app.path().app_data_dir()
        .expect("Failed to get app data dir")
        .join("caelab.db");
    let database = Database::new(db_path).map_err(|e| format!("Failed to open database for API server: {}", e))?;
    let db_arc = Arc::new(std::sync::Mutex::new(database));

    api_state.is_running.store(true, std::sync::atomic::Ordering::Relaxed);

    // Spawn the API server in a background task
    let is_running_flag = api_state.is_running.clone();
    tauri::async_runtime::spawn(async move {
        if let Err(e) = api_server::start_api_server(actual_port, db_arc, shutdown_rx).await {
            tracing::error!("API server error: {}", e);
        }
        is_running_flag.store(false, std::sync::atomic::Ordering::Relaxed);
    });

    tracing::info!("REST API server started on port {}", actual_port);

    Ok(serde_json::json!({
        "status": "started",
        "port": actual_port,
        "message": format!("API server is running on http://127.0.0.1:{}", actual_port),
        "docs_url": format!("http://127.0.0.1:{}/docs/swagger-ui", actual_port),
    }))
}

/// Stop the REST API server
#[tauri::command]
async fn stop_api_server_cmd(
    api_state: tauri::State<'_, ApiServerState>,
) -> Result<serde_json::Value, String> {
    if !api_state.is_running.load(std::sync::atomic::Ordering::Relaxed) {
        return Err("API server is not running".to_string());
    }

    // Send shutdown signal
    if let Ok(mut tx_guard) = api_state.shutdown_tx.lock() {
        if let Some(tx) = tx_guard.take() {
            let _ = tx.send(true);
        }
    }

    api_state.is_running.store(false, std::sync::atomic::Ordering::Relaxed);

    tracing::info!("REST API server stopped");

    Ok(serde_json::json!({
        "status": "stopped",
        "message": "API server has been stopped",
    }))
}

/// Get API server status
#[tauri::command]
async fn get_api_server_status(
    api_state: tauri::State<'_, ApiServerState>,
) -> Result<serde_json::Value, String> {
    let is_running = api_state.is_running.load(std::sync::atomic::Ordering::Relaxed);
    let port = *api_state.port.lock().map_err(|e| e.to_string())?;

    Ok(serde_json::json!({
        "is_running": is_running,
        "port": port,
        "docs_url": if is_running { Some(format!("http://127.0.0.1:{}/docs/swagger-ui", port)) } else { None },
        "base_url": if is_running { Some(format!("http://127.0.0.1:{}/api/v1", port)) } else { None },
    }))
}

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

            // Initialize API server state
            app.manage(ApiServerState {
                port: StdMutex::new(3001),
                is_running: Arc::new(AtomicBool::new(false)),
                shutdown_tx: StdMutex::new(None),
            });

            // Initialize plugin system state (V1.2-010)
            let plugin_dir = app_data_dir.join("plugins");
            std::fs::create_dir_all(&plugin_dir).ok();
            app.manage(plugin::api::PluginState::new(plugin_dir));

            // Initialize trajectory viewer cache (V1.5)
            app.manage(commands::trajectory_viewer::TrajectoryCache::default());

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
            // Knowledge graph command
            commands::file::get_knowledge_graph,
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
            commands::parametric::run_doe_study,
            commands::parametric::calculate_sensitivity,
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
            // V1.3-001: Bidirectional thermal-structural coupling
            commands::thermal_coupling::run_bidirectional_thermal_structural,
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
            // V1.3-003: Conjugate Heat Transfer commands
            commands::cfd::run_conjugate_heat_transfer,
            commands::cfd::optimize_heat_sink,
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
            // Shape optimization commands (V1.1-006)
            commands::optimization_commands::run_shape_optimization_v2,
            commands::optimization_commands::export_shape_opt_to_stl,
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
            // Abaqus INP import commands (V1.2-009)
            commands::step_import::import_abaqus_inp,
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
            // Collaboration commands
            commands::collaboration::share_project,
            commands::collaboration::list_project_shares,
            commands::collaboration::remove_project_share,
            commands::collaboration::update_share_permission,
            commands::collaboration::add_comment,
            commands::collaboration::list_comments,
            commands::collaboration::update_comment,
            commands::collaboration::delete_comment,
            commands::collaboration::resolve_comment,
            // REST API server commands
            start_api_server_cmd,
            stop_api_server_cmd,
            get_api_server_status,
            // Plugin system commands (V1.2-010)
            plugin::api::list_plugins,
            plugin::api::load_plugin,
            plugin::api::unload_plugin,
            plugin::api::get_plugin_info,
            // V1.3-002: FSI commands
            commands::fsi::run_fsi_analysis,
            commands::fsi::map_cfd_to_structural,
            commands::fsi::get_fsi_templates,
            // V1.5: MD post-processing commands
            commands::md_postprocess::calculate_rdf,
            commands::md_postprocess::calculate_msd,
            commands::md_postprocess::calculate_vacf,
            commands::md_postprocess::calculate_diffusion_coefficient,
            commands::md_postprocess::coarse_grain_to_phase_field,
            commands::md_postprocess::map_to_fe_boundary,
            commands::md_postprocess::get_md_post_process_templates,
            // V1.5: Trajectory viewer commands
            commands::trajectory_viewer::load_trajectory,
            commands::trajectory_viewer::get_frame,
            commands::trajectory_viewer::parse_time_series,
            commands::trajectory_viewer::export_frame_image,
            commands::trajectory_viewer::export_time_series_csv,
            commands::trajectory_viewer::get_trajectory_stats,
            // V1.8: Multiscale Data Management & Validation Framework
            commands::ontology::search_quantities,
            commands::ontology::convert_unit,
            commands::ontology::batch_convert,
            commands::ontology::get_quantity_definition,
            commands::ontology::list_categories,
            commands::coordinate_mapping::create_mapping,
            commands::coordinate_mapping::map_points,
            commands::coordinate_mapping::get_mapping_info,
            commands::coordinate_mapping::validate_alignment,
            commands::coarse_graining::run_coarse_graining,
            commands::coarse_graining::recommend_method,
            commands::coarse_graining::compare_methods,
            commands::coarse_graining::get_coarse_graining_presets,
            commands::error_tracking::create_error_pipeline,
            commands::error_tracking::add_error_contribution,
            commands::error_tracking::get_propagation_steps,
            commands::error_tracking::generate_error_report,
            commands::error_tracking::run_sensitivity_analysis,
            commands::benchmark::list_benchmarks,
            commands::benchmark::get_benchmark_detail,
            commands::benchmark::run_benchmark,
            commands::benchmark::compare_with_reference,
            commands::benchmark::get_benchmark_statistics,
            commands::regression_ci::trigger_ci_pipeline,
            commands::regression_ci::get_ci_pipeline_status,
            commands::regression_ci::get_ci_pipeline_history,
            commands::regression_ci::get_ci_test_detail,
            commands::regression_ci::update_notification_config,
            commands::regression_ci::get_ci_statistics,
            commands::audit_log::create_audit_chain,
            commands::audit_log::add_audit_entry,
            commands::audit_log::get_audit_chain,
            commands::audit_log::validate_chain,
            commands::audit_log::export_audit_report,
            commands::audit_log::search_audit_logs,
            commands::cross_scale_viz::create_cross_scale_viz,
            commands::cross_scale_viz::map_atom_to_field,
            commands::cross_scale_viz::overlay_field_on_mesh,
            commands::cross_scale_viz::generate_scale_bridge_animation,
            commands::cross_scale_viz::get_available_color_maps,
            // V1.5: Molecular dynamics commands
            commands::molecular_dynamics::check_lammps_available,
            commands::molecular_dynamics::run_md_simulation,
            commands::molecular_dynamics::run_lammps_simulation,
            commands::molecular_dynamics::generate_lammps_input,
            commands::molecular_dynamics::parse_lammps_output,
            commands::molecular_dynamics::get_md_templates,
            commands::molecular_dynamics::validate_md_config,
            commands::molecular_dynamics::estimate_memory,
            // V1.5: Atom builder commands
            commands::atom_builder::build_supercell,
            commands::atom_builder::build_amorphous,
            commands::atom_builder::build_interface,
            commands::atom_builder::build_defect,
            commands::atom_builder::build_cnt,
            commands::atom_builder::build_cluster,
            commands::atom_builder::get_element_library,
            commands::atom_builder::calculate_density,
            commands::atom_builder::get_atom_builder_templates,
            // V1.6: Phase field commands
            commands::phase_field::run_phase_field_simulation,
            commands::phase_field::generate_pf_initial_condition,
            commands::phase_field::calculate_pf_free_energy,
            commands::phase_field::get_phase_field_templates,
            commands::phase_field::validate_pf_config,
            commands::phase_field::estimate_pf_memory,
            // V1.6: Phase field post-processing commands
            commands::phase_field_postprocess::run_mechanical_coupling,
            commands::phase_field_postprocess::analyze_grains,
            commands::phase_field_postprocess::calculate_grain_size_distribution,
            commands::phase_field_postprocess::calculate_orientation_distribution,
            commands::phase_field_postprocess::detect_grain_boundaries,
            commands::phase_field_postprocess::export_field_to_csv,
            commands::phase_field_postprocess::get_phase_field_post_process_templates,
            // V1.6: Phase field bridge commands
            commands::phase_field_bridge::coarse_grain_md_to_pf,
            commands::phase_field_bridge::homogenize_pf_to_fe,
            commands::phase_field_bridge::validate_bridge_quality,
            commands::phase_field_bridge::get_bridge_templates,
            commands::phase_field_bridge::export_bridge_data,
            // V1.7: DFT input commands
            commands::dft_input::generate_vasp_input,
            commands::dft_input::generate_qe_input,
            commands::dft_input::parse_vasp_poscar,
            commands::dft_input::parse_vasp_incar,
            commands::dft_input::get_vasp_templates,
            commands::dft_input::get_qe_templates,
            commands::dft_input::export_input_files,
            // V1.7: DFT task commands
            commands::dft_task::submit_dft_job,
            commands::dft_task::cancel_dft_job,
            commands::dft_task::get_dft_job_status,
            commands::dft_task::get_dft_job_list,
            commands::dft_task::generate_slurm_script,
            commands::dft_task::generate_pbs_script,
            commands::dft_task::configure_dft_queue,
            commands::dft_task::get_queue_status,
            // V1.7: DFT post-processing commands
            commands::dft_postprocess::parse_dft_output,
            commands::dft_postprocess::parse_vasp_outcar,
            commands::dft_postprocess::parse_vasp_doscar,
            commands::dft_postprocess::parse_vasp_eigenv,
            commands::dft_postprocess::parse_vasp_contcar,
            commands::dft_postprocess::parse_qe_output,
            commands::dft_postprocess::parse_qe_dos,
            commands::dft_postprocess::run_validation,
            commands::dft_postprocess::get_validation_test_suite,
            // V1.7: DFT bridge commands
            commands::dft_bridge::prepare_training_data,
            commands::dft_bridge::train_potential,
            commands::dft_bridge::validate_potential,
            commands::dft_bridge::export_potential,
            commands::dft_bridge::extract_gl_params,
            commands::dft_bridge::calculate_phase_diagram,
            commands::dft_bridge::align_chemical_potential,
            commands::dft_bridge::get_dft_bridge_templates,
            // V1.9: Multiscale integration commands
            commands::multiscale_integration::run_integration_pipeline,
            commands::multiscale_integration::get_pipeline_status,
            commands::multiscale_integration::get_step_result,
            commands::multiscale_integration::retry_step,
            commands::multiscale_integration::list_end_to_end_cases,
            commands::multiscale_integration::run_end_to_end_case,
            commands::multiscale_integration::compare_with_literature,
            // V1.9: Workflow template commands
            commands::workflow_template::list_templates,
            commands::workflow_template::get_template_detail,
            commands::workflow_template::download_template,
            commands::workflow_template::upload_template,
            commands::workflow_template::rate_template,
            commands::workflow_template::get_template_reviews,
            commands::workflow_template::create_template_from_pipeline,
            // V1.9: High throughput commands
            commands::high_throughput::create_parameter_scan,
            commands::high_throughput::get_scan_status,
            commands::high_throughput::cancel_scan,
            commands::high_throughput::query_scan_results,
            commands::high_throughput::export_scan_results,
            commands::high_throughput::get_scan_statistics,
            commands::high_throughput::get_scan_dashboard,
            // V1.9: AI recommend commands
            commands::ai_recommend::get_recommendations,
            commands::ai_recommend::get_recommendation_history,
            commands::ai_recommend::apply_recommendation,
            commands::ai_recommend::get_model_performance,
            commands::ai_recommend::feedback_recommendation,
            // V1.9: Nightly CI commands
            commands::nightly_ci::trigger_nightly_build,
            commands::nightly_ci::get_nightly_status,
            commands::nightly_ci::get_nightly_history,
            commands::nightly_ci::update_nightly_config,
            commands::nightly_ci::get_nightly_config,
            commands::nightly_ci::get_nightly_statistics,
            commands::nightly_ci::get_nightly_report,
            // V1.9: Multiscale workspace commands
            commands::multiscale_workspace::create_ms_project,
            commands::multiscale_workspace::get_ms_project,
            commands::multiscale_workspace::list_ms_projects,
            commands::multiscale_workspace::add_ms_task,
            commands::multiscale_workspace::connect_ms_tasks,
            commands::multiscale_workspace::get_ms_project_dashboard,
            commands::multiscale_workspace::run_ms_project,
            commands::multiscale_workspace::archive_ms_project,
            // V2.0: Workflow editor commands
            commands::workflow_editor::create_wf_graph,
            commands::workflow_editor::load_wf_graph,
            commands::workflow_editor::save_wf_graph,
            commands::workflow_editor::add_wf_node,
            commands::workflow_editor::remove_wf_node,
            commands::workflow_editor::update_wf_node_config,
            commands::workflow_editor::add_wf_edge,
            commands::workflow_editor::remove_wf_edge,
            commands::workflow_editor::add_wf_conditional_branch,
            commands::workflow_editor::validate_wf_graph,
            commands::workflow_editor::list_wf_graphs,
            // V2.0: Data transfer commands
            commands::data_transfer::get_scale_output_schema,
            commands::data_transfer::validate_data_transfer,
            commands::data_transfer::execute_data_transfer,
            commands::data_transfer::get_transfer_rules,
            commands::data_transfer::update_transfer_rule,
            commands::data_transfer::get_transfer_history,
            // V2.0: Workflow scheduler commands
            commands::workflow_scheduler::start_wf_execution,
            commands::workflow_scheduler::get_wf_execution_status,
            commands::workflow_scheduler::pause_wf_execution,
            commands::workflow_scheduler::resume_wf_execution,
            commands::workflow_scheduler::cancel_wf_execution,
            commands::workflow_scheduler::retry_wf_from_node,
            commands::workflow_scheduler::get_wf_execution_checkpoints,
            commands::workflow_scheduler::list_wf_executions,
            // V2.0: Param mapping commands
            commands::param_mapping::get_mapping_table,
            commands::param_mapping::update_mapping_rule,
            commands::param_mapping::add_mapping_rule,
            commands::param_mapping::delete_mapping_rule,
            commands::param_mapping::auto_map_parameters,
            commands::param_mapping::apply_mapping,
            commands::param_mapping::get_mapping_sensitivity,
            // V2.0: Workflow presets commands
            commands::workflow_presets::list_wf_presets,
            commands::workflow_presets::get_wf_preset_detail,
            commands::workflow_presets::apply_wf_preset,
            commands::workflow_presets::create_wf_preset_from_graph,
            commands::workflow_presets::get_wf_preset_recommendation,
            // V2.0: Result comparison commands
            commands::result_comparison::generate_comparison,
            commands::result_comparison::get_scale_result_summary,
            commands::result_comparison::drill_down,
            commands::result_comparison::get_cross_scale_links,
            commands::result_comparison::export_comparison_data,
            // V2.0: Report generator commands
            commands::report_generator::generate_report,
            commands::report_generator::preview_report,
            commands::report_generator::get_report_templates,
            commands::report_generator::save_report_template,
            commands::report_generator::get_report_history,
            commands::report_generator::download_report,
            // V2.0: Adaptive precision commands
            commands::adaptive_precision::evaluate_precision,
            commands::adaptive_precision::apply_adjustments,
            commands::adaptive_precision::run_adaptive_loop,
            commands::adaptive_precision::get_precision_history,
            commands::adaptive_precision::update_control_config,
            commands::adaptive_precision::get_recommended_strategy,
            // V2.2: Solver manager commands
            commands::solver_manager::detect_solvers,
            commands::solver_manager::check_solver_works,
            commands::solver_manager::install_solver,
            commands::solver_manager::uninstall_solver,
            commands::solver_manager::get_install_methods,
            commands::solver_manager::get_solver_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
