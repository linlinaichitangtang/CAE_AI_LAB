"""
仿真模块 — 配置分析类型、边界条件、荷载并运行求解
"""

from __future__ import annotations

from typing import Any

from ._client import get_client
from .result import Result


class Simulation:
    """CAELab 仿真对象

    Parameters
    ----------
    project_id : str
        关联的项目 ID。
    """

    def __init__(self, project_id: str) -> None:
        self.project_id: str = project_id
        self._analysis_type: str = "static"
        self._boundary_conditions: list[dict[str, Any]] = []
        self._loads: list[dict[str, Any]] = []

    # ------------------------------------------------------------------
    # 分析类型
    # ------------------------------------------------------------------

    def set_analysis_type(self, analysis_type: str) -> None:
        """设置分析类型

        Parameters
        ----------
        analysis_type : str
            分析类型，可选值：
            ``"static"`` | ``"modal"`` | ``"buckling"`` | ``"thermal"`` |
            ``"frequency_response"`` | ``"transient"`` | ``"cfd"``。
        """
        valid = {"static", "modal", "buckling", "thermal", "frequency_response", "transient", "cfd"}
        if analysis_type not in valid:
            raise ValueError(f"Invalid analysis type '{analysis_type}'. Valid: {valid}")
        self._analysis_type = analysis_type

    # ------------------------------------------------------------------
    # 边界条件
    # ------------------------------------------------------------------

    def add_fixed_bc(self, node_ids: list[int]) -> None:
        """添加固定边界条件

        Parameters
        ----------
        node_ids : list[int]
            固定的节点 ID 列表。
        """
        bc = {"type": "fixed", "node_ids": node_ids}
        self._boundary_conditions.append(bc)

    def add_displacement_bc(
        self,
        node_ids: list[int],
        ux: float | None = None,
        uy: float | None = None,
        uz: float | None = None,
    ) -> None:
        """添加指定位移边界条件

        Parameters
        ----------
        node_ids : list[int]
            节点 ID 列表。
        ux, uy, uz : float | None
            各方向指定位移，None 表示自由。
        """
        bc = {
            "type": "displacement",
            "node_ids": node_ids,
            "ux": ux,
            "uy": uy,
            "uz": uz,
        }
        self._boundary_conditions.append(bc)

    def add_symmetry_bc(self, plane: str, offset: float = 0.0) -> None:
        """添加对称边界条件

        Parameters
        ----------
        plane : str
            对称面，``"xy"`` | ``"xz"`` | ``"yz"``。
        offset : float
            对称面偏移量。
        """
        bc = {"type": "symmetry", "plane": plane, "offset": offset}
        self._boundary_conditions.append(bc)

    # ------------------------------------------------------------------
    # 荷载
    # ------------------------------------------------------------------

    def add_load(
        self,
        node_ids: list[int],
        values: list[float],
        load_type: str = "force",
    ) -> None:
        """添加荷载

        Parameters
        ----------
        node_ids : list[int]
            受载节点 ID 列表。
        values : list[float]
            荷载值列表，每个元素为 [Fx, Fy, Fz]。
        load_type : str
            荷载类型，``"force"`` | ``"pressure"`` | ``"temperature"``。
        """
        load = {
            "type": load_type,
            "node_ids": node_ids,
            "values": values,
        }
        self._loads.append(load)

    def add_pressure(self, surface_name: str, magnitude: float) -> None:
        """添加面压力荷载

        Parameters
        ----------
        surface_name : str
            受压面名称。
        magnitude : float
            压力值 (Pa)，正值表示压。
        """
        load = {
            "type": "pressure",
            "surface": surface_name,
            "magnitude": magnitude,
        }
        self._loads.append(load)

    def add_gravity(self, magnitude: float = 9.81, direction: str = "z") -> None:
        """添加重力荷载

        Parameters
        ----------
        magnitude : float
            重力加速度 (m/s^2)。
        direction : str
            方向，``"x"`` | ``"y"`` | ``"z"``。
        """
        load = {
            "type": "gravity",
            "magnitude": magnitude,
            "direction": direction,
        }
        self._loads.append(load)

    # ------------------------------------------------------------------
    # 求解
    # ------------------------------------------------------------------

    def run(self) -> Result:
        """运行仿真求解

        Returns
        -------
        Result
            仿真结果对象。
        """
        resp = get_client().call(
            "run_simulation",
            {
                "projectId": self.project_id,
                "analysisType": self._analysis_type,
                "boundaryConditions": self._boundary_conditions,
                "loads": self._loads,
            },
        )
        if not resp.get("success"):
            raise RuntimeError(resp.get("error", "Simulation failed"))
        return Result(resp["data"])

    def run_parametric(self, parameters: list[dict[str, Any]]) -> list[Result]:
        """运行参数化仿真

        Parameters
        ----------
        parameters : list[dict]
            每个字典为一组参数变体。

        Returns
        -------
        list[Result]
            各参数组合的仿真结果列表。
        """
        resp = get_client().call(
            "run_parametric",
            {
                "projectId": self.project_id,
                "analysisType": self._analysis_type,
                "boundaryConditions": self._boundary_conditions,
                "loads": self._loads,
                "parameters": parameters,
            },
        )
        if not resp.get("success"):
            raise RuntimeError(resp.get("error", "Parametric simulation failed"))
        return [Result(item) for item in resp["data"]]

    # ------------------------------------------------------------------
    # 魔术方法
    # ------------------------------------------------------------------

    def __repr__(self) -> str:
        return (
            f"Simulation(project_id={self.project_id!r}, "
            f"analysis_type={self._analysis_type!r})"
        )
