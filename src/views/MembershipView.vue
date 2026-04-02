<template>
  <div class="membership-page">
    <div class="membership-header">
      <h1 class="page-title">会员方案</h1>
      <p class="page-subtitle">选择适合您的方案，解锁更多功能</p>

      <!-- Billing Toggle -->
      <div class="billing-toggle">
        <span :class="{ active: !isAnnual }">月付</span>
        <button class="toggle-switch" @click="isAnnual = !isAnnual">
          <span class="toggle-knob" :class="{ moved: isAnnual }"></span>
        </button>
        <span :class="{ active: isAnnual }">
          年付
          <span class="save-badge">省 33%</span>
        </span>
      </div>
    </div>

    <!-- Plan Cards -->
    <div class="plan-cards">
      <!-- Free -->
      <div
        class="plan-card"
        :class="{ current: currentTier === 'free' }"
      >
        <div class="plan-header">
          <h3 class="plan-name">Free</h3>
          <p class="plan-desc">个人学习与研究</p>
          <div class="plan-price">
            <span class="price-amount">¥0</span>
            <span class="price-period">/永久</span>
          </div>
        </div>
        <ul class="plan-features">
          <li class="feature included">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
            本地项目不限
          </li>
          <li class="feature included">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
            结构仿真 20 次/月
          </li>
          <li class="feature included">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
            CFD 仿真 5 次/月
          </li>
          <li class="feature excluded">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
            云端存储
          </li>
          <li class="feature excluded">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
            高级仿真
          </li>
          <li class="feature excluded">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
            Python API
          </li>
        </ul>
        <button
          v-if="currentTier === 'free'"
          class="btn btn-secondary btn-block current-plan"
          disabled
        >当前方案</button>
        <button
          v-else
          class="btn btn-secondary btn-block"
          @click="handleDowngrade('free')"
        >降级到 Free</button>
      </div>

      <!-- Pro -->
      <div
        class="plan-card featured"
        :class="{ current: currentTier === 'pro' }"
      >
        <div class="plan-badge">推荐</div>
        <div class="plan-header">
          <h3 class="plan-name">Pro</h3>
          <p class="plan-desc">专业工程师与科研人员</p>
          <div class="plan-price">
            <span class="price-amount">¥{{ isAnnual ? '799' : '99' }}</span>
            <span class="price-period">/{{ isAnnual ? '年' : '月' }}</span>
          </div>
          <div v-if="isAnnual" class="price-note">相当于 ¥66.6/月</div>
        </div>
        <ul class="plan-features">
          <li class="feature included">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
            本地项目不限
          </li>
          <li class="feature included">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
            结构仿真 200 次/月
          </li>
          <li class="feature included">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
            CFD 仿真 50 次/月
          </li>
          <li class="feature included">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
            云端存储 10 GB
          </li>
          <li class="feature included">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
            高级仿真（热耦合/疲劳）
          </li>
          <li class="feature included">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
            Python API
          </li>
          <li class="feature included">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
            协同成员 3 人
          </li>
          <li class="feature included">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
            参数化扫描
          </li>
          <li class="feature excluded">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
            拓扑优化
          </li>
          <li class="feature excluded">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
            REST API
          </li>
        </ul>
        <button
          v-if="currentTier === 'pro'"
          class="btn btn-primary btn-block current-plan"
          disabled
        >当前方案</button>
        <button
          v-else
          class="btn btn-primary btn-block"
          @click="handleUpgrade('pro')"
        >升级到 Pro</button>
      </div>

      <!-- Enterprise -->
      <div
        class="plan-card"
        :class="{ current: currentTier === 'enterprise' }"
      >
        <div class="plan-header">
          <h3 class="plan-name">Enterprise</h3>
          <p class="plan-desc">团队与企业级应用</p>
          <div class="plan-price">
            <span class="price-amount">¥{{ isAnnual ? '2,499' : '299' }}</span>
            <span class="price-period">/{{ isAnnual ? '年' : '月' }}</span>
          </div>
          <div v-if="isAnnual" class="price-note">相当于 ¥208.3/月</div>
        </div>
        <ul class="plan-features">
          <li class="feature included">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
            本地项目不限
          </li>
          <li class="feature included">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
            仿真次数不限
          </li>
          <li class="feature included">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
            云端存储 100 GB
          </li>
          <li class="feature included">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
            高级仿真（热耦合/疲劳）
          </li>
          <li class="feature included">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
            拓扑优化
          </li>
          <li class="feature included">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
            Python API + REST API
          </li>
          <li class="feature included">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
            协同成员不限
          </li>
          <li class="feature included">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
            1v1 专属客服
          </li>
          <li class="feature included">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
            SSO 单点登录
          </li>
          <li class="feature included">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
            私有化部署
          </li>
        </ul>
        <button
          v-if="currentTier === 'enterprise'"
          class="btn btn-secondary btn-block current-plan"
          disabled
        >当前方案</button>
        <button
          v-else
          class="btn btn-secondary btn-block"
          @click="handleUpgrade('enterprise')"
        >升级到 Enterprise</button>
      </div>
    </div>

    <!-- Feature Comparison Table -->
    <div class="comparison-section">
      <h2 class="section-title">功能对比</h2>
      <div class="comparison-table-wrapper">
        <table class="comparison-table">
          <thead>
            <tr>
              <th class="feature-col">功能</th>
              <th>Free</th>
              <th class="highlight-col">Pro</th>
              <th>Enterprise</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="row in comparisonData" :key="row.feature">
              <td class="feature-col">{{ row.feature }}</td>
              <td>
                <span v-if="row.free === true" class="check">&#10003;</span>
                <span v-else-if="row.free === false" class="cross">&#10007;</span>
                <span v-else>{{ row.free }}</span>
              </td>
              <td class="highlight-col">
                <span v-if="row.pro === true" class="check">&#10003;</span>
                <span v-else-if="row.pro === false" class="cross">&#10007;</span>
                <span v-else>{{ row.pro }}</span>
              </td>
              <td>
                <span v-if="row.enterprise === true" class="check">&#10003;</span>
                <span v-else-if="row.enterprise === false" class="cross">&#10007;</span>
                <span v-else>{{ row.enterprise }}</span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useAuthStore } from '@/stores/authStore'

