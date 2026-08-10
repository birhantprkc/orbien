<script setup lang="ts">
import {computed, onMounted, reactive, ref, watch} from "vue";
import {useI18n} from "vue-i18n";
import {
  defaultProxyForm,
  defaultProxyPlugin,
  defaultProxyTransport,
  getConfig,
  joinList,
  listProxies,
  normalizeProxyFromServer,
  saveProxies,
  splitList,
  type ProxyConfig,
  type ProxyItem,
} from "@/api/desktop";
import PathField from "@/components/PathField.vue";
import type {MessageSchema} from "@/i18n";

const {t} = useI18n<{ message: MessageSchema }>();

const proxies = ref<ProxyItem[]>([]);
const draft = ref<ProxyConfig[]>([]);
const error = ref("");
const saving = ref(false);
const editorOpen = ref(false);
const editingIndex = ref<number | null>(null);
const showAdvanced = ref(false);
const copiedName = ref("");
let copiedTimer: ReturnType<typeof setTimeout> | null = null;

const form = reactive(defaultProxyForm());
const customDomainsText = ref("");
const locationsText = ref("");
const httpsMode = ref<"passthrough" | "https2http">("passthrough");

const isPortProxy = computed(() => form.proxyType === "tcp" || form.proxyType === "udp");
const isHttp = computed(() => form.proxyType === "http");
const isHttps = computed(() => form.proxyType === "https");
const isVhost = computed(() => isHttp.value || isHttps.value);
const useHttps2Http = computed(() => isHttps.value && httpsMode.value === "https2http");
const showLocalDial = computed(() => !useHttps2Http.value);
const showBandwidthMode = computed(() => !!form.transport.bandwidthLimit.trim());

watch(
    () => form.proxyType,
    (ty, prev) => {
      if (!editorOpen.value || ty === prev) return;
      if (ty === "tcp" || ty === "udp") {
        httpsMode.value = "passthrough";
        form.remotePort = ty === "udp" ? 7001 : 6000;
        form.localPort = ty === "udp" ? 12001 : 8080;
        form.localIp = form.localIp || "127.0.0.1";
      } else {
        form.remotePort = 0;
        if (ty === "http") {
          httpsMode.value = "passthrough";
          form.localPort = form.localPort || 80;
        }
        if (ty === "https") {
          if (httpsMode.value === "passthrough") {
            form.localPort = form.localPort && form.localPort !== 80 ? form.localPort : 443;
          }
        }
      }
    },
);

watch(httpsMode, (mode) => {
  if (!editorOpen.value || !isHttps.value) return;
  if (mode === "https2http") {
    form.plugin = {...defaultProxyPlugin(), ...(form.plugin ?? {})};
    form.plugin.type = "https2http";
    if (!form.plugin.localAddr) form.plugin.localAddr = "127.0.0.1:80";
  } else {
    form.plugin = null;
    if (!form.localPort) form.localPort = 443;
  }
});

async function refresh() {
  try {
    const [items, cfg] = await Promise.all([listProxies(), getConfig()]);
    draft.value = (cfg.proxies ?? []).map((p) => normalizeProxyFromServer(p));
    proxies.value = items;
    error.value = "";
  } catch (e) {
    error.value = String(e);
  }
}

function hasAdvancedValues(p: ProxyConfig, pluginMode: boolean): boolean {
  return !!(
      p.transport.bandwidthLimit ||
      p.transport.proxyProtocolVersion ||
      (p.proxyType === "http" && (p.locations.length || p.hostHeaderRewrite)) ||
      (p.localIp && p.localIp !== "127.0.0.1") ||
      (pluginMode &&
          p.plugin &&
          (p.plugin.crtPath || p.plugin.keyPath || p.plugin.hostHeaderRewrite))
  );
}

function assignForm(src: ProxyConfig) {
  const p = normalizeProxyFromServer(src);
  form.name = p.name;
  form.proxyType = p.proxyType;
  form.localIp = p.localIp || "127.0.0.1";
  form.localPort = p.localPort;
  form.remotePort = p.remotePort;
  form.customDomains = [...p.customDomains];
  form.subdomain = p.subdomain;
  form.locations = [...p.locations];
  form.httpUser = "";
  form.httpPassword = "";
  form.hostHeaderRewrite = p.hostHeaderRewrite;
  form.routeByHttpUser = "";
  form.transport = {...defaultProxyTransport(), ...p.transport};
  form.plugin = p.plugin ? {...defaultProxyPlugin(), ...p.plugin} : null;
  customDomainsText.value = joinList(p.customDomains);
  locationsText.value = joinList(p.locations);
  const pluginOn = !!(p.plugin && p.plugin.type === "https2http");
  httpsMode.value = pluginOn ? "https2http" : "passthrough";
  showAdvanced.value = hasAdvancedValues(p, pluginOn);
}

function openAdd() {
  editingIndex.value = null;
  assignForm(defaultProxyForm("tcp"));
  showAdvanced.value = false;
  editorOpen.value = true;
  error.value = "";
}

function openEdit(index: number) {
  const p = draft.value[index];
  if (!p) return;
  editingIndex.value = index;
  assignForm(p);
  editorOpen.value = true;
  error.value = "";
}

function closeEditor() {
  editorOpen.value = false;
  editingIndex.value = null;
}

function buildEntry(): ProxyConfig | null {
  const name = form.name.trim();
  if (!name) {
    error.value = t("proxy.nameRequired");
    return null;
  }

  const ty = form.proxyType || "tcp";
  const pluginOn = ty === "https" && httpsMode.value === "https2http";

  const entry = normalizeProxyFromServer({
    ...form,
    name,
    proxyType: ty,
    customDomains: splitList(customDomainsText.value),
    locations: ty === "http" ? splitList(locationsText.value) : [],
    localIp: form.localIp.trim() || "127.0.0.1",
    subdomain: form.subdomain.trim(),
    httpUser: "",
    httpPassword: "",
    hostHeaderRewrite: ty === "http" ? form.hostHeaderRewrite.trim() : "",
    routeByHttpUser: "",
    transport: {
      bandwidthLimit: form.transport.bandwidthLimit.trim(),
      bandwidthLimitMode: form.transport.bandwidthLimitMode || "client",
      proxyProtocolVersion: pluginOn ? "" : form.transport.proxyProtocolVersion,
    },
    plugin: pluginOn
        ? {
          type: "https2http",
          localAddr: (form.plugin?.localAddr || "").trim(),
          crtPath: (form.plugin?.crtPath || "").trim(),
          keyPath: (form.plugin?.keyPath || "").trim(),
          hostHeaderRewrite: (form.plugin?.hostHeaderRewrite || "").trim(),
        }
        : null,
  });

  if (ty === "tcp" || ty === "udp") {
    if (!entry.remotePort) {
      error.value = t("proxy.remotePortRequired");
      return null;
    }
    if (!entry.localPort) {
      error.value = t("proxy.localPortRequired");
      return null;
    }
  }

  if (ty === "http" || ty === "https") {
    if (!entry.customDomains.length && !entry.subdomain) {
      error.value = t("proxy.domainRequired");
      return null;
    }
  }

  if (pluginOn) {
    if (!entry.plugin?.localAddr) {
      error.value = t("proxy.pluginLocalAddrRequired");
      return null;
    }
  } else if ((ty === "http" || ty === "https") && !entry.localPort) {
    error.value = t("proxy.localPortRequired");
    return null;
  }

  return entry;
}

async function persist(next: ProxyConfig[]) {
  saving.value = true;
  error.value = "";
  try {
    const result = await saveProxies(next);
    draft.value = next.map((p) => normalizeProxyFromServer(p));
    proxies.value = result.proxies;
    closeEditor();
  } catch (e) {
    error.value = String(e);
  } finally {
    saving.value = false;
  }
}

async function submitEditor() {
  const entry = buildEntry();
  if (!entry) return;
  const next = [...draft.value];
  const idx = editingIndex.value;
  if (idx === null) {
    if (next.some((p) => p.name === entry.name)) {
      error.value = t("proxy.nameExists");
      return;
    }
    next.push(entry);
  } else {
    if (next.some((p, i) => i !== idx && p.name === entry.name)) {
      error.value = t("proxy.nameExists");
      return;
    }
    next[idx] = entry;
  }
  await persist(next);
}

async function removeAt(index: number) {
  const next = draft.value.filter((_, i) => i !== index);
  await persist(next);
}

async function copyAddress(p: ProxyItem) {
  const text = (p.copyValue || "").trim();
  if (!text) return;
  try {
    await navigator.clipboard.writeText(text);
    copiedName.value = p.name;
    if (copiedTimer) clearTimeout(copiedTimer);
    copiedTimer = setTimeout(() => {
      copiedName.value = "";
      copiedTimer = null;
    }, 1500);
  } catch (e) {
    error.value = String(e);
  }
}

onMounted(() => {
  void refresh();
});
</script>

<template>
  <section class="page">
    <header class="page-head">
      <h1 class="page-title">
        <svg viewBox="0 0 24 24" aria-hidden="true">
          <path d="M7 18h10a4 4 0 0 0 .3-8 5.5 5.5 0 0 0-10.6 1.5A3.5 3.5 0 0 0 7 18z"/>
        </svg>
        {{ t("proxy.title") }}
      </h1>
      <button
          class="btn btn-icon"
          type="button"
          :title="t('common.addProxy')"
          @click="openAdd"
      >
        <svg viewBox="0 0 24 24" aria-hidden="true">
          <path d="M12 5v14M5 12h14"/>
        </svg>
      </button>
    </header>

    <p v-if="error" class="err">{{ error }}</p>

    <div v-if="editorOpen" class="panel editor">
      <div class="editor-head">
        <div class="editor-title">
          {{ editingIndex === null ? t("proxy.addTitle") : t("proxy.editTitle") }}
        </div>
        <div class="editor-actions">
          <button class="btn btn-ghost" type="button" :disabled="saving" @click="closeEditor">
            {{ t("common.cancel") }}
          </button>
          <button class="btn btn-primary" type="button" :disabled="saving" @click="submitEditor">
            {{ t("common.save") }}
          </button>
        </div>
      </div>

      <div class="grid">
        <label class="field">
          <span>{{ t("proxy.name") }}</span>
          <input v-model="form.name" :placeholder="t('proxy.nameHint')"/>
        </label>
        <label class="field">
          <span>{{ t("proxy.type") }}</span>
          <select v-model="form.proxyType">
            <option value="tcp">TCP</option>
            <option value="udp">UDP</option>
            <option value="http">HTTP</option>
            <option value="https">HTTPS</option>
          </select>
        </label>

        <!-- TCP / UDP -->
        <template v-if="isPortProxy">
          <label class="field">
            <span>{{ t("proxy.localPort") }}</span>
            <input v-model.number="form.localPort" type="number" min="1" max="65535"/>
          </label>
          <label class="field">
            <span>{{ t("proxy.remotePort") }}</span>
            <input v-model.number="form.remotePort" type="number" min="1" max="65535"/>
          </label>
        </template>

        <template v-if="isVhost">
          <label v-if="isHttps" class="field span-2">
            <span>{{ t("proxy.httpsMode") }}</span>
            <select v-model="httpsMode">
              <option value="passthrough">{{ t("proxy.httpsModePassthrough") }}</option>
              <option value="https2http">{{ t("proxy.httpsModePlugin") }}</option>
            </select>
          </label>

          <label v-if="showLocalDial" class="field">
            <span>{{ t("proxy.localPort") }}</span>
            <input v-model.number="form.localPort" type="number" min="1" max="65535"/>
          </label>
          <label v-if="useHttps2Http && form.plugin" class="field">
            <span>{{ t("proxy.pluginLocalAddr") }}</span>
            <input
                v-model="form.plugin.localAddr"
                :placeholder="t('proxy.pluginLocalAddrHint')"
            />
          </label>
          <label class="field">
            <span>{{ t("proxy.subdomain") }}</span>
            <input v-model="form.subdomain" :placeholder="t('proxy.subdomainHint')"/>
          </label>

          <label class="field span-2">
            <span>{{ t("proxy.customDomains") }}</span>
            <input
                v-model="customDomainsText"
                :placeholder="t('proxy.customDomainsHint')"
            />
          </label>
        </template>
      </div>

      <button class="advanced-toggle" type="button" @click="showAdvanced = !showAdvanced">
        {{ showAdvanced ? t("proxy.hideAdvanced") : t("proxy.showAdvanced") }}
      </button>

      <div v-if="showAdvanced" class="advanced grid">
        <label v-if="showLocalDial" class="field">
          <span>{{ t("proxy.localIp") }}</span>
          <input v-model="form.localIp" placeholder="127.0.0.1"/>
        </label>

        <template v-if="isHttp">
          <label class="field">
            <span>{{ t("proxy.locations") }}</span>
            <input v-model="locationsText" :placeholder="t('proxy.locationsHint')"/>
          </label>
          <label class="field">
            <span>{{ t("proxy.hostHeaderRewrite") }}</span>
            <input
                v-model="form.hostHeaderRewrite"
                :placeholder="t('proxy.hostHeaderRewriteHint')"
            />
          </label>
        </template>

        <template v-if="useHttps2Http && form.plugin">
          <PathField
              v-model="form.plugin.crtPath"
              :label="t('proxy.pluginCrt')"
              :placeholder="t('proxy.pathHint')"
          />
          <PathField
              v-model="form.plugin.keyPath"
              :label="t('proxy.pluginKey')"
              :placeholder="t('proxy.pathHint')"
          />
          <label class="field span-2">
            <span>{{ t("proxy.hostHeaderRewrite") }}</span>
            <input
                v-model="form.plugin.hostHeaderRewrite"
                :placeholder="t('proxy.pluginHostRewriteHint')"
            />
          </label>
        </template>

        <label class="field">
          <span>{{ t("proxy.bandwidthLimit") }}</span>
          <input
              v-model="form.transport.bandwidthLimit"
              :placeholder="t('proxy.bandwidthLimitHint')"
          />
        </label>
        <label v-if="showBandwidthMode" class="field">
          <span>{{ t("proxy.bandwidthMode") }}</span>
          <select v-model="form.transport.bandwidthLimitMode">
            <option value="client">client</option>
            <option value="server">server</option>
          </select>
        </label>
        <label v-if="showLocalDial" class="field">
          <span>{{ t("proxy.proxyProtocol") }}</span>
          <select v-model="form.transport.proxyProtocolVersion">
            <option value="">{{ t("proxy.proxyProtocolOff") }}</option>
            <option value="v1">v1</option>
            <option value="v2">v2</option>
          </select>
        </label>
      </div>
    </div>

    <div class="list">
      <article v-for="(p, index) in proxies" :key="p.name" class="panel proxy-card">
        <div class="card-top">
          <div class="name-row">
            <div class="name">{{ p.name }}</div>
            <span class="tag">{{ p.proxyType }}</span>
          </div>
          <div class="actions">
            <button class="btn btn-ghost" type="button" @click="openEdit(index)">
              {{ t("common.modify") }}
            </button>
            <button class="btn btn-ghost danger" type="button" @click="removeAt(index)">
              {{ t("common.delete") }}
            </button>
          </div>
        </div>
        <dl class="meta">
          <div class="meta-row">
            <dt>{{ t("proxy.inner") }}</dt>
            <dd>{{ p.local }}</dd>
          </div>
          <div class="meta-row">
            <dt>{{ t("proxy.remote") }}</dt>
            <dd>
              <span class="remote-text" :title="p.remote">{{ p.remote }}</span>
              <button
                  v-if="p.copyValue"
                  class="copy-btn"
                  type="button"
                  :title="copiedName === p.name ? t('proxy.copied') : t('proxy.copy')"
                  @click="copyAddress(p)"
              >
                {{ copiedName === p.name ? t("proxy.copied") : t("proxy.copy") }}
              </button>
            </dd>
          </div>
        </dl>
      </article>
    </div>
  </section>
</template>

<style scoped>
.editor {
  padding: 1.15rem 1.25rem 1.35rem;
  display: grid;
  gap: 1rem;
}

.editor-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
}

.editor-title {
  font-weight: 700;
  color: var(--text);
  font-size: 1.05rem;
}

.editor-actions {
  display: flex;
  gap: 0.45rem;
  flex-shrink: 0;
}

.grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 1rem 1.1rem;
}

.span-2 {
  grid-column: 1 / -1;
}

.field {
  display: grid;
  gap: 0.4rem;
  min-width: 0;
}

.field > span {
  font-weight: 600;
  font-size: 0.9rem;
  color: var(--text);
}

.field input,
.field select {
  border: 1px solid var(--line);
  border-radius: var(--radius);
  padding: 0.7rem 0.8rem;
  background: #fff;
  color: var(--text);
  width: 100%;
}

.field input:focus,
.field select:focus {
  outline: none;
  border-color: rgba(59, 130, 246, 0.55);
  box-shadow: 0 0 0 3px var(--accent-soft);
}

.advanced-toggle {
  justify-self: start;
  border: 0;
  background: transparent;
  color: var(--accent);
  font: inherit;
  font-weight: 600;
  font-size: 0.9rem;
  padding: 0;
  cursor: pointer;
  width: fit-content;
}

.advanced-toggle:hover {
  text-decoration: underline;
}

.advanced {
  padding-top: 0.15rem;
  border-top: 1px solid var(--line);
}

.list {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 0.85rem;
  align-items: stretch;
}

.proxy-card {
  display: flex;
  flex-direction: column;
  gap: 0.85rem;
  padding: 1rem 1.1rem;
  min-width: 0;
}

.card-top {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 0.75rem;
}

.name-row {
  display: flex;
  align-items: center;
  gap: 0.55rem;
  min-width: 0;
}

.name {
  color: var(--text);
  font-weight: 700;
  font-size: 1.05rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.tag {
  flex-shrink: 0;
  background: var(--accent-soft);
  color: var(--accent);
  border-radius: var(--radius);
  padding: 0.12rem 0.55rem;
  font-size: 0.72rem;
  font-weight: 700;
  text-transform: uppercase;
}

.meta {
  margin: 0;
  display: grid;
  gap: 0.45rem;
}

.meta-row {
  display: grid;
  grid-template-columns: 2.5rem minmax(0, 1fr);
  gap: 0.75rem;
  align-items: center;
}

.meta-row dt {
  margin: 0;
  color: var(--muted);
  font-size: 0.85rem;
}

.meta-row dd {
  margin: 0;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  min-width: 0;
  color: var(--text);
  font-size: 0.95rem;
  font-weight: 600;
  font-variant-numeric: tabular-nums;
}

.remote-text {
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.copy-btn {
  flex-shrink: 0;
  border: 1px solid var(--line);
  background: transparent;
  color: var(--accent);
  border-radius: var(--radius);
  padding: 0.2rem 0.5rem;
  font: inherit;
  font-size: 0.78rem;
  font-weight: 600;
  cursor: pointer;
}

.copy-btn:hover {
  background: var(--accent-soft);
}

.actions {
  display: flex;
  gap: 0.15rem;
  flex-shrink: 0;
}

.btn.danger {
  color: var(--danger);
}

.err {
  margin: 0;
  font-size: 0.9rem;
  color: var(--danger);
}

@media (max-width: 900px) {
  .list,
  .grid {
    grid-template-columns: 1fr;
  }

  .editor-head {
    flex-direction: column;
    align-items: stretch;
  }

  .editor-actions {
    justify-content: flex-end;
  }
}
</style>
