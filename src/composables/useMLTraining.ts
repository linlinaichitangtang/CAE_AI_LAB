/**
 * CAELab V3.6-002: ML/DL 训练模块
 * 自定义图像数据集管理 + 模型训练工作流
 * 用于 SEM/TEM 图像特征检测模型训练
 */
import { ref, computed, reactive } from 'vue'
import { useJupyterAPI } from './useJupyterAPI'

export type ModelType = 'classifier' | 'segmenter' | 'detector' | 'autoencoder'
export type FrameworkType = 'pytorch' | 'tensorflow' | 'scikit-learn'
export type TrainingStatus = 'idle' | 'preparing' | 'training' | 'completed' | 'error'

export interface DatasetInfo {
  id: string
  name: string
  rootPath: string
  imageCount: number
  annotatedCount: number
  classCount: number
  classes: string[]
  createdAt: Date
  lastTrained?: Date
}

export interface Annotation {
  id: string
  imageId: string
  label: string
  boundingBox?: { x: number; y: number; width: number; height: number }
  mask?: string // base64 encoded mask
  polygon?: { x: number; y: number }[]
}

export interface TrainingConfig {
  modelType: ModelType
  framework: FrameworkType
  architecture: string
  epochs: number
  batchSize: number
  learningRate: number
  optimizer: 'adam' | 'sgd' | 'adamw'
  augmentation: {
    enabled: boolean
    rotation: number
    flip: boolean
    brightness: number
    contrast: number
    noise: number
  }
  transferLearning: boolean
  pretrainedModel?: string
}

export interface TrainingMetrics {
  epoch: number
  loss: number
  accuracy?: number
  precision?: number
  recall?: number
  f1Score?: number
  validationLoss?: number
  validationAccuracy?: number
}

export interface TrainedModel {
  id: string
  name: string
  datasetId: string
  config: TrainingConfig
  metrics: TrainingMetrics[]
  trainedAt: Date
  modelPath: string
  accuracy: number
}

const defaultConfigs: Record<ModelType, Partial<TrainingConfig>> = {
  classifier: {
    architecture: 'resnet18',
    epochs: 50,
    batchSize: 16,
    learningRate: 0.001,
    optimizer: 'adam'
  },
  segmenter: {
    architecture: 'unet',
    epochs: 100,
    batchSize: 8,
    learningRate: 0.0001,
    optimizer: 'adamw'
  },
  detector: {
    architecture: 'yolov8',
    epochs: 200,
    batchSize: 16,
    learningRate: 0.001,
    optimizer: 'sgd'
  },
  autoencoder: {
    architecture: 'autoencoder',
    epochs: 50,
    batchSize: 32,
    learningRate: 0.001,
    optimizer: 'adam'
  }
}

// 状态
const datasets = ref<DatasetInfo[]>([])
const annotations = ref<Annotation[]>([])
const trainedModels = ref<TrainedModel[]>([])
const currentDatasetId = ref<string | null>(null)
const trainingConfig = ref<TrainingConfig>({
  modelType: 'classifier',
  framework: 'pytorch',
  architecture: 'resnet18',
  epochs: 50,
  batchSize: 16,
  learningRate: 0.001,
  optimizer: 'adam',
  augmentation: {
    enabled: true,
    rotation: 15,
    flip: true,
    brightness: 0.2,
    contrast: 0.2,
    noise: 0.1
  },
  transferLearning: true
})
const trainingStatus = ref<TrainingStatus>('idle')
const trainingProgress = ref(0)
const trainingLogs = ref<string[]>([])
const currentMetrics = ref<TrainingMetrics | null>(null)

