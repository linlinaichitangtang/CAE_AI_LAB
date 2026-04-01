//! Solver facade — re-exports from src/solver/
//! Used by command modules that reference crate::commands::solver::*

pub use crate::solver::CalculiXSolver;
pub use crate::solver::mesh::GridConfig;
pub use crate::solver::mesh::MeshElementType;
pub use crate::solver::mesh::MeshError;
pub use crate::solver::mesh::MeshGenerator;
pub use crate::solver::mesh::MeshQualityMetrics;
pub use crate::solver::mesh::RefinementConfig;
pub use crate::solver::mesh::RefinementRegionType;
pub use crate::solver::mesh::StructuredMesh;
pub use crate::solver::SolverConfig;
pub use crate::solver::SolverError;
pub use crate::solver::SolverEvent;
pub use crate::solver::SolverResult;

pub mod mesh {
    pub use crate::solver::mesh::check_mesh_quality;
}

pub mod bc {
    pub use crate::solver::bc::BcContainer;
    pub use crate::solver::bc::BcType;
    pub use crate::solver::bc::Dof;
    pub use crate::solver::bc::FixedBc;
    pub use crate::solver::bc::LoadDirection;
    pub use crate::solver::bc::PointLoad;
    pub use crate::solver::bc::UniformLoad;
    pub use crate::solver::bc::UniformLoadType;
}
