<script setup lang="ts">
import {computed, ref} from 'vue'
import ConfigValue from '@/components/ConfigValue.vue'
import DescList, {type DescItem} from '@/components/DescList.vue'
import DonutChart, {type ChartSlice} from '@/components/DonutChart.vue'
import SectionCard from '@/components/SectionCard.vue'
import StatCard from '@/components/StatCard.vue'
import TrafficChart from '@/components/TrafficChart.vue'
import TrafficSummary from '@/components/TrafficSummary.vue'
import type {TrafficRange} from '@/api/client'
import {useDashboardStore} from '@/stores/dashboard'
import {useLocale} from '@/composables/useLocale'
import {isUnsetPort, isUnsetText} from '@/utils/format'

const store = useDashboardStore()
const {t} = useLocale()
const trafficRange = ref<TrafficRange>('24h')
const chartVariant = ref<'bar' | 'line'>('line')

const PROXY_COLORS: Record<string, string> = {
  http: '#3b82f6',
  https: '#93c5fd',
  tcp: '#cbd5e1',
  udp: '#2dd4bf',
  socks5: '#f97316',
  file: '#fb7185',
  stcp: '#a78bfa',
  xtcp: '#f472b6',
}

const FALLBACK_COLORS = ['#60a5fa', '#34d399', '#fbbf24', '#f87171', '#818cf8', '#94a3b8']

const cfg = computed(() => store.info?.config)
const status = computed(() => store.info?.status)

const trafficIn = computed(() => status.value?.totalTrafficIn ?? 0)
const trafficOut = computed(() => status.value?.totalTrafficOut ?? 0)

const onlineClients = computed(() => status.value?.clientCounts ?? 0)
const totalClients = computed(() =>
    Math.max(status.value?.totalClientCounts ?? 0, onlineClients.value),
)

const proxyTotal = computed(() => {
  const m = status.value?.proxyTypeCount || {}
  return Object.values(m).reduce((a, b) => a + b, 0)
})

const chartSlices = computed<ChartSlice[]>(() => {
  const m = status.value?.proxyTypeCount || {}
  const entries = Object.entries(m).sort(([a], [b]) => a.localeCompare(b))
  return entries.map(([key, value], i) => ({
    key,
    label: key,
    value,
    color: PROXY_COLORS[key.toLowerCase()] ?? FALLBACK_COLORS[i % FALLBACK_COLORS.length]!,
  }))
})

function formatHeartbeat(secs: number | undefined | null): string {
  if (secs == null) return '—'
  if (secs < 0) return t('common.disabled')
  return `${secs}s`
}

type ConfigValueType = 'text' | 'port' | 'bool' | 'raw'

interface ConfigField {
  key: string
  label: string
  type: ConfigValueType
  value: string | number | boolean | null
}

/** Compact server config — hide unset optional ports / empty host. */
const configFields = computed<ConfigField[]>(() => {
  const c = cfg.value
  if (!c) return []

  const fields: ConfigField[] = [
    {
      key: 'listen',
      label: t('monitor.bindAddr'),
      type: 'raw',
      value: `${c.bindAddr || '—'}:${c.bindPort ?? '—'}`,
    },
  ]

  if (!isUnsetPort(c.kcpBindPort)) {
    fields.push({
      key: 'kcp',
      label: t('monitor.kcpBindPort'),
      type: 'port',
      value: c.kcpBindPort,
    })
  }
  if (!isUnsetPort(c.quicBindPort)) {
    fields.push({
      key: 'quic',
      label: t('monitor.quicBindPort'),
      type: 'port',
      value: c.quicBindPort,
    })
  }
  if (!isUnsetPort(c.vhostHTTPPort)) {
    fields.push({
      key: 'http',
      label: t('monitor.vhostHTTPPort'),
      type: 'port',
      value: c.vhostHTTPPort,
    })
  }
  if (!isUnsetPort(c.vhostHTTPSPort)) {
    fields.push({
      key: 'https',
      label: t('monitor.vhostHTTPSPort'),
      type: 'port',
      value: c.vhostHTTPSPort,
    })
  }
  if (!isUnsetText(c.subDomainHost ?? '')) {
    fields.push({
      key: 'subdomain',
      label: t('monitor.subDomainHost'),
      type: 'text',
      value: c.subDomainHost,
    })
  }

  fields.push(
      {
        key: 'mux',
        label: t('monitor.tcpMux'),
        type: 'bool',
        value: c.tcpMux,
      },
      {
        key: 'tls',
        label: t('monitor.tlsForce'),
        type: 'bool',
        value: c.tlsForce,
      },
      {
        key: 'pool',
        label: t('monitor.metricMaxPool'),
        type: 'raw',
        value: c.maxPoolCount ?? 0,
      },
      {
        key: 'heartbeat',
        label: t('monitor.metricHeartbeat'),
        type: 'raw',
        value: formatHeartbeat(c.heartbeatTimeout),
      },
  )

  return fields
})

