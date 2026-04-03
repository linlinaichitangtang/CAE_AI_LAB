<!--
  DFT 输入生成器 - DftInputView.vue
  功能：VASP / Quantum ESPRESSO 输入文件可视化编辑，模板支持
  V1.7-001: VASP 输入生成器
  V1.7-002: QE 输入生成器
-->
<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">DFT 输入生成器</h2>
        <p class="text-sm" style="color: var(--text-muted)">VASP / Quantum ESPRESSO 输入文件可视化编辑，模板支持</p>
      </div>
      <div class="flex items-center gap-2 flex-wrap justify-end">
        <button
          v-for="tpl in templateButtons"
          :key="tpl.id"
          @click="applyTemplate(tpl.id)"
          class="px-3 py-1.5 text-xs border rounded transition"
          style="border-color: var(--border-default); background: var(--bg-elevated); color: var(--text-secondary)"
        >
          {{ tpl.name }}
        </button>
      </div>
    </div>

    <!-- Code Tab Switch -->
    <div class="flex items-center gap-1 px-4 py-2 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <button
        v-for="tab in codeTabs"
        :key="tab.key"
        @click="activeCode = tab.key"
        class="px-3 py-1.5 text-xs rounded-full transition"
        :style="activeCode === tab.key
          ? 'background: var(--primary); color: #fff'
          : 'background: var(--bg-elevated); color: var(--text-secondary)'"
      >
        {{ tab.label }}
      </button>
      <div class="flex-1"></div>
      <button @click="handleGenerate" class="btn btn-primary text-xs">生成输入文件</button>
      <button @click="handleExport" :disabled="!generatedResult" class="btn btn-ghost text-xs">导出文件</button>
      <button @click="handleImportPoscar" v-if="activeCode === 'vasp'" class="btn btn-ghost text-xs">导入POSCAR</button>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">

      <!-- ========== VASP Panel ========== -->
      <template v-if="activeCode === 'vasp'">
        <div class="flex-1 flex overflow-hidden">
          <!-- Left: Editors -->
          <div class="w-[420px] overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

            <!-- VASP Sub-tabs -->
            <div class="flex items-center gap-1 mb-2">
              <button
                v-for="st in vaspSubTabs"
                :key="st.key"
                @click="vaspSubTab = st.key"
                class="px-2.5 py-1 text-[11px] rounded-full transition border"
                :style="vaspSubTab === st.key
                  ? 'background: var(--primary); color: #fff; border-color: var(--primary)'
                  : 'background: var(--bg-elevated); color: var(--text-secondary); border-color: var(--border-default)'"
              >
                {{ st.label }}
              </button>
            </div>

            <!-- POSCAR Editor -->
            <div v-show="vaspSubTab === 'poscar'" class="space-y-3">
              <div class="panel-section">
                <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">POSCAR</h4>
                <div class="space-y-2">
                  <div>
                    <label class="label">注释</label>
                    <input v-model="vaspPoscar.comment" type="text" class="input w-full text-xs" placeholder="System description" />
                  </div>
                  <div>
                    <label class="label">缩放因子</label>
                    <input v-model.number="vaspPoscar.scaling_factor" type="number" step="0.1" class="input w-full text-xs" />
                  </div>
                  <div>
                    <label class="label">晶格参数</label>
                    <div class="grid grid-cols-3 gap-1.5">
                      <div>
                        <span class="text-[10px]" style="color: var(--text-muted)">a (A)</span>
                        <input v-model.number="vaspPoscar.lattice.a" type="number" step="0.01" class="input w-full text-xs" />
                      </div>
                      <div>
                        <span class="text-[10px]" style="color: var(--text-muted)">b (A)</span>
                        <input v-model.number="vaspPoscar.lattice.b" type="number" step="0.01" class="input w-full text-xs" />
                      </div>
                      <div>
                        <span class="text-[10px]" style="color: var(--text-muted)">c (A)</span>
                        <input v-model.number="vaspPoscar.lattice.c" type="number" step="0.01" class="input w-full text-xs" />
                      </div>
                      <div>
                        <span class="text-[10px]" style="color: var(--text-muted)">alpha</span>
                        <input v-model.number="vaspPoscar.lattice.alpha" type="number" step="0.1" class="input w-full text-xs" />
                      </div>
                      <div>
                        <span class="text-[10px]" style="color: var(--text-muted)">beta</span>
                        <input v-model.number="vaspPoscar.lattice.beta" type="number" step="0.1" class="input w-full text-xs" />
                      </div>
                      <div>
                        <span class="text-[10px]" style="color: var(--text-muted)">gamma</span>
                        <input v-model.number="vaspPoscar.lattice.gamma" type="number" step="0.1" class="input w-full text-xs" />
                      </div>
                    </div>
                  </div>
                  <div>
                    <label class="label">原子类型</label>
                    <input v-model="atomTypesStr" type="text" class="input w-full text-xs" placeholder="Fe Cu (空格分隔)" @change="syncAtomTypes" />
                  </div>
                  <div>
                    <label class="label">各类型原子数</label>
                    <input v-model="atomCountsStr" type="text" class="input w-full text-xs" placeholder="4 4 (空格分隔)" @change="syncAtomCounts" />
                  </div>
                  <div class="flex items-center gap-2">
                    <label class="text-xs" style="color: var(--text-secondary)">选择性动力学</label>
                    <button
                      @click="vaspPoscar.selective_dynamics = vaspPoscar.selective_dynamics.length > 0 ? [] : vaspPoscar.positions.map(() => [true, true, true])"
                      :class="['relative w-9 h-5 rounded-full transition-colors']"
                      :style="{ background: vaspPoscar.selective_dynamics.length > 0 ? 'var(--primary)' : 'var(--border-default)' }"
                    >
                      <span
                        class="absolute top-0.5 w-4 h-4 rounded-full bg-white transition-transform"
                        :style="{ left: vaspPoscar.selective_dynamics.length > 0 ? '18px' : '2px' }"
                      ></span>
                    </button>
                  </div>
                  <div>
                    <div class="flex items-center justify-between mb-1">
                      <label class="label">坐标类型</label>
                      <div class="flex gap-1">
                        <button
                          @click="vaspPoscar.coordinate_type = 'cartesian'"
                          class="px-2 py-0.5 text-[10px] rounded border transition"
                          :style="vaspPoscar.coordinate_type === 'cartesian'
                            ? 'background: var(--primary); color: #fff; border-color: var(--primary)'
                            : 'background: var(--bg-elevated); color: var(--text-secondary); border-color: var(--border-default)'"
                        >Cartesian</button>
                        <button
                          @click="vaspPoscar.coordinate_type = 'direct'"
                          class="px-2 py-0.5 text-[10px] rounded border transition"
                          :style="vaspPoscar.coordinate_type === 'direct'
                            ? 'background: var(--primary); color: #fff; border-color: var(--primary)'
                            : 'background: var(--bg-elevated); color: var(--text-secondary); border-color: var(--border-default)'"
                        >Direct</button>
                      </div>
                    </div>
                  </div>
                  <!-- Position Table -->
                  <div class="overflow-x-auto">
                    <table class="w-full text-[11px]">
                      <thead>
                        <tr style="color: var(--text-muted)">
                          <th class="text-left pb-1 pr-1">Type</th>
                          <th class="text-right pb-1 px-1">x</th>
                          <th class="text-right pb-1 px-1">y</th>
                          <th class="text-right pb-1 px-1">z</th>
                          <th v-if="vaspPoscar.selective_dynamics.length > 0" class="text-center pb-1 px-1">dx</th>
                          <th v-if="vaspPoscar.selective_dynamics.length > 0" class="text-center pb-1 px-1">dy</th>
                          <th v-if="vaspPoscar.selective_dynamics.length > 0" class="text-center pb-1 px-1">dz</th>
                          <th class="pb-1 pl-1"></th>
                        </tr>
                      </thead>
                      <tbody>
                        <tr v-for="(pos, idx) in vaspPoscar.positions" :key="idx">
                          <td class="pr-1 py-0.5">
                            <select v-model="pos.type" class="input w-full text-[10px] py-0.5 px-1">
                              <option v-for="at in vaspPoscar.atom_types" :key="at" :value="at">{{ at }}</option>
                            </select>
                          </td>
                          <td class="px-1 py-0.5"><input v-model.number="pos.x" type="number" step="0.001" class="input w-full text-[10px] py-0.5 px-1" /></td>
                          <td class="px-1 py-0.5"><input v-model.number="pos.y" type="number" step="0.001" class="input w-full text-[10px] py-0.5 px-1" /></td>
                          <td class="px-1 py-0.5"><input v-model.number="pos.z" type="number" step="0.001" class="input w-full text-[10px] py-0.5 px-1" /></td>
                          <template v-if="vaspPoscar.selective_dynamics.length > 0">
                            <td class="px-1 py-0.5 text-center"><input type="checkbox" v-model="vaspPoscar.selective_dynamics[idx][0]" class="rounded" /></td>
                            <td class="px-1 py-0.5 text-center"><input type="checkbox" v-model="vaspPoscar.selective_dynamics[idx][1]" class="rounded" /></td>
                            <td class="px-1 py-0.5 text-center"><input type="checkbox" v-model="vaspPoscar.selective_dynamics[idx][2]" class="rounded" /></td>
                          </template>
                          <td class="pl-1 py-0.5">
                            <button @click="removePosition(idx)" class="text-[10px]" style="color: var(--accent-red)">x</button>
                          </td>
                        </tr>
                      </tbody>
                    </table>
                    <button @click="addPosition" class="btn btn-ghost text-[10px] mt-1">+ 添加原子</button>
                  </div>
                </div>
              </div>
            </div>

            <!-- INCAR Editor -->
            <div v-show="vaspSubTab === 'incar'" class="space-y-3">
              <div class="panel-section">
                <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">INCAR</h4>
                <div class="space-y-2">
                  <div>
                    <label class="label">SYSTEM</label>
                    <input v-model="vaspIncar.system" type="text" class="input w-full text-xs" />
                  </div>
                  <div class="grid grid-cols-2 gap-2">
                    <div>
                      <label class="label">ENCUT (eV)</label>
                      <input v-model.number="vaspIncar.encut" type="number" step="10" class="input w-full text-xs" />
                    </div>
                    <div>
                      <label class="label">NBANDS</label>
                      <input v-model.number="vaspIncar.nbands" type="number" class="input w-full text-xs" />
                    </div>
                  </div>
                  <div class="grid grid-cols-2 gap-2">
                    <div>
                      <label class="label">ISMEAR</label>
                      <input v-model.number="vaspIncar.ismear" type="number" class="input w-full text-xs" />
                    </div>
                    <div>
                      <label class="label">SIGMA</label>
                      <input v-model.number="vaspIncar.sigma" type="number" step="0.01" class="input w-full text-xs" />
                    </div>
                  </div>
                  <div class="grid grid-cols-2 gap-2">
                    <div>
                      <label class="label">ISIF</label>
                      <input v-model.number="vaspIncar.isif" type="number" class="input w-full text-xs" />
                    </div>
                    <div>
                      <label class="label">IBRION</label>
                      <input v-model.number="vaspIncar.ibrion" type="number" class="input w-full text-xs" />
                    </div>
                  </div>
                  <div class="grid grid-cols-2 gap-2">
                    <div>
                      <label class="label">NSW</label>
                      <input v-model.number="vaspIncar.nsw" type="number" class="input w-full text-xs" />
                    </div>
                    <div>
                      <label class="label">NELM</label>
                      <input v-model.number="vaspIncar.nelm" type="number" class="input w-full text-xs" />
                    </div>
                  </div>
                  <div class="grid grid-cols-2 gap-2">
                    <div>
                      <label class="label">EDIFF</label>
                      <input v-model.number="vaspIncar.ediff" type="number" step="1e-8" class="input w-full text-xs" />
                    </div>
                    <div>
                      <label class="label">EDIFFG</label>
                      <input v-model.number="vaspIncar.ediffg" type="number" step="0.001" class="input w-full text-xs" />
                    </div>
                  </div>
                  <div class="grid grid-cols-2 gap-2">
                    <div>
                      <label class="label">ICHARG</label>
                      <input v-model.number="vaspIncar.icharg" type="number" class="input w-full text-xs" />
                    </div>
                    <div>
                      <label class="label">PREC</label>
                      <select v-model="vaspIncar.prec" class="input w-full text-xs">
                        <option value="Normal">Normal</option>
                        <option value="Accurate">Accurate</option>
                        <option value="Single">Single</option>
                      </select>
                    </div>
                  </div>
                  <div class="grid grid-cols-2 gap-2">
                    <div>
                      <label class="label">GGA</label>
                      <select v-model="vaspIncar.gga" class="input w-full text-xs">
                        <option value="PE">PE</option>
                        <option value="91">91 (PW91)</option>
                        <option value="PS">PS (PBEsol)</option>
                        <option value="">None (LDA)</option>
                      </select>
                    </div>
                    <div>
                      <label class="label">KPOINTS_AUTO</label>
                      <button
                        @click="vaspIncar.kpoints_auto = !vaspIncar.kpoints_auto"
                        :class="['relative w-9 h-5 rounded-full transition-colors']"
                        :style="{ background: vaspIncar.kpoints_auto ? 'var(--primary)' : 'var(--border-default)' }"
                      >
                        <span
                          class="absolute top-0.5 w-4 h-4 rounded-full bg-white transition-transform"
                          :style="{ left: vaspIncar.kpoints_auto ? '18px' : '2px' }"
                        ></span>
                      </button>
                    </div>
                  </div>
                  <div class="grid grid-cols-2 gap-2">
                    <div class="flex items-center gap-2">
                      <label class="text-xs" style="color: var(--text-secondary)">LCHARG</label>
                      <button
                        @click="vaspIncar.lcharg = !vaspIncar.lcharg"
                        :class="['relative w-9 h-5 rounded-full transition-colors']"
                        :style="{ background: vaspIncar.lcharg ? 'var(--primary)' : 'var(--border-default)' }"
                      >
                        <span
                          class="absolute top-0.5 w-4 h-4 rounded-full bg-white transition-transform"
                          :style="{ left: vaspIncar.lcharg ? '18px' : '2px' }"
                        ></span>
                      </button>
                    </div>
                    <div class="flex items-center gap-2">
                      <label class="text-xs" style="color: var(--text-secondary)">LWAVE</label>
                      <button
                        @click="vaspIncar.lwave = !vaspIncar.lwave"
                        :class="['relative w-9 h-5 rounded-full transition-colors']"
                        :style="{ background: vaspIncar.lwave ? 'var(--primary)' : 'var(--border-default)' }"
                      >
                        <span
                          class="absolute top-0.5 w-4 h-4 rounded-full bg-white transition-transform"
                          :style="{ left: vaspIncar.lwave ? '18px' : '2px' }"
                        ></span>
                      </button>
                    </div>
                  </div>
                  <!-- LDA+U -->
                  <div class="border-t pt-2 mt-2" style="border-color: var(--border-subtle)">
                    <div class="flex items-center gap-2 mb-2">
                      <label class="text-xs font-medium" style="color: var(--text-primary)">LDA+U</label>
                      <button
                        @click="vaspIncar.ldau_luj = !vaspIncar.ldau_luj"
                        :class="['relative w-9 h-5 rounded-full transition-colors']"
                        :style="{ background: vaspIncar.ldau_luj ? 'var(--primary)' : 'var(--border-default)' }"
                      >
                        <span
                          class="absolute top-0.5 w-4 h-4 rounded-full bg-white transition-transform"
                          :style="{ left: vaspIncar.ldau_luj ? '18px' : '2px' }"
                        ></span>
                      </button>
                    </div>
                    <div v-if="vaspIncar.ldau_luj" class="space-y-2">
                      <div v-for="(lp, li) in vaspIncar.ldau_params" :key="li" class="flex items-center gap-1.5">
                        <input v-model="lp.element" type="text" class="input w-12 text-[10px] py-0.5 px-1" placeholder="Fe" />
                        <span class="text-[10px]" style="color: var(--text-muted)">U=</span>
                        <input v-model.number="lp.U" type="number" step="0.1" class="input w-14 text-[10px] py-0.5 px-1" />
                        <span class="text-[10px]" style="color: var(--text-muted)">J=</span>
                        <input v-model.number="lp.J" type="number" step="0.1" class="input w-14 text-[10px] py-0.5 px-1" />
                        <button @click="removeLdauParam(li)" class="text-[10px]" style="color: var(--accent-red)">x</button>
                      </div>
                      <button @click="addLdauParam" class="btn btn-ghost text-[10px]">+ 添加元素</button>
                    </div>
                  </div>
                  <!-- MAGMOM -->
                  <div class="border-t pt-2 mt-2" style="border-color: var(--border-subtle)">
                    <label class="label">MAGMOM</label>
                    <input v-model="magmomStr" type="text" class="input w-full text-xs" placeholder="5*2.0 5*-2.0" @change="syncMagmom" />
                    <div class="text-[10px] mt-1" style="color: var(--text-muted)">格式: N*value 或 value value ...</div>
                  </div>
                </div>
              </div>
            </div>

            <!-- KPOINTS Editor -->
            <div v-show="vaspSubTab === 'kpoints'" class="space-y-3">
              <div class="panel-section">
                <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">KPOINTS</h4>
                <div class="space-y-2">
                  <div>
                    <label class="label">K 点样式</label>
                    <div class="grid grid-cols-4 gap-1">
                      <button
                        v-for="ks in kpointStyles"
                        :key="ks.value"
                        @click="vaspKpoints.style = ks.value"
                        :class="['px-2 py-1.5 rounded text-[10px] text-center transition border', vaspKpoints.style === ks.value ? 'text-white' : '']"
                        :style="vaspKpoints.style === ks.value
                          ? 'background: var(--primary); border-color: var(--primary)'
                          : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
                      >
                        {{ ks.label }}
                      </button>
                    </div>
                  </div>
                  <div v-if="vaspKpoints.style !== 'line'">
                    <label class="label">K 点网格</label>
                    <div class="grid grid-cols-3 gap-1.5">
                      <div>
                        <span class="text-[10px]" style="color: var(--text-muted)">n1</span>
                        <input v-model.number="vaspKpoints.grid.n1" type="number" min="1" class="input w-full text-xs" />
                      </div>
                      <div>
                        <span class="text-[10px]" style="color: var(--text-muted)">n2</span>
                        <input v-model.number="vaspKpoints.grid.n2" type="number" min="1" class="input w-full text-xs" />
                      </div>
                      <div>
                        <span class="text-[10px]" style="color: var(--text-muted)">n3</span>
                        <input v-model.number="vaspKpoints.grid.n3" type="number" min="1" class="input w-full text-xs" />
                      </div>
                    </div>
                  </div>
                  <div v-if="vaspKpoints.style !== 'line'">
                    <label class="label">偏移</label>
                    <div class="grid grid-cols-3 gap-1.5">
                      <div>
                        <span class="text-[10px]" style="color: var(--text-muted)">s1</span>
                        <input v-model.number="vaspKpoints.shift.s1" type="number" step="0.5" class="input w-full text-xs" />
                      </div>
                      <div>
                        <span class="text-[10px]" style="color: var(--text-muted)">s2</span>
                        <input v-model.number="vaspKpoints.shift.s2" type="number" step="0.5" class="input w-full text-xs" />
                      </div>
                      <div>
                        <span class="text-[10px]" style="color: var(--text-muted)">s3</span>
                        <input v-model.number="vaspKpoints.shift.s3" type="number" step="0.5" class="input w-full text-xs" />
                      </div>
                    </div>
                  </div>
                  <!-- K-path for line mode -->
                  <div v-if="vaspKpoints.style === 'line'">
                    <label class="label">高对称 K 路径</label>
                    <div class="space-y-1">
                      <div v-for="(kp, ki) in vaspKpoints.kpath" :key="ki" class="flex items-center gap-1.5">
                        <input v-model="kp.label1" type="text" class="input w-12 text-[10px] py-0.5 px-1" placeholder="G" />
                        <span class="text-[10px]" style="color: var(--text-muted)">-></span>
                        <input v-model="kp.label2" type="text" class="input w-12 text-[10px] py-0.5 px-1" placeholder="X" />
                        <span class="text-[10px]" style="color: var(--text-muted)">n=</span>
                        <input v-model.number="kp.npoints" type="number" min="1" class="input w-14 text-[10px] py-0.5 px-1" />
                        <button @click="removeKpath(ki)" class="text-[10px]" style="color: var(--accent-red)">x</button>
                      </div>
                      <button @click="addKpath" class="btn btn-ghost text-[10px]">+ 添加路径段</button>
                    </div>
                  </div>
                </div>
              </div>
            </div>

          </div>

          <!-- Right: File Preview -->
          <div class="flex-1 flex flex-col overflow-hidden">
            <div class="flex items-center gap-1 px-4 py-2 border-b" style="background: var(--bg-elevated); border-color: var(--border-subtle)">
              <button
                v-for="ft in vaspFileTabs"
                :key="ft.key"
                @click="vaspFileTab = ft.key"
                class="px-2.5 py-1 text-[11px] rounded-full transition border"
                :style="vaspFileTab === ft.key
                  ? 'background: var(--primary); color: #fff; border-color: var(--primary)'
                  : 'background: var(--bg-surface); color: var(--text-secondary); border-color: var(--border-default)'"
              >
                {{ ft.label }}
              </button>
            </div>
            <div class="flex-1 overflow-auto p-4" style="background: var(--bg-base)">
              <pre v-if="vaspFileTab === 'poscar'" class="text-xs font-mono whitespace-pre leading-relaxed" style="color: var(--text-primary)">{{ vaspPoscarPreview }}</pre>
              <pre v-else-if="vaspFileTab === 'incar'" class="text-xs font-mono whitespace-pre leading-relaxed" style="color: var(--text-primary)">{{ vaspIncarPreview }}</pre>
              <pre v-else-if="vaspFileTab === 'kpoints'" class="text-xs font-mono whitespace-pre leading-relaxed" style="color: var(--text-primary)">{{ vaspKpointsPreview }}</pre>
            </div>
          </div>
        </div>
      </template>

      <!-- ========== QE Panel ========== -->
      <template v-if="activeCode === 'quantum_espresso'">
        <div class="flex-1 flex overflow-hidden">
          <!-- Left: Editors -->
          <div class="w-[420px] overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

            <!-- QE Sub-tabs -->
            <div class="flex items-center gap-1 mb-2">
              <button
                v-for="st in qeSubTabs"
                :key="st.key"
                @click="qeSubTab = st.key"
                class="px-2.5 py-1 text-[11px] rounded-full transition border"
                :style="qeSubTab === st.key
                  ? 'background: var(--primary); color: #fff; border-color: var(--primary)'
                  : 'background: var(--bg-elevated); color: var(--text-secondary); border-color: var(--border-default)'"
              >
                {{ st.label }}
              </button>
            </div>

            <!-- Control Block -->
            <div v-show="qeSubTab === 'control'" class="space-y-3">
              <div class="panel-section">
                <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">Control</h4>
                <div class="space-y-2">
                  <div>
                    <label class="label">calculation</label>
                    <select v-model="qeInput.control.calculation" class="input w-full text-xs">
                      <option value="scf">scf</option>
                      <option value="relax">relax</option>
                      <option value="vc-relax">vc-relax</option>
                      <option value="md">md</option>
                      <option value="nscf">nscf</option>
                      <option value="bands">bands</option>
                      <option value="dos">dos</option>
                    </select>
                  </div>
                  <div>
                    <label class="label">prefix</label>
                    <input v-model="qeInput.control.prefix" type="text" class="input w-full text-xs" />
                  </div>
                  <div>
                    <label class="label">outdir</label>
                    <input v-model="qeInput.control.outdir" type="text" class="input w-full text-xs" />
                  </div>
                  <div>
                    <label class="label">pseudo_dir</label>
                    <input v-model="qeInput.control.pseudo_dir" type="text" class="input w-full text-xs" />
                  </div>
                  <div class="grid grid-cols-2 gap-2">
                    <div>
                      <label class="label">etot_conv (Ry)</label>
                      <input v-model.number="qeInput.control.etot_conv" type="number" step="1e-6" class="input w-full text-xs" />
                    </div>
                    <div>
                      <label class="label">forc_conv (Ry/a.u.)</label>
                      <input v-model.number="qeInput.control.forc_conv" type="number" step="1e-5" class="input w-full text-xs" />
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- System Block -->
            <div v-show="qeSubTab === 'system'" class="space-y-3">
              <div class="panel-section">
                <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">System</h4>
                <div class="space-y-2">
                  <div class="grid grid-cols-2 gap-2">
                    <div>
                      <label class="label">ibrav</label>
                      <input v-model.number="qeInput.system.ibrav" type="number" class="input w-full text-xs" />
                    </div>
                    <div>
                      <label class="label">nat</label>
                      <input v-model.number="qeInput.system.nat" type="number" class="input w-full text-xs" />
                    </div>
                  </div>
                  <div class="grid grid-cols-2 gap-2">
                    <div>
                      <label class="label">nbp</label>
                      <input v-model.number="qeInput.system.nbp" type="number" class="input w-full text-xs" />
                    </div>
                    <div>
                      <label class="label">ecutwfc (Ry)</label>
                      <input v-model.number="qeInput.system.ecutwfc" type="number" step="1" class="input w-full text-xs" />
                    </div>
                  </div>
                  <div class="grid grid-cols-2 gap-2">
                    <div>
                      <label class="label">ecutrho (Ry)</label>
                      <input v-model.number="qeInput.system.ecutrho" type="number" step="1" class="input w-full text-xs" />
                    </div>
                    <div>
                      <label class="label">degauss (Ry)</label>
                      <input v-model.number="qeInput.system.degauss" type="number" step="0.001" class="input w-full text-xs" />
                    </div>
                  </div>
                  <div class="grid grid-cols-2 gap-2">
                    <div>
                      <label class="label">occupations</label>
                      <select v-model="qeInput.system.occupations" class="input w-full text-xs">
                        <option value="smearing">smearing</option>
                        <option value="fixed">fixed</option>
                        <option value="tetrahedra">tetrahedra</option>
                      </select>
                    </div>
                    <div>
                      <label class="label">smearing</label>
                      <select v-model="qeInput.system.smearing" class="input w-full text-xs">
                        <option value="gaussian">gaussian</option>
                        <option value="mp">mp (Methfessel-Paxton)</option>
                        <option value="fd">fd (Fermi-Dirac)</option>
                      </select>
                    </div>
                  </div>
                  <div>
                    <label class="label">celldm (6 values)</label>
                    <div class="grid grid-cols-6 gap-1">
                      <div v-for="(cd, ci) in qeInput.system.celldm" :key="ci">
                        <span class="text-[10px]" style="color: var(--text-muted)">{{ ci + 1 }}</span>
                        <input v-model.number="qeInput.system.celldm[ci]" type="number" step="0.1" class="input w-full text-[10px] py-0.5 px-1" />
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- Electrons Block -->
            <div v-show="qeSubTab === 'electrons'" class="space-y-3">
              <div class="panel-section">
                <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">Electrons</h4>
                <div class="space-y-2">
                  <div>
                    <label class="label">conv_thr (Ry)</label>
                    <input v-model.number="qeInput.electrons.conv_thr" type="number" step="1e-10" class="input w-full text-xs" />
                  </div>
                  <div>
                    <label class="label">mixing_beta</label>
                    <input v-model.number="qeInput.electrons.mixing_beta" type="number" step="0.01" min="0" max="1" class="input w-full text-xs" />
                  </div>
                  <div>
                    <label class="label">electron_maxstep</label>
                    <input v-model.number="qeInput.electrons.electron_maxstep" type="number" min="1" class="input w-full text-xs" />
                  </div>
                </div>
              </div>
            </div>

            <!-- Atomic Species -->
            <div v-show="qeSubTab === 'species'" class="space-y-3">
              <div class="panel-section">
                <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">原子种类</h4>
                <div class="space-y-1">
                  <div v-for="(sp, si) in qeInput.atomic_species" :key="si" class="flex items-center gap-1.5">
                    <input v-model="sp.name" type="text" class="input w-12 text-[10px] py-0.5 px-1" placeholder="Si" />
                    <input v-model.number="sp.mass" type="number" step="0.001" class="input w-16 text-[10px] py-0.5 px-1" placeholder="28.086" />
                    <input v-model="sp.pseudo" type="text" class="input flex-1 text-[10px] py-0.5 px-1" placeholder="Si.pbe-n-rrkjus_psl.1.0.0.UPF" />
                    <button @click="removeSpecies(si)" class="text-[10px]" style="color: var(--accent-red)">x</button>
                  </div>
                  <button @click="addSpecies" class="btn btn-ghost text-[10px]">+ 添加种类</button>
                </div>
              </div>
            </div>

            <!-- Atomic Positions -->
            <div v-show="qeSubTab === 'positions'" class="space-y-3">
              <div class="panel-section">
                <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">原子坐标</h4>
                <div class="flex items-center gap-2 mb-2">
                  <label class="text-xs" style="color: var(--text-secondary)">坐标类型</label>
                  <select v-model="qeInput.atomic_positions.type" class="input w-24 text-[10px] py-0.5">
                    <option value="crystal">crystal</option>
                    <option value="alat">alat</option>
                    <option value="bohr">bohr</option>
                    <option value="angstrom">angstrom</option>
                  </select>
                </div>
                <div class="overflow-x-auto">
                  <table class="w-full text-[11px]">
                    <thead>
                      <tr style="color: var(--text-muted)">
                        <th class="text-left pb-1 pr-1">Atom</th>
                        <th class="text-right pb-1 px-1">x</th>
                        <th class="text-right pb-1 px-1">y</th>
                        <th class="text-right pb-1 px-1">z</th>
                        <th class="pb-1 pl-1"></th>
                      </tr>
                    </thead>
                    <tbody>
                      <tr v-for="(ap, ai) in qeInput.atomic_positions.positions" :key="ai">
                        <td class="pr-1 py-0.5">
                          <input v-model="ap.atom" type="text" class="input w-12 text-[10px] py-0.5 px-1" />
                        </td>
                        <td class="px-1 py-0.5"><input v-model.number="ap.x" type="number" step="0.001" class="input w-full text-[10px] py-0.5 px-1" /></td>
                        <td class="px-1 py-0.5"><input v-model.number="ap.y" type="number" step="0.001" class="input w-full text-[10px] py-0.5 px-1" /></td>
                        <td class="px-1 py-0.5"><input v-model.number="ap.z" type="number" step="0.001" class="input w-full text-[10px] py-0.5 px-1" /></td>
                        <td class="pl-1 py-0.5">
                          <button @click="removeAtomicPosition(ai)" class="text-[10px]" style="color: var(--accent-red)">x</button>
                        </td>
                      </tr>
                    </tbody>
                  </table>
                  <button @click="addAtomicPosition" class="btn btn-ghost text-[10px] mt-1">+ 添加原子</button>
                </div>
              </div>
            </div>

            <!-- K Points -->
            <div v-show="qeSubTab === 'kpoints'" class="space-y-3">
              <div class="panel-section">
                <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">K 点</h4>
                <div class="space-y-2">
                  <div>
                    <label class="label">类型</label>
                    <select v-model="qeInput.k_points.type" class="input w-full text-xs">
                      <option value="automatic">automatic</option>
                      <option value="gamma">gamma</option>
                      <option value="crystal">crystal</option>
                    </select>
                  </div>
                  <div>
                    <label class="label">网格</label>
                    <div class="grid grid-cols-3 gap-1.5">
                      <div>
                        <span class="text-[10px]" style="color: var(--text-muted)">n1</span>
                        <input v-model.number="qeInput.k_points.grid.n1" type="number" min="1" class="input w-full text-xs" />
                      </div>
                      <div>
                        <span class="text-[10px]" style="color: var(--text-muted)">n2</span>
                        <input v-model.number="qeInput.k_points.grid.n2" type="number" min="1" class="input w-full text-xs" />
                      </div>
                      <div>
                        <span class="text-[10px]" style="color: var(--text-muted)">n3</span>
                        <input v-model.number="qeInput.k_points.grid.n3" type="number" min="1" class="input w-full text-xs" />
                      </div>
                    </div>
                  </div>
                  <div>
                    <label class="label">偏移</label>
                    <div class="grid grid-cols-3 gap-1.5">
                      <div>
                        <span class="text-[10px]" style="color: var(--text-muted)">s1</span>
                        <input v-model.number="qeInput.k_points.offset.s1" type="number" step="0.5" class="input w-full text-xs" />
                      </div>
                      <div>
                        <span class="text-[10px]" style="color: var(--text-muted)">s2</span>
                        <input v-model.number="qeInput.k_points.offset.s2" type="number" step="0.5" class="input w-full text-xs" />
                      </div>
                      <div>
                        <span class="text-[10px]" style="color: var(--text-muted)">s3</span>
                        <input v-model.number="qeInput.k_points.offset.s3" type="number" step="0.5" class="input w-full text-xs" />
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>

          </div>

          <!-- Right: File Preview -->
          <div class="flex-1 flex flex-col overflow-hidden">
            <div class="px-4 py-2 border-b" style="background: var(--bg-elevated); border-color: var(--border-subtle)">
              <span class="text-[11px] font-medium" style="color: var(--text-secondary)">pw.x 输入文件预览</span>
            </div>
            <div class="flex-1 overflow-auto p-4" style="background: var(--bg-base)">
              <pre class="text-xs font-mono whitespace-pre leading-relaxed" style="color: var(--text-primary)">{{ qeInputPreview }}</pre>
            </div>
          </div>
        </div>
      </template>

    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed } from 'vue'
