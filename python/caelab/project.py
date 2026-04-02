"""
项目模块 — 管理 CAELab 工程项目
"""

from __future__ import annotations

import json
import os
import uuid
from datetime import datetime, timezone
from typing import Any

from ._client import get_client


class Project:
    """CAELab 工程项目

    Parameters
    ----------
    name : str
        项目名称。
    description : str
        项目描述。
    """

    def __init__(self, name: str, description: str = "") -> None:
        self.id: str = uuid.uuid4().hex[:12]
        self.name: str = name
        self.description: str = description
        self.created_at: str = datetime.now(timezone.utc).isoformat()
        self.updated_at: str = self.created_at
        self._metadata: dict[str, Any] = {}

    # ------------------------------------------------------------------
    # CRUD
    # ------------------------------------------------------------------

    def save(self) -> str:
        """保存项目到 CAELab，返回项目 ID

        Returns
        -------
        str
            项目唯一标识符。
        """
        self.updated_at = datetime.now(timezone.utc).isoformat()
        resp = get_client().call(
            "save_project",
            {
                "id": self.id,
                "name": self.name,
                "description": self.description,
                "metadata": self._metadata,
            },
        )
        if not resp.get("success"):
            raise RuntimeError(resp.get("error", "Failed to save project"))
        self.id = resp.get("data", {}).get("id", self.id)
        return self.id

    @staticmethod
    def load(project_id: str) -> "Project":
        """从 CAELab 加载已有项目

        Parameters
        ----------
        project_id : str
            项目 ID。

        Returns
        -------
        Project
        """
        resp = get_client().call("load_project", {"id": project_id})
        if not resp.get("success"):
            raise RuntimeError(resp.get("error", "Failed to load project"))
        data = resp["data"]
        proj = Project(data["name"], data.get("description", ""))
        proj.id = data["id"]
        proj.created_at = data.get("created_at", proj.created_at)
        proj.updated_at = data.get("updated_at", proj.updated_at)
        proj._metadata = data.get("metadata", {})
        return proj

    @staticmethod
    def list() -> list["Project"]:
        """列出所有项目

        Returns
        -------
        list[Project]
        """
        resp = get_client().call("list_projects", {})
        if not resp.get("success"):
            raise RuntimeError(resp.get("error", "Failed to list projects"))
        projects: list[Project] = []
        for item in resp.get("data", []):
            proj = Project(item["name"], item.get("description", ""))
            proj.id = item["id"]
            proj.created_at = item.get("created_at", "")
            proj.updated_at = item.get("updated_at", "")
            proj._metadata = item.get("metadata", {})
            projects.append(proj)
        return projects

    def delete(self) -> None:
        """删除当前项目"""
        resp = get_client().call("delete_project", {"id": self.id})
        if not resp.get("success"):
            raise RuntimeError(resp.get("error", "Failed to delete project"))

    # ------------------------------------------------------------------
    # 导入 / 导出
    # ------------------------------------------------------------------

    def export(self, path: str) -> None:
        """将项目导出为 .caelabzip 文件

        Parameters
        ----------
        path : str
            导出文件路径。
        """
        resp = get_client().call(
            "export_project", {"id": self.id, "path": os.path.abspath(path)}
        )
        if not resp.get("success"):
            raise RuntimeError(resp.get("error", "Failed to export project"))

    # ------------------------------------------------------------------
    # 元数据
    # ------------------------------------------------------------------

    def set_metadata(self, key: str, value: Any) -> None:
        """设置项目元数据"""
        self._metadata[key] = value

    def get_metadata(self, key: str, default: Any = None) -> Any:
        """获取项目元数据"""
        return self._metadata.get(key, default)

    # ------------------------------------------------------------------
    # 魔术方法
    # ------------------------------------------------------------------

    def __repr__(self) -> str:
        return f"Project(id={self.id!r}, name={self.name!r})"