const descItems = computed<DescItem[]>(() =>
    configFields.value.map(({key, label}) => ({key, label})),
)
</script>

<template>
  <div class="monitor">
    <section class="grid stats">
      <StatCard :label="t('overview.totalClients')" icon="users" tone="blue">
        {{ totalClients }}
      </StatCard>
      <StatCard :label="t('overview.onlineClients')" icon="user" tone="green">
        {{ onlineClients }}
      </StatCard>
      <StatCard :label="t('overview.proxies')" icon="proxies" tone="violet">
        {{ proxyTotal }}
      </StatCard>
      <StatCard :label="t('overview.connections')" icon="link" tone="orange">
        {{ status?.curConns ?? 0 }}
      </StatCard>
    </section>

    <div class="monitor-panels">
      <SectionCard class="panel" :title="t('traffic.network')">
        <template #extra>
          <span class="badge">{{ t('traffic.today') }}</span>
        </template>
        <TrafficSummary :traffic-in="trafficIn" :traffic-out="trafficOut"/>
      </SectionCard>

      <SectionCard class="panel" :title="t('monitor.proxyDist')">
        <DonutChart :slices="chartSlices"/>
      </SectionCard>
    </div>

    <SectionCard class="history-panel" :title="t('traffic.historyAll')">
      <template #extra>
        <div class="chart-toolbar">
          <div class="range-toggle" role="group" :aria-label="t('traffic.chartType')">
            <button
                type="button"
                class="range-btn"
                :class="{ active: chartVariant === 'line' }"
                @click="chartVariant = 'line'"
            >
              {{ t('traffic.chartLine') }}
            </button>
            <button
                type="button"
                class="range-btn"
                :class="{ active: chartVariant === 'bar' }"
                @click="chartVariant = 'bar'"
            >
              {{ t('traffic.chartBar') }}
            </button>
          </div>
          <div class="range-toggle" role="group" :aria-label="t('traffic.range')">
            <button
                type="button"
                class="range-btn"
                :class="{ active: trafficRange === '24h' }"
                @click="trafficRange = '24h'"
            >
              {{ t('traffic.range24h') }}
            </button>
            <button
                type="button"
                class="range-btn"
                :class="{ active: trafficRange === '7d' }"
                @click="trafficRange = '7d'"
            >
              {{ t('traffic.range7d') }}
            </button>
          </div>
        </div>
      </template>
      <TrafficChart :variant="chartVariant" :range="trafficRange" :refresh-ms="5000"/>
    </SectionCard>

    <SectionCard class="config-panel" :title="t('monitor.serverConfig')">
      <DescList v-if="configFields.length" :items="descItems">
        <template v-for="field in configFields" :key="field.key" #[field.key]>
          <ConfigValue :type="field.type" :value="field.value"/>
        </template>
      </DescList>
      <p v-else class="empty">{{ t('overview.emptyConfig') }}</p>
    </SectionCard>
  </div>
</template>

<style scoped>
.monitor {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.stats {
  margin: 0;
}

.monitor-panels {
  display: grid;
  grid-template-columns: minmax(280px, 1.05fr) minmax(260px, 0.95fr);
  gap: 1rem;
  align-items: stretch;
}

.panel {
  height: 100%;
}

.badge {
  display: inline-flex;
  align-items: center;
  padding: 0.18rem 0.55rem;
  border-radius: 999px;
  font-size: 0.78rem;
  font-weight: 600;
  color: var(--accent-text);
  background: var(--accent-soft);
  border: 1px solid color-mix(in srgb, var(--accent) 28%, transparent);
}

.chart-toolbar {
  display: inline-flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  justify-content: flex-end;
}

.range-toggle {
  display: inline-flex;
  padding: 2px;
  border-radius: 999px;
  background: color-mix(in srgb, var(--muted) 10%, transparent);
  border: 1px solid var(--line);
}

.range-btn {
  border: 0;
  background: transparent;
  color: var(--muted);
  font: inherit;
  font-size: 0.78rem;
  font-weight: 600;
  padding: 0.28rem 0.7rem;
  border-radius: 999px;
  cursor: pointer;
}

.range-btn.active {
  color: var(--accent-text);
  background: var(--panel);
  box-shadow: var(--shadow);
}

.range-btn:hover:not(.active) {
  color: var(--text);
}

.empty {
  margin: 0;
  color: var(--muted);
  font-size: 0.9rem;
}

@media (max-width: 1100px) {
  .monitor-panels {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 720px) {
  .stats {
    grid-template-columns: 1fr 1fr;
  }
}

@media (max-width: 560px) {
  .stats {
    grid-template-columns: 1fr;
  }
}
</style>