import type {
  DftInputResult,
  VaspPoscar,
  VaspIncar,
  VaspKpoints,
  VaspLdauParam,
  VaspKpointPath,
  QeInput,
  QeAtomicSpecies,
  QeAtomicPosition,
} from '../api/dftInput'

// ============ Code Tab ============

const activeCode = ref<'vasp' | 'quantum_espresso'>('vasp')

const codeTabs = [
  { key: 'vasp' as const, label: 'VASP' },
  { key: 'quantum_espresso' as const, label: 'Quantum ESPRESSO' },
]

// ============ VASP Sub-tabs ============

const vaspSubTab = ref<'poscar' | 'incar' | 'kpoints'>('poscar')
const vaspSubTabs = [
  { key: 'poscar' as const, label: 'POSCAR' },
  { key: 'incar' as const, label: 'INCAR' },
  { key: 'kpoints' as const, label: 'KPOINTS' },
]

const vaspFileTab = ref<'poscar' | 'incar' | 'kpoints'>('poscar')
const vaspFileTabs = [
  { key: 'poscar' as const, label: 'POSCAR' },
  { key: 'incar' as const, label: 'INCAR' },
  { key: 'kpoints' as const, label: 'KPOINTS' },
]

// ============ QE Sub-tabs ============

const qeSubTab = ref<'control' | 'system' | 'electrons' | 'species' | 'positions' | 'kpoints'>('control')
const qeSubTabs = [
  { key: 'control' as const, label: 'Control' },
  { key: 'system' as const, label: 'System' },
  { key: 'electrons' as const, label: 'Electrons' },
  { key: 'species' as const, label: '原子种类' },
  { key: 'positions' as const, label: '原子坐标' },
  { key: 'kpoints' as const, label: 'K 点' },
]