const authStore = useAuthStore()
const isAnnual = ref(true)

const currentTier = computed(() => authStore.membership?.tier || 'free')

const comparisonData = [
  { feature: '本地项目数', free: '不限', pro: '不限', enterprise: '不限' },
  { feature: '云端存储', free: false, pro: '10 GB', enterprise: '100 GB' },
  { feature: '协同成员数', free: false, pro: '3 人', enterprise: '不限' },
  { feature: 'CFD 仿真次数/月', free: '5 次', pro: '50 次', enterprise: '不限' },
  { feature: '结构仿真次数/月', free: '20 次', pro: '200 次', enterprise: '不限' },
  { feature: '高级仿真（热耦合/疲劳）', free: false, pro: true, enterprise: true },
  { feature: '参数化扫描', free: false, pro: true, enterprise: true },
  { feature: '拓扑优化', free: false, pro: false, enterprise: true },
  { feature: 'Python API', free: false, pro: true, enterprise: true },
  { feature: 'REST API', free: false, pro: false, enterprise: true },
  { feature: '优先客服支持', free: false, pro: '邮件', enterprise: '1v1' },
  { feature: '发票', free: false, pro: true, enterprise: true },
  { feature: 'SSO 单点登录', free: false, pro: false, enterprise: true },
  { feature: '私有化部署', free: false, pro: false, enterprise: true },
]

function handleUpgrade(tier: string) {
  // TODO: redirect to payment
  console.log('Upgrade to:', tier)
}

function handleDowngrade(tier: string) {
  // TODO: confirm and downgrade
  console.log('Downgrade to:', tier)
}
</script>

<style scoped>
.membership-page {
  max-width: 1100px;
  margin: 0 auto;
  padding: 32px 24px;
}

.membership-header {
  text-align: center;
  margin-bottom: 40px;
}

