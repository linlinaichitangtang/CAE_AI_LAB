/**
 * 帮助文档系统
 */

export interface HelpArticle {
  id: string
  title: string
  category: 'getting-started' | 'modeling' | 'mesh' | 'simulation' | 'postprocessing' | 'advanced'
  icon: string  // emoji
  content: string  // HTML content
  keywords: string[]
}

export const helpArticles: HelpArticle[] = [
  {
    id: 'quick-start',
    title: '快速入门',
    category: 'getting-started',
    icon: '🚀',
    content: `
      <h2>快速入门指南</h2>
      <h3>1. 创建项目</h3>
      <p>点击首页的"新建项目"按钮，输入项目名称和描述。</p>
      <h3>2. 生成网格</h3>
      <p>在仿真视图中，选择 2D 或 3D 网格类型，设置几何参数和网格密度，点击"生成网格"。</p>
      <h3>3. 设置材料</h3>
      <p>在材料面板中，选择材料类型并输入材料参数（弹性模量、泊松比等），或从材料库中选择预设材料。</p>
      <h3>4. 定义边界条件</h3>
      <p>添加固定约束和载荷。固定约束限制节点的自由度，载荷施加力或压力。</p>
      <h3>5. 运行求解</h3>
      <p>选择分析类型（结构分析、模态分析等），点击"运行求解器"。</p>
      <h3>6. 查看结果</h3>
      <p>求解完成后，在 3D 视图中查看应力云图、变形图等后处理结果。</p>
    `,
    keywords: ['入门', '教程', '开始', '新手', 'guide', 'tutorial']
  },
  {
    id: 'mesh-guide',
    title: '网格生成指南',
    category: 'mesh',
    icon: '🔲',
    content: `
      <h2>网格生成指南</h2>
      <h3>结构化网格</h3>
      <p>CAELab 支持矩形/长方体区域的结构化网格生成。支持 11 种单元类型：</p>
      <ul>
        <li><strong>2D:</strong> S4(四边形4节点), S8(四边形8节点), S3(三角形3节点), S6(三角形6节点)</li>
        <li><strong>3D:</strong> C3D8(六面体8节点), C3D20(六面体20节点), C3D4(四面体4节点), C3D10(四面体10节点), C3D6(楔形体6节点), C3D15(楔形体15节点), C3D27(六面体27节点)</li>
      </ul>
      <h3>局部网格加密</h3>
      <p>在应力集中区域（如孔洞、缺口、载荷施加点）使用局部加密功能：</p>
      <ol>
        <li>选择加密区域类型：边/面/体</li>
        <li>指定加密方向和范围</li>
        <li>设置加密比例（1.5x ~ 4.0x）</li>
      </ol>
      <h3>网格质量检查</h3>
      <p>生成网格后，使用"网格质量检查"功能评估网格质量。关注以下指标：</p>
      <ul>
        <li><strong>长宽比 (Aspect Ratio):</strong> 越接近 1 越好，建议 &lt; 5</li>
        <li><strong>雅可比比 (Jacobian):</strong> 应 &gt; 0，建议 &gt; 0.5</li>
        <li><strong>偏斜度 (Skewness):</strong> 越接近 0 越好，建议 &lt; 0.5</li>
      </ul>
    `,
    keywords: ['网格', 'mesh', '单元', 'element', '加密', 'refinement', '质量']
  },
  {
    id: 'simulation-guide',
    title: '仿真分析指南',
    category: 'simulation',
    icon: '⚡',
    content: `
      <h2>仿真分析指南</h2>
      <h3>支持的分析类型</h3>
      <ul>
        <li><strong>结构分析 (Static):</strong> 线性静力分析，计算位移、应力和应变</li>
        <li><strong>模态分析 (Modal):</strong> 计算固有频率和振型</li>
        <li><strong>屈曲分析 (Buckling):</strong> 计算临界屈曲载荷</li>
        <li><strong>热传导 (Thermal):</strong> 稳态/瞬态热分析</li>
        <li><strong>CFD 流场:</strong> 不可压缩流体分析</li>
        <li><strong>瞬态动力学:</strong> 动态载荷响应分析</li>
      </ul>
      <h3>边界条件设置</h3>
      <p>固定约束类型：Fixed（全固定）、Pinned（铰支）、Roller（滚动支座）、Symmetry（对称边界）</p>
      <p>载荷类型：集中力、均布载荷、重力</p>
      <h3>求解器</h3>
      <p>CAELab 使用 CalculiX 求解器进行结构分析。求解过程中可查看实时进度和日志。</p>
    `,
    keywords: ['仿真', 'simulation', '分析', '求解', 'solver', '边界条件', '载荷']
  },
  {
    id: 'postprocessing-guide',
    title: '后处理指南',
    category: 'postprocessing',
    icon: '📊',
    content: `
      <h2>后处理指南</h2>
      <h3>结果显示模式</h3>
      <ul>
        <li><strong>位移 (Displacement):</strong> 显示节点位移幅值</li>
        <li><strong>应力 (Stress):</strong> 显示应力分量 (Sxx, Syy, Szz, Sxy, Syz, Sxz)</li>
        <li><strong>Von Mises:</strong> 显示 Von Mises 等效应力</li>
      </ul>
      <h3>剖切面工具</h3>
      <p>使用剖切面工具查看模型内部截面：</p>
      <ol>
        <li>启用"剖切面"开关</li>
        <li>选择剖切方向 (XY/XZ/YZ)</li>
        <li>拖动滑块调整剖切位置</li>
      </ol>
      <h3>变形动画</h3>
      <p>对于瞬态分析，可播放多时间步的变形过程动画。支持调整播放速度 (0.25x ~ 2x)。</p>
      <h3>报告导出</h3>
      <p>支持导出 HTML 和 PDF 格式的仿真报告，包含完整的模型参数和结果数据。</p>
    `,
    keywords: ['后处理', 'postprocessing', '云图', 'contour', '剖切', 'clipping', '动画', '报告']
  },
  {
    id: 'import-guide',
    title: '模型导入指南',
    category: 'modeling',
    icon: '📁',
    content: `
      <h2>模型导入指南</h2>
      <h3>支持的格式</h3>
      <ul>
        <li><strong>STEP (.step, .stp):</strong> 推荐格式，支持实体模型</li>
        <li><strong>IGES (.iges, .igs):</strong> 传统 CAD 交换格式</li>
      </ul>
      <h3>前置要求</h3>
      <p>模型导入需要系统安装以下工具之一：</p>
      <ul>
        <li><strong>GMSH</strong> (推荐): <a href="https://gmsh.info" target="_blank">https://gmsh.info</a></li>
        <li><strong>FreeCAD</strong>: <a href="https://freecad.org" target="_blank">https://freecad.org</a></li>
      </ul>
      <h3>导入步骤</h3>
      <ol>
        <li>在建模视图中点击"导入"按钮</li>
        <li>选择 STEP/IGES 文件</li>
        <li>等待转换完成（自动调用 GMSH/FreeCAD）</li>
        <li>导入的网格将自动加载到仿真模块</li>
      </ol>
    `,
    keywords: ['导入', 'import', 'STEP', 'IGES', 'CAD', 'GMSH', 'FreeCAD']
  },
  {
    id: 'advanced-features',
    title: '高级功能',
    category: 'advanced',
    icon: '🔬',
    content: `
      <h2>高级功能</h2>
      <h3>接触分析</h3>
      <p>支持面对面 (S2S)、点对面 (N2S)、绑定 (Tie) 和螺栓预紧力等接触类型。</p>
      <h3>参数化扫描</h3>
      <p>对设计参数进行参数化扫描，自动运行多组仿真并对比结果。</p>
      <h3>拓扑优化</h3>
      <p>基于 SIMP 方法的拓扑优化，支持体积约束和位移约束。</p>
      <h3>脚本自动化</h3>
      <p>使用脚本录制和回放功能自动化重复性工作流程。</p>
      <h3>标准算例验证</h3>
      <p>内置悬臂梁、简支梁等标准算例，自动对比理论解验证求解精度。</p>
    `,
    keywords: ['高级', 'advanced', '接触', 'contact', '优化', 'optimization', '参数化', '脚本']
  }
]

export function searchHelp(query: string): HelpArticle[] {
  const q = query.toLowerCase()
  return helpArticles.filter(article =>
    article.title.toLowerCase().includes(q) ||
    article.keywords.some(k => k.toLowerCase().includes(q)) ||
    article.content.toLowerCase().includes(q)
  )
}

export function getArticlesByCategory(category: string): HelpArticle[] {
  return helpArticles.filter(a => a.category === category)
}
