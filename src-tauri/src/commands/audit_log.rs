// Audit Log Commands - V1.8
// Cryptographic audit chain for tracking multiscale simulation data provenance.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tauri::command;

// ============================================================================
// Data Structures
// ============================================================================

/// A complete audit chain with hash-linked entries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditChain {
    pub id: String,
    pub name: String,
    pub description: String,
    pub entries: Vec<AuditEntry>,
    pub created_at: String,
    pub updated_at: String,
    pub is_valid: bool,
}

/// A single audit entry in the chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: String,
    pub chain_id: String,
    pub sequence_number: u32,
    pub timestamp: String,
    pub action: String,           // "created", "modified", "validated", "exported", "approved"
    pub actor: String,            // User or system that performed the action
    pub module: String,           // Which module/command generated this entry
    pub data_hash: String,        // SHA-256 hash of the data payload
    pub previous_hash: String,    // SHA-256 hash of the previous entry (chain link)
    pub metadata: HashMap<String, Value>,
    pub description: String,
}

/// Result of adding an entry to an audit chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddEntryResult {
    pub entry: AuditEntry,
    pub chain_length: u32,
    pub chain_valid: bool,
}

/// Result of validating an audit chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainValidationResult {
    pub chain_id: String,
    pub is_valid: bool,
    pub total_entries: u32,
    pub valid_entries: u32,
    pub invalid_entries: u32,
    pub issues: Vec<String>,
    pub first_invalid_index: Option<u32>,
    pub integrity_hash: String,
}

/// An exported audit report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReport {
    pub chain_id: String,
    pub chain_name: String,
    pub generated_at: String,
    pub format: String,
    pub entries: Vec<AuditEntry>,
    pub statistics: AuditReportStatistics,
    pub content: String,
}

/// Statistics for an audit report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReportStatistics {
    pub total_entries: u32,
    pub entries_by_action: HashMap<String, u32>,
    pub entries_by_module: HashMap<String, u32>,
    pub entries_by_actor: HashMap<String, u32>,
    pub time_span_days: f64,
    pub first_entry: String,
    pub last_entry: String,
}

/// Result of searching audit logs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditSearchResult {
    pub query: String,
    pub total_matches: u32,
    pub entries: Vec<AuditEntry>,
    pub facets: HashMap<String, Vec<String>>,
}

// ============================================================================
// Mock Data
// ============================================================================

/// Simple mock hash function (in production, use actual SHA-256).
fn mock_hash(data: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    let hash_val = hasher.finish();
    format!("{:064x}", hash_val)
}

fn build_mock_chain() -> AuditChain {
    let chain_id = uuid::Uuid::new_v4().to_string();
    let created_at = "2026-03-20T08:00:00Z".to_string();

    let actions = vec![
        ("created", "system", "project", "Simulation project initialized with DFT-MD-PF-FE workflow configuration"),
        ("created", "dr_zhang", "dft_task", "DFT calculation created: Cu FCC equilibrium with PBE functional, 520 eV cutoff"),
        ("modified", "dr_zhang", "dft_task", "DFT parameters updated: k-point mesh changed from 6x6x6 to 8x8x8"),
        ("validated", "system", "dft_postprocess", "DFT results validated: lattice constant 3.615 A within 1% of reference"),
        ("created", "dr_zhang", "molecular_dynamics", "MD simulation created: NVT ensemble, 10000 atoms, NEP potential"),
        ("modified", "dr_li", "molecular_dynamics", "MD timestep changed from 1.0 fs to 0.5 fs for improved energy conservation"),
        ("validated", "system", "md_postprocess", "MD trajectory validated: energy drift < 0.1 meV/atom/ps over 100 ps"),
        ("exported", "dr_zhang", "phase_field_bridge", "Coarse-grained MD density exported to phase field grid (64x64x64)"),
    ];

    let mut entries = Vec::new();
    let mut prev_hash = "0".repeat(64); // Genesis hash

    for (i, (action, actor, module, desc)) in actions.iter().enumerate() {
        let entry_data = format!("{}:{}:{}:{}:{}", chain_id, i, action, actor, desc);
        let data_hash = mock_hash(&entry_data);
        let chain_hash = mock_hash(&format!("{}{}", prev_hash, data_hash));

        let entry = AuditEntry {
            id: uuid::Uuid::new_v4().to_string(),
            chain_id: chain_id.clone(),
            sequence_number: i as u32,
            timestamp: format!("2026-03-{}T{:02}:00:00Z", 20 + i / 4, 8 + i),
            action: action.to_string(),
            actor: actor.to_string(),
            module: module.to_string(),
            data_hash,
            previous_hash: prev_hash.clone(),
            metadata: HashMap::new(),
            description: desc.to_string(),
        };

        prev_hash = chain_hash;
        entries.push(entry);
    }

    AuditChain {
        id: chain_id,
        name: "Cu Multiscale Simulation Audit Trail".into(),
        description: "Complete audit trail for copper multiscale simulation from DFT through MD to phase-field bridging.".into(),
        entries,
        created_at,
        updated_at: "2026-03-27T16:00:00Z".into(),
        is_valid: true,
    }
}

