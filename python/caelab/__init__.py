"""
CAELab Python API
CAE engineering simulation platform
"""

__version__ = "1.2.0"

from .project import Project
from .mesh import Mesh
from .material import Material
from .simulation import Simulation
from .result import Result

__all__ = [
    "Project",
    "Mesh",
    "Material",
    "Simulation",
    "Result",
]