// ============ Template Buttons ============

const templateButtons = [
  { id: 'vasp-scf', name: 'VASP-SCF' },
  { id: 'vasp-relax', name: 'VASP-结构优化' },
  { id: 'vasp-dos', name: 'VASP-态密度' },
  { id: 'vasp-band', name: 'VASP-能带' },
  { id: 'qe-scf', name: 'QE-SCF' },
  { id: 'qe-relax', name: 'QE-结构优化' },
]

// ============ K-point Styles ============

const kpointStyles = [
  { value: 'gamma' as const, label: 'Gamma' },
  { value: 'monkhorst' as const, label: 'Monkhorst' },
  { value: 'automatic' as const, label: 'Automatic' },
  { value: 'line' as const, label: 'Line' },
]

// ============ VASP POSCAR Default (Fe FCC) ============

const vaspPoscar = reactive<VaspPoscar>({
  comment: 'Fe FCC',
  scaling_factor: 1.0,
  lattice: { a: 2.87, b: 2.87, c: 2.87, alpha: 90, beta: 90, gamma: 90 },
  atom_types: ['Fe'],
  atom_counts: [4],
  selective_dynamics: [],
  positions: [
    { type: 'Fe', x: 0.0, y: 0.0, z: 0.0, dx: true, dy: true, dz: true },
    { type: 'Fe', x: 0.5, y: 0.5, z: 0.0, dx: true, dy: true, dz: true },
    { type: 'Fe', x: 0.5, y: 0.0, z: 0.5, dx: true, dy: true, dz: true },
    { type: 'Fe', x: 0.0, y: 0.5, z: 0.5, dx: true, dy: true, dz: true },
  ],
  coordinate_type: 'direct',
})