// ============================================================================
// Tauri Commands
// ============================================================================

/// Create a new audit chain.
#[command]
pub fn create_audit_chain(
    name: String,
    description: Option<String>,
) -> Result<AuditChain, String> {
    tracing::info!(name = %name, "create_audit_chain called");

    if name.is_empty() {
        return Err("Chain name cannot be empty".to_string());
    }

    let now = chrono::Utc::now().to_rfc3339();
    let chain_id = uuid::Uuid::new_v4().to_string();

    // Create genesis entry
    let genesis_data = format!("{}:0:created:system:Audit chain '{}' created", chain_id, name);
    let genesis_hash = mock_hash(&genesis_data);
    let _genesis_chain_hash = mock_hash(&format!("{}{}", "0".repeat(64), 0));

    let genesis_entry = AuditEntry {
        id: uuid::Uuid::new_v4().to_string(),
        chain_id: chain_id.clone(),
        sequence_number: 0,
        timestamp: now.clone(),
        action: "created".into(),
        actor: "system".into(),
        module: "audit_log".into(),
        data_hash: genesis_hash,
        previous_hash: "0".repeat(64),
        metadata: HashMap::new(),
        description: format!("Audit chain '{}' created", name),
    };

    tracing::info!(chain_id = %chain_id, "Audit chain created");

    Ok(AuditChain {
        id: chain_id,
        name,
        description: description.unwrap_or_default(),
        entries: vec![genesis_entry],
        created_at: now.clone(),
        updated_at: now,
        is_valid: true,
    })
}

/// Add a new entry to an existing audit chain.
#[command]
pub fn add_audit_entry(
    chain_id: String,
    action: String,
    actor: String,
    module: String,
    description: String,
    metadata: Option<HashMap<String, Value>>,
) -> Result<AddEntryResult, String> {
    tracing::info!(
        chain = %chain_id,
        action = %action,
        actor = %actor,
        module = %module,
        "add_audit_entry called"
    );

    if action.is_empty() {
        return Err("Action cannot be empty".to_string());
    }

    let chain = build_mock_chain();
    let seq = chain.entries.len() as u32;
    let prev_hash = chain
        .entries
        .last()
        .map(|e| e.data_hash.clone())
        .unwrap_or_else(|| "0".repeat(64));

    let entry_data = format!("{}:{}:{}:{}:{}", chain_id, seq, action, actor, description);
    let data_hash = mock_hash(&entry_data);

    let entry = AuditEntry {
        id: uuid::Uuid::new_v4().to_string(),
        chain_id: chain_id.clone(),
        sequence_number: seq,
        timestamp: chrono::Utc::now().to_rfc3339(),
        action,
        actor,
        module,
        data_hash,
        previous_hash: prev_hash,
        metadata: metadata.unwrap_or_default(),
        description,
    };

    tracing::info!(
        entry_id = %entry.id,
        sequence = seq,
        "Audit entry added"
    );

    Ok(AddEntryResult {
        entry,
        chain_length: seq + 1,
        chain_valid: true,
    })
}

