<script setup lang="ts">
import {computed, ref, watch} from 'vue'
import {useRouter} from 'vue-router'
import PaginationBar from '@/components/PaginationBar.vue'
import TrafficIO from '@/components/TrafficIO.vue'
import {useDashboardStore} from '@/stores/dashboard'
import {useLocale} from '@/composables/useLocale'
import {formatProxyEndpoint, isHttpProxyType} from '@/utils/format'

type ProtocolFilter = 'all' | 'tcp' | 'udp' | 'http' | 'https'

const PROTOCOLS: ProtocolFilter[] = ['all', 'tcp', 'udp', 'http', 'https']

const store = useDashboardStore()
const {t} = useLocale()
const router = useRouter()

const page = ref(1)
const pageSize = ref(10)
const protocol = ref<ProtocolFilter>('all')

const filtered = computed(() => {
  const list = store.proxies
  if (protocol.value === 'all') return list
  return list.filter((p) => (p.type || '').toLowerCase() === protocol.value)
})

const total = computed(() => filtered.value.length)

const pageItems = computed(() => {
  const start = (page.value - 1) * pageSize.value
  return filtered.value.slice(start, start + pageSize.value)
})

const typeCounts = computed(() => {
  const counts: Record<ProtocolFilter, number> = {
    all: store.proxies.length,
    tcp: 0,
    udp: 0,
    http: 0,
    https: 0,
  }
  for (const p of store.proxies) {
    const ty = (p.type || '').toLowerCase() as ProtocolFilter
    if (ty in counts && ty !== 'all') counts[ty] += 1
  }
  return counts
})

watch([total, pageSize, protocol], () => {
  const maxPage = Math.max(1, Math.ceil(total.value / Math.max(pageSize.value, 1)))
  if (page.value > maxPage) page.value = maxPage
})

watch(protocol, () => {
  page.value = 1
})

function protocolLabel(key: ProtocolFilter) {
  if (key === 'all') return t('proxies.filterAll')
  return key.toUpperCase()
}

function statusLabel(raw?: string) {
  if (!raw || raw === 'online') return t('status.online')
  return raw
}

function isOnline(raw?: string) {
  return !raw || raw === 'online'
}

function openDetail(name: string) {
  router.push({name: 'proxy-detail', params: {name}})
}

function onKeyOpen(evt: KeyboardEvent, name: string) {
  if (evt.key === 'Enter' || evt.key === ' ') {
    evt.preventDefault()
    openDetail(name)
  }
}
</script>

<template>
  <section class="proxy-list">
    <div class="list-toolbar" role="group" :aria-label="t('proxies.filter')">
      <button
          v-for="key in PROTOCOLS"
          :key="key"
          type="button"
          class="filter-chip"
          :class="{ active: protocol === key }"
          @click="protocol = key"
      >
        <span>{{ protocolLabel(key) }}</span>
        <em>{{ typeCounts[key] }}</em>
      </button>
    </div>

    <div v-if="!store.proxies.length" class="empty-card">
      {{ t('proxies.empty') }}
    </div>
    <div v-else-if="!filtered.length" class="empty-card">
      {{ t('proxies.filterEmpty') }}
    </div>

    <article
        v-for="p in pageItems"
        :key="`${p.name}:${p.clientId}`"
        class="proxy-card"
        role="button"
        tabindex="0"
        @click="openDetail(p.name)"
        @keydown="onKeyOpen($event, p.name)"
    >
      <div class="proxy-main">
        <div class="proxy-title">
          <h3 class="proxy-name">{{ p.name }}</h3>
          <span class="proxy-type">{{ (p.type || '—').toUpperCase() }}</span>
        </div>
        <div class="proxy-meta">
          <span class="meta-endpoint">
            <em>{{ isHttpProxyType(p.type) ? t('proxies.domain') : t('proxies.port') }}</em>
            <code>{{ formatProxyEndpoint(p.type, p.remoteAddr) }}</code>
          </span>
          <span class="meta-arrow" aria-hidden="true">→</span>
          <span class="meta-endpoint">
            <em>{{ t('proxies.localAddr') }}</em>
            <code>{{ p.localAddr || '—' }}</code>
          </span>
          <span>
            <em>{{ t('proxies.curConns') }}</em>
            {{ p.curConns ?? 0 }}
          </span>
          <span class="meta-client">
            <em>{{ t('proxies.client') }}</em>
            {{ p.clientId || '—' }}
          </span>
        </div>
      </div>

      <div class="proxy-side">
        <TrafficIO :traffic-in="p.todayTrafficIn" :traffic-out="p.todayTrafficOut"/>
        <span class="status-badge" :class="{ online: isOnline(p.status) }">
          {{ statusLabel(p.status) }}
        </span>
      </div>
    </article>

    <PaginationBar
        v-model:page="page"
        v-model:page-size="pageSize"
        :total="total"
    />
  </section>