// ============ VASP INCAR Default ============

const vaspIncar = reactive<VaspIncar>({
  system: 'Fe FCC',
  encut: 400,
  ismear: 1,
  sigma: 0.2,
  isif: 2,
  ibrion: -1,
  nsw: 0,
  ediff: 1e-6,
  ediffg: -0.01,
  nelm: 60,
  lcharg: true,
  lwave: false,
  icharg: 0,
  prec: 'Accurate',
  kpoints_auto: true,
  nbands: 20,
  gga: 'PE',
  ldau_luj: false,
  ldau_params: [],
  magmom: [5.0, 5.0, 5.0, 5.0],
})

// ============ VASP KPOINTS Default ============

const vaspKpoints = reactive<VaspKpoints>({
  style: 'monkhorst',
  grid: { n1: 11, n2: 11, n3: 11 },
  shift: { s1: 0, s2: 0, s3: 0 },
  kpath: [
    { label1: 'G', label2: 'X', npoints: 20 },
    { label2: 'W', label1: 'X', npoints: 10 },
    { label1: 'W', label2: 'K', npoints: 10 },
    { label2: 'G', label1: 'K', npoints: 20 },
    { label1: 'G', label2: 'L', npoints: 20 },
    { label2: 'U', label1: 'L', npoints: 10 },
    { label1: 'U', label2: 'W', npoints: 10 },
    { label2: 'K', label1: 'U', npoints: 10 },
  ],
})

