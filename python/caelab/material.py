"""
材料模块 — 定义材料属性并分配给项目
"""

from __future__ import annotations

from typing import Any

from ._client import get_client


class Material:
    """CAELab 材料对象

    Parameters
    ----------
    name : str
        材料名称。
    **kwargs
        任意材料属性键值对。
    """

    def __init__(self, name: str, **kwargs: Any) -> None:
        self.name: str = name
        self._id: str | None = None
        self._properties: dict[str, Any] = {
            "youngs_modulus": kwargs.get("youngs_modulus"),
            "poisson_ratio": kwargs.get("poisson_ratio"),
            "density": kwargs.get("density"),
            "thermal_conductivity": kwargs.get("thermal_conductivity"),
            "thermal_expansion": kwargs.get("thermal_expansion"),
            "yield_strength": kwargs.get("yield_strength"),
            "ultimate_strength": kwargs.get("ultimate_strength"),
        }
        # 写入任意额外属性
        for key, value in kwargs.items():
            if key not in self._properties:
                self._properties[key] = value

    # ------------------------------------------------------------------
    # 工厂方法 — 常用材料
    # ------------------------------------------------------------------

    @staticmethod
    def steel() -> "Material":
        """创建结构钢材料（Q235）"""
        return Material(
            "Steel_Q235",
            youngs_modulus=210e9,
            poisson_ratio=0.3,
            density=7850.0,
            thermal_conductivity=50.0,
            thermal_expansion=12e-6,
            yield_strength=235e6,
            ultimate_strength=370e6,
        )

    @staticmethod
    def aluminum() -> "Material":
        """创建铝合金材料（6061-T6）"""
        return Material(
            "Aluminum_6061T6",
            youngs_modulus=68.9e9,
            poisson_ratio=0.33,
            density=2700.0,
            thermal_conductivity=167.0,
            thermal_expansion=23.6e-6,
            yield_strength=276e6,
            ultimate_strength=310e6,
        )

    @staticmethod
    def titanium() -> "Material":
        """创建钛合金材料（Ti-6Al-4V）"""
        return Material(
            "Titanium_Ti6Al4V",
            youngs_modulus=113.8e9,
            poisson_ratio=0.342,
            density=4430.0,
            thermal_conductivity=6.7,
            thermal_expansion=8.6e-6,
            yield_strength=880e6,
            ultimate_strength=950e6,
        )

    # ------------------------------------------------------------------
    # 属性设置
    # ------------------------------------------------------------------

    def set_elastic(self, youngs_modulus: float, poisson_ratio: float) -> None:
        """设置弹性属性

        Parameters
        ----------
        youngs_modulus : float
            杨氏模量 (Pa)。
        poisson_ratio : float
            泊松比。
        """
        self._properties["youngs_modulus"] = youngs_modulus
        self._properties["poisson_ratio"] = poisson_ratio

    def set_density(self, density: float) -> None:
        """设置密度

        Parameters
        ----------
        density : float
            密度 (kg/m^3)。
        """
        self._properties["density"] = density

    def set_thermal(self, conductivity: float, expansion: float) -> None:
        """设置热属性

        Parameters
        ----------
        conductivity : float
            热导率 (W/(m*K))。
        expansion : float
            热膨胀系数 (1/K)。
        """
        self._properties["thermal_conductivity"] = conductivity
        self._properties["thermal_expansion"] = expansion

    def set_plastic(self, yield_strength: float, ultimate_strength: float | None = None) -> None:
        """设置塑性属性

        Parameters
        ----------
        yield_strength : float
            屈服强度 (Pa)。
        ultimate_strength : float | None
            极限强度 (Pa)。
        """
        self._properties["yield_strength"] = yield_strength
        if ultimate_strength is not None:
            self._properties["ultimate_strength"] = ultimate_strength

    # ------------------------------------------------------------------
    # 分配
    # ------------------------------------------------------------------

    def assign_to_project(self, project_id: str) -> None:
        """将材料分配给项目

        Parameters
        ----------
        project_id : str
            项目 ID。
        """
        resp = get_client().call(
            "set_material",
            {"projectId": project_id, "material": self._properties, "name": self.name},
        )
        if not resp.get("success"):
            raise RuntimeError(resp.get("error", "Failed to assign material"))
        self._id = resp.get("data", {}).get("material_id")

    # ------------------------------------------------------------------
    # 查询
    # ------------------------------------------------------------------

    def get_properties(self) -> dict[str, Any]:
        """获取所有材料属性"""
        return dict(self._properties)

    # ------------------------------------------------------------------
    # 魔术方法
    # ------------------------------------------------------------------

    def __repr__(self) -> str:
        return f"Material(name={self.name!r})"

    def __str__(self) -> str:
        lines = [f"Material: {self.name}"]
        for key, value in self._properties.items():
            if value is not None:
                lines.append(f"  {key}: {value}")
        return "\n".join(lines)
