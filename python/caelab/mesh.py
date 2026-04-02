"""
网格模块 — 生成、导入、查询网格
"""

from __future__ import annotations

from typing import Any

from ._client import get_client


class Mesh:
    """CAELab 网格对象

    Parameters
    ----------
    project_id : str
        关联的项目 ID。
    """

    def __init__(self, project_id: str) -> None:
        self.project_id: str = project_id
        self._mesh_id: str | None = None
        self._stats: dict[str, Any] = {}

    # ------------------------------------------------------------------
    # 结构化网格
    # ------------------------------------------------------------------

    def generate_structured(
        self,
        nx: int,
        ny: int,
        nz: int,
        x_min: float = 0.0,
        x_max: float = 1.0,
        y_min: float = 0.0,
        y_max: float = 1.0,
        z_min: float = 0.0,
        z_max: float = 1.0,
        element_type: str = "C3D8",
    ) -> dict[str, Any]:
        """生成结构化六面体网格

        Parameters
        ----------
        nx, ny, nz : int
            各方向网格划分数。
        x_min, x_max, y_min, y_max, z_min, z_max : float
            网格区域边界。
        element_type : str
            单元类型，默认 ``"C3D8"``（8 节点六面体）。

        Returns
        -------
        dict
            包含 ``mesh_id``、``node_count``、``element_count`` 的字典。
        """
        resp = get_client().call(
            "generate_mesh",
            {
                "projectId": self.project_id,
                "elementType": element_type,
                "xDiv": nx,
                "yDiv": ny,
                "zDiv": nz,
                "xMin": x_min,
                "xMax": x_max,
                "yMin": y_min,
                "yMax": y_max,
                "zMin": z_min,
                "zMax": z_max,
            },
        )
        if not resp.get("success"):
            raise RuntimeError(resp.get("error", "Failed to generate mesh"))
        data = resp["data"]
        self._mesh_id = data.get("mesh_id")
        self._stats = data
        return data

    # ------------------------------------------------------------------
    # 文件导入
    # ------------------------------------------------------------------

    def import_file(self, path: str) -> dict[str, Any]:
        """导入外部网格文件（STEP / STL / INP）

        Parameters
        ----------
        path : str
            网格文件路径。

        Returns
        -------
        dict
            导入结果。
        """
        resp = get_client().call(
            "import_mesh",
            {"projectId": self.project_id, "path": path},
        )
        if not resp.get("success"):
            raise RuntimeError(resp.get("error", "Failed to import mesh"))
        data = resp["data"]
        self._mesh_id = data.get("mesh_id")
        self._stats = data
        return data

    # ------------------------------------------------------------------
    # 查询
    # ------------------------------------------------------------------

    def get_stats(self) -> dict[str, Any]:
        """获取网格统计信息

        Returns
        -------
        dict
            包含 ``node_count``、``element_count``、``element_types`` 等字段。
        """
        if self._mesh_id:
            resp = get_client().call(
                "get_mesh_stats", {"projectId": self.project_id, "meshId": self._mesh_id}
            )
            if resp.get("success"):
                self._stats = resp["data"]
        return self._stats

    # ------------------------------------------------------------------
    # 设置
    # ------------------------------------------------------------------

    def set_element_size(self, size: float) -> None:
        """设置全局单元尺寸

        Parameters
        ----------
        size : float
            目标单元尺寸。
        """
        resp = get_client().call(
            "set_element_size",
            {"projectId": self.project_id, "meshId": self._mesh_id, "size": size},
        )
        if not resp.get("success"):
            raise RuntimeError(resp.get("error", "Failed to set element size"))

    # ------------------------------------------------------------------
    # 魔术方法
    # ------------------------------------------------------------------

    def __repr__(self) -> str:
        return f"Mesh(project_id={self.project_id!r}, mesh_id={self._mesh_id!r})"
