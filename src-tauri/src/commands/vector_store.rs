/**
 * V3.10 Agent RAG 向量推理 — Rust 后端命令
 * - TF-IDF 嵌入生成 (128维特征向量)
 * - 余弦相似度搜索
 * - 跨会话知识积累
 * - 主动回忆机制
 *
 * 核心能力:
 * - 向量嵌入存储与检索
 * - 基于 TF-IDF 的语义相似度计算
 * - 主动回忆 (仿真前自动检索相关历史)
 * - 跨会话知识摘要
 */

use rusqlite::{Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// V3.10-001: TF-IDF 嵌入生成
// ============================================================================

/// 停用词列表 (工程仿真领域常见停用词)
const STOP_WORDS: &[&str] = &[
    "的", "了", "在", "是", "我", "有", "和", "就", "不", "人", "都", "一", "一个",
    "上", "也", "很", "到", "说", "要", "去", "你", "会", "着", "没有", "看", "好",
    "自己", "这", "那", "它", "什么", "将", "进行", "可以", "通过", "使用", "或者",
    "以及", "对于", "其中", "作为", "基于", "根据", "不同", "可能", "需要", "如果",
    "the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for", "of", "with",
    "by", "from", "is", "are", "was", "were", "be", "been", "being", "have", "has",
    "had", "do", "does", "did", "will", "would", "could", "should", "may", "might",
];

/// 词汇 IDF 表 (预计算常用词 IDF 值)
struct IDFTable {
    idf_map: HashMap<String, f64>,
}

impl IDFTable {
    fn new() -> Self {
        // 常见工程仿真词汇的 IDF 预计算值 (基于大规模语料库统计)
        let idf_map = vec![
            ("tc4", 5.5), ("ti-6al-4v", 5.2), ("fatigue", 4.8), ("hcf", 5.0),
            ("lcf", 5.0), ("tmf", 5.5), ("creep", 4.5), ("buckling", 4.2),
            ("modal", 4.0), ("thermal", 3.5), ("stress", 2.8), ("strain", 3.0),
            ("von mises", 4.5), ("displacement", 3.2), ("mesh", 3.5),
            ("material", 2.5), ("simulation", 2.0), ("analysis", 1.8),
            ("carbon", 4.0), ("steel", 3.0), ("aluminum", 3.5), ("titanium", 4.0),
            ("yield", 4.0), ("strength", 2.5), ("modulus", 4.2), ("elastic", 4.0),
            ("plastic", 4.0), ("fracture", 4.5), ("crack", 4.0), ("propagation", 4.5),
            ("lammps", 5.0), ("vasp", 5.2), ("calculix", 5.0), ("abaqus", 4.8),
            ("result", 2.0), ("error", 3.0), ("convergence", 4.5), ("iterations", 4.0),
            ("boundary", 3.5), ("condition", 2.8), ("load", 2.5), ("pressure", 3.0),
            ("temperature", 3.0), ("displacement", 3.2), ("force", 2.8), ("constraint", 3.5),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();

        Self { idf_map }
    }

    fn get(&self, term: &str) -> f64 {
        self.idf_map.get(term).copied().unwrap_or(3.0) // 默认 IDF
    }
}

/// TF-IDF 向量生成器
pub struct TFIDFVectorizer {
    idf_table: IDFTable,
    vocab: HashMap<String, usize>,
}

impl TFIDFVectorizer {
    pub fn new() -> Self {
        Self {
            idf_table: IDFTable::new(),
            vocab: HashMap::new(),
        }
    }

    /// 分词 (简单中文按字+bigram，英文按空格)
    fn tokenize(text: &str) -> Vec<String> {
        let mut tokens = Vec::new();

        // 英文分词
        for word in text.split_whitespace() {
            let word = word.trim().to_lowercase();
            if word.len() >= 2 && !STOP_WORDS.contains(&word.as_str()) {
                tokens.push(word.clone());
            }
        }

        // 中文 bigram 分词
        let chars: Vec<char> = text.chars().collect();
        for i in 0..chars.len() - 1 {
            let bigram = format!("{}{}", chars[i], chars[i + 1]);
            if !STOP_WORDS.contains(&bigram.as_str()) && bigram.len() >= 2 {
                tokens.push(bigram);
            }
        }

        tokens
    }

    /// 生成 TF 向量
    fn compute_tf(tokens: &[String]) -> HashMap<String, f64> {
        let mut tf = HashMap::new();
        let total = tokens.len() as f64;

        for token in tokens {
            *tf.entry(token.clone()).or_insert(0.0) += 1.0;
        }

        for count in tf.values_mut() {
            *count /= total;
        }

        tf
    }

    /// 生成 TF-IDF 向量 (128维)
    pub fn fit_transform(&mut self, text: &str) -> Vec<f64> {
        let tokens = Self::tokenize(text);
        let tf = Self::compute_tf(&tokens);

        // 构建 vocab (如果为空)
        if self.vocab.is_empty() {
            let feature_terms = vec![
                "fatigue", "hcf", "lcf", "tmf", "creep", "stress", "strain", "displacement",
                "mesh", "material", "tc4", "ti-6al-4v", "steel", "aluminum", "titanium",
                "modulus", "elastic", "yield", "strength", "buckling", "modal", "thermal",
                "simulation", "analysis", "result", "error", "convergence", "iterations",
                "boundary", "condition", "load", "pressure", "temperature", "force",
                "carbon", "inconel", "nickel", "chrome", "molybdenum",
                "fracture", "crack", "propagation", "wear", "corrosion",
                "lammps", "vasp", "calculix", "abaqus", "openfoam",
                "static", "dynamic", "transient", "nonlinear", "linear",
                "von mises", "principal", "tresca", "mises",
                "element", "node", "geometry", "model", " solver",
                "rigid", "contact", "gap", "friction", "sliding",
                "refine", "quality", "aspect", "skewness", "jacobian",
                "plasticity", "viscoelastic", "hyperelastic", "creep",
                "damage", "fatigue", "lifetime", "sn_curve", "basquin",
                "residual", "stress", "concentration", "notch", "factor",
                "safety", "factor", "allowable", "limit", "design",
                "optimization", "topology", "size", "shape", "parameter",
                "sensitivity", "analysis", "robust", "reliability",
            ];
            for (i, term) in feature_terms.iter().enumerate() {
                self.vocab.insert(term.to_string(), i);
            }
        }

        // 生成 128 维向量
        let mut vector = vec![0.0; 128];
        let vocab_size = self.vocab.len().max(128);

        for (term, tf_val) in &tf {
            let idf_val = self.idf_table.get(term);
            let tfidf = tf_val * idf_val;

            if let Some(&idx) = self.vocab.get(term) {
                if idx < 128 {
                    vector[idx] = tfidf;
                }
            } else if vocab_size < 128 {
                // 动态扩展 vocab
                let idx = vocab_size;
                self.vocab.insert(term.clone(), idx);
            }
        }

        // L2 归一化
        let norm = vector.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm > 0.0 {
            for v in vector.iter_mut() {
                *v /= norm;
            }
        }

        vector
    }

    /// 将向量转换为 JSON 字符串
    pub fn vector_to_json(vector: &[f64]) -> String {
        serde_json::to_string(vector).unwrap_or_else(|_| "[]".to_string())
    }

    /// 将 JSON 字符串转换为向量
    pub fn json_to_vector(json: &str) -> Vec<f64> {
        serde_json::from_str(json).unwrap_or_else(|_| Vec::new())
    }
}

/// 计算余弦相似度
pub fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for i in 0..a.len() {
        dot += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }

    let norm_product = norm_a.sqrt() * norm_b.sqrt();
    if norm_product == 0.0 {
        return 0.0;
    }

    dot / norm_product
}

// ============================================================================
// V3.10-002: 向量存储表结构
// ============================================================================

/// 向量嵌入记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbeddingRecordRow {
    pub id: String,
    pub source_type: String,
    pub project_id: Option<String>,
    pub user_id: Option<String>,
    pub content: String,
    pub embedding_json: String,
    pub tags_json: String,
    pub created_at: String,
    pub last_accessed_at: Option<i64>,
    pub access_count: i32,
    pub metadata_json: Option<String>,
}

