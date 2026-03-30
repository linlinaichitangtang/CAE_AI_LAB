<template>
  <div class="space-y-4">
    <!-- Contact Pairs List -->
    <div class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">接触对</h4>
      <div class="space-y-2">
        <div v-if="contactPairs.length === 0" class="text-xs text-[var(--text-muted)] text-center py-4">
          暂无接触对
        </div>
        <div v-for="(pair, index) in contactPairs" :key="index" 
             class="bg-[var(--bg-elevated)] rounded-lg p-3 space-y-2">
          <div class="flex justify-between items-center">
            <span class="text-xs font-medium">{{ pair.name || `接触对 ${index + 1}` }}</span>
            <button @click="removeContactPair(index)" class="text-[var(--text-muted)] hover:text-red-400">
              <span class="text-xs">✕</span>
            </button>
          </div>
          <div class="grid grid-cols-2 gap-2 text-xs">
            <div>
              <span class="text-[var(--text-muted)]">主面:</span>
              <span class="ml-1">{{ pair.master }}</span>
            </div>
            <div>
              <span class="text-[var(--text-muted)]">从面:</span>
              <span class="ml-1">{{ pair.slave }}</span>
            </div>
          </div>
          <div class="text-xs text-[var(--text-muted)]">
            {{ getContactTypeLabel(pair.type) }} | 摩擦: {{ pair.friction }}
          </div>
        </div>
        <button @click="showAddDialog = true" class="btn btn-secondary w-full text-xs">
          <span class="mr-1">+</span> 添加接触对
        </button>
      </div>
    </div>

    <!-- Contact Type Settings -->
    <div class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">接触类型</h4>
      <div class="space-y-3">
        <div>
          <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">类型</label>
          <select v-model="selectedContactType" class="input w-full text-xs">
            <option value="surface_to_surface">面对面接触 (S2S)</option>
            <option value="node_to_surface">点对面接触 (N2S)</option>
            <option value="tie">绑定接触 (Tie)</option>
            <option value="bolt_preload">螺栓预紧力</option>
          </select>
        </div>
        <div>
          <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">接触算法</label>
          <select v-model="selectedAlgorithm" class="input w-full text-xs">
            <option value="penalty">Penalty</option>
            <option value="lagrange">Lagrange</option>
            <option value="direct">Direct</option>
          </select>
        </div>
      </div>
    </div>

    <!-- Friction Settings -->
    <div class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">摩擦设置</h4>
      <div class="space-y-3">
        <div>
          <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">摩擦系数</label>
          <input v-model.number="friction" type="number" step="0.01" min="0" max="1" class="input w-full text-xs" />
        </div>
        <div v-if="selectedContactType !== 'tie'">
          <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">切向刚度</label>
          <input v-model.number="tangentialStiffness" type="number" step="0.1" min="0" class="input w-full text-xs" />
        </div>
      </div>
    </div>

    <!-- Contact Stiffness -->
    <div class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">接触刚度</h4>
      <div class="space-y-3">
        <div>
          <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">法向刚度 (Normal)</label>
          <input v-model.number="normalStiffness" type="number" step="0.1" min="0.01" class="input w-full text-xs" />
        </div>
        <div>
          <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">切向刚度 (Tangential)</label>
          <input v-model.number="tangentialStiffness" type="number" step="0.1" min="0.01" class="input w-full text-xs" />
        </div>
      </div>
    </div>

    <!-- Gap Settings -->
    <div class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">间隙设置</h4>
      <div class="space-y-3">
        <label class="flex items-center gap-2 cursor-pointer">
          <input v-model="checkInitialGap" type="checkbox" class="rounded" />
          <span class="text-xs text-[var(--text-primary)]">初始间隙判断</span>
        </label>
        <div v-if="checkInitialGap">
          <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">间隙容差</label>
          <input v-model.number="gapTolerance" type="number" step="0.001" class="input w-full text-xs" />
        </div>
      </div>
    </div>

    <!-- Solver Settings -->
    <div class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">求解设置</h4>
      <div class="space-y-3">
        <label class="flex items-center gap-2 cursor-pointer">
          <input v-model="autoIncrement" type="checkbox" class="rounded" />
          <span class="text-xs text-[var(--text-primary)]">自动增加迭代步</span>
        </label>
        <div>
          <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">收敛容差</label>
          <input v-model.number="convergenceTolerance" type="number" step="0.0001" min="0" class="input w-full text-xs" />
        </div>
        <div>
          <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">非线性迭代次数</label>
          <input v-model.number="maxIterations" type="number" min="1" max="1000" class="input w-full text-xs" />
        </div>
      </div>
    </div>

    <!-- Add Contact Pair Dialog -->
    <div v-if="showAddDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div class="bg-[var(--bg-surface)] rounded-lg p-4 w-80 space-y-4">
        <h3 class="text-sm font-medium">添加接触对</h3>
        
        <div>
          <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">名称</label>
          <input v-model="newPair.name" type="text" class="input w-full text-xs" placeholder="接触对名称" />
        </div>
        
        <div>
          <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">主面 (Master)</label>
          <select v-model="newPair.master" class="input w-full text-xs">
            <option value="">选择主面...</option>
            <option v-for="s in availableSurfaces" :key="s" :value="s">{{ s }}</option>
          </select>
        </div>
        
        <div>
          <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">从面 (Slave)</label>
          <select v-model="newPair.slave" class="input w-full text-xs">
            <option value="">选择从面...</option>
            <option v-for="s in availableSurfaces" :key="s" :value="s">{{ s }}</option>
          </select>
        </div>
        
        <div>
          <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">接触类型</label>
          <select v-model="newPair.type" class="input w-full text-xs">
            <option value="surface_to_surface">面对面接触</option>
            <option value="node_to_surface">点对面接触</option>
            <option value="tie">绑定接触</option>
            <option value="bolt_preload">螺栓预紧力</option>
          </select>
        </div>
        
        <div>
          <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">摩擦系数</label>
          <input v-model.number="newPair.friction" type="number" step="0.01" min="0" max="1" class="input w-full text-xs" />
        </div>
        
        <div class="flex gap-2">
          <button @click="showAddDialog = false" class="btn btn-secondary flex-1 text-xs">取消</button>
          <button @click="addContactPair" class="btn btn-primary flex-1 text-xs">确定</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed } from 'vue';