// ============ QE Input Default (Si) ============

const qeInput = reactive<QeInput>({
  control: {
    calculation: 'scf',
    prefix: 'si',
    outdir: './out/',
    pseudo_dir: './pseudo/',
    etot_conv: 1e-6,
    forc_conv: 1e-4,
  },
  system: {
    ibrav: 2,
    celldm: [10.26, 0, 0, 0, 0, 0],
    nat: 2,
    nbp: 1,
    ecutwfc: 40,
    ecutrho: 320,
    occupations: 'smearing',
    smearing: 'gaussian',
    degauss: 0.01,
  },
  electrons: {
    conv_thr: 1e-8,
    mixing_beta: 0.7,
    electron_maxstep: 100,
  },
  atomic_species: [
    { name: 'Si', mass: 28.086, pseudo: 'Si.pbe-n-rrkjus_psl.1.0.0.UPF' },
  ],
  atomic_positions: {
    type: 'crystal',
    positions: [
      { atom: 'Si', x: 0.0, y: 0.0, z: 0.0 },
      { atom: 'Si', x: 0.25, y: 0.25, z: 0.25 },
    ],
  },
  k_points: {
    type: 'automatic',
    grid: { n1: 8, n2: 8, n3: 8 },
    offset: { s1: 0, s2: 0, s3: 0 },
  },
})