/// 写入嵌入记录请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbeddingWriteRequest {
    pub source_type: String,
    pub project_id: Option<String>,
    pub user_id: Option<String>,
    pub content: String,
    pub tags: Vec<String>,
    pub metadata_json: Option<String>,
}

/// 向量搜索结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VectorSearchResultRow {
    pub id: String,
    pub source_type: String,
    pub project_id: Option<String>,
    pub user_id: Option<String>,
    pub content: String,
    pub tags_json: String,
    pub created_at: String,
    pub similarity: f64,
    pub rank: i32,
}

/// 主动回忆结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActiveRecallResultRow {
    pub records_json: String,
    pub context_completeness: f64,
    pub suggested_directions_json: String,
    pub source_summary: String,
}

/// 跨会话知识摘要
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CrossSessionSummaryRow {
    pub total_records: i32,
    pub top_sources_json: String,
    pub recent_topics_json: String,
    pub mastery_level: String,
    pub recommendations_json: String,
}

// ============================================================================
// 数据库操作
// ============================================================================

/// 创建向量存储表
pub fn create_vector_store_table(conn: &Connection) -> SqliteResult<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS vector_store (
            id TEXT PRIMARY KEY,
            source_type TEXT NOT NULL,
            project_id TEXT,
            user_id TEXT,
            content TEXT NOT NULL,
            embedding_json TEXT NOT NULL,
            tags_json TEXT NOT NULL,
            created_at TEXT NOT NULL,
            last_accessed_at INTEGER,
            access_count INTEGER NOT NULL DEFAULT 0,
            metadata_json TEXT,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE SET NULL,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // 向量存储索引 (用于加速搜索)
    conn.execute("CREATE INDEX IF NOT EXISTS idx_vector_source ON vector_store(source_type)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_vector_project ON vector_store(project_id)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_vector_user ON vector_store(user_id)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_vector_created ON vector_store(created_at DESC)", [])?;

    Ok(())
}

