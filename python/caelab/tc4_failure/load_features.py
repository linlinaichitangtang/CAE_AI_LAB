"""
load_features.py — TC4 载荷特征提取器
V3.8

载荷历史数据 → 30 维特征向量
用于多模态融合的载荷输入支路

特征维度:
- 载荷类型 one-hot — 3维
- 应力范围 — 1维
- 平均应力 — 1维
- 应力比 R — 1维
- 频率 — 1维
- 循环次数 — 1维
- 温度 — 1维
- 其他统计 — 22维
总计 30维
"""

import csv
from pathlib import Path
from typing import Optional, Union
import math


class LoadFeatureExtractor:
    """
    TC4 载荷特征提取器

    用法:
        extractor = LoadFeatureExtractor()
        features = extractor.extract_from_csv("load_history.csv")
    """

    LOAD_DIM = 30

    # 载荷类型映射
    LOAD_TYPES = ["uniaxial", "multiaxial", "thermal"]

    def __init__(self):
        pass

    def extract_from_csv(self, csv_path: str) -> list[float]:
        """
        从 CSV 文件提取载荷特征

        Args:
            csv_path: load_synthetic.csv 路径

        Returns:
            30维特征向量
        """
        features = []
        with open(csv_path, "r") as f:
            reader = csv.DictReader(f)
            row = next(reader)
            for i in range(self.LOAD_DIM):
                key = f"load_{i}"
                features.append(float(row.get(key, 0.0)))
        return features

    def extract_from_raw(self, time_series: list[dict]) -> list[float]:
        """
        从原始时序数据提取载荷特征

        Args:
            time_series: [{"time": t, "stress": s, "temp": T}, ...]

        Returns:
            30维特征向量
        """
        if not time_series:
            return [0.0] * self.LOAD_DIM

        # 提取基本统计
        stresses = [d.get("stress", 0) for d in time_series]
        temps = [d.get("temp", 25) for d in time_series]

        # 应力统计
        max_stress = max(stresses) if stresses else 0
        min_stress = min(stresses) if stresses else 0
        mean_stress = sum(stresses) / len(stresses) if stresses else 0
        stress_range = max_stress - min_stress

        # 温度统计
        max_temp = max(temps) if temps else 25
        mean_temp = sum(temps) / len(temps) if temps else 25

        # 判断载荷类型
        is_thermal = max_temp - mean_temp > 100
        is_multiaxial = len(time_series) > 0 and "tau" in time_series[0]

        features = [
            float(not is_thermal and not is_multiaxial),  # uniaxial
            float(is_multiaxial),  # multiaxial
            float(is_thermal),  # thermal
            stress_range,  # 应力范围
            mean_stress,  # 平均应力
            (min_stress / max_stress) if max_stress != 0 else -1,  # 应力比 R
            1.0 / (len(time_series) + 1),  # 伪频率（循环数反比）
            float(len(time_series)),  # 循环次数
            max_temp,  # 温度
            # 统计特征
            max_stress,
            min_stress,
            mean_stress,
            stress_range,
            mean_temp,
            # 标准差等
            self._std(stresses),
            self._std(temps),
            self._skewness(stresses),
            self._kurtosis(stresses),
        ]

        # 填充到 30 维
        while len(features) < self.LOAD_DIM:
            features.append(0.0)

        return features[:self.LOAD_DIM]

    def _std(self, values: list[float]) -> float:
        """计算标准差"""
        if len(values) < 2:
            return 0.0
        mean = sum(values) / len(values)
        variance = sum((x - mean) ** 2 for x in values) / len(values)
        return math.sqrt(variance)

    def _skewness(self, values: list[float]) -> float:
        """计算偏度"""
        if len(values) < 3:
            return 0.0
        mean = sum(values) / len(values)
        std = self._std(values)
        if std == 0:
            return 0.0
        n = len(values)
        skew = sum((x - mean) ** 3 for x in values) / (n * std ** 3)
        return skew

    def _kurtosis(self, values: list[float]) -> float:
        """计算峰度"""
        if len(values) < 4:
            return 0.0
        mean = sum(values) / len(values)
        std = self._std(values)
        if std == 0:
            return 0.0
        n = len(values)
        kurt = sum((x - mean) ** 4 for x in values) / (n * std ** 4) - 3
        return kurt

    def extract_batch(self, csv_paths: list[str]) -> list[list[float]]:
        """
        批量提取载荷特征

        Args:
            csv_paths: CSV 文件路径列表

        Returns:
            list[特征向量]
        """
        return [self.extract_from_csv(p) for p in csv_paths]


# ============ CLI 入口 ============

if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description="TC4 载荷特征提取")
    parser.add_argument("-i", "--input", required=True, help="输入 CSV 文件")
    parser.add_argument("-o", "--output", help="输出 CSV（可选）")

    args = parser.parse_args()

    extractor = LoadFeatureExtractor()
    features = extractor.extract_from_csv(args.input)

    print(f"[LoadFeatureExtractor] 提取 30 维特征:")
    print(f"  应力范围: {features[3]:.1f} MPa")
    print(f"  平均应力: {features[4]:.1f} MPa")
    print(f"  温度: {features[8]:.1f} °C")

    if args.output:
        with open(args.output, "w", newline="") as f:
            writer = csv.writer(f)
            writer.writerow([f"load_{i}" for i in range(30)])
            writer.writerow(features)
        print(f"  保存到: {args.output}")