// ============ Generated Result ============

const generatedResult = ref<DftInputResult | null>(null)

// ============ Helper: atom types / counts strings ============

const atomTypesStr = computed({
  get: () => vaspPoscar.atom_types.join(' '),
  set: (v: string) => { vaspPoscar.atom_types = v.trim().split(/\s+/).filter(Boolean) },
})

const atomCountsStr = computed({
  get: () => vaspPoscar.atom_counts.join(' '),
  set: (v: string) => { vaspPoscar.atom_counts = v.trim().split(/\s+/).map(Number) },
})

const magmomStr = computed({
  get: () => vaspIncar.magmom.join(' '),
  set: (v: string) => {
    const parts = v.trim().split(/\s+/)
    const result: number[] = []
    for (const p of parts) {
      if (p.includes('*')) {
        const [n, val] = p.split('*')
        const count = parseInt(n)
        const value = parseFloat(val)
        for (let i = 0; i < count; i++) result.push(value)
      } else {
        result.push(parseFloat(p))
      }
    }
    vaspIncar.magmom = result
  },
})

function syncAtomTypes() { /* handled by computed setter */ }
function syncAtomCounts() { /* handled by computed setter */ }
function syncMagmom() { /* handled by computed setter */ }

// ============ Position Management ============

function addPosition() {
  const defaultType = vaspPoscar.atom_types[0] || 'X'
  vaspPoscar.positions.push({ type: defaultType, x: 0, y: 0, z: 0, dx: true, dy: true, dz: true })
  if (vaspPoscar.selective_dynamics.length > 0) {
    vaspPoscar.selective_dynamics.push([true, true, true])
  }
}

function removePosition(idx: number) {
  vaspPoscar.positions.splice(idx, 1)
  if (vaspPoscar.selective_dynamics.length > 0) {
    vaspPoscar.selective_dynamics.splice(idx, 1)
  }
}

// ============ LDA+U Management ============

function addLdauParam() {
  vaspIncar.ldau_params.push({ element: '', U: 0, J: 0 })
}

function removeLdauParam(idx: number) {
  vaspIncar.ldau_params.splice(idx, 1)
}

// ============ K-path Management ============

function addKpath() {
  vaspKpoints.kpath.push({ label1: '', label2: '', npoints: 10 })
}

function removeKpath(idx: number) {
  vaspKpoints.kpath.splice(idx, 1)
}

// ============ QE Species Management ============

function addSpecies() {
  qeInput.atomic_species.push({ name: '', mass: 0, pseudo: '' })
}

function removeSpecies(idx: number) {
  qeInput.atomic_species.splice(idx, 1)
}

// ============ QE Atomic Position Management ============

function addAtomicPosition() {
  qeInput.atomic_positions.positions.push({ atom: '', x: 0, y: 0, z: 0 })
}

function removeAtomicPosition(idx: number) {
  qeInput.atomic_positions.positions.splice(idx, 1)
}

// ============ VASP File Previews ============

const vaspPoscarPreview = computed(() => {
  const p = vaspPoscar
  let text = ''
  text += p.comment + '\n'
  text += p.scaling_factor.toFixed(1) + '\n'
  text += `  ${p.lattice.a.toFixed(6)}  ${p.lattice.b.toFixed(6)}  ${p.lattice.c.toFixed(6)}\n`
  text += `  ${p.lattice.alpha.toFixed(6)}  ${p.lattice.beta.toFixed(6)}  ${p.lattice.gamma.toFixed(6)}\n`
  text += p.atom_types.join('  ') + '\n'
  text += p.atom_counts.join('  ') + '\n'
  if (p.selective_dynamics.length > 0) {
    text += 'Selective dynamics\n'
  }
  text += (p.coordinate_type === 'cartesian' ? 'Cartesian' : 'Direct') + '\n'
  for (let i = 0; i < p.positions.length; i++) {
    const pos = p.positions[i]
    let line = `  ${pos.x.toFixed(8)}  ${pos.y.toFixed(8)}  ${pos.z.toFixed(8)}`
    if (p.selective_dynamics.length > 0 && p.selective_dynamics[i]) {
      const sd = p.selective_dynamics[i]
      line += `   ${sd[0] ? 'T' : 'F'}  ${sd[1] ? 'T' : 'F'}  ${sd[2] ? 'T' : 'F'}`
    }
    text += line + '\n'
  }
  return text
})

const vaspIncarPreview = computed(() => {
  const inc = vaspIncar
  let text = ''
  text += `SYSTEM = "${inc.system}"\n`
  text += `ENCUT = ${inc.encut}\n`
  text += `PREC = ${inc.prec}\n`
  text += `GGA = ${inc.gga || '! (LDA)'}\n`
  text += `ISMEAR = ${inc.ismear}\n`
  text += `SIGMA = ${inc.sigma}\n`
  text += `ISIF = ${inc.isif}\n`
  text += `IBRION = ${inc.ibrion}\n`
  text += `NSW = ${inc.nsw}\n`
  text += `EDIFF = ${inc.ediff}\n`
  text += `EDIFFG = ${inc.ediffg}\n`
  text += `NELM = ${inc.nelm}\n`
  text += `NBANDS = ${inc.nbands}\n`
  text += `ICHARG = ${inc.icharg}\n`
  text += `LCHARG = .${inc.lcharg ? 'TRUE' : 'FALSE'}.\n`
  text += `LWAVE = .${inc.lwave ? 'TRUE' : 'FALSE'}.\n`
  if (inc.magmom.length > 0) {
    text += `MAGMOM = ${inc.magmom.join(' ')}\n`
  }
  if (inc.ldau_luj && inc.ldau_params.length > 0) {
    text += `LDAU = .TRUE.\n`
    text += `LDAUTYPE = 2\n`
    for (const lp of inc.ldau_params) {
      text += `LDAUL(${lp.element}) = 2\n`
      text += `LDAUU(${lp.element}) = ${lp.U}\n`
      text += `LDAUJ(${lp.element}) = ${lp.J}\n`
    }
  }
  return text
})