/// 写入嵌入记录
pub fn write_embedding(conn: &Connection, req: &EmbeddingWriteRequest) -> Result<EmbeddingRecordRow, String> {
    let id = nanoid::nanoid!(12);
    let now = chrono::Utc::now().to_rfc3339();

    // 生成 TF-IDF 向量
    let mut vectorizer = TFIDFVectorizer::new();
    let embedding = vectorizer.fit_transform(&req.content);
    let embedding_json = TFIDFVectorizer::vector_to_json(&embedding);

    let tags_json = serde_json::to_string(&req.tags).unwrap_or_else(|_| "[]".to_string());

    conn.execute(
        "INSERT INTO vector_store
         (id, source_type, project_id, user_id, content, embedding_json, tags_json, created_at, access_count, metadata_json)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 0, ?9)",
        rusqlite::params![
            &id, &req.source_type, &req.project_id, &req.user_id, &req.content,
            &embedding_json, &tags_json, &now, &req.metadata_json
        ],
    ).map_err(|e| format!("写入嵌入记录失败: {}", e))?;

    Ok(EmbeddingRecordRow {
        id,
        source_type: req.source_type.clone(),
        project_id: req.project_id.clone(),
        user_id: req.user_id.clone(),
        content: req.content.clone(),
        embedding_json,
        tags_json,
        created_at: now,
        last_accessed_at: None,
        access_count: 0,
        metadata_json: req.metadata_json.clone(),
    })
}

/// 批量写入嵌入记录
pub fn db_batch_write_embeddings(conn: &Connection, requests: Vec<EmbeddingWriteRequest>) -> Result<i32, String> {
    let mut count = 0i32;
    for req in requests {
        if write_embedding(conn, &req).is_ok() {
            count += 1;
        }
    }
    Ok(count)
}

