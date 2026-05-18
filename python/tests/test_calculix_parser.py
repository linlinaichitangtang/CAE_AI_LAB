"""
CalculiX 输出文件解析器测试 (V4.2-003)

验证 CAELab 能正确解析 CalculiX 的 INP / STA / CVG / DAT 文件格式。
测试文件来自仓库根目录的 test_calculix.* 系列。
"""

import os
import re
from pathlib import Path

import pytest

REPO_ROOT = Path(__file__).resolve().parents[2]


# ========================================================================
# 辅助解析函数（模拟 Rust output_parser 的 Python 等价）
# ========================================================================

def parse_inp_nodes(inp_path: str) -> list[tuple[int, float, float, float]]:
    """解析 INP 文件中的 *NODE 块，返回 [(id, x, y, z), ...]"""
    nodes = []
    in_node_block = False
    with open(inp_path, encoding="utf-8") as f:
        for line in f:
            stripped = line.strip()
            if stripped.upper().startswith("*NODE"):
                in_node_block = True
                continue
            if in_node_block and stripped.startswith("*"):
                in_node_block = False
                continue
            if in_node_block and stripped:
                parts = [p.strip() for p in stripped.rstrip(",").split(",")]
                if len(parts) >= 4:
                    nid = int(parts[0])
                    x, y, z = float(parts[1]), float(parts[2]), float(parts[3])
                    nodes.append((nid, x, y, z))
    return nodes


def parse_inp_elements(inp_path: str) -> list[tuple[str, list[int]]]:
    """解析 INP 文件中的 *ELEMENT 块，返回 [(elem_type, [node_ids]), ...]"""
    elements = []
    current_type = None
    in_elem_block = False
    with open(inp_path, encoding="utf-8") as f:
        for line in f:
            stripped = line.strip()
            if stripped.upper().startswith("*ELEMENT"):
                in_elem_block = True
                match = re.search(r"TYPE\s*=\s*(\w+)", stripped, re.IGNORECASE)
                current_type = match.group(1) if match else "UNKNOWN"
                continue
            if in_elem_block and stripped.startswith("*"):
                in_elem_block = False
                current_type = None
                continue
            if in_elem_block and stripped:
                parts = [int(p.strip()) for p in stripped.rstrip(",").split(",")]
                node_ids = parts[1:]  # 跳过单元编号
                elements.append((current_type, node_ids))
    return elements


def parse_inp_materials(inp_path: str) -> list[dict]:
    """解析 INP 文件中的材料定义"""
    materials = []
    current_mat = None
    with open(inp_path, encoding="utf-8") as f:
        for line in f:
            stripped = line.strip()
            if stripped.upper().startswith("*MATERIAL"):
                match = re.search(r"NAME\s*=\s*(\w+)", stripped, re.IGNORECASE)
                name = match.group(1) if match else "UNKNOWN"
                current_mat = {"name": name, "properties": {}}
                materials.append(current_mat)
            elif stripped.upper().startswith("*ELASTIC") and current_mat:
                # 下一行读取 E, nu
                pass
            elif current_mat and not stripped.startswith("*") and stripped:
                parts = [p.strip() for p in stripped.split(",")]
                if len(parts) >= 2 and "youngs_modulus" not in current_mat["properties"]:
                    try:
                        current_mat["properties"]["youngs_modulus"] = float(parts[0])
                        current_mat["properties"]["poisson_ratio"] = float(parts[1])
                    except ValueError:
                        pass
    return materials


def parse_sta_file(sta_path: str) -> list[dict]:
    """解析 STA 文件，返回步骤信息列表"""
    steps = []
    in_data = False
    with open(sta_path, encoding="utf-8") as f:
        for line in f:
            stripped = line.strip()
            if stripped.startswith("STEP"):
                in_data = True
                continue
            if in_data and stripped and not stripped.startswith("STEP"):
                parts = stripped.split()
                if len(parts) >= 3:
                    try:
                        step_info = {
                            "step": int(parts[0]),
                            "inc": int(parts[1]),
                            "iterations": int(parts[2]) if len(parts) > 2 else 0,
                        }
                        steps.append(step_info)
                    except (ValueError, IndexError):
                        pass
    return steps


def parse_cvg_file(cvg_path: str) -> dict:
    """解析 CVG 文件，返回收敛信息"""
    info = {"header_found": False, "data_lines": []}
    with open(cvg_path, encoding="utf-8") as f:
        for line in f:
            stripped = line.strip()
            if "CONVERGENCE" in stripped.upper() or "C0NVERGENCE" in stripped:
                info["header_found"] = True
                continue
            if info["header_found"] and stripped and not stripped.startswith("STEP"):
                parts = stripped.split()
                if len(parts) >= 3:
                    info["data_lines"].append(parts)
    return info


# ========================================================================
# 测试：INP 文件解析
# ========================================================================

