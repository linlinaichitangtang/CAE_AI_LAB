"""
CAELab 预训练模型仓库 (V4.4-002)

管理 M3GNet / CHGNet / MACE / NequIP / SevenNet 等 SOTA 模型的
下载、缓存、推理和 ONNX 导出。
"""

__all__ = [
    "ModelRegistry",
    "Predictor",
    "list_models",
    "download_model",
    "predict",
    "export_onnx",
]
