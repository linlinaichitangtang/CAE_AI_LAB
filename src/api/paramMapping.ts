/**
 * V2.0-004: 跨尺度参数映射表
 * CAELab V2.0 - 多尺度工作流编排器
 *
 * 提供跨尺度参数映射规则的增删改查、自动映射、灵敏度分析等功能。
 */
import { invoke } from '@tauri-apps/api/core';

// ============ 类型定义 ============

/** 参数映射转换方式 */
export type PmTransformation = 'direct' | 'linear_fit' | 'polynomial' | 'interpolation' | 'ml_model';

/** 单位转换信息 */
export interface PmUnitConversion {
  from_unit: string;
  to_unit: string;
  factor: number;
}

/** 跨尺度参数映射规则 */
export interface PmMappingRule {
  /** 规则唯一标识 */
  id: string;
  /** 源尺度 */
  source_scale: string;
  /** 目标尺度 */
  target_scale: string;
  /** 源物理量 */
  source_quantity: string;
  /** 目标参数 */
  target_parameter: string;
  /** 转换方式 */
  transformation: PmTransformation;
  /** 转换系数 */
  coefficients: number[];
  /** 单位转换信息 */
  unit_conversion: PmUnitConversion;
  /** 规则描述 */
  description: string;
  /** 置信度 (0~1) */
  confidence: number;
}

/** 参数映射表 */
export interface PmMappingTable {
  /** 映射规则列表 */
  rules: PmMappingRule[];
  /** 映射表版本号 */
  version: string;
  /** 最后更新时间 (ISO 8601) */
  last_updated: string;
}

/** 自动映射结果中的单条映射 */
export interface PmAutoMappingItem {
  source_quantity: string;
  target_parameter: string;
  value: unknown;
  unit: string;
  confidence: number;
}

/** 自动参数映射结果 */
export interface PmAutoMappingResult {
  /** 源节点 ID */
  source_node_id: string;
  /** 目标节点 ID */
  target_node_id: string;
  /** 映射结果列表 */
  mappings: PmAutoMappingItem[];
  /** 未映射的源字段 */
  unmapped_source_fields: string[];
  /** 未映射的目标参数 */
  unmapped_target_params: string[];
}

/** 映射灵敏度条目 */
export interface PmSensitivityEntry {
  /** 参数名称 */
  parameter: string;
  /** 灵敏度值 */
  sensitivity: number;
  /** 源尺度 */
  source_scale: string;
  /** 目标尺度 */
  target_scale: string;
}

// ============ API 函数 ============

/**
 * 获取当前跨尺度参数映射表
 * @returns 完整的参数映射表，包含所有规则及版本信息
 */
export async function GetMappingTable(): Promise<PmMappingTable> {
  return invoke<PmMappingTable>('get_mapping_table');
}

/**
 * 更新已有的参数映射规则
 * @param rule - 要更新的映射规则（需包含有效的 id）
 */
export async function updateMappingRule(rule: PmMappingRule): Promise<void> {
  return invoke<void>('update_mapping_rule', { rule });
}

/**
 * 新增一条参数映射规则
 * @param rule - 新的映射规则
 * @returns 服务端创建后返回的完整规则（含生成的 id）
 */
export async function addMappingRule(rule: PmMappingRule): Promise<PmMappingRule> {
  return invoke<PmMappingRule>('add_mapping_rule', { rule });
}

/**
 * 删除指定的参数映射规则
 * @param ruleId - 要删除的规则 ID
 */
export async function deleteMappingRule(ruleId: string): Promise<void> {
  return invoke<void>('delete_mapping_rule', { ruleId });
}

/**
 * 自动映射两个节点之间的参数
 * 根据源节点数据和目标节点参数定义，自动推断最佳映射关系。
 * @param sourceNodeId - 源节点 ID
 * @param targetNodeId - 目标节点 ID
 * @param sourceData - 源节点输出的数据
 * @returns 自动映射结果，包含已映射项和未映射项
 */
export async function autoMapParameters(
  sourceNodeId: string,
  targetNodeId: string,
  sourceData: Record<string, unknown>
): Promise<PmAutoMappingResult> {
  return invoke<PmAutoMappingResult>('auto_map_parameters', {
    sourceNodeId,
    targetNodeId,
    sourceData,
  });
}

/**
 * 批量应用映射结果到目标节点
 * @param mappings - 自动映射产生的映射项列表
 * @returns 应用结果，包含成功状态和实际应用的数量
 */
export async function applyMapping(
  mappings: PmAutoMappingResult['mappings']
): Promise<{ success: boolean; applied_count: number }> {
  return invoke<{ success: boolean; applied_count: number }>('apply_mapping', { mappings });
}

/**
 * 获取指定尺度对之间的参数映射灵敏度
 * 用于评估各参数对跨尺度传递结果的影响程度。
 * @param sourceScale - 源尺度名称
 * @param targetScale - 目标尺度名称
 * @returns 灵敏度条目列表
 */
export async function getMappingSensitivity(
  sourceScale: string,
  targetScale: string
): Promise<PmSensitivityEntry[]> {
  return invoke<PmSensitivityEntry[]>('get_mapping_sensitivity', {
    sourceScale,
    targetScale,
  });
}
