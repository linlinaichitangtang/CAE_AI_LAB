"""
与 CAELab 桌面端通信的客户端

支持两种通信模式:
1. Tauri IPC 模式 — 在 CAELab 桌面端内嵌 Python 环境中运行
2. 子进程模式 — 独立脚本通过 caelab CLI 与桌面端通信
"""

from __future__ import annotations

import json
import os
import subprocess
import sys
from typing import Any


class CAELabClient:
    """通过 Tauri IPC 或子进程与 CAELab 桌面端通信"""

    def __init__(self) -> None:
        self._base_url: str | None = None
        self._mode: str = "cli"

        # 检测运行环境
        if self._is_jupyter():
            self._mode = "http"
            self._base_url = os.environ.get(
                "CAELAB_API_URL", "http://localhost:45678/api"
            )
        elif os.environ.get("CAELAB_TAURI") == "1":
            self._mode = "ipc"

    # ------------------------------------------------------------------
    # 环境检测
    # ------------------------------------------------------------------

    @staticmethod
    def _is_jupyter() -> bool:
        """检测当前是否运行在 Jupyter 环境中"""
        try:
            from IPython import get_ipython  # type: ignore

            return get_ipython() is not None and "IPKernelApp" in get_ipython().config
        except Exception:
            return False

    # ------------------------------------------------------------------
    # 公共接口
    # ------------------------------------------------------------------

    def call(self, command: str, args: dict[str, Any] | None = None) -> dict[str, Any]:
        """调用 CAELab 命令并返回结果字典

        Parameters
        ----------
        command : str
            命令名称，例如 ``"generate_mesh"``、``"run_simulation"``。
        args : dict | None
            传递给命令的参数。

        Returns
        -------
        dict
            包含 ``success``、``data``、``error`` 键的响应字典。
        """
        args = args or {}

        if self._mode == "http":
            return self._call_http(command, args)
        elif self._mode == "ipc":
            return self._call_ipc(command, args)
        else:
            return self._call_cli(command, args)

    # ------------------------------------------------------------------
    # HTTP 模式 (Jupyter / 远程)
    # ------------------------------------------------------------------

    def _call_http(self, command: str, args: dict[str, Any]) -> dict[str, Any]:
        try:
            import urllib.request
            import urllib.error

            url = f"{self._base_url}/{command}"
            payload = json.dumps({"command": command, "args": args}).encode("utf-8")
            req = urllib.request.Request(
                url,
                data=payload,
                headers={"Content-Type": "application/json"},
                method="POST",
            )
            with urllib.request.urlopen(req, timeout=120) as resp:
                return json.loads(resp.read().decode("utf-8"))
        except ImportError:
            return {"success": False, "error": "urllib is required for HTTP mode"}
        except Exception as exc:
            return {"success": False, "error": str(exc)}

    # ------------------------------------------------------------------
    # IPC 模式 (Tauri 内嵌)
    # ------------------------------------------------------------------

    def _call_ipc(self, command: str, args: dict[str, Any]) -> dict[str, Any]:
        try:
            import urllib.request

            url = f"http://localhost:{os.environ.get('CAELAB_IPC_PORT', '45679')}/ipc"
            payload = json.dumps({"command": command, "args": args}).encode("utf-8")
            req = urllib.request.Request(
                url,
                data=payload,
                headers={"Content-Type": "application/json"},
                method="POST",
            )
            with urllib.request.urlopen(req, timeout=120) as resp:
                return json.loads(resp.read().decode("utf-8"))
        except Exception as exc:
            return {"success": False, "error": str(exc)}

    # ------------------------------------------------------------------
    # CLI 模式 (子进程)
    # ------------------------------------------------------------------

    def _call_cli(self, command: str, args: dict[str, Any]) -> dict[str, Any]:
        cli_path = os.environ.get("CAELAB_CLI", "caelab")
        cmd_args = [cli_path, command, "--json"]

        for key, value in args.items():
            cmd_args.extend([f"--{key}", str(value)])

        try:
            result = subprocess.run(
                cmd_args,
                capture_output=True,
                text=True,
                timeout=300,
                check=False,
            )
            if result.returncode != 0:
                return {"success": False, "error": result.stderr.strip()}
            return json.loads(result.stdout)
        except FileNotFoundError:
            return {
                "success": False,
                "error": (
                    "caelab CLI not found. Please install CAELab or set CAELAB_CLI env var."
                ),
            }
        except subprocess.TimeoutExpired:
            return {"success": False, "error": "Command timed out after 300 seconds"}
        except json.JSONDecodeError as exc:
            return {"success": False, "error": f"Invalid JSON response: {exc}"}


# 模块级单例
_client: CAELabClient | None = None


def get_client() -> CAELabClient:
    """获取全局 CAELabClient 实例"""
    global _client
    if _client is None:
        _client = CAELabClient()
    return _client
