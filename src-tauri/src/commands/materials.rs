use crate::db::Database;
use crate::models::{CreateMaterialInput, Material, UpdateMaterialInput};
use tauri::command;
use tauri::State;

/// List all materials (both built-in and custom)
#[command]
pub fn list_materials(db: State<'_, Database>) -> Result<Vec<Material>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, name, youngs_modulus, poissons_ratio, density, thermal_expansion,
                    yield_strength, ultimate_strength, description, is_builtin, created_at, updated_at
             FROM materials ORDER BY is_builtin DESC, name ASC",
        )
        .map_err(|e| e.to_string())?;

    let materials = stmt
        .query_map([], |row| {
            Ok(Material {
                id: row.get(0)?,
                name: row.get(1)?,
                youngs_modulus: row.get(2)?,
                poissons_ratio: row.get(3)?,
                density: row.get(4)?,
                thermal_expansion: row.get(5)?,
                yield_strength: row.get(6)?,
                ultimate_strength: row.get(7)?,
                description: row.get(8)?,
                is_builtin: row.get::<_, i32>(9)? != 0,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    tracing::info!("Listed {} materials", materials.len());
    Ok(materials)
}

/// Get a single material by ID
#[command]
pub fn get_material(db: State<'_, Database>, id: String) -> Result<Material, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let material = conn
        .query_row(
            "SELECT id, name, youngs_modulus, poissons_ratio, density, thermal_expansion,
                    yield_strength, ultimate_strength, description, is_builtin, created_at, updated_at
             FROM materials WHERE id = ?1",
            [&id],
            |row| {
                Ok(Material {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    youngs_modulus: row.get(2)?,
                    poissons_ratio: row.get(3)?,
                    density: row.get(4)?,
                    thermal_expansion: row.get(5)?,
                    yield_strength: row.get(6)?,
                    ultimate_strength: row.get(7)?,
                    description: row.get(8)?,
                    is_builtin: row.get::<_, i32>(9)? != 0,
                    created_at: row.get(10)?,
                    updated_at: row.get(11)?,
                })
            },
        )
        .map_err(|e| format!("Material not found: {}", e))?;

    Ok(material)
}

/// Create a custom material
#[command]
pub fn create_material(
    db: State<'_, Database>,
    input: CreateMaterialInput,
) -> Result<Material, String> {
    let now = chrono::Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();

    let material = Material {
        id: id.clone(),
        name: input.name,
        youngs_modulus: input.youngs_modulus,
        poissons_ratio: input.poissons_ratio,
        density: input.density,
        thermal_expansion: input.thermal_expansion,
        yield_strength: input.yield_strength,
        ultimate_strength: input.ultimate_strength,
        description: input.description,
        is_builtin: false,
        created_at: now.clone(),
        updated_at: now,
    };

    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO materials 
         (id, name, youngs_modulus, poissons_ratio, density, thermal_expansion,
          yield_strength, ultimate_strength, description, is_builtin, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 0, ?10, ?10)",
        rusqlite::params![
            &material.id,
            &material.name,
            &material.youngs_modulus,
            &material.poissons_ratio,
            &material.density,
            &material.thermal_expansion,
            &material.yield_strength,
            &material.ultimate_strength,
            &material.description,
            &material.created_at
        ],
    )
    .map_err(|e| format!("Failed to create material: {}", e))?;

    tracing::info!("Created material: {} - {}", material.id, material.name);
    Ok(material)
}

/// Update a material (only custom materials can be updated)
#[command]
pub fn update_material(
    db: State<'_, Database>,
    input: UpdateMaterialInput,
) -> Result<Material, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Check if material exists and is not built-in
    let current: Material = conn
        .query_row(
            "SELECT id, name, youngs_modulus, poissons_ratio, density, thermal_expansion,
                    yield_strength, ultimate_strength, description, is_builtin, created_at, updated_at
             FROM materials WHERE id = ?1",
            [&input.id],
            |row| {
                Ok(Material {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    youngs_modulus: row.get(2)?,
                    poissons_ratio: row.get(3)?,
                    density: row.get(4)?,
                    thermal_expansion: row.get(5)?,
                    yield_strength: row.get(6)?,
                    ultimate_strength: row.get(7)?,
                    description: row.get(8)?,
                    is_builtin: row.get::<_, i32>(9)? != 0,
                    created_at: row.get(10)?,
                    updated_at: row.get(11)?,
                })
            },
        )
        .map_err(|e| format!("Material not found: {}", e))?;

    if current.is_builtin {
        return Err("Cannot update built-in materials".to_string());
    }

    let now = chrono::Utc::now().to_rfc3339();
    let name = input.name.unwrap_or(current.name);
    let youngs_modulus = input.youngs_modulus.unwrap_or(current.youngs_modulus);
    let poissons_ratio = input.poissons_ratio.unwrap_or(current.poissons_ratio);
    let density = input.density.unwrap_or(current.density);
    let thermal_expansion = input.thermal_expansion.unwrap_or(current.thermal_expansion);
    let yield_strength = input.yield_strength.unwrap_or(current.yield_strength);
    let ultimate_strength = input.ultimate_strength.or(current.ultimate_strength);
    let description = input.description.or(current.description);

    conn.execute(
        "UPDATE materials SET 
         name = ?1, youngs_modulus = ?2, poissons_ratio = ?3, density = ?4,
         thermal_expansion = ?5, yield_strength = ?6, ultimate_strength = ?7,
         description = ?8, updated_at = ?9
         WHERE id = ?10",
        rusqlite::params![
            &name,
            &youngs_modulus,
            &poissons_ratio,
            &density,
            &thermal_expansion,
            &yield_strength,
            &ultimate_strength,
            &description,
            &now,
            &input.id
        ],
    )
    .map_err(|e| format!("Failed to update material: {}", e))?;

    let updated = Material {
        id: current.id,
        name,
        youngs_modulus,
        poissons_ratio,
        density,
        thermal_expansion,
        yield_strength,
        ultimate_strength,
        description,
        is_builtin: false,
        created_at: current.created_at,
        updated_at: now,
    };

    tracing::info!("Updated material: {} - {}", updated.id, updated.name);
    Ok(updated)
}

/// Delete a custom material (built-in materials cannot be deleted)
#[command]
pub fn delete_material(db: State<'_, Database>, id: String) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Check if material exists and is not built-in
    let is_builtin: bool = conn
        .query_row("SELECT is_builtin FROM materials WHERE id = ?1", [&id], |row| {
            Ok(row.get::<_, i32>(0)? != 0)
        })
        .map_err(|e| format!("Material not found: {}", e))?;

    if is_builtin {
        return Err("Cannot delete built-in materials".to_string());
    }

    conn.execute("DELETE FROM materials WHERE id = ?1", [&id])
        .map_err(|e| format!("Failed to delete material: {}", e))?;

    tracing::info!("Deleted material: {}", id);
    Ok(())
}

/// Get list of built-in material names
#[command]
pub fn get_builtin_material_names(db: State<'_, Database>) -> Result<Vec<String>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT name FROM materials WHERE is_builtin = 1 ORDER BY name")
        .map_err(|e| e.to_string())?;

    let names = stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<String>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(names)
}