/// Get the complete audit chain with all entries.
#[command]
pub fn get_audit_chain(chain_id: String) -> Result<AuditChain, String> {
    tracing::info!(chain = %chain_id, "get_audit_chain called");

    let chain = build_mock_chain();

    tracing::info!(
        entries = chain.entries.len(),
        is_valid = chain.is_valid,
        "Audit chain retrieved"
    );

    Ok(chain)
}

/// Validate the integrity of an audit chain by checking hash links.
#[command]
pub fn validate_chain(chain_id: String) -> Result<ChainValidationResult, String> {
    tracing::info!(chain = %chain_id, "validate_chain called");

    let chain = build_mock_chain();
    let total = chain.entries.len() as u32;
    let mut valid_count = 0u32;
    let mut invalid_count = 0u32;
    let mut issues = Vec::new();
    let mut first_invalid: Option<u32> = None;

    for (i, entry) in chain.entries.iter().enumerate() {
        let mut is_valid = true;

        // Check previous hash link
        if i == 0 {
            // Genesis entry should have zero hash as previous
            if entry.previous_hash != "0".repeat(64) {
                is_valid = false;
                issues.push(format!(
                    "Entry {}: genesis entry has non-zero previous hash",
                    i
                ));
            }
        } else {
            let expected_prev = chain.entries[i - 1].data_hash.clone();
            if entry.previous_hash != expected_prev {
                is_valid = false;
                issues.push(format!(
                    "Entry {}: hash chain broken (expected prev={}, got={})",
                    i, &expected_prev[..16], &entry.previous_hash[..16]
                ));
            }
        }

        // Verify data hash is not empty
        if entry.data_hash.is_empty() {
            is_valid = false;
            issues.push(format!("Entry {}: empty data hash", i));
        }

        // Verify sequence number
        if entry.sequence_number != i as u32 {
            is_valid = false;
            issues.push(format!(
                "Entry {}: sequence number mismatch (expected {}, got {})",
                i, i, entry.sequence_number
            ));
        }

        if is_valid {
            valid_count += 1;
        } else {
            invalid_count += 1;
            if first_invalid.is_none() {
                first_invalid = Some(i as u32);
            }
        }
    }

    // Compute overall integrity hash
    let all_hashes: String = chain.entries.iter().map(|e| e.data_hash.clone()).collect();
    let integrity_hash = mock_hash(&all_hashes);

    let is_valid = invalid_count == 0;

    tracing::info!(
        valid = valid_count,
        invalid = invalid_count,
        is_valid = is_valid,
        "Chain validation complete"
    );

    Ok(ChainValidationResult {
        chain_id,
        is_valid,
        total_entries: total,
        valid_entries: valid_count,
        invalid_entries: invalid_count,
        issues,
        first_invalid_index: first_invalid,
        integrity_hash,
    })
}

/// Export an audit chain as a formatted report.
#[command]
pub fn export_audit_report(
    chain_id: String,
    format: Option<String>,
) -> Result<AuditReport, String> {
    tracing::info!(
        chain = %chain_id,
        format = format.as_deref().unwrap_or("text"),
        "export_audit_report called"
    );

    let chain = build_mock_chain();
    let fmt = format.unwrap_or_else(|| "text".into());

    let mut by_action = HashMap::new();
    let mut by_module = HashMap::new();
    let mut by_actor = HashMap::new();

    for entry in &chain.entries {
        *by_action.entry(entry.action.clone()).or_insert(0) += 1;
        *by_module.entry(entry.module.clone()).or_insert(0) += 1;
        *by_actor.entry(entry.actor.clone()).or_insert(0) += 1;
    }

    let first_ts = chain.entries.first().map(|e| e.timestamp.clone()).unwrap_or_default();
    let last_ts = chain.entries.last().map(|e| e.timestamp.clone()).unwrap_or_default();
    let time_span = 7.0; // mock: 7 days

    let statistics = AuditReportStatistics {
        total_entries: chain.entries.len() as u32,
        entries_by_action: by_action,
        entries_by_module: by_module,
        entries_by_actor: by_actor,
        time_span_days: time_span,
        first_entry: first_ts,
        last_entry: last_ts,
    };

    // Generate text report
    let mut content = String::new();
    content.push_str(&format!("=== Audit Report: {} ===\n\n", chain.name));
    content.push_str(&format!("Chain ID: {}\n", chain.id));
    content.push_str(&format!("Generated: {}\n\n", chrono::Utc::now().to_rfc3339()));
    content.push_str(&format!("Total Entries: {}\n", chain.entries.len()));
    content.push_str(&format!("Time Span: {:.1} days\n\n", time_span));
    content.push_str("--- Entries ---\n\n");

    for entry in &chain.entries {
        content.push_str(&format!(
            "[#{}] {} | {} | {} | {}\n",
            entry.sequence_number,
            entry.timestamp,
            entry.action,
            entry.actor,
            entry.description
        ));
        content.push_str(&format!("    Module: {}\n", entry.module));
        content.push_str(&format!("    Data Hash: {}...\n", &entry.data_hash[..16]));
        content.push_str(&format!("    Prev Hash: {}...\n\n", &entry.previous_hash[..16]));
    }

    tracing::info!(
        entries = chain.entries.len(),
        format = %fmt,
        "Audit report exported"
    );

    Ok(AuditReport {
        chain_id: chain.id,
        chain_name: chain.name,
        generated_at: chrono::Utc::now().to_rfc3339(),
        format: fmt,
        entries: chain.entries,
        statistics,
        content,
    })
}

/// Search audit logs across all chains.
#[command]
pub fn search_audit_logs(
    query: String,
    action: Option<String>,
    module: Option<String>,
    actor: Option<String>,
    limit: Option<u32>,
) -> Result<AuditSearchResult, String> {
    tracing::info!(
        query = %query,
        action = action.as_deref().unwrap_or("all"),
        module = module.as_deref().unwrap_or("all"),
        actor = actor.as_deref().unwrap_or("all"),
        "search_audit_logs called"
    );

    let chain = build_mock_chain();
    let limit = limit.unwrap_or(50) as usize;

    let mut facets = HashMap::new();
    let mut action_set = std::collections::HashSet::new();
    let mut module_set = std::collections::HashSet::new();
    let mut actor_set = std::collections::HashSet::new();

    let matches: Vec<AuditEntry> = chain
        .entries
        .into_iter()
        .filter(|e| {
            let query_match = if query.is_empty() {
                true
            } else {
                e.description.to_lowercase().contains(&query.to_lowercase())
                    || e.module.to_lowercase().contains(&query.to_lowercase())
            };
            let action_match = match &action {
                Some(a) if !a.is_empty() => e.action == *a,
                _ => true,
            };
            let module_match = match &module {
                Some(m) if !m.is_empty() => e.module == *m,
                _ => true,
            };
            let actor_match = match &actor {
                Some(a) if !a.is_empty() => e.actor == *a,
                _ => true,
            };

            action_set.insert(e.action.clone());
            module_set.insert(e.module.clone());
            actor_set.insert(e.actor.clone());

            query_match && action_match && module_match && actor_match
        })
        .take(limit)
        .collect();

    facets.insert("actions".into(), action_set.into_iter().collect());
    facets.insert("modules".into(), module_set.into_iter().collect());
    facets.insert("actors".into(), actor_set.into_iter().collect());

    tracing::info!(matches = matches.len(), "Search complete");

    Ok(AuditSearchResult {
        query,
        total_matches: matches.len() as u32,
        entries: matches,
        facets,
    })
}