/// 向量搜索
pub fn search_vectors(
    conn: &Connection,
    query: &str,
    scope: &str,
    project_id: Option<&str>,
    user_id: Option<&str>,
    limit: i32,
    similarity_threshold: f64,
    source_types: Option<Vec<String>>,
) -> Result<Vec<VectorSearchResultRow>, String> {
    // 生成查询向量
    let mut vectorizer = TFIDFVectorizer::new();
    let query_vector = vectorizer.fit_transform(query);

    // 构建 SQL 查询
    let mut sql = String::from("SELECT id, source_type, project_id, user_id, content, embedding_json, tags_json, created_at FROM vector_store WHERE 1=1");
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    match scope {
        "project" => {
            if let Some(pid) = project_id {
                sql.push_str(" AND project_id = ?");
                params.push(Box::new(pid.to_string()));
            }
        }
        "user" => {
            if let Some(uid) = user_id {
                sql.push_str(" AND user_id = ?");
                params.push(Box::new(uid.to_string()));
            }
        }
        _ => {} // global: 不加过滤
    }

    if let Some(types) = source_types {
        if !types.is_empty() {
            sql.push_str(" AND source_type IN (");
            for (i, t) in types.iter().enumerate() {
                if i > 0 { sql.push(','); }
                sql.push_str(&format!("?{}", i + params.len() + 1));
                params.push(Box::new(t.clone()));
            }
            sql.push(')');
        }
    }

    sql.push_str(" ORDER BY created_at DESC LIMIT 100"); // 预取候选

    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    let mut stmt = conn.prepare(&sql).map_err(|e| format!("查询失败: {}", e))?;

    let mut results = Vec::new();
    let rows = stmt.query_map(param_refs.as_slice(), |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, Option<String>>(2)?,
            row.get::<_, Option<String>>(3)?,
            row.get::<_, String>(4)?,
            row.get::<_, String>(5)?,
            row.get::<_, String>(6)?,
            row.get::<_, String>(7)?,
        ))
    }).map_err(|e| format!("读取行失败: {}", e))?;

    let mut candidates: Vec<(VectorSearchResultRow, f64)> = Vec::new();

    for row in rows {
        let (id, source_type, proj_id, usr_id, content, embedding_json, tags_json, created_at) =
            row.map_err(|e| format!("解析行失败: {}", e))?;

        let embedding = TFIDFVectorizer::json_to_vector(&embedding_json);
        let similarity = cosine_similarity(&query_vector, &embedding);

        if similarity >= similarity_threshold {
            candidates.push((VectorSearchResultRow {
                id,
                source_type,
                project_id: proj_id,
                user_id: usr_id,
                content,
                tags_json,
                created_at,
                similarity,
                rank: 0,
            }, similarity));
        }
    }

    // 按相似度降序排序
    candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // 取 top N
    for (i, (mut result, _)) in candidates.into_iter().enumerate() {
        if i >= limit as usize { break; }
        result.rank = (i + 1) as i32;
        results.push(result);
    }

    Ok(results)
}

/// 更新访问记录
pub fn update_access_count(conn: &Connection, id: &str) -> Result<(), String> {
    let now = chrono::Utc::now().timestamp();
    conn.execute(
        "UPDATE vector_store SET access_count = access_count + 1, last_accessed_at = ?1 WHERE id = ?2",
        rusqlite::params![now, id],
    ).map_err(|e| format!("更新访问记录失败: {}", e))?;
    Ok(())
}

/// 删除嵌入记录
pub fn delete_embedding(conn: &Connection, id: &str) -> Result<(), String> {
    conn.execute("DELETE FROM vector_store WHERE id = ?1", [id])
        .map_err(|e| format!("删除失败: {}", e))?;
    Ok(())
}