</template>

<style scoped>
.proxy-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.list-toolbar {
  display: flex;
  flex-wrap: wrap;
  gap: 0.45rem;
}

.filter-chip {
  display: inline-flex;
  align-items: center;
  gap: 0.4rem;
  border: 1px solid var(--line);
  background: var(--panel);
  color: var(--muted);
  font: inherit;
  font-size: 0.78rem;
  font-weight: 650;
  padding: 0.38rem 0.75rem;
  border-radius: 999px;
  cursor: pointer;
  box-shadow: var(--shadow);
  transition: border-color 0.15s ease,
  color 0.15s ease,
  background 0.15s ease;
}

.filter-chip em {
  font-style: normal;
  font-variant-numeric: tabular-nums;
  font-size: 0.72rem;
  font-weight: 600;
  min-width: 1.1rem;
  padding: 0.05rem 0.4rem;
  border-radius: 999px;
  text-align: center;
  color: var(--muted);
  background: color-mix(in srgb, var(--muted) 12%, transparent);
}

.filter-chip:hover:not(.active) {
  color: var(--text);
  border-color: var(--line-strong);
}

.filter-chip.active {
  color: var(--accent-text);
  border-color: color-mix(in srgb, var(--accent) 35%, transparent);
  background: var(--accent-soft);
}

.filter-chip.active em {
  color: var(--accent-text);
  background: color-mix(in srgb, var(--accent) 18%, transparent);
}

.empty-card {
  padding: 2.5rem 1rem;
  text-align: center;
  color: var(--muted);
  background: var(--panel);
  border: 1px solid var(--line);
  border-radius: 14px;
  box-shadow: var(--shadow);
}

.proxy-card {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1.25rem;
  padding: 1rem 1.2rem;
  background: var(--panel);
  border: 1px solid var(--line);
  border-radius: 14px;
  box-shadow: var(--shadow);
  cursor: pointer;
  transition: border-color 0.18s ease,
  box-shadow 0.18s ease,
  transform 0.18s ease;
}

.proxy-card:hover {
  border-color: var(--line-strong);
  box-shadow: 0 6px 18px color-mix(in srgb, var(--text) 6%, transparent);
}

.proxy-card:focus-visible {
  outline: 2px solid color-mix(in srgb, var(--accent) 55%, transparent);
  outline-offset: 2px;
}

.proxy-main {
  min-width: 0;
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 0.45rem;
}

.proxy-title {
  display: flex;
  align-items: baseline;
  flex-wrap: wrap;
  gap: 0.55rem;
}

.proxy-name {
  margin: 0;
  font-size: 0.95rem;
  font-weight: 700;
  color: var(--text);
  letter-spacing: -0.01em;
}

.proxy-type {
  font-size: 0.72rem;
  font-weight: 700;
  letter-spacing: 0.06em;
  color: var(--muted);
}

.proxy-meta {
  display: flex;
  flex-wrap: wrap;
  align-items: baseline;
  gap: 0.35rem 1.15rem;
  font-size: 0.8rem;
  color: var(--text);
}

.proxy-meta em {
  font-style: normal;
  color: var(--muted);
  margin-right: 0.3rem;
  font-weight: 500;
}

.meta-endpoint {
  display: inline-flex;
  align-items: baseline;
  min-width: 0;
}

.meta-endpoint code {
  font-family: 'IBM Plex Mono', ui-monospace, monospace;
  font-size: 0.78rem;
  font-weight: 600;
  color: var(--text);
  word-break: break-all;
}

.meta-arrow {
  color: var(--muted);
  font-size: 0.85rem;
  font-weight: 600;
  margin: 0 -0.55rem;
  user-select: none;
}

.meta-client {
  font-family: 'IBM Plex Mono', ui-monospace, monospace;
  font-size: 0.76rem;
  word-break: break-all;
}

@media (max-width: 560px) {
  .meta-arrow {
    display: none;
  }
}

.proxy-side {
  display: flex;
  align-items: center;
  gap: 1.1rem;
  flex-shrink: 0;
}

.status-badge {
  display: inline-flex;
  align-items: center;
  min-width: 3.6rem;
  justify-content: center;
  padding: 0.28rem 0.75rem;
  border-radius: 999px;
  font-size: 0.78rem;
  font-weight: 650;
  color: var(--muted);
  background: color-mix(in srgb, var(--muted) 12%, transparent);
  border: 1px solid color-mix(in srgb, var(--muted) 18%, transparent);
}

.status-badge.online {
  color: var(--status-ok);
  background: var(--status-ok-soft);
  border-color: color-mix(in srgb, var(--status-ok) 22%, transparent);
}

@media (max-width: 720px) {
  .proxy-card {
    flex-direction: column;
    align-items: stretch;
    gap: 0.85rem;
  }

  .proxy-side {
    justify-content: space-between;
  }
}
</style>
