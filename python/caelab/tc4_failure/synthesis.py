"""
synthesis.py — TC4 合成数据生成器
V3.8 — 用于验证多模态失效分析全流程

先用 Al FCC 跑通框架（无势函数依赖），
10 月替换为 TC4 HCP 真实数据和 Rolls-Royce 势函数。

输出：
- ebsd_synthetic.csv    (晶格参数)
- sem_synthetic.csv     (图像特征)
- load_synthetic.csv    (载荷统计)
- failure_labels.csv    (失效模式 + 寿命)
"""

import csv
import random
from dataclasses import dataclass
from pathlib import Path
from typing import Optional

# HCP 晶格参数范围 (Ti-6Al-4V)
HCP_A_RANGE = (2.92, 2.98)   # Å
HCP_C_RANGE = (4.64, 4.72)   # Å

# EBSD 特征维度
EBSD_DIM = 80

# SEM 图像特征维度 (ViT-B/32)
SEM_DIM = 768

# 载荷特征维度
LOAD_DIM = 30

# 失效模式
FAILURE_MODES = ["HCF", "LCF", "TMF", "CREEP", "OVERLOAD", "FOD"]


@dataclass
class SyntheticSample:
    """一个合成样本"""
    sample_id: str
    # EBSD 晶格特征
    ebsd_features: list[float]
    # SEM 图像特征
    sem_features: list[float]
    # 载荷特征
    load_features: list[float]
    # 标签
    failure_mode: str
    log_cycles: float  # log10(cycles)