.page-title {
  font-size: 28px;
  font-weight: 700;
  color: var(--text-primary, #1f2937);
  margin-bottom: 8px;
}

.page-subtitle {
  font-size: 15px;
  color: var(--text-muted, #9ca3af);
  margin-bottom: 24px;
}

/* Billing Toggle */
.billing-toggle {
  display: inline-flex;
  align-items: center;
  gap: 12px;
  font-size: 14px;
  color: var(--text-muted, #9ca3af);
}

.billing-toggle span.active {
  color: var(--text-primary, #1f2937);
  font-weight: 500;
}

.toggle-switch {
  position: relative;
  width: 44px;
  height: 24px;
  background: var(--bg-hover, #e5e7eb);
  border: none;
  border-radius: 12px;
  cursor: pointer;
  transition: background var(--duration-fast, 150ms);
  padding: 0;
}

.toggle-switch:hover {
  background: var(--border-hover, #d1d5db);
}

.toggle-knob {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 20px;
  height: 20px;
  background: white;
  border-radius: 50%;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
  transition: transform var(--duration-fast, 150ms) var(--ease-spring, cubic-bezier(0.34, 1.56, 0.64, 1));
}

.toggle-knob.moved {
  transform: translateX(20px);
}

.save-badge {
  display: inline-block;
  padding: 2px 6px;
  background: #dcfce7;
  color: #16a34a;
  font-size: 11px;
  font-weight: 600;
  border-radius: 4px;
  margin-left: 4px;
}

/* Plan Cards */
.plan-cards {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 20px;
  margin-bottom: 48px;
}

.plan-card {
  background: var(--bg-surface, #ffffff);
  border: 1px solid var(--border-default, #e5e7eb);
  border-radius: var(--radius-xl, 16px);
  padding: 28px 24px;
  display: flex;
  flex-direction: column;
  transition: all var(--duration-normal, 200ms) var(--ease-out, cubic-bezier(0.16, 1, 0.3, 1));
  position: relative;
}

.plan-card:hover {
  box-shadow: var(--shadow-card-hover, 0 4px 16px rgba(0, 0, 0, 0.12));
  transform: translateY(-2px);
}

.plan-card.featured {
  border-color: var(--primary, #2563EB);
  box-shadow: 0 4px 20px rgba(37, 99, 235, 0.15);
}

.plan-card.current {
  border-color: var(--accent-green, #22c55e);
}

.plan-badge {
  position: absolute;
  top: -10px;
  left: 50%;
  transform: translateX(-50%);
  padding: 4px 16px;
  background: linear-gradient(135deg, var(--primary, #2563EB), var(--primary-hover, #3B82F6));
  color: white;
  font-size: 12px;
  font-weight: 600;
  border-radius: 12px;
  white-space: nowrap;
}

.plan-header {
  text-align: center;
  margin-bottom: 24px;
  padding-bottom: 20px;
  border-bottom: 1px solid var(--border-subtle, #f3f4f6);
}

.plan-name {
  font-size: 20px;
  font-weight: 700;
  color: var(--text-primary, #1f2937);
  margin-bottom: 4px;
}

.plan-desc {
  font-size: 13px;
  color: var(--text-muted, #9ca3af);
  margin-bottom: 16px;
}

.plan-price {
  display: flex;
  align-items: baseline;
  justify-content: center;
  gap: 2px;
}

.price-amount {
  font-size: 36px;
  font-weight: 800;
  color: var(--text-primary, #1f2937);
}

.price-period {
  font-size: 14px;
  color: var(--text-muted, #9ca3af);
}

.price-note {
  font-size: 12px;
  color: var(--text-muted, #9ca3af);
  margin-top: 4px;
}

/* Features List */
.plan-features {
  list-style: none;
  padding: 0;
  margin: 0 0 24px 0;
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.feature {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
}

.feature.included {
  color: var(--text-primary, #1f2937);
}

.feature.included svg {
  color: var(--accent-green, #22c55e);
  flex-shrink: 0;
}

.feature.excluded {
  color: var(--text-muted, #9ca3af);
}

.feature.excluded svg {
  color: var(--border-default, #e5e7eb);
  flex-shrink: 0;
}

/* Buttons */
.btn-block {
  width: 100%;
  padding: 10px 16px;
  font-size: 14px;
}

.current-plan {
  opacity: 0.6;
  cursor: default;
}

/* Comparison Table */
.comparison-section {
  margin-bottom: 48px;
}

.section-title {
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary, #1f2937);
  margin-bottom: 20px;
  text-align: center;
}

.comparison-table-wrapper {
  overflow-x: auto;
  border: 1px solid var(--border-default, #e5e7eb);
  border-radius: var(--radius-lg, 12px);
}

.comparison-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.comparison-table th,
.comparison-table td {
  padding: 12px 16px;
  text-align: center;
  border-bottom: 1px solid var(--border-subtle, #f3f4f6);
}

.comparison-table th {
  background: var(--bg-elevated, #f3f4f6);
  font-weight: 600;
  color: var(--text-secondary, #4b5563);
  font-size: 13px;
}

.comparison-table th.highlight-col {
  background: rgba(37, 99, 235, 0.06);
  color: var(--primary, #2563EB);
}

.comparison-table td.highlight-col {
  background: rgba(37, 99, 235, 0.02);
}

.comparison-table td.feature-col {
  text-align: left;
  font-weight: 500;
  color: var(--text-primary, #1f2937);
}

.comparison-table th.feature-col {
  text-align: left;
}

.comparison-table tr:last-child td {
  border-bottom: none;
}

.check {
  color: var(--accent-green, #22c55e);
  font-weight: 600;
}

.cross {
  color: var(--text-muted, #9ca3af);
}

/* Responsive */
@media (max-width: 900px) {
  .plan-cards {
    grid-template-columns: 1fr;
    max-width: 400px;
    margin-left: auto;
    margin-right: auto;
  }
}

@media (max-width: 768px) {
  .membership-page {
    padding: 20px 16px;
  }

  .page-title {
    font-size: 22px;
  }
}
</style>
