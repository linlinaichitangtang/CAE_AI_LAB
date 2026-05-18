"""
pytest conftest — 将 python/ 目录加入 sys.path，使 caelab 包可直接导入。
CI 中使用 pip install -e ./python，此处为本地开发兜底。
"""

import sys
from pathlib import Path

_python_dir = str(Path(__file__).resolve().parents[1])
if _python_dir not in sys.path:
    sys.path.insert(0, _python_dir)