const vaspKpointsPreview = computed(() => {
  const kp = vaspKpoints
  let text = ''
  text += 'K-Points\n'
  text += '0\n'
  if (kp.style === 'line') {
    text += 'Line-mode\n'
    text += 'Reciprocal\n'
    for (const seg of kp.kpath) {
      text += `${seg.label1.padEnd(4)} 0.0 0.0 0.0  ${seg.label2.padEnd(4)} ! ${seg.npoints}\n`
    }
  } else {
    const styleLabel = kp.style === 'gamma' ? 'Gamma' : kp.style === 'monkhorst' ? 'Monkhorst-Pack' : 'Automatic'
    text += styleLabel + '\n'
    text += `${kp.grid.n1}  ${kp.grid.n2}  ${kp.grid.n3}\n`
    text += `${kp.shift.s1}  ${kp.shift.s2}  ${kp.shift.s3}\n`
  }
  return text
})

// ============ QE Input Preview ============

const qeInputPreview = computed(() => {
  const qe = qeInput
  let text = '&CONTROL\n'
  text += `  calculation = '${qe.control.calculation}',\n`
  text += `  prefix = '${qe.control.prefix}',\n`
  text += `  outdir = '${qe.control.outdir}',\n`
  text += `  pseudo_dir = '${qe.control.pseudo_dir}',\n`
  text += `  etot_conv_thr = ${qe.control.etot_conv},\n`
  text += `  forc_conv_thr = ${qe.control.forc_conv},\n`
  text += '/\n\n'
  text += '&SYSTEM\n'
  text += `  ibrav = ${qe.system.ibrav},\n`
  text += `  celldm(1) = ${qe.system.celldm[0]},\n`
  if (qe.system.celldm[1] !== 0) text += `  celldm(2) = ${qe.system.celldm[1]},\n`
  if (qe.system.celldm[2] !== 0) text += `  celldm(3) = ${qe.system.celldm[2]},\n`
  if (qe.system.celldm[3] !== 0) text += `  celldm(4) = ${qe.system.celldm[3]},\n`
  if (qe.system.celldm[4] !== 0) text += `  celldm(5) = ${qe.system.celldm[4]},\n`
  if (qe.system.celldm[5] !== 0) text += `  celldm(6) = ${qe.system.celldm[5]},\n`
  text += `  nat = ${qe.system.nat},\n`
  text += `  ntyp = ${qe.system.nbp},\n`
  text += `  ecutwfc = ${qe.system.ecutwfc},\n`
  text += `  ecutrho = ${qe.system.ecutrho},\n`
  text += `  occupations = '${qe.system.occupations}',\n`
  text += `  smearing = '${qe.system.smearing}',\n`
  text += `  degauss = ${qe.system.degauss},\n`
  text += '/\n\n'
  text += '&ELECTRONS\n'
  text += `  conv_thr = ${qe.electrons.conv_thr},\n`
  text += `  mixing_beta = ${qe.electrons.mixing_beta},\n`
  text += `  electron_maxstep = ${qe.electrons.electron_maxstep},\n`
  text += '/\n\n'
  text += 'ATOMIC_SPECIES\n'
  for (const sp of qe.atomic_species) {
    text += `  ${sp.name.padEnd(6)} ${sp.mass.toFixed(3).padStart(8)} ${sp.pseudo}\n`
  }
  text += '\n'
  text += 'ATOMIC_POSITIONS { ' + qe.atomic_positions.type + ' }\n'
  for (const ap of qe.atomic_positions.positions) {
    text += `  ${ap.atom.padEnd(6)} ${ap.x.toFixed(6).padStart(12)} ${ap.y.toFixed(6).padStart(12)} ${ap.z.toFixed(6).padStart(12)}\n`
  }
  text += '\n'
  text += 'K_POINTS { ' + qe.k_points.type + ' }\n'
  text += `  ${qe.k_points.grid.n1}  ${qe.k_points.grid.n2}  ${qe.k_points.grid.n3}  ${qe.k_points.offset.s1}  ${qe.k_points.offset.s2}  ${qe.k_points.offset.s3}\n`
  return text
})

// ============ Actions ============

function handleGenerate() {
  if (activeCode.value === 'vasp') {
    generatedResult.value = {
      success: true,
      code: 'vasp',
      files: {
        'POSCAR': vaspPoscarPreview.value,
        'INCAR': vaspIncarPreview.value,
        'KPOINTS': vaspKpointsPreview.value,
      },
      file_paths: {
        'POSCAR': './POSCAR',
        'INCAR': './INCAR',
        'KPOINTS': './KPOINTS',
      },
    }
  } else {
    generatedResult.value = {
      success: true,
      code: 'quantum_espresso',
      files: {
        'pw.in': qeInputPreview.value,
      },
      file_paths: {
        'pw.in': './pw.in',
      },
    }
  }
}

