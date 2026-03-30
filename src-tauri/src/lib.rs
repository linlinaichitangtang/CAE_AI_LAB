use std::path::PathBuf;
use tauri::Manager;

mod commands;
mod db;
mod models;
pub mod solver;

use db::Database;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    tracing::info!("Starting CAELab application");

    tauri::Builder::default()
        .setup(|app| {
            tracing::info!("Application setup complete");
            
            // Initialize database
            let app_data_dir = app.path().app_data_dir().expect("Failed to get app data dir");
            std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");
            
            let db_path = app_data_dir.join("caelab.db");
            tracing::info!("Database path: {:?}", db_path);
            
            let database = Database::new(db_path).expect("Failed to initialize database");
            app.manage(database);
            
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
            // Settings commands
            commands::settings::save_settings,
            commands::settings::get_settings,
            commands::settings::delete_settings,
            // CAE API commands
            commands::cae_api::check_solver,
            commands::cae_api::generate_input,
            commands::cae_api::run_solver,
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