/// 主动回忆
pub fn db_active_recall(
    conn: &Connection,
    simulation_type: &str,
    material: Option<&str>,
    project_id: Option<&str>,
    user_id: Option<&str>,
    limit: i32,
) -> Result<ActiveRecallResultRow, String> {
    // 构建查询文本 (模拟前自动检索相关历史)
    let mut query_parts = vec![simulation_type.to_string()];
    if let Some(mat) = material {
        query_parts.push(mat.to_string());
    }

    // 从历史记录中学习标签
    let _tags = if let Some(mat) = material {
        vec![mat.to_string(), simulation_type.to_string()]
    } else {
        vec![simulation_type.to_string()]
    };

    // 先搜索相关记录
    let search_results = search_vectors(
        conn,
        &query_parts.join(" "),
        "global",
        project_id,
        user_id,
        limit,
        0.3,
        None,
    ).unwrap_or_default();

    // 计算上下文完整性
    let content_lengths: Vec<usize> = search_results.iter()
        .map(|r| r.content.len())
        .collect();
    let avg_content_len = if content_lengths.is_empty() {
        0.0
    } else {
        content_lengths.iter().sum::<usize>() as f64 / content_lengths.len() as f64
    };
    let context_completeness = (search_results.len() as f64 / 5.0).min(1.0) * (avg_content_len / 500.0).min(1.0);

    // 建议补充方向
    let mut suggested_directions = Vec::new();
    if search_results.len() < 3 {
        suggested_directions.push("补充更多仿真结果记录".to_string());
    }
    if search_results.iter().all(|r| r.source_type != "failure_analysis") {
        suggested_directions.push("补充失效分析案例".to_string());
    }
    if search_results.iter().all(|r| !r.content.contains("热") && !r.content.contains("thermal")) {
        suggested_directions.push("补充热分析相关知识".to_string());
    }

    let suggested_json = serde_json::to_string(&suggested_directions).unwrap_or_else(|_| "[]".to_string());
    let records_json = serde_json::to_string(&search_results).unwrap_or_else(|_| "[]".to_string());
    let source_summary = format!("找到 {} 条相关记录，来源类型: {}",
        search_results.len(),
        search_results.iter().map(|r| r.source_type.as_str()).collect::<std::collections::HashSet<_>>().len()
    );

    Ok(ActiveRecallResultRow {
        records_json,
        context_completeness,
        suggested_directions_json: suggested_json,
        source_summary,
    })
}

/// 跨会话知识摘要
pub fn db_cross_session_summary(
    conn: &Connection,
    user_id: &str,
) -> Result<CrossSessionSummaryRow, String> {
    // 统计该用户的嵌入记录
    let mut stmt = conn.prepare(
        "SELECT COUNT(*), source_type FROM vector_store WHERE user_id = ?1 GROUP BY source_type ORDER BY COUNT(*) DESC"
    ).map_err(|e| format!("查询失败: {}", e))?;

    let mut source_counts: HashMap<String, i32> = HashMap::new();
    let mut total = 0i32;

    let rows = stmt.query_map([user_id], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
    }).map_err(|e| format!("读取失败: {}", e))?;

    for row in rows {
        let (count, source) = row.map_err(|e| format!("解析失败: {}", e))?;
        total += count;
        *source_counts.entry(source).or_insert(0) += count;
    }

    // Top 来源
    let mut top_sources: Vec<(String, i32)> = source_counts.into_iter().collect();
    top_sources.sort_by(|a, b| b.1.cmp(&a.1));
    let top_sources: Vec<String> = top_sources.iter().take(3).map(|(s, _)| s.clone()).collect();

    // 最近主题 (从 tags 提取)
    let mut recent_stmt = conn.prepare(
        "SELECT tags_json FROM vector_store WHERE user_id = ?1 ORDER BY created_at DESC LIMIT 20"
    ).map_err(|e| format!("查询失败: {}", e))?;

    let mut tag_counts: HashMap<String, i32> = HashMap::new();
    let recent_rows = recent_stmt.query_map([user_id], |row| {
        Ok(row.get::<_, String>(0)?)
    }).map_err(|e| format!("读取失败: {}", e))?;

    for row in recent_rows {
        if let Ok(tags_json) = row {
            if let Ok(tags) = serde_json::from_str::<Vec<String>>(&tags_json) {
                for tag in tags {
                    *tag_counts.entry(tag).or_insert(0) += 1;
                }
            }
        }
    }

    let mut recent_topics_vec: Vec<(String, i32)> = tag_counts.into_iter()
        .filter(|(_, c)| *c >= 2)
        .collect();
    recent_topics_vec.sort_by(|a, b| b.1.cmp(&a.1));
    let recent_topics: Vec<String> = recent_topics_vec.into_iter()
        .take(5)
        .map(|(t, _)| t)
        .collect();

    // 熟练度判断
    let mastery_level = if total < 10 {
        "beginner"
    } else if total < 30 {
        "intermediate"
    } else if total < 100 {
        "advanced"
    } else {
        "expert"
    };

    // 推荐
    let mut recommendations = Vec::new();
    if recent_topics.is_empty() {
        recommendations.push("开始记录您的第一个仿真分析".to_string());
    }
    if !recent_topics.contains(&"fatigue".to_string()) {
        recommendations.push("补充疲劳分析相关知识".to_string());
    }
    if !recent_topics.contains(&"tc4".to_string()) && !recent_topics.contains(&"titanium".to_string()) {
        recommendations.push("了解 TC4 钛合金的仿真方法".to_string());
    }

    Ok(CrossSessionSummaryRow {
        total_records: total,
        top_sources_json: serde_json::to_string(&top_sources).unwrap_or_else(|_| "[]".to_string()),
        recent_topics_json: serde_json::to_string(&recent_topics).unwrap_or_else(|_| "[]".to_string()),
        mastery_level: mastery_level.to_string(),
        recommendations_json: serde_json::to_string(&recommendations).unwrap_or_else(|_| "[]".to_string()),
    })
}

