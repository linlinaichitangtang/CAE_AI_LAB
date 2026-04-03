/**
 * V2.0-005: 典型工作流模板
 * CAELab V2.0 - 多尺度工作流编排器
 *
 * 提供五种典型材料计算工作流模板的查询、应用、创建及智能推荐功能。
 */
import { invoke } from '@tauri-apps/api/core';

// ============ 类型定义 ============

/** 工作流模板类别 */
export type PresetCategory = 'creep' | 'solidification' | 'precipitation' | 'fracture' | 'diffusion';

/** 模板步骤中的关键参数定义 */
export interface PresetKeyParameter {
  /** 参数名称 */
  name: string;
  /** 默认值 */
  default_value: unknown;
  /** 参数描述 */
  description: string;
}

/** 模板步骤中的预期输出 */
export interface PresetExpectedOutput {
  /** 物理量名称 */
  quantity: string;
  /** 单位 */
  unit: string;
}

/** 工作流模板中的单个步骤 */
export interface PresetStep {
  /** 所属尺度 */
  scale: string;
  /** 步骤名称 */
  name: string;
  /** 步骤描述 */
  description: string;
  /** 关键参数列表 */
  key_parameters: PresetKeyParameter[];
  /** 预期输出列表 */
  expected_output: PresetExpectedOutput[];
}

/** 工作流模板难度 */
export type PresetDifficulty = 'beginner' | 'intermediate' | 'advanced';

/** 工作流模板 */
export interface WorkflowPreset {
  /** 模板唯一标识 */
  id: string;
  /** 模板名称（英文） */
  name: string;
  /** 模板名称（中文） */
  name_zh: string;
  /** 模板类别 */
  category: PresetCategory;
  /** 模板描述 */
  description: string;
  /** 适用材料体系示例 */
  material_examples: string[];
  /** 工作流步骤列表 */
  steps: PresetStep[];
  /** 预估计算时间（分钟） */
  estimated_time_min: number;
  /** 难度等级 */
  difficulty: PresetDifficulty;
}

/** 模板自定义配置 */
export interface PresetCustomization {
  /** 要自定义的模板 ID */
  preset_id: string;
  /** 参数覆盖值 */
  parameter_overrides: Record<string, unknown>;
  /** 要跳过的步骤名称列表 */
  skip_steps: string[];
  /** 额外追加的步骤 */
  additional_steps: PresetStep[];
}

// ============ API 函数 ============

/**
 * 获取工作流模板列表
 * 可按类别筛选，不传类别则返回全部模板。
 * @param category - 可选，按模板类别筛选
 * @returns 符合条件的模板列表
 */
export async function listPresets(category?: PresetCategory): Promise<WorkflowPreset[]> {
  return invoke<WorkflowPreset[]>('list_presets', { category });
}

/**
 * 获取指定工作流模板的详细信息
 * @param id - 模板 ID
 * @returns 完整的模板详情
 */
export async function getPresetDetail(id: string): Promise<WorkflowPreset> {
  return invoke<WorkflowPreset>('get_preset_detail', { id });
}

/**
 * 应用工作流模板，创建新的工作流图
 * 可传入自定义配置来覆盖默认参数、跳过步骤或追加步骤。
 * @param presetId - 要应用的模板 ID
 * @param customization - 可选，自定义配置
 * @returns 新创建的工作流图 ID 和图数据
 */
export async function applyPreset(
  presetId: string,
  customization?: PresetCustomization
): Promise<{ graph_id: string; graph: object }> {
  return invoke<{ graph_id: string; graph: object }>('apply_preset', {
    presetId,
    customization,
  });
}

/**
 * 从已有工作流图创建新模板
 * 将当前工作流图保存为可复用的模板，方便后续快速启动类似工作流。
 * @param graphId - 源工作流图 ID
 * @param name - 新模板名称
 * @param category - 模板类别
 * @param description - 模板描述
 * @returns 新创建的模板 ID
 */
export async function createPresetFromGraph(
  graphId: string,
  name: string,
  category: PresetCategory,
  description: string
): Promise<{ preset_id: string }> {
  return invoke<{ preset_id: string }>('create_preset_from_graph', {
    graphId,
    name,
    category,
    description,
  });
}

/**
 * 根据材料体系和目标性能智能推荐工作流模板
 * @param materialSystem - 材料体系名称（如 "Ni-based superalloy"）
 * @param targetProperty - 目标性能（如 "creep resistance"）
 * @returns 推荐的模板列表，按匹配度排序
 */
export async function getPresetRecommendation(
  materialSystem: string,
  targetProperty: string
): Promise<WorkflowPreset[]> {
  return invoke<WorkflowPreset[]>('get_preset_recommendation', {
    materialSystem,
    targetProperty,
  });
}
