"""
结果模块 — 读取、分析、可视化仿真结果
"""

from __future__ import annotations

import json
import os
from typing import Any

from ._client import get_client


class Result:
    """CAELab 仿真结果

    Parameters
    ----------
    result_data : dict
        原始结果数据字典。
    """

    def __init__(self, result_data: dict[str, Any]) -> None:
        self._data: dict[str, Any] = result_data
        self._displacement_field: dict[str, Any] | None = None
        self._stress_field: dict[str, Any] | None = None

    # ------------------------------------------------------------------
    # 标量结果提取
    # ------------------------------------------------------------------

    def max_displacement(self) -> float:
        """获取最大位移值 (m)

        Returns
        -------
        float
        """
        return float(self._data.get("max_displacement", 0.0))

    def max_stress(self) -> float:
        """获取最大 von Mises 应力 (Pa)

        Returns
        -------
        float
        """
        return float(self._data.get("max_stress", 0.0))

    def max_strain(self) -> float:
        """获取最大等效应变"""
        return float(self._data.get("max_strain", 0.0))

    def reaction_forces(self) -> dict[str, float]:
        """获取反力"""
        return self._data.get("reaction_forces", {})

    def total_strain_energy(self) -> float:
        """获取总应变能 (J)"""
        return float(self._data.get("total_strain_energy", 0.0))

    # ------------------------------------------------------------------
    # 场数据
    # ------------------------------------------------------------------

    def get_displacement_field(self) -> dict[str, Any]:
        """获取位移场数据

        Returns
        -------
        dict
            包含 ``node_ids``、``ux``、``uy``、``uz``、``magnitude`` 的字典。
        """
        if self._displacement_field is None:
            resp = get_client().call(
                "get_simulation_result",
                {
                    "resultId": self._data.get("id"),
                    "field": "displacement",
                },
            )
            if resp.get("success"):
                self._displacement_field = resp["data"]
            else:
                self._displacement_field = self._data.get("displacement_field", {})
        return self._displacement_field

    def get_stress_field(self) -> dict[str, Any]:
        """获取应力场数据

        Returns
        -------
        dict
            包含 ``element_ids``、``von_mises``、``sxx``、``syy``、``szz`` 等。
        """
        if self._stress_field is None:
            resp = get_client().call(
                "get_simulation_result",
                {
                    "resultId": self._data.get("id"),
                    "field": "stress",
                },
            )
            if resp.get("success"):
                self._stress_field = resp["data"]
            else:
                self._stress_field = self._data.get("stress_field", {})
        return self._stress_field

    # ------------------------------------------------------------------
    # 导出
    # ------------------------------------------------------------------

    def export_report(self, path: str) -> None:
        """导出仿真报告

        Parameters
        ----------
        path : str
            报告文件路径（支持 .pdf / .html / .docx）。
        """
        resp = get_client().call(
            "export_report",
            {"resultId": self._data.get("id"), "path": os.path.abspath(path)},
        )
        if not resp.get("success"):
            raise RuntimeError(resp.get("error", "Failed to export report"))

    def export_data(self, path: str, field: str = "all") -> None:
        """导出原始数据为 CSV / JSON

        Parameters
        ----------
        path : str
            输出文件路径。
        field : str
            数据场名称，``"displacement"`` | ``"stress"`` | ``"all"``。
        """
        ext = os.path.splitext(path)[1].lower()
        if ext == ".json":
            with open(path, "w", encoding="utf-8") as f:
                json.dump(self._data, f, indent=2, ensure_ascii=False)
        elif ext == ".csv":
            import csv

            field_data = self._data.get(f"{field}_field", self._data)
            if isinstance(field_data, dict) and "node_ids" in field_data:
                keys = [k for k in field_data if k != "node_ids"]
                with open(path, "w", newline="", encoding="utf-8") as f:
                    writer = csv.writer(f)
                    writer.writerow(["node_id"] + keys)
                    for i, nid in enumerate(field_data["node_ids"]):
                        row = [nid] + [field_data[k][i] for k in keys]
                        writer.writerow(row)
        else:
            raise ValueError(f"Unsupported export format: {ext}")

    # ------------------------------------------------------------------
    # 可视化
    # ------------------------------------------------------------------

    def plot(self, field: str = "von_mises") -> None:
        """使用 matplotlib 可视化结果

        Parameters
        ----------
        field : str
            绘制场，``"von_mises"`` | ``"displacement"`` | ``"sxx"`` | ``"syy"`` | ``"szz"``。
        """
        try:
            import matplotlib.pyplot as plt
            import numpy as np
        except ImportError:
            raise ImportError(
                "matplotlib and numpy are required for plotting. "
                "Install with: pip install caelab[plot]"
            )

        if field == "von_mises":
            stress = self.get_stress_field()
            values = stress.get("von_mises", [])
            title = "Von Mises Stress"
            ylabel = "Stress (Pa)"
        elif field == "displacement":
            disp = self.get_displacement_field()
            values = disp.get("magnitude", [])
            title = "Displacement Magnitude"
            ylabel = "Displacement (m)"
        else:
            stress = self.get_stress_field()
            values = stress.get(field, [])
            title = f"Stress Component: {field}"
            ylabel = "Stress (Pa)"

        if not values:
            print(f"No data available for field '{field}'")
            return

        values = np.asarray(values)

        fig, axes = plt.subplots(1, 2, figsize=(14, 5))

        # 直方图
        axes[0].hist(values, bins=50, color="steelblue", edgecolor="black", alpha=0.7)
        axes[0].set_title(f"{title} Distribution")
        axes[0].set_xlabel(ylabel)
        axes[0].set_ylabel("Count")
        axes[0].grid(True, alpha=0.3)

        # 云图（如果存在坐标）
        stress = self.get_stress_field()
        coords = stress.get("coordinates")
        if coords is not None:
            coords = np.asarray(coords)
            sc = axes[1].scatter(
                coords[:, 0], coords[:, 1], c=values, cmap="jet", s=1
            )
            fig.colorbar(sc, ax=axes[1], label=ylabel)
            axes[1].set_title(f"{title} Contour")
            axes[1].set_xlabel("X (m)")
            axes[1].set_ylabel("Y (m)")
            axes[1].set_aspect("equal")
        else:
            axes[1].plot(values, "b-", linewidth=0.5)
            axes[1].set_title(f"{title} Profile")
            axes[1].set_xlabel("Index")
            axes[1].set_ylabel(ylabel)
            axes[1].grid(True, alpha=0.3)

        plt.tight_layout()
        plt.show()

    # ------------------------------------------------------------------
    # 魔术方法
    # ------------------------------------------------------------------

    def __repr__(self) -> str:
        rid = self._data.get("id", "?")
        return f"Result(id={rid!r})"

    def summary(self) -> str:
        """生成结果摘要文本"""
        lines = [
            "=== Simulation Result Summary ===",
            f"  Max Displacement : {self.max_displacement():.6e} m",
            f"  Max Stress (VM)  : {self.max_stress():.6e} Pa",
            f"  Max Strain       : {self.max_strain():.6e}",
            f"  Strain Energy    : {self.total_strain_energy():.6e} J",
        ]
        rf = self.reaction_forces()
        if rf:
            lines.append("  Reaction Forces  :")
            for k, v in rf.items():
                lines.append(f"    {k}: {v:.4e} N")
        return "\n".join(lines)
