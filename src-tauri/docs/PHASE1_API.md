# CAELab Phase 1 后端数据层 - API 文档

## 技术栈
- **语言**: Rust
- **框架**: Tauri 2.x
- **数据库**: SQLite (rusqlite, bundled)
- **依赖**: uuid, chrono, serde, tracing

## 数据结构

### Project (项目)
```rust
pub struct Project {
    pub id: String,           // UUID
    pub name: String,        // 项目名称
    pub description: String, // 项目描述
    pub created_at: String,  // RFC3339 时间戳
    pub updated_at: String,  // RFC3339 时间戳
}
```

### ProjectFile (项目文件)
```rust
pub struct ProjectFile {
    pub id: String,           // UUID
    pub project_id: String,  // 所属项目ID
    pub file_type: FileType, // 文件类型
    pub file_name: String,   // 文件名
    pub content: Option<String>, // 文件内容
    pub file_path: String,   // 文件路径
    pub created_at: String,  // 创建时间
    pub updated_at: String,  // 更新时间
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FileType {
    Note,         // 笔记
    ModelingData, // 建模数据
    CodeFile,    // 代码文件
}
```

### ProjectSettings (项目配置)
```rust
pub struct ProjectSettings {
    pub id: String,           // UUID
    pub project_id: String,  // 项目ID
    pub settings_json: String, // JSON配置
    pub updated_at: String,  // 更新时间
}
```

### 输入结构
```rust
// 创建项目
pub struct CreateProjectInput {
    pub name: String,
    pub description: String,
}

// 更新项目
pub struct UpdateProjectInput {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

// 创建文件
pub struct CreateFileInput {
    pub project_id: String,
    pub file_type: FileType,
    pub file_name: String,
    pub content: Option<String>,
    pub file_path: String,
}

// 更新文件
pub struct UpdateFileInput {
    pub id: String,
    pub file_name: Option<String>,
    pub content: Option<String>,
}

// 保存配置
pub struct SaveSettingsInput {
    pub project_id: String,
    pub settings_json: String,
}
```

## 命令函数列表

### 项目管理 (commands/project.rs)

#### create_project
```rust
#[command]
pub fn create_project(
    db: State<'_, Database>,
    input: CreateProjectInput,
) -> Result<Project, String>
```
- **功能**: 创建新项目
- **参数**: name (必填), description (必填)
- **返回**: Project 对象

#### list_projects
```rust
#[command]
pub fn list_projects(db: State<'_, Database>) -> Result<Vec<Project>, String>
```
- **功能**: 列出所有项目
- **返回**: Vec<Project> 按更新时间倒序

#### get_project
```rust
#[command]
pub fn get_project(db: State<'_, Database>, id: String) -> Result<Project, String>
```
- **功能**: 获取单个项目
- **参数**: id (项目ID)
- **返回**: Project 对象

#### update_project
```rust
#[command]
pub fn update_project(
    db: State<'_, Database>,
    input: UpdateProjectInput,
) -> Result<Project, String>
```
- **功能**: 更新项目 (重命名/修改描述)
- **参数**: id (必填), name (可选), description (可选)
- **返回**: 更新后的 Project 对象

#### delete_project
```rust
#[command]
pub fn delete_project(db: State<'_, Database>, id: String) -> Result<(), String>
```
- **功能**: 删除项目 (级联删除文件与配置)
- **参数**: id (项目ID)

### 文件存储 (commands/file.rs)

#### create_file
```rust
#[command]
pub fn create_file(
    db: State<'_, Database>,
    input: CreateFileInput,
) -> Result<ProjectFile, String>
```
- **功能**: 创建新文件 (笔记/建模数据/代码)
- **参数**: project_id, file_type, file_name, content, file_path
- **返回**: ProjectFile 对象

#### list_files
```rust
#[command]
pub fn list_files(
    db: State<'_, Database>,
    project_id: String,
) -> Result<Vec<ProjectFile>, String>
```
- **功能**: 列出项目所有文件
- **参数**: project_id
- **返回**: Vec<ProjectFile> 按更新时间倒序

#### get_file
```rust
#[command]
pub fn get_file(db: State<'_, Database>, id: String) -> Result<ProjectFile, String>
```
- **功能**: 获取单个文件
- **参数**: id (文件ID)
- **返回**: ProjectFile 对象

#### update_file
```rust
#[command]
pub fn update_file(
    db: State<'_, Database>,
    input: UpdateFileInput,
) -> Result<ProjectFile, String>
```
- **功能**: 更新文件 (重命名/修改内容)
- **参数**: id (必填), file_name (可选), content (可选)
- **返回**: 更新后的 ProjectFile 对象

#### delete_file
```rust
#[command]
pub fn delete_file(db: State<'_, Database>, id: String) -> Result<(), String>
```
- **功能**: 删除文件
- **参数**: id (文件ID)

#### read_file_content
```rust
#[command]
pub fn read_file_content(db: State<'_, Database>, id: String) -> Result<String, String>
```
- **功能**: 读取文件内容
- **参数**: id (文件ID)
- **返回**: 文件内容字符串

#### write_file_content
```rust
#[command]
pub fn write_file_content(
    db: State<'_, Database>,
    id: String,
    content: String,
) -> Result<(), String>
```
- **功能**: 写入文件内容
- **参数**: id (文件ID), content (内容)

### 项目配置 (commands/settings.rs)

#### save_settings
```rust
#[command]
pub fn save_settings(
    db: State<'_, Database>,
    input: SaveSettingsInput,
) -> Result<ProjectSettings, String>
```
- **功能**: 保存项目配置 (JSON)
- **参数**: project_id, settings_json
- **返回**: ProjectSettings 对象

#### get_settings
```rust
#[command]
pub fn get_settings(
    db: State<'_, Database>,
    project_id: String,
) -> Result<ProjectSettings, String>
```
- **功能**: 获取项目配置
- **参数**: project_id
- **返回**: ProjectSettings 对象

#### delete_settings
```rust
#[command]
pub fn delete_settings(db: State<'_, Database>, project_id: String) -> Result<(), String>
```
- **功能**: 删除项目配置
- **参数**: project_id

## 数据库表

### projects (项目表)
```sql
CREATE TABLE projects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
```

### project_files (文件表)
```sql
CREATE TABLE project_files (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    file_type TEXT NOT NULL,
    file_name TEXT NOT NULL,
    content TEXT,
    file_path TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);
```

### project_settings (配置表)
```sql
CREATE TABLE project_settings (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL UNIQUE,
    settings_json TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);
```

## Tauri 注册命令
```rust
.invoke_handler(tauri::generate_handler![
    // Project
    commands::project::create_project,
    commands::project::list_projects,
    commands::project::get_project,
    commands::project::update_project,
    commands::project::delete_project,
    // File
    commands::file::create_file,
    commands::file::list_files,
    commands::file::get_file,
    commands::file::update_file,
    commands::file::delete_file,
    commands::file::read_file_content,
    commands::file::write_file_content,
    // Settings
    commands::settings::save_settings,
    commands::settings::get_settings,
    commands::settings::delete_settings,
])
```

## 文件位置
- 数据库: `{app_data_dir}/caelab.db`
- 源代码: `projects/cae/src-tauri/src/`
  - lib.rs (主入口)
  - commands/project.rs (项目管理)
  - commands/file.rs (文件存储)
  - commands/settings.rs (配置持久化)
  - db/mod.rs (数据库层)
  - models/mod.rs (数据结构)
