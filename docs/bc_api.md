# 边界条件模块 (Boundary Conditions Module)

## 概述

边界条件模块 `bc.rs` 实现了 CAE 仿真中的边界条件施加功能，包括：

- **固定约束 (Fixed Constraint)**: 完全固定、铰接、滚轴等
- **集中力载荷 (Point Load)**: 点载荷，支持 X/Y/Z 方向和自定义向量
- **均布载荷 (Uniform Load)**: 面载荷，压力、牵引力、热通量等

## 核心类型

### 自由度 (DOF)

```rust
pub enum Dof {
    TranslationX = 1,  // X方向平移
    TranslationY = 2,  // Y方向平移
    TranslationZ = 3,  // Z方向平移
    RotationX = 4,      // X轴旋转
    RotationY = 5,      // Y轴旋转
    RotationZ = 6,      // Z轴旋转
    Temperature = 11,    // 温度
}
```

### 边界条件类型 (BcType)

| 类型 | 约束的DOF | 描述 |
|------|----------|------|
| `Fixed` / `Encastre` | 全部6个 | 完全固定 |
| `Pinned` | TX, TY, TZ | 铰接（固定位移，释放旋转） |
| `RollerX` | TY, TZ | X方向滚轴 |
| `RollerY` | TX, TZ | Y方向滚轴 |
| `RollerZ` | TX, TY | Z方向滚轴 |
| `Symmetry` | TY, TZ, RX | 对称约束 |
| `Custom` | 用户指定 | 自定义DOF约束 |

### 载荷方向 (LoadDirection)

```rust
pub enum LoadDirection {
    X,           // X方向
    Y,           // Y方向
    Z,           // Z方向
    Normal,      // 面法向
    Custom(f64, f64, f64),  // 自定义向量 (fx, fy, fz)
}
```

## API 使用示例

### 1. 创建固定约束

```rust
use crate::solver::bc::*;

// 完全固定
let fixed = FixedBc::new("Fixed-Left", vec![1, 2, 3, 4]);

// 铰接约束
let pinned = FixedBc::pinned("Pinned-Support", vec![5, 6, 7, 8]);

// 滚轴约束 (Z方向释放)
let roller = FixedBc::roller_z("Roller", vec![9, 10, 11, 12]);

// 自定义约束
let custom = FixedBc::custom("Custom", vec![1, 2], vec![Dof::TranslationX, Dof::TranslationZ]);
```

### 2. 创建集中力载荷

```rust
use crate::solver::bc::*;

// Z方向点载荷
let load_z = PointLoad::z("Load-Z", 100, 5000.0);

// Y方向点载荷
let load_y = PointLoad::y("Load-Y", 100, 5000.0);

// 自定义向量载荷
let vector_load = PointLoad::vector("Vector-Load", 100, 1.0, 0.5, 0.0);
```

### 3. 创建均布载荷

```rust
use crate::solver::bc::*;

// 压力载荷
let pressure = UniformLoad::pressure("Pressure-Load", "S1".to_string(), 0.1);

// 牵引力载荷
let traction = UniformLoad::traction(
    "Traction-Load", 
    "S2".to_string(), 
    LoadDirection::Y, 
    100.0
);

// 热通量载荷
let heat_flux = UniformLoad::heat_flux("Heat-Flux", "S3".to_string(), 50.0);
```

### 4. 使用边界条件容器

```rust
use crate::solver::bc::*;

// 创建容器
let mut container = BcContainer::new();

// 添加边界条件
container.add_fixed(FixedBc::encastre("Fixed", vec![1, 2, 3, 4]));
container.add_point_load(PointLoad::z("Load", 100, 5000.0));
container.add_uniform_load(UniformLoad::pressure("P1", "S1".to_string(), 0.1));

// 生成INP格式输出
let inp_content = container.to_inp();
```

### 5. 悬臂梁典型应用

```rust
use crate::solver::bc::*;
use crate::commands::input_gen::Node;

// 假设有一组节点
let nodes: Vec<Node> = /* ... */;

// 创建固定端约束 (左端 x=0)
let fixed_nodes: Vec<usize> = nodes.iter()
    .filter(|n| n.x.abs() < 1e-6)
    .map(|n| n.id)
    .collect();
let fixed_bc = FixedBc::encastre("Cantilever-Fixed", fixed_nodes);

// 创建加载端点载荷 (右端 x=L)
let load_node = nodes.iter()
    .filter(|n| (n.x - max_x).abs() < 1e-6)
    .max_by(|a, b| a.y.partial_cmp(&b.y).unwrap())
    .map(|n| n.id)
    .unwrap();
let point_load = PointLoad::y("Cantilever-Load", load_node, 1000.0);
```

## Tauri 前端 API

### 创建固定边界条件

```javascript
const fixedBc = await invoke('create_fixed_bc', {
    name: "Fixed-Support",
    nodeIds: [1, 2, 3, 4],
    bcType: "fixed"  // "fixed", "encastre", "pinned", "roller_x", etc.
});
```

### 创建点载荷

```javascript
const pointLoad = await invoke('create_point_load', {
    name: "Point-Load-1",
    nodeId: 100,
    magnitude: 5000,
    direction: "y"  // "x", "y", "z", "normal"
});
```

### 创建均布载荷

```javascript
const pressureLoad = await invoke('create_pressure_load', {
    name: "Pressure-Load",
    surfaceName: "S1",
    magnitude: 0.1
});
```

### 查找特定面的节点

```javascript
// 查找 x=0 面上的所有节点
const fixedNodes = await invoke('find_nodes_at_face', {
    nodes: meshNodes,
    axis: "x",
    value: 0.0,
    tolerance: 0.000001
});

// 创建固定约束
const fixedBc = await invoke('create_fixed_bc', {
    name: "Fixed-Left",
    nodeIds: fixedNodes,
    bcType: "encastre"
});
```

### 生成完整INP文件

```javascript
const outputPath = await invoke('generate_complete_inp', {
    nodes: meshNodes,
    elements: meshElements,
    material: { name: "Steel", youngsModulus: 200000, poissonRatio: 0.3 },
    fixedBc: fixedBc,
    pointLoad: pointLoad,
    uniformLoads: [],
    outputPath: "/path/to/model.inp"
});
```

## INP 格式输出

### 固定约束格式

```
*BOUNDARY
1, 1, 2, 3, 4, 5, 6, 0
2, 1, 2, 3, 4, 5, 6, 0
```

### 点载荷格式 (CLOAD)

```
*CLOAD
100, 2, 5000.000000
```

### 均布载荷格式 (DLOAD)

```
*DLOAD
S1, P, 0.100000
```

## 辅助函数

```rust
use crate::solver::bc::helpers::*;

// 在特定坐标面创建固定约束
let left_face = fixed_left_face(&nodes, 0.0, 1e-6);
let right_face = fixed_right_face(&nodes, max_x, 1e-6);
let bottom_face = fixed_bottom_face(&nodes);

// 创建标准点载荷
let load = point_load_on_node(100, 1000.0, LoadDirection::Y);

// 创建压力载荷
let pressure = pressure_on_face("S1".to_string(), 0.1);

// 生成重力载荷
let gravity_inp = gravity_load(9.81, LoadDirection::Y);
```