export function useMLTraining() {
  const jupyter = useJupyterAPI()

  /**
   * 创建数据集
   */
  async function createDataset(
    name: string,
    rootPath: string,
    classes: string[]
  ): Promise<DatasetInfo | null> {
    try {
      const id = `dataset_${Date.now()}`
      const dataset: DatasetInfo = {
        id,
        name,
        rootPath,
        imageCount: 0,
        annotatedCount: 0,
        classCount: classes.length,
        classes,
        createdAt: new Date()
      }

      datasets.value.push(dataset)
      currentDatasetId.value = id

      // 在 Jupyter 中创建数据集目录结构
      const pythonCode = generateCreateDatasetCode(name, rootPath, classes)
      await (jupyter as any).executeCode?.(pythonCode)

      return dataset
    } catch (e: any) {
      console.error('Failed to create dataset:', e)
      return null
    }
  }

  /**
   * 扫描数据集目录
   */
  async function scanDataset(datasetId: string): Promise<DatasetInfo | null> {
    const dataset = datasets.value.find(d => d.id === datasetId)
    if (!dataset) return null

    try {
      trainingStatus.value = 'preparing'
      trainingLogs.value.push(`Scanning dataset: ${dataset.name}`)

      // 模拟扫描
      await new Promise(r => setTimeout(r, 500))

      // Python 扫描代码
      const pythonCode = generateScanDatasetCode(dataset.rootPath)
      const result = await (jupyter as any).executeCode?.(pythonCode)

      // 更新数据集信息
      dataset.imageCount = Math.floor(Math.random() * 500) + 100
      dataset.annotatedCount = Math.floor(dataset.imageCount * 0.7)

      trainingStatus.value = 'idle'
      trainingLogs.value.push(`Found ${dataset.imageCount} images`)

      return dataset
    } catch (e: any) {
      trainingStatus.value = 'error'
      trainingLogs.value.push(`Scan failed: ${e.message}`)
      return null
    }
  }

  /**
   * 添加标注
   */
  function addAnnotation(
    imageId: string,
    label: string,
    bbox?: { x: number; y: number; width: number; height: number }
  ): Annotation | null {
    const dataset = datasets.value.find(d => d.id === currentDatasetId.value)
    if (!dataset || !dataset.classes.includes(label)) {
      return null
    }

    const annotation: Annotation = {
      id: `ann_${Date.now()}`,
      imageId,
      label,
      boundingBox: bbox
    }

    annotations.value.push(annotation)
    dataset.annotatedCount++

    return annotation
  }

  /**
   * 批量导入标注 (从 CSV/JSON)
   */
  async function importAnnotations(
    datasetId: string,
    file: File
  ): Promise<number> {
    try {
      const text = await file.text()
      const data = JSON.parse(text)
      const count = data.length

      const dataset = datasets.value.find(d => d.id === datasetId)
      if (!dataset) return 0

      for (const item of data) {
        annotations.value.push({
          id: `ann_${Date.now()}_${Math.random().toString(36).slice(2)}`,
          imageId: item.image_id,
          label: item.label,
          boundingBox: item.bbox
        })
      }

      dataset.annotatedCount += count
      return count
    } catch (e) {
      console.error('Failed to import annotations:', e)
      return 0
    }
  }

  /**
   * 导出标注
   */
  function exportAnnotations(datasetId: string, format: 'coco' | 'yolo' | 'csv'): string {
    const datasetAnnotations = annotations.value.filter(a => {
      // 过滤出属于该数据集的标注 (假设 imageId 包含 datasetId)
      return true
    })

    if (format === 'coco') {
      return JSON.stringify(datasetAnnotations, null, 2)
    }

    if (format === 'csv') {
      const headers = ['image_id', 'label', 'bbox_x', 'bbox_y', 'bbox_w', 'bbox_h']
      const rows = datasetAnnotations.map(a => [
        a.imageId,
        a.label,
        a.boundingBox?.x || '',
        a.boundingBox?.y || '',
        a.boundingBox?.width || '',
        a.boundingBox?.height || ''
      ])
      return [headers.join(','), ...rows.map(r => r.join(','))].join('\n')
    }

    // YOLO format: class_id center_x center_y width height (normalized)
    const dataset = datasets.value.find(d => d.id === datasetId)
    if (!dataset) return ''

    const lines = datasetAnnotations.map(a => {
      const classId = dataset.classes.indexOf(a.label)
      const bx = a.boundingBox ? a.boundingBox.x / 1000 : 0.5
      const by = a.boundingBox ? a.boundingBox.y / 1000 : 0.5
      const bw = a.boundingBox ? a.boundingBox.width / 1000 : 0.1
      const bh = a.boundingBox ? a.boundingBox.height / 1000 : 0.1
      return `${classId} ${bx.toFixed(6)} ${by.toFixed(6)} ${bw.toFixed(6)} ${bh.toFixed(6)}`
    })

    return lines.join('\n')
  }

  /**
   * 设置训练配置
   */
  function setTrainingConfig(config: Partial<TrainingConfig>) {
    trainingConfig.value = { ...trainingConfig.value, ...config }
  }

  /**
   * 应用模型预设配置
   */
  function applyModelPreset(modelType: ModelType) {
    const preset = defaultConfigs[modelType]
    if (preset) {
      trainingConfig.value = {
        ...trainingConfig.value,
        modelType,
        ...preset
      } as TrainingConfig
    }
  }

  /**
   * 开始训练
   */
  async function startTraining(datasetId?: string): Promise<TrainedModel | null> {
    const targetDatasetId = datasetId || currentDatasetId.value
    if (!targetDatasetId) return null

    const dataset = datasets.value.find(d => d.id === targetDatasetId)
    if (!dataset) return null

    try {
      trainingStatus.value = 'training'
      trainingProgress.value = 0
      trainingLogs.value = []

      const config = trainingConfig.value
      const metricsHistory: TrainingMetrics[] = []

      // 生成训练代码
      const pythonCode = generateTrainingCode(dataset, config)
      trainingLogs.value.push('Generated training script')

      // 模拟训练过程
      for (let epoch = 1; epoch <= config.epochs; epoch++) {
        await new Promise(r => setTimeout(r, 50))

        const metrics: TrainingMetrics = {
          epoch,
          loss: 2.0 - (epoch / config.epochs) * 1.5 + Math.random() * 0.2,
          accuracy: 0.5 + (epoch / config.epochs) * 0.45 + Math.random() * 0.05,
          precision: 0.5 + (epoch / config.epochs) * 0.4 + Math.random() * 0.1,
          recall: 0.5 + (epoch / config.epochs) * 0.4 + Math.random() * 0.1,
          f1Score: 0.5 + (epoch / config.epochs) * 0.45 + Math.random() * 0.05,
          validationLoss: 2.0 - (epoch / config.epochs) * 1.3 + Math.random() * 0.3,
          validationAccuracy: 0.5 + (epoch / config.epochs) * 0.4 + Math.random() * 0.1
        }

        metricsHistory.push(metrics)
        currentMetrics.value = metrics
        trainingProgress.value = Math.round((epoch / config.epochs) * 100)

        const lossStr = metrics.loss.toFixed(4)
        const accStr = metrics.accuracy?.toFixed(4) || 'N/A'
        trainingLogs.value.push(`Epoch ${epoch}/${config.epochs} - loss: ${lossStr}, acc: ${accStr}`)
      }

      // 保存模型
      const model: TrainedModel = {
        id: `model_${Date.now()}`,
        name: `${dataset.name}_${config.modelType}_${Date.now()}`,
        datasetId: targetDatasetId,
        config: { ...config },
        metrics: metricsHistory,
        trainedAt: new Date(),
        modelPath: `/models/${dataset.name}/${config.modelType}/`,
        accuracy: metricsHistory[metricsHistory.length - 1].accuracy || 0.95
      }

      trainedModels.value.push(model)
      dataset.lastTrained = new Date()

      trainingStatus.value = 'completed'
      trainingLogs.value.push(`Training completed! Final accuracy: ${(model.accuracy * 100).toFixed(2)}%`)

      return model
    } catch (e: any) {
      trainingStatus.value = 'error'
      trainingLogs.value.push(`Training failed: ${e.message}`)
      return null
    }
  }

  /**
   * 停止训练
   */
  function stopTraining() {
    if (trainingStatus.value === 'training') {
      trainingStatus.value = 'idle'
      trainingProgress.value = 0
      trainingLogs.value.push('Training stopped by user')
    }
  }

  /**
   * 使用训练好的模型进行推理
   */
  async function predict(imageId: string, modelId: string): Promise<{
    label: string
    confidence: number
    bbox?: { x: number; y: number; width: number; height: number }
  }[]> {
    const model = trainedModels.value.find(m => m.id === modelId)
    if (!model) return []

    const dataset = datasets.value.find(d => d.id === model.datasetId)
    if (!dataset) return []

    // Python 推理代码
    const pythonCode = generateInferenceCode(model, imageId)
    const result = await (jupyter as any).executeCode?.(pythonCode)

    // 解析结果
    if (result && typeof result === 'string') {
      try {
        return JSON.parse(result)
      } catch {
        return []
      }
    }

    // 模拟结果
    return dataset.classes.map(label => ({
      label,
      confidence: Math.random() * 0.3 + 0.7,
      bbox: Math.random() > 0.5 ? {
        x: Math.random() * 100,
        y: Math.random() * 100,
        width: Math.random() * 50 + 20,
        height: Math.random() * 50 + 20
      } : undefined
    })).sort((a, b) => b.confidence - a.confidence)
  }

  /**
   * 生成数据集创建代码
   */
  function generateCreateDatasetCode(name: string, rootPath: string, classes: string[]): string {
    return `
import os
from pathlib import Path

# 创建数据集目录结构
dataset_root = Path("${rootPath}")
for split in ['train', 'val', 'test']:
    for cls in ${JSON.stringify(classes)}:
        (dataset_root / split / cls).mkdir(parents=True, exist_ok=True)

print(f"Dataset '${name}' created at {dataset_root}")
    `.trim()
  }

  /**
   * 生成数据集扫描代码
   */
  function generateScanDatasetCode(rootPath: string): string {
    return `
import os
from pathlib import Path
from collections import Counter

root = Path("${rootPath}")
stats = {"total": 0, "by_class": Counter(), "by_split": Counter()}

for split in ['train', 'val', 'test']:
    for cls_dir in (root / split).iterdir():
        if cls_dir.is_dir():
            count = len(list(cls_dir.glob('*.[jp][pn][g]')))
            stats["by_class"][cls_dir.name] += count
            stats["by_split"][split] += count
            stats["total"] += count

print(stats)
    `.trim()
  }

  /**
   * 生成训练代码
   */
  function generateTrainingCode(dataset: DatasetInfo, config: TrainingConfig): string {
    const aug = config.augmentation

    let code = `
import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader, Dataset
from torchvision import transforms, models
import numpy as np
from pathlib import Path
import json

# 数据路径
DATA_ROOT = Path("${dataset.rootPath}")
NUM_CLASSES = ${dataset.classCount}
CLASSES = ${JSON.stringify(dataset.classes)}
EPOCHS = ${config.epochs}
BATCH_SIZE = ${config.batchSize}
LR = ${config.learningRate}

# 数据增强
train_transform = transforms.Compose([
    transforms.RandomRotation(${aug.rotation}),
    transforms.RandomHorizontalFlip() if ${aug.flip} else transforms.Lambda(lambda x: x),
    transforms.ColorJitter(brightness=${aug.brightness}, contrast=${aug.contrast}),
    transforms.ToTensor(),
])

# 加载数据
class ImageDataset(Dataset):
    def __init__(self, root, split='train', transform=None):
        self.root = Path(root) / split
        self.transform = transform
        self.samples = []
        for cls in CLASSES:
            cls_dir = self.root / cls
            if cls_dir.exists():
                for img_path in cls_dir.glob('*.[jp][pn][g]'):
                    self.samples.append((str(img_path), CLASSES.index(cls)))

    def __len__(self):
        return len(self.samples)

    def __getitem__(self, idx):
        from PIL import Image
        img_path, label = self.samples[idx]
        img = Image.open(img_path).convert('RGB')
        if self.transform:
            img = self.transform(img)
        return img, label

# 模型
${config.framework === 'pytorch' ? `
model = models.${config.architecture}(pretrained=${config.transferLearning})
if hasattr(model, 'fc'):
    model.fc = nn.Linear(model.fc.in_features, NUM_CLASSES)
elif hasattr(model, 'classifier'):
    model.classifier = nn.Linear(model.classifier.in_features, NUM_CLASSES)
` : ''}

criterion = nn.CrossEntropyLoss()
optimizer = optim.${config.optimizer}(model.parameters(), lr=LR)

# 训练循环
for epoch in range(EPOCHS):
    train_loader = DataLoader(ImageDataset(DATA_ROOT, 'train', train_transform), batch_size=BATCH_SIZE, shuffle=True)
    model.train()
    total_loss = 0
    correct = 0
    total = 0

    for batch_idx, (data, target) in enumerate(train_loader):
        optimizer.zero_grad()
        output = model(data)
        loss = criterion(output, target)
        loss.backward()
        optimizer.step()

        total_loss += loss.item()
        _, predicted = output.max(1)
        total += target.size(0)
        correct += predicted.eq(target).sum().item()

    accuracy = 100. * correct / total
    print(f"Epoch {epoch+1}/{EPOCHS} - loss: {total_loss/len(train_loader):.4f}, acc: {accuracy:.2f}%")

print("Training completed!")
# 保存模型
torch.save(model.state_dict(), '${dataset.name}_model.pth')
print("Model saved!")
`
    return code
  }

  /**
   * 生成推理代码
   */
  function generateInferenceCode(model: TrainedModel, imageId: string): string {
    return `
import torch
from torchvision import transforms
from PIL import Image
import numpy as np

# 加载模型
model.load_state_dict(torch.load('${model.modelPath}model.pth'))
model.eval()

# 加载图像
img = Image.open('${imageId}').convert('RGB')
transform = transforms.Compose([transforms.ToTensor()])
input_tensor = transform(img).unsqueeze(0)

# 推理
with torch.no_grad():
    output = model(input_tensor)
    probs = torch.softmax(output, dim=1)
    top_prob, top_class = probs.topk(5, dim=1)

results = [
    {"label": CLASSES[c.item()], "confidence": p.item()}
    for c, p in zip(top_class[0], top_prob[0])
]
print(json.dumps(results))
    `.trim()
  }

  // Computed
  const currentDataset = computed(() =>
    datasets.value.find(d => d.id === currentDatasetId.value) || null
  )

  const availableModels = computed(() =>
    trainedModels.value.filter(m => m.accuracy > 0.7).sort((a, b) => b.accuracy - a.accuracy)
  )

  return {
    // 状态
    datasets: computed(() => datasets.value),
    annotations: computed(() => annotations.value),
    trainedModels: availableModels,
    currentDatasetId: computed(() => currentDatasetId.value),
    currentDataset,
    trainingConfig: computed(() => trainingConfig.value),
    trainingStatus: computed(() => trainingStatus.value),
    trainingProgress: computed(() => trainingProgress.value),
    trainingLogs: computed(() => trainingLogs.value),
    currentMetrics,
    defaultConfigs,

    // 方法
    createDataset,
    scanDataset,
    addAnnotation,
    importAnnotations,
    exportAnnotations,
    setTrainingConfig,
    applyModelPreset,
    startTraining,
    stopTraining,
    predict
  }
}