<script setup lang="ts">
import {computed, onMounted, onUnmounted, ref, watch} from 'vue'
import {useRoute, useRouter} from 'vue-router'
import AppIcon from '@/components/AppIcon.vue'
import OsBadge from '@/components/OsBadge.vue'
import PaginationBar from '@/components/PaginationBar.vue'
import TrafficIO from '@/components/TrafficIO.vue'
import {fetchClient, fetchProxies, kickClient} from '@/api'
import {ApiError} from '@/api/errors'
import type {ClientInfo, ProxyInfo} from '@/types/api'
import {useLocale} from '@/composables/useLocale'
import {formatProxyEndpoint, isHttpProxyType} from '@/utils/format'

const route = useRoute()
const router = useRouter()
const {t} = useLocale()

const runId = computed(() => String(route.params.runId || ''))

const client = ref<ClientInfo | null>(null)
const loading = ref(true)
const notFound = ref(false)
const kicking = ref(false)

const proxies = ref<ProxyInfo[]>([])
const proxiesLoading = ref(false)
const proxySearch = ref('')
const page = ref(1)
const pageSize = ref(10)
const total = ref(0)
const ready = ref(false)

let refreshTimer: number | null = null
let searchDebounce: number | null = null
let proxyReqSeq = 0

function isOnline(raw?: string) {
  return !raw || raw === 'online'
}

function statusLabel(raw?: string) {
  if (isOnline(raw)) return t('status.online')
  if (raw === 'offline') return t('status.offline')
  return raw || t('status.offline')
}

function formatSeen(secs: number, online: boolean) {
  const n = Math.max(0, Math.floor(secs || 0))
  if (online) {
    if (n < 60) return t('clients.uptimeSecs', {n})
    if (n < 3600) return t('clients.uptimeMins', {n: Math.floor(n / 60)})
    if (n < 86400) return t('clients.uptimeHours', {n: Math.floor(n / 3600)})
    return t('clients.uptimeDays', {n: Math.floor(n / 86400)})
  }
  if (n < 60) return t('clients.agoSecs', {n})
  if (n < 3600) return t('clients.agoMins', {n: Math.floor(n / 60)})
  if (n < 86400) return t('clients.agoHours', {n: Math.floor(n / 3600)})
  return t('clients.agoDays', {n: Math.floor(n / 86400)})
}

function goBack() {
  if (window.history.length > 1) router.back()
  else router.push({name: 'clients'})
}

function openProxy(name: string) {
  router.push({name: 'proxy-detail', params: {name}})
}

function onKeyOpenProxy(evt: KeyboardEvent, name: string) {
  if (evt.key === 'Enter' || evt.key === ' ') {
    evt.preventDefault()
    openProxy(name)
  }
}

async function loadClient() {
  const id = runId.value
  if (!id) {
    loading.value = false
    notFound.value = true
    return false
  }
  try {
    client.value = await fetchClient(id)
    notFound.value = false
    return true
  } catch (e) {
    if (e instanceof ApiError && e.code === 'http' && e.params?.status === 404) {
      client.value = null
      notFound.value = true
      return false
    }
    // Soft-fail refresh: keep previous client if we already have one.
    if (!client.value) notFound.value = true
    return false
  } finally {
    loading.value = false
  }
}

async function loadProxies() {
  const id = client.value?.runId || runId.value
  if (!id) return
  const seq = ++proxyReqSeq
  proxiesLoading.value = true
  try {
    const q = proxySearch.value.trim()
    const data = await fetchProxies({
      page: page.value,
      pageSize: pageSize.value,
      clientId: id,
      q: q || undefined,
    })
    if (seq !== proxyReqSeq) return

    const maxPage = Math.max(1, Math.ceil(data.total / Math.max(data.pageSize, 1)))
    if (data.items.length === 0 && data.total > 0 && data.page > maxPage) {
      page.value = maxPage
      await loadProxies()
      return
    }

    proxies.value = data.items ?? []
    total.value = data.total
    page.value = data.page
    pageSize.value = data.pageSize
  } catch {
    if (seq !== proxyReqSeq) return
  } finally {
    if (seq === proxyReqSeq) proxiesLoading.value = false
  }
}

async function refreshAll() {
  const ok = await loadClient()
  if (ok) await loadProxies()
}

async function onKick() {
  const id = client.value?.runId
  if (!id || kicking.value) return
  if (!window.confirm(t('clients.kickConfirm'))) return
  kicking.value = true
  try {
    await kickClient(id)
    await refreshAll()
  } catch (e) {
    const msg = e instanceof Error ? e.message : String(e)
    window.alert(t('clients.kickFailed', {msg}))
  } finally {
    kicking.value = false
  }
}

function clearSearchDebounce() {
  if (searchDebounce !== null) {
    window.clearTimeout(searchDebounce)
    searchDebounce = null
  }
}

watch(proxySearch, () => {
  if (!ready.value) return
  clearSearchDebounce()
  proxyReqSeq++
  proxiesLoading.value = false
  page.value = 1
  searchDebounce = window.setTimeout(() => {
    searchDebounce = null
    loadProxies()
  }, 300)
})

watch([page, pageSize], () => {
  if (!ready.value) return
  if (searchDebounce !== null) return
  loadProxies()
})

watch(runId, async () => {
  if (!ready.value) return
  loading.value = true
  client.value = null
  proxies.value = []
  total.value = 0
  page.value = 1
  proxySearch.value = ''
  await refreshAll()
})

onMounted(async () => {
  await refreshAll()
  ready.value = true
  refreshTimer = window.setInterval(() => {
    refreshAll()
  }, 5000)
})

onUnmounted(() => {
  clearSearchDebounce()
  if (refreshTimer !== null) {
    window.clearInterval(refreshTimer)
    refreshTimer = null
  }
})
</script>

<template>
  <div class="detail">
    <nav class="breadcrumb" :aria-label="t('clients.detail')">
      <button type="button" class="crumb-back" @click="goBack" :aria-label="t('clients.back')">
        ←
      </button>
      <button type="button" class="crumb-link" @click="router.push({ name: 'clients' })">
        {{ t('nav.clients') }}
      </button>
      <span class="crumb-sep" aria-hidden="true">/</span>
      <span class="crumb-current mono">{{ client?.runId || runId }}</span>
    </nav>

    <div v-if="loading && !client" class="empty-card">{{ t('traffic.loading') }}</div>

    <template v-else-if="client">
      <section class="summary card">
        <div class="summary-head">
          <div class="head-left">
            <div class="avatar" aria-hidden="true">
              <OsBadge :os="client.os" :arch="client.arch" icon-only size="md"/>
            </div>
            <div class="head-body">
              <div class="title-row">
                <h2 class="name mono">{{ client.runId }}</h2>
                <span v-if="client.version" class="tag version">v{{ client.version }}</span>
                <span v-if="client.user" class="tag">{{ client.user }}</span>
              </div>
              <div class="meta">
                <span v-if="client.clientIP" class="mono">{{ client.clientIP }}</span>
                <OsBadge :os="client.os" :arch="client.arch" size="md" text-only/>
              </div>
            </div>
          </div>
          <div class="head-right">
            <button
                v-if="isOnline(client.status)"
                type="button"
                class="kick-btn"
                :disabled="kicking"
                :title="t('clients.kick')"
                :aria-label="t('clients.kick')"
                @click="onKick"
            >
              <AppIcon name="kick"/>
            </button>
            <span class="status-badge" :class="{ online: isOnline(client.status) }">
              {{ statusLabel(client.status) }}
            </span>
          </div>
        </div>

        <div class="info-row" role="list">
          <div class="info-item" role="listitem">
            <em>{{ t('clients.connections') }}</em>
            <strong>{{ client.curConns ?? 0 }}</strong>
          </div>
          <div class="info-item" role="listitem">
            <em>{{ t('clients.proxies') }}</em>
            <strong>{{ client.proxyCount ?? 0 }}</strong>
          </div>
          <div class="info-item" role="listitem">
            <em>{{ isOnline(client.status) ? t('clients.connected') : t('clients.disconnected') }}</em>
            <strong>{{ formatSeen(client.connectedSecs, isOnline(client.status)) }}</strong>
          </div>
          <div v-if="client.hostname" class="info-item" role="listitem">
            <em>{{ t('clients.hostname') }}</em>
            <strong>{{ client.hostname }}</strong>
          </div>
        </div>
      </section>

      <section class="proxies-panel card">
        <div class="proxies-header">
          <div class="proxies-title">
            <h3>{{ t('nav.proxies') }}</h3>
            <span class="count">{{ total }}</span>
          </div>
          <label class="search">
            <svg viewBox="0 0 16 16" aria-hidden="true">
              <circle cx="7" cy="7" r="4.5"/>
              <path d="M10.5 10.5 14 14"/>
            </svg>
            <input
                v-model="proxySearch"
                type="search"
                :placeholder="t('clients.searchProxies')"
                autocomplete="off"
            />
          </label>
        </div>

        <div v-if="proxiesLoading && !proxies.length" class="panel-empty">
          {{ t('traffic.loading') }}
        </div>
        <div v-else-if="!proxies.length" class="panel-empty">
          {{
            proxySearch.trim()
                ? t('clients.proxiesSearchEmpty', {q: proxySearch.trim()})
                : t('clients.proxiesEmpty')
          }}
        </div>

        <div v-else class="proxy-list">
          <article
              v-for="p in proxies"
              :key="`${p.name}:${p.clientId}`"
              class="proxy-card"
              role="button"
              tabindex="0"
              @click="openProxy(p.name)"
              @keydown="onKeyOpenProxy($event, p.name)"
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
        </div>

        <PaginationBar
            v-if="total > 0"
            v-model:page="page"
            v-model:page-size="pageSize"
            :total="total"
        />
      </section>
    </template>

    <section v-else-if="notFound" class="not-found card">
      <h2>{{ t('clients.notFound') }}</h2>
      <p>{{ t('clients.notFoundDesc') }}</p>
      <button type="button" class="back-btn" @click="router.push({ name: 'clients' })">
        {{ t('clients.back') }}
      </button>
    </section>
  </div>
