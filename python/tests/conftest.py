"""
pytest conftest — 将 python/ 目录加入 sys.path，使 caelab 包可直接导入。
CI 中使用 pip install -e ./python，此处为本地开发兜底。

同时为 DFT/OpenKIM 等模块设置临时 CAELAB_HOME 目录，
避免 macOS 沙箱环境下 ~/.caelab 权限问题。
"""
import os
import sys
import tempfile
from pathlib import Path

_python_dir = str(Path(__file__).resolve().parents[1])
if _python_dir not in sys.path:
    sys.path.insert(0, _python_dir)

# 创建临时 CAELAB_HOME（CI 或本地沙箱环境下确保可写）
_caelab_tmp = os.environ.get("CAELAB_HOME")
if not _caelab_tmp:
    _caelab_tmp = tempfile.mkdtemp(prefix="caelab_test_")
    os.environ["CAELAB_HOME"] = _caelab_tmp
    os.environ["CAELAB_OPENKIM_DIR"] = os.path.join(_caelab_tmp, "kim")
    for _sub in ["kim", "materials"]:
        os.makedirs(os.path.join(_caelab_tmp, _sub), exist_ok=True)