// Contact pair interface
interface ContactPair {
  id: string;
  name: string;
  master: string;
  slave: string;
  type: string;
  friction: number;
  algorithm?: string;
  normalStiffness?: number;
  tangentialStiffness?: number;
}

// State
const contactPairs = ref<ContactPair[]>([]);
const showAddDialog = ref(false);

// Settings
const selectedContactType = ref('surface_to_surface');
const selectedAlgorithm = ref('penalty');
const friction = ref(0.2);
const normalStiffness = ref(1.0);
const tangentialStiffness = ref(1.0);
const checkInitialGap = ref(false);
const gapTolerance = ref(0.001);
const autoIncrement = ref(true);
const convergenceTolerance = ref(0.0001);
const maxIterations = ref(100);

// Available surfaces (should be populated from the model)
const availableSurfaces = ref([
  'Surf-Bottom',
  'Surf-Top',
  'Surf-Side1',
  'Surf-Side2',
  'Surf-Bolt1',
  'Surf-Bolt2',
]);

// New pair template
const newPair = reactive({
  name: '',
  master: '',
  slave: '',
  type: 'surface_to_surface',
  friction: 0.2,
});

// Methods
const getContactTypeLabel = (type: string): string => {
  const labels: Record<string, string> = {
    'surface_to_surface': 'S2S 面对面',
    'node_to_surface': 'N2S 点对面',
    'tie': 'Tie 绑定',
    'bolt_preload': '螺栓预紧',
  };
  return labels[type] || type;
};

const addContactPair = () => {
  if (!newPair.master || !newPair.slave) {
    return;
  }
  
  const pair: ContactPair = {
    id: `cp_${Date.now()}`,
    name: newPair.name || `接触对 ${contactPairs.value.length + 1}`,
    master: newPair.master,
    slave: newPair.slave,
    type: newPair.type,
    friction: newPair.friction,
    algorithm: selectedAlgorithm.value,
    normalStiffness: normalStiffness.value,
    tangentialStiffness: tangentialStiffness.value,
  };
  
  contactPairs.value.push(pair);
  
  // Reset form
  Object.assign(newPair, {
    name: '',
    master: '',
    slave: '',
    type: 'surface_to_surface',
    friction: 0.2,
  });
  showAddDialog.value = false;
};

const removeContactPair = (index: number) => {
  contactPairs.value.splice(index, 1);
};

// Export settings for backend
const getContactSettings = computed(() => ({
  contactPairs: contactPairs.value,
  settings: {
    contactType: selectedContactType.value,
    algorithm: selectedAlgorithm.value,
    friction: friction.value,
    normalStiffness: normalStiffness.value,
    tangentialStiffness: tangentialStiffness.value,
    checkInitialGap: checkInitialGap.value,
    gapTolerance: gapTolerance.value,
    autoIncrement: autoIncrement.value,
    convergenceTolerance: convergenceTolerance.value,
    maxIterations: maxIterations.value,
  },
}));

// Emit settings when needed
const emit = defineEmits<{
  (e: 'update:contactSettings', value: typeof getContactSettings.value): void;
}>();
</script>

<style scoped>
.panel-section {
  padding-bottom: 16px;
  border-bottom: 1px solid var(--border-subtle);
}

.panel-section:last-child {
  border-bottom: none;
  padding-bottom: 0;
}

input[type="number"]::-webkit-inner-spin-button,
input[type="number"]::-webkit-outer-spin-button {
  -webkit-appearance: none;
  margin: 0;
}
</style>