</template>

<style scoped>
.detail {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.breadcrumb {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 0.45rem;
  font-size: 0.85rem;
  color: var(--muted);
}

.crumb-back,
.crumb-link {
  border: 0;
  background: transparent;
  color: var(--muted);
  font: inherit;
  cursor: pointer;
  padding: 0;
}

.crumb-back:hover,
.crumb-link:hover {
  color: var(--accent-text);
}

.crumb-sep {
  opacity: 0.55;
}

.crumb-current {
  color: var(--text);
  font-weight: 600;
}

.mono {
  font-family: 'IBM Plex Mono', ui-monospace, monospace;
}

.summary,
.proxies-panel,
.not-found,
.empty-card {
  background: var(--panel);
  border: 1px solid var(--line);
  border-radius: 14px;
  box-shadow: var(--shadow);
}

.summary {
  display: flex;
  flex-direction: column;
  gap: 1.4rem;
  padding: 1.15rem 1.25rem 1.3rem;
  overflow: hidden;
}

.summary-head {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 1rem;
  padding: 0;
}

.head-left {
  display: flex;
  align-items: center;
  gap: 0.9rem;
  min-width: 0;
}

.avatar {
  width: 2.75rem;
  height: 2.75rem;
  border-radius: 12px;
  display: grid;
  place-items: center;
  flex-shrink: 0;
  background: color-mix(in srgb, var(--muted) 10%, transparent);
  border: 1px solid color-mix(in srgb, var(--muted) 14%, transparent);
}

.avatar :deep(.os-icon) {
  width: 1.45rem;
  height: 1.45rem;
}

.head-body {
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 0.35rem;
}

.title-row {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 0.45rem;
}

.name {
  margin: 0;
  font-size: 1.15rem;
  font-weight: 700;
  letter-spacing: -0.02em;
  line-height: 1.25;
  word-break: break-all;
}

.tag {
  display: inline-flex;
  align-items: center;
  padding: 0.12rem 0.5rem;
  border-radius: 999px;
  font-size: 0.72rem;
  font-weight: 600;
  color: var(--muted);
  background: color-mix(in srgb, var(--muted) 10%, transparent);
  border: 1px solid color-mix(in srgb, var(--muted) 14%, transparent);
}

.tag.version {
  color: var(--status-ok);
  background: var(--status-ok-soft);
  border-color: color-mix(in srgb, var(--status-ok) 22%, transparent);
}

.meta {
  display: flex;
  flex-wrap: wrap;
  gap: 0.65rem 1rem;
  font-size: 0.82rem;
  color: var(--muted);
}

.head-right {
  display: flex;
  align-items: center;
  gap: 0.55rem;
  flex-shrink: 0;
}

.kick-btn {
  box-sizing: border-box;
  width: 1.85rem;
  height: 1.85rem;
  padding: 0;
  border-radius: 8px;
  border: 1px solid color-mix(in srgb, var(--danger, #ef4444) 45%, transparent);
  background: transparent;
  color: var(--danger, #ef4444);
  display: inline-grid;
  place-items: center;
  cursor: pointer;
  font-size: 1rem;
}

.kick-btn:hover:not(:disabled) {
  border-color: var(--danger, #ef4444);
}

.kick-btn:disabled {
  opacity: 0.45;
  cursor: wait;
}

.info-row {
  display: flex;
  flex-wrap: wrap;
  gap: 0.85rem 1.75rem;
  padding: 0;
}

.info-item {
  display: inline-flex;
  align-items: baseline;
  gap: 0.4rem;
  min-width: 0;
  font-size: 0.82rem;
}

.info-item em {
  font-style: normal;
  color: var(--muted);
}

.info-item em::after {
  content: ':';
}

.info-item strong {
  font-weight: 650;
  color: var(--text);
  word-break: break-all;
}

.proxies-panel {
  display: flex;
  flex-direction: column;
  gap: 0;
  padding: 0;
  overflow: hidden;
}

.proxies-header {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem;
  padding: 0.95rem 1.15rem;
  border-bottom: 1px solid var(--line);
}

.proxies-title {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
}

.proxies-title h3 {
  margin: 0;
  font-size: 0.95rem;
  font-weight: 650;
}

.count {
  min-width: 1.35rem;
  padding: 0.1rem 0.45rem;
  border-radius: 8px;
  text-align: center;
  font-size: 0.72rem;
  font-weight: 650;
  font-variant-numeric: tabular-nums;
  color: var(--muted);
  background: color-mix(in srgb, var(--muted) 12%, transparent);
}

.search {
  display: inline-flex;
  align-items: center;
  gap: 0.4rem;
  min-width: min(100%, 14rem);
  padding: 0.35rem 0.7rem;
  border-radius: 10px;
  border: 1px solid var(--line);
  background: color-mix(in srgb, var(--muted) 6%, transparent);
}

.search svg {
  width: 0.95rem;
  height: 0.95rem;
  fill: none;
  stroke: var(--muted);
  stroke-width: 1.6;
  stroke-linecap: round;
  flex-shrink: 0;
}

.search input {
  flex: 1;
  min-width: 0;
  border: 0;
  outline: none;
  background: transparent;
  color: var(--text);
  font: inherit;
  font-size: 0.82rem;
}

.search input::placeholder {
  color: var(--muted);
}

.panel-empty,
.empty-card {
  padding: 2.25rem 1rem;
  text-align: center;
  color: var(--muted);
}

.proxy-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  padding: 0.9rem 1rem;
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

.proxies-panel :deep(.pagination-bar) {
  padding: 0 1rem 0.9rem;
  border-top: 0;
}

.not-found {
  padding: 2.5rem 1.25rem;
  text-align: center;
}

.not-found h2 {
  margin: 0 0 0.45rem;
  font-size: 1.1rem;
}

.not-found p {
  margin: 0 0 1.1rem;
  color: var(--muted);
  font-size: 0.88rem;
}

.back-btn {
  border: 1px solid color-mix(in srgb, var(--accent) 35%, transparent);
  background: var(--accent-soft);
  color: var(--accent-text);
  font: inherit;
  font-size: 0.85rem;
  font-weight: 650;
  padding: 0.45rem 0.95rem;
  border-radius: 10px;
  cursor: pointer;
}

.back-btn:hover {
  border-color: var(--accent);
}

@media (max-width: 560px) {
  .meta-arrow {
    display: none;
  }
}

@media (max-width: 720px) {
  .summary-head {
    flex-direction: column;
  }

  .head-right {
    align-self: flex-end;
  }

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