// ============================================================================
// Tauri 命令
// ============================================================================

use crate::db::Database;

/// 写入嵌入记录
#[tauri::command]
pub async fn write_embedding_record(
    db: tauri::State<'_, Database>,
    request: EmbeddingWriteRequest,
) -> Result<EmbeddingRecordRow, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    write_embedding(&conn, &request)
}

/// 批量写入嵌入记录
#[tauri::command]
pub async fn batch_write_embeddings(
    db: tauri::State<'_, Database>,
    requests: Vec<EmbeddingWriteRequest>,
) -> Result<i32, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db_batch_write_embeddings(&conn, requests)
}

/// 向量搜索
#[tauri::command]
pub async fn search_vector_store(
    db: tauri::State<'_, Database>,
    query: String,
    scope: String,
    project_id: Option<String>,
    user_id: Option<String>,
    limit: Option<i32>,
    similarity_threshold: Option<f64>,
    source_types: Option<Vec<String>>,
) -> Result<Vec<VectorSearchResultRow>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    search_vectors(
        &conn,
        &query,
        &scope,
        project_id.as_deref(),
        user_id.as_deref(),
        limit.unwrap_or(5),
        similarity_threshold.unwrap_or(0.5),
        source_types,
    )
}

/// 主动回忆
#[tauri::command]
pub async fn active_recall(
    db: tauri::State<'_, Database>,
    simulation_type: String,
    material: Option<String>,
    project_id: Option<String>,
    user_id: Option<String>,
    limit: Option<i32>,
) -> Result<ActiveRecallResultRow, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db_active_recall(
        &conn,
        &simulation_type,
        material.as_deref(),
        project_id.as_deref(),
        user_id.as_deref(),
        limit.unwrap_or(5),
    )
}

/// 跨会话知识摘要
#[tauri::command]
pub async fn get_cross_session_summary(
    db: tauri::State<'_, Database>,
    user_id: String,
) -> Result<CrossSessionSummaryRow, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db_cross_session_summary(&conn, &user_id)
}

/// 删除嵌入记录
#[tauri::command]
pub async fn delete_embedding_record(
    db: tauri::State<'_, Database>,
    id: String,
) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    delete_embedding(&conn, &id)
}

/// 更新访问计数
#[tauri::command]
pub async fn update_embedding_access(
    db: tauri::State<'_, Database>,
    id: String,
) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    update_access_count(&conn, &id)
}