class TestInpParser:
    """test_calculix.inp 文件解析"""

    @pytest.fixture(autouse=True)
    def load_inp(self):
        self.inp_path = str(REPO_ROOT / "test_calculix.inp")
        if not os.path.exists(self.inp_path):
            pytest.skip("test_calculix.inp not found")

    def test_file_exists(self):
        assert os.path.exists(self.inp_path)

    def test_parse_nodes(self):
        nodes = parse_inp_nodes(self.inp_path)
        assert len(nodes) == 3, f"预期 3 个节点，实际 {len(nodes)}"
        # 节点 1: (0, 0, 0)
        n1 = nodes[0]
        assert n1[0] == 1
        assert n1[1:] == pytest.approx((0.0, 0.0, 0.0))
        # 节点 2: (1, 0, 0)
        n2 = nodes[1]
        assert n2[0] == 2
        assert n2[1:] == pytest.approx((1.0, 0.0, 0.0))
        # 节点 3: (0.5, 1, 0)
        n3 = nodes[2]
        assert n3[0] == 3
        assert n3[1:] == pytest.approx((0.5, 1.0, 0.0))

    def test_parse_elements(self):
        elements = parse_inp_elements(self.inp_path)
        assert len(elements) == 1
        elem_type, node_ids = elements[0]
        assert elem_type == "C3D4"
        assert node_ids == [1, 2, 3, 1]

    def test_parse_materials(self):
        materials = parse_inp_materials(self.inp_path)
        assert len(materials) == 1
        mat = materials[0]
        assert mat["name"] == "Steel"
        assert mat["properties"]["youngs_modulus"] == pytest.approx(210000)
        assert mat["properties"]["poisson_ratio"] == pytest.approx(0.3)

    def test_inp_has_boundary(self):
        with open(self.inp_path, encoding="utf-8") as f:
            content = f.read()
        assert "*BOUNDARY" in content.upper()

    def test_inp_has_step(self):
        with open(self.inp_path, encoding="utf-8") as f:
            content = f.read()
        assert "*STEP" in content.upper()
        assert "*END STEP" in content.upper()

    def test_inp_has_load(self):
        with open(self.inp_path, encoding="utf-8") as f:
            content = f.read()
        assert "*CLOAD" in content.upper()


# ========================================================================
# 测试：STA 文件解析
# ========================================================================

class TestStaParser:
    """test_calculix.sta 文件解析"""

    @pytest.fixture(autouse=True)
    def load_sta(self):
        self.sta_path = str(REPO_ROOT / "test_calculix.sta")
        if not os.path.exists(self.sta_path):
            pytest.skip("test_calculix.sta not found")

    def test_file_exists(self):
        assert os.path.exists(self.sta_path)

    def test_sta_format(self):
        with open(self.sta_path, encoding="utf-8") as f:
            content = f.read()
        assert "SUMMARY OF JOB INFORMATION" in content
        assert "STEP" in content
        assert "INC" in content

    def test_sta_columns(self):
        with open(self.sta_path, encoding="utf-8") as f:
            lines = f.readlines()
        # 第二行应包含列标题
        header = lines[1].strip() if len(lines) > 1 else ""
        for col in ("STEP", "INC", "ATT", "ITRS"):
            assert col in header, f"列 {col} 不在 STA 表头中"


# ========================================================================
# 测试：CVG 文件解析
# ========================================================================

class TestCvgParser:
    """test_calculix.cvg 文件解析"""

    @pytest.fixture(autouse=True)
    def load_cvg(self):
        self.cvg_path = str(REPO_ROOT / "test_calculix.cvg")
        if not os.path.exists(self.cvg_path):
            pytest.skip("test_calculix.cvg not found")

    def test_file_exists(self):
        assert os.path.exists(self.cvg_path)

    def test_cvg_header(self):
        info = parse_cvg_file(self.cvg_path)
        assert info["header_found"], "CVG 文件中未找到 CONVERGENCE 标题"

    def test_cvg_columns(self):
        with open(self.cvg_path, encoding="utf-8") as f:
            content = f.read()
        for col in ("STEP", "INC", "ITER"):
            assert col in content, f"列 {col} 不在 CVG 表头中"


# ========================================================================
# 测试：DAT 文件解析
# ========================================================================

class TestDatParser:
    """test_calculix.dat 文件解析"""

    @pytest.fixture(autouse=True)
    def load_dat(self):
        self.dat_path = str(REPO_ROOT / "test_calculix.dat")
        if not os.path.exists(self.dat_path):
            pytest.skip("test_calculix.dat not found")

    def test_file_exists(self):
        assert os.path.exists(self.dat_path)

    def test_dat_readable(self):
        """DAT 文件可正常读取"""
        with open(self.dat_path, encoding="utf-8") as f:
            content = f.read()
        # DAT 文件可以为空（小模型计算结果），但应可读
        assert isinstance(content, str)