class Tc4SyntheticDataGenerator:
    """
    TC4 合成数据生成器

    用法:
        generator = Tc4SyntheticDataGenerator(seed=42)
        samples = generator.generate(n=100)
        generator.save_to_csv("./tc4_synthetic/", samples)
    """

    def __init__(
        self,
        seed: Optional[int] = None,
        material: str = "TC4",
        use_tc4: bool = False,
    ):
        """
        Args:
            seed: 随机种子，None 则不设置
            material: "TC4" (HCP) 或 "Al" (FCC，先跑通用)
            use_tc4: True=用 TC4 HCP 参数，False=用 Al FCC 参数
        """
        if seed is not None:
            random.seed(seed)

        self.material = material
        self.use_tc4 = use_tc4

    def _generate_ebsd_features(self) -> list[float]:
        """生成合成 EBSD 晶格特征 (80维)"""
        features = []

        if self.use_tc4:
            # TC4 HCP 晶格参数
            a = random.uniform(*HCP_A_RANGE)
            c = random.uniform(*HCP_C_RANGE)
            c_to_a = c / a

            # 晶格参数特征
            features.extend([a, c, c_to_a])

            # 取向关系特征 (RD/TD/ND)
            for _ in range(3):
                features.append(random.uniform(0, 1))

            # KAM (Kernel Average Misorientation)
            for _ in range(5):
                features.append(random.uniform(0, 3))  # 度

            # 晶界特征
            for _ in range(10):
                features.append(random.uniform(0, 180))  # 角度

            # 纹理特征 (IPF)
            for _ in range(12):
                features.append(random.uniform(0, 1))

        else:
            # Al FCC 简化特征
            a = 4.05  # Å
            features.append(a)
            features.extend([random.uniform(0, 1) for _ in range(EBSD_DIM - 1)])

        # 填充到 80 维
        while len(features) < EBSD_DIM:
            features.append(random.uniform(-1, 1))

        return features[:EBSD_DIM]

    def _generate_sem_features(self) -> list[float]:
        """生成合成 SEM 图像特征 (768维 via ViT)"""
        # 简化：模拟 ViT 输出的类别概率分布
        features = []

        # 韧窝特征 (主要)
        dimple_score = random.uniform(0.2, 0.8)
        features.append(dimple_score)

        # 裂纹特征
        crack_score = random.uniform(0.0, 0.3)
        features.append(crack_score)

        # 条纹特征
        striation_score = random.uniform(0.0, 0.2)
        features.append(striation_score)

        # 填充到 768 维
        while len(features) < SEM_DIM:
            features.append(random.uniform(0, 1))

        return features[:SEM_DIM]

    def _generate_load_features(self) -> list[float]:
        """生成合成载荷历史特征 (30维)"""
        features = []

        # 载荷类型
        load_type = random.choice(["uniaxial", "multiaxial", "thermal"])
        features.append(float(load_type == "uniaxial"))
        features.append(float(load_type == "multiaxial"))
        features.append(float(load_type == "thermal"))

        # 应力范围
        stress_range = random.uniform(200, 1000)  # MPa
        features.append(stress_range)

        # 平均应力
        mean_stress = random.uniform(-100, 300)  # MPa
        features.append(mean_stress)

        # 应力比 R
        r_ratio = random.uniform(-1, 0.5)
        features.append(r_ratio)

        # 频率
        frequency = random.uniform(0.1, 50)  # Hz
        features.append(frequency)

        # 循环次数
        n_cycles = random.randint(100, 1_000_000)
        features.append(float(n_cycles))

        # 温度 (TMF 时有)
        temperature = random.uniform(20, 600)  # °C
        features.append(temperature)

        # 其他统计特征
        for _ in range(LOAD_DIM - 10):
            features.append(random.uniform(0, 1))

        return features[:LOAD_DIM]

    def _assign_failure_mode(self, ebsd: list[float], load: list[float]) -> tuple[str, float]:
        """
        根据晶格和载荷特征分配失效模式和寿命
        简化规则，实际用训练好的模型
        """
        # 温度判断 TMF/CREEP
        temp = load[9] if len(load) > 9 else 300

        if temp > 500:
            if random.random() < 0.7:
                return "TMF", random.uniform(4.0, 6.0)  # log10(cycles)
            else:
                return "CREEP", random.uniform(3.0, 5.0)

        # 频率判断 HCF/LCF
        freq = load[6] if len(load) > 6 else 1.0
        stress_range = load[3] if len(load) > 3 else 500

        if freq > 20:
            if stress_range > 600:
                return "HCF", random.uniform(7.0, 9.0)
            else:
                return "HCF", random.uniform(6.0, 8.0)
        else:
            if stress_range > 500:
                return "LCF", random.uniform(3.0, 5.0)
            else:
                return "LCF", random.uniform(4.0, 6.0)

    def generate(self, n: int = 100) -> list[SyntheticSample]:
        """
        生成 n 个合成样本

        Returns:
            list[SyntheticSample]
        """
        samples = []

        for i in range(n):
            sample_id = f"SYNTH_{i+1:04d}"

            ebsd = self._generate_ebsd_features()
            sem = self._generate_sem_features()
            load = self._generate_load_features()

            failure_mode, log_cycles = self._assign_failure_mode(ebsd, load)

            # 偶尔添加异常标签
            if random.random() < 0.05:
                failure_mode = random.choice(FAILURE_MODES)

            samples.append(SyntheticSample(
                sample_id=sample_id,
                ebsd_features=ebsd,
                sem_features=sem,
                load_features=load,
                failure_mode=failure_mode,
                log_cycles=log_cycles,
            ))

        return samples

    def save_to_csv(self, output_dir: str, samples: list[SyntheticSample]):
        """
        保存合成数据到 CSV 文件

        Args:
            output_dir: 输出目录
            samples: 合成样本列表
        """
        output_path = Path(output_dir)
        output_path.mkdir(parents=True, exist_ok=True)

        # 1. EBSD 晶格特征
        ebsd_path = output_path / "ebsd_synthetic.csv"
        with open(ebsd_path, "w", newline="") as f:
            writer = csv.writer(f)
            writer.writerow(["sample_id"] + [f"ebsd_{i}" for i in range(EBSD_DIM)])
            for s in samples:
                writer.writerow([s.sample_id] + s.ebsd_features)

        # 2. SEM 图像特征
        sem_path = output_path / "sem_synthetic.csv"
        with open(sem_path, "w", newline="") as f:
            writer = csv.writer(f)
            writer.writerow(["sample_id"] + [f"sem_{i}" for i in range(SEM_DIM)])
            for s in samples:
                writer.writerow([s.sample_id] + s.sem_features)

        # 3. 载荷特征
        load_path = output_path / "load_synthetic.csv"
        with open(load_path, "w", newline="") as f:
            writer = csv.writer(f)
            writer.writerow(["sample_id"] + [f"load_{i}" for i in range(LOAD_DIM)])
            for s in samples:
                writer.writerow([s.sample_id] + s.load_features)

        # 4. 标签
        labels_path = output_path / "failure_labels.csv"
        with open(labels_path, "w", newline="") as f:
            writer = csv.writer(f)
            writer.writerow(["sample_id", "failure_mode", "log_cycles", "cycles"])
            for s in samples:
                writer.writerow([
                    s.sample_id,
                    s.failure_mode,
                    f"{s.log_cycles:.4f}",
                    f"{10**s.log_cycles:.0f}",
                ])

        print(f"[Tc4SyntheticDataGenerator] 保存 {len(samples)} 个样本到 {output_dir}")
        print(f"  - {ebsd_path}")
        print(f"  - {sem_path}")
        print(f"  - {load_path}")
        print(f"  - {labels_path}")


# ============ CLI 入口 ============

if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description="TC4 合成数据生成器")
    parser.add_argument("-n", "--num-samples", type=int, default=100, help="样本数量")
    parser.add_argument("-o", "--output-dir", type=str, default="./tc4_synthetic", help="输出目录")
    parser.add_argument("--seed", type=int, default=42, help="随机种子")
    parser.add_argument("--tc4", action="store_true", help="使用 TC4 HCP 参数（默认用 Al FCC）")

    args = parser.parse_args()

    generator = Tc4SyntheticDataGenerator(
        seed=args.seed,
        use_tc4=args.tc4,
    )
    samples = generator.generate(n=args.num_samples)
    generator.save_to_csv(args.output_dir, samples)

    print(f"\n生成完成！共 {len(samples)} 个样本")
    print(f"输出目录: {args.output_dir}")