function handleExport() {
  if (!generatedResult.value) return
  const files = generatedResult.value.files
  for (const [name, content] of Object.entries(files)) {
    const blob = new Blob([content], { type: 'text/plain' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = name
    a.click()
    URL.revokeObjectURL(url)
  }
}

function handleImportPoscar() {
  const input = document.createElement('input')
  input.type = 'file'
  input.accept = '.poscar,.contcar,.vasp'
  input.onchange = (e: Event) => {
    const file = (e.target as HTMLInputElement).files?.[0]
    if (!file) return
    const reader = new FileReader()
    reader.onload = () => {
      const content = reader.result as string
      parsePoscarContent(content)
    }
    reader.readAsText(file)
  }
  input.click()
}

function parsePoscarContent(content: string) {
  const lines = content.split('\n').filter(l => l.trim() !== '')
  if (lines.length < 8) return

  let idx = 0
  vaspPoscar.comment = lines[idx++]
  vaspPoscar.scaling_factor = parseFloat(lines[idx++])

  const latA = lines[idx++].trim().split(/\s+/).map(Number)
  const latB = lines[idx++].trim().split(/\s+/).map(Number)
  vaspPoscar.lattice.a = latA[0]
  vaspPoscar.lattice.b = latA[1]
  vaspPoscar.lattice.c = latA[2]
  vaspPoscar.lattice.alpha = latB[0]
  vaspPoscar.lattice.beta = latB[1]
  vaspPoscar.lattice.gamma = latB[2]

  vaspPoscar.atom_types = lines[idx++].trim().split(/\s+/)
  vaspPoscar.atom_counts = lines[idx++].trim().split(/\s+/).map(Number)

  let selDyn = false
  if (lines[idx].trim().toLowerCase().startsWith('sel')) {
    selDyn = true
    vaspPoscar.selective_dynamics = []
    idx++
  } else {
    vaspPoscar.selective_dynamics = []
  }

  const coordLine = lines[idx++].trim().toLowerCase()
  vaspPoscar.coordinate_type = coordLine.startsWith('c') ? 'cartesian' : 'direct'

  const totalAtoms = vaspPoscar.atom_counts.reduce((a, b) => a + b, 0)
  vaspPoscar.positions = []
  for (let i = 0; i < totalAtoms && idx < lines.length; i++) {
    const parts = lines[idx++].trim().split(/\s+/)
    if (parts.length < 3) continue
    const pos = {
      type: '',
      x: parseFloat(parts[0]),
      y: parseFloat(parts[1]),
      z: parseFloat(parts[2]),
      dx: true,
      dy: true,
      dz: true,
    }
    if (selDyn && parts.length >= 6) {
      pos.dx = parts[3].toUpperCase() === 'T'
      pos.dy = parts[4].toUpperCase() === 'T'
      pos.dz = parts[5].toUpperCase() === 'T'
      vaspPoscar.selective_dynamics.push([pos.dx, pos.dy, pos.dz])
    }
    vaspPoscar.positions.push(pos)
  }

  // Assign types based on atom_types and atom_counts
  let typeIdx = 0
  let countInType = 0
  for (const pos of vaspPoscar.positions) {
    pos.type = vaspPoscar.atom_types[typeIdx] || 'X'
    countInType++
    if (countInType >= vaspPoscar.atom_counts[typeIdx]) {
      typeIdx++
      countInType = 0
    }
  }
}

// ============ Template Application ============

function applyTemplate(id: string) {
  switch (id) {
    case 'vasp-scf':
      activeCode.value = 'vasp'
      Object.assign(vaspIncar, {
        system: 'SCF Calculation',
        encut: 400, ismear: 1, sigma: 0.2, isif: 2, ibrion: -1, nsw: 0,
        ediff: 1e-6, ediffg: -0.01, nelm: 60, lcharg: true, lwave: false,
        icharg: 0, prec: 'Accurate', kpoints_auto: true, nbands: 20,
        gga: 'PE', ldau_luj: false, ldau_params: [], magmom: [5.0, 5.0, 5.0, 5.0],
      })
      Object.assign(vaspKpoints, {
        style: 'monkhorst' as const,
        grid: { n1: 11, n2: 11, n3: 11 },
        shift: { s1: 0, s2: 0, s3: 0 },
        kpath: vaspKpoints.kpath,
      })
      break

    case 'vasp-relax':
      activeCode.value = 'vasp'
      Object.assign(vaspIncar, {
        system: 'Geometry Optimization',
        encut: 450, ismear: 1, sigma: 0.2, isif: 3, ibrion: 2, nsw: 100,
        ediff: 1e-6, ediffg: -0.01, nelm: 60, lcharg: true, lwave: false,
        icharg: 0, prec: 'Accurate', kpoints_auto: true, nbands: 20,
        gga: 'PE', ldau_luj: false, ldau_params: [], magmom: [5.0, 5.0, 5.0, 5.0],
      })
      Object.assign(vaspKpoints, {
        style: 'monkhorst' as const,
        grid: { n1: 9, n2: 9, n3: 9 },
        shift: { s1: 0, s2: 0, s3: 0 },
        kpath: vaspKpoints.kpath,
      })
      break

    case 'vasp-dos':
      activeCode.value = 'vasp'
      Object.assign(vaspIncar, {
        system: 'DOS Calculation',
        encut: 500, ismear: -5, sigma: 0.05, isif: 2, ibrion: -1, nsw: 0,
        ediff: 1e-6, ediffg: -0.01, nelm: 60, lcharg: true, lwave: true,
        icharg: 1, prec: 'Accurate', kpoints_auto: true, nbands: 30,
        gga: 'PE', ldau_luj: false, ldau_params: [], magmom: [5.0, 5.0, 5.0, 5.0],
      })
      Object.assign(vaspKpoints, {
        style: 'monkhorst' as const,
        grid: { n1: 15, n2: 15, n3: 15 },
        shift: { s1: 0, s2: 0, s3: 0 },
        kpath: vaspKpoints.kpath,
      })
      break

    case 'vasp-band':
      activeCode.value = 'vasp'
      Object.assign(vaspIncar, {
        system: 'Band Structure',
        encut: 500, ismear: 0, sigma: 0.05, isif: 2, ibrion: -1, nsw: 0,
        ediff: 1e-6, ediffg: -0.01, nelm: 60, lcharg: true, lwave: true,
        icharg: 11, prec: 'Accurate', kpoints_auto: false, nbands: 30,
        gga: 'PE', ldau_luj: false, ldau_params: [], magmom: [5.0, 5.0, 5.0, 5.0],
      })
      Object.assign(vaspKpoints, {
        style: 'line' as const,
        grid: { n1: 11, n2: 11, n3: 11 },
        shift: { s1: 0, s2: 0, s3: 0 },
        kpath: [
          { label1: 'G', label2: 'X', npoints: 20 },
          { label2: 'W', label1: 'X', npoints: 10 },
          { label1: 'W', label2: 'K', npoints: 10 },
          { label2: 'G', label1: 'K', npoints: 20 },
          { label1: 'G', label2: 'L', npoints: 20 },
          { label2: 'U', label1: 'L', npoints: 10 },
          { label1: 'U', label2: 'W', npoints: 10 },
          { label2: 'K', label1: 'U', npoints: 10 },
        ],
      })
      break

    case 'qe-scf':
      activeCode.value = 'quantum_espresso'
      Object.assign(qeInput.control, {
        calculation: 'scf', prefix: 'si', outdir: './out/',
        pseudo_dir: './pseudo/', etot_conv: 1e-6, forc_conv: 1e-4,
      })
      Object.assign(qeInput.system, {
        ibrav: 2, celldm: [10.26, 0, 0, 0, 0, 0], nat: 2, nbp: 1,
        ecutwfc: 40, ecutrho: 320, occupations: 'smearing', smearing: 'gaussian', degauss: 0.01,
      })
      Object.assign(qeInput.electrons, {
        conv_thr: 1e-8, mixing_beta: 0.7, electron_maxstep: 100,
      })
      qeInput.atomic_species = [{ name: 'Si', mass: 28.086, pseudo: 'Si.pbe-n-rrkjus_psl.1.0.0.UPF' }]
      qeInput.atomic_positions = {
        type: 'crystal',
        positions: [
          { atom: 'Si', x: 0.0, y: 0.0, z: 0.0 },
          { atom: 'Si', x: 0.25, y: 0.25, z: 0.25 },
        ],
      }
      Object.assign(qeInput.k_points, {
        type: 'automatic',
        grid: { n1: 8, n2: 8, n3: 8 },
        offset: { s1: 0, s2: 0, s3: 0 },
      })
      break

    case 'qe-relax':
      activeCode.value = 'quantum_espresso'
      Object.assign(qeInput.control, {
        calculation: 'vc-relax', prefix: 'si', outdir: './out/',
        pseudo_dir: './pseudo/', etot_conv: 1e-6, forc_conv: 1e-4,
      })
      Object.assign(qeInput.system, {
        ibrav: 2, celldm: [10.26, 0, 0, 0, 0, 0], nat: 2, nbp: 1,
        ecutwfc: 40, ecutrho: 320, occupations: 'smearing', smearing: 'gaussian', degauss: 0.01,
      })
      Object.assign(qeInput.electrons, {
        conv_thr: 1e-8, mixing_beta: 0.7, electron_maxstep: 100,
      })
      qeInput.atomic_species = [{ name: 'Si', mass: 28.086, pseudo: 'Si.pbe-n-rrkjus_psl.1.0.0.UPF' }]
      qeInput.atomic_positions = {
        type: 'crystal',
        positions: [
          { atom: 'Si', x: 0.0, y: 0.0, z: 0.0 },
          { atom: 'Si', x: 0.25, y: 0.25, z: 0.25 },
        ],
      }
      Object.assign(qeInput.k_points, {
        type: 'automatic',
        grid: { n1: 6, n2: 6, n3: 6 },
        offset: { s1: 0, s2: 0, s3: 0 },
      })
      break
  }
}
</script>

<style scoped>
.label {
  display: block;
  font-size: 11px;
  font-weight: 500;
  color: var(--text-secondary);
  margin-bottom: 2px;
}

.panel-section {
  margin-bottom: 4px;
}

.result-card {
  background: var(--bg-elevated);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  padding: 12px;
}

.result-label {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}
</style>
