<script setup lang="ts">
import {computed, onMounted, reactive, ref, watch} from "vue";
import {useI18n} from "vue-i18n";
import {
  defaultClientForm,
  defaultQuicConfig,
  defaultTlsConfig,
  getConfig,
  saveClientConfig,
  type ClientConfig,
  type ProxyConfig,
} from "@/api/desktop";
import PathField from "@/components/PathField.vue";
import type {MessageSchema} from "@/i18n";

const {t} = useI18n<{ message: MessageSchema }>();

const form = reactive(defaultClientForm());
const proxies = ref<ProxyConfig[]>([]);

const error = ref("");
const saving = ref(false);
const showAdvanced = ref(false);
const confirmReset = ref(false);
let applyingConfig = false;

const showQuic = computed(() => form.protocol === "quic");
const showMuxKeepalive = computed(
    () => form.tcpMux && form.protocol !== "quic",
);

function formatOptionalSecs(value: number): string {
  return value < 0 ? "" : String(value);
}

function parseOptionalSecs(raw: string): number {
  const text = raw.trim();
  if (text === "" || text === "-1") return -1;
  const n = Number(text);
  if (!Number.isFinite(n) || n < 0) return -1;
  return Math.floor(n);
}

const heartbeatIntervalInput = computed({
  get: () => formatOptionalSecs(form.heartbeatInterval),
  set: (raw: string) => {
    form.heartbeatInterval = parseOptionalSecs(raw);
  },
});

const heartbeatTimeoutInput = computed({
  get: () => formatOptionalSecs(form.heartbeatTimeout),
  set: (raw: string) => {
    form.heartbeatTimeout = parseOptionalSecs(raw);
  },
});

watch(
    () => form.tcpMux,
    (mux) => {
      if (applyingConfig) return;
      if (!mux) {
        if (form.heartbeatInterval < 0) form.heartbeatInterval = 30;
        if (form.heartbeatTimeout < 0) form.heartbeatTimeout = 90;
      } else {
        if (form.heartbeatInterval === 30) form.heartbeatInterval = -1;
        if (form.heartbeatTimeout === 90) form.heartbeatTimeout = -1;
      }
    },
);

function applyConfig(cfg: ClientConfig) {
  applyingConfig = true;
  try {
    const tls = {...defaultTlsConfig(), ...(cfg.tls ?? {})};
    const quic = {...defaultQuicConfig(), ...(cfg.quic ?? {})};
    Object.assign(form, {
      serverAddr: cfg.serverAddr,
      serverPort: cfg.serverPort,
      user: cfg.user ?? "",
      token: cfg.token,
      udpPacketSize: cfg.udpPacketSize ?? 1500,
      protocol: cfg.protocol,
      poolCount: cfg.poolCount ?? 1,
      tcpMux: cfg.tcpMux,
      tcpMuxKeepaliveInterval: cfg.tcpMuxKeepaliveInterval ?? 30,
      heartbeatInterval: cfg.heartbeatInterval ?? -1,
      heartbeatTimeout: cfg.heartbeatTimeout ?? -1,
      orbienPath: "",
    });
    Object.assign(form.tls, tls);
    Object.assign(form.quic, quic);
    proxies.value = cfg.proxies ?? [];
  } finally {
    applyingConfig = false;
  }
}

onMounted(async () => {
  try {
    applyConfig(await getConfig());
  } catch (e) {
    error.value = String(e);
  }
});

function cancelReset() {
  confirmReset.value = false;
}

async function resetToDefaults() {
  if (!confirmReset.value) {
    confirmReset.value = true;
    error.value = "";
    return;
  }

  confirmReset.value = false;
  const keptProxies = proxies.value;
  const defaults = defaultClientForm();
  applyConfig({
    ...defaults,
    tls: {...defaults.tls},
    quic: {...defaults.quic},
    proxies: keptProxies,
  });
  showAdvanced.value = false;
  await persist();
}

async function save() {
  confirmReset.value = false;
  await persist();
}

async function persist() {
  saving.value = true;
  error.value = "";
  try {
    const payload: ClientConfig = {
      serverAddr: form.serverAddr,
      serverPort: form.serverPort,
      user: form.user,
      token: form.token,
      udpPacketSize: form.udpPacketSize,
      protocol: form.protocol,
      poolCount: form.poolCount,
      tcpMux: form.tcpMux,
      tcpMuxKeepaliveInterval: form.tcpMuxKeepaliveInterval,
      heartbeatInterval: form.heartbeatInterval,
      heartbeatTimeout: form.heartbeatTimeout,
      orbienPath: "",
      tls: {...form.tls},
      quic: {...form.quic},
      proxies: proxies.value,
    };
    const result = await saveClientConfig(payload);
    applyConfig(result.config);
  } catch (e) {
    error.value = String(e);
  } finally {
    saving.value = false;
  }
}
</script>

<template>
  <section class="page">
    <header class="page-head">
      <h1 class="page-title">
        <svg viewBox="0 0 24 24" aria-hidden="true">
          <circle cx="12" cy="12" r="3"/>
          <path
              d="M19.4 15a1.7 1.7 0 0 0 .3 1.8l.1.1a2 2 0 1 1-2.8 2.8l-.1-.1a1.7 1.7 0 0 0-1.8-.3 1.7 1.7 0 0 0-1 1.5V21a2 2 0 1 1-4 0v-.1a1.7 1.7 0 0 0-1-1.5 1.7 1.7 0 0 0-1.8.3l-.1.1a2 2 0 1 1-2.8-2.8l.1-.1a1.7 1.7 0 0 0 .3-1.8 1.7 1.7 0 0 0-1.5-1H3a2 2 0 1 1 0-4h.1a1.7 1.7 0 0 0 1.5-1 1.7 1.7 0 0 0-.3-1.8l-.1-.1a2 2 0 1 1 2.8-2.8l.1.1a1.7 1.7 0 0 0 1.8.3H9a1.7 1.7 0 0 0 1-1.5V3a2 2 0 1 1 4 0v.1a1.7 1.7 0 0 0 1 1.5 1.7 1.7 0 0 0 1.8-.3l.1-.1a2 2 0 1 1 2.8 2.8l-.1.1a1.7 1.7 0 0 0-.3 1.8V9c.3.6.9 1 1.5 1H21a2 2 0 1 1 0 4h-.1a1.7 1.7 0 0 0-1.5 1z"
          />
        </svg>
        {{ t("config.title") }}
      </h1>
      <div class="head-actions">
        <template v-if="confirmReset">
          <button class="btn btn-secondary" type="button" :disabled="saving" @click="cancelReset">
            {{ t("common.cancel") }}
          </button>
          <button class="btn btn-primary" type="button" :disabled="saving" @click="resetToDefaults">
            {{ t("config.resetConfirmAction") }}
          </button>
        </template>
        <template v-else>
          <button class="btn btn-secondary" type="button" :disabled="saving" @click="resetToDefaults">
            {{ t("config.resetDefaults") }}
          </button>
          <button class="btn btn-primary" type="button" :disabled="saving" @click="save">
            {{ t("common.save") }}
          </button>
        </template>
      </div>
    </header>

    <p v-if="error" class="err">{{ error }}</p>

    <div class="panel form-card">
      <h2 class="block-title">{{ t("config.serverSection") }}</h2>
      <div class="grid">
        <label class="field">
          <span>{{ t("config.serverAddr") }}</span>
          <input v-model="form.serverAddr" placeholder="127.0.0.1"/>
        </label>
        <label class="field">
          <span>{{ t("config.serverPort") }}</span>
          <input v-model.number="form.serverPort" type="number" min="1" max="65535"/>
        </label>
        <label class="field">
          <span>{{ t("config.token") }}</span>
          <input
              v-model="form.token"
              type="password"
              autocomplete="off"
              :placeholder="t('config.tokenHint')"
          />
        </label>
        <label class="field">
          <span>{{ t("config.user") }}</span>
          <input v-model="form.user" :placeholder="t('config.userHint')"/>
        </label>
      </div>
    </div>

    <div class="panel form-card">
      <h2 class="block-title">{{ t("config.transportSection") }}</h2>
      <div class="grid">
        <label class="field">
          <span>{{ t("config.protocol") }}</span>
          <select v-model="form.protocol">
            <option value="tcp">TCP</option>
            <option value="websocket">WebSocket</option>
            <option value="quic">QUIC</option>
            <option value="kcp">KCP</option>
          </select>
        </label>
        <label class="field">
          <span>{{ t("config.poolCount") }}</span>
          <input v-model.number="form.poolCount" type="number" min="0" max="100"/>
        </label>
        <label class="switch-row span-2">
          <span class="switch-label">{{ t("config.tcpMux") }}</span>
          <input v-model="form.tcpMux" type="checkbox" :disabled="form.protocol === 'quic'"/>
        </label>
        <label class="switch-row span-2">
          <span class="switch-label">{{ t("config.tlsEnable") }}</span>
          <input v-model="form.tls.enable" type="checkbox"/>
        </label>
      </div>
    </div>

    <div class="panel form-card">
      <button class="advanced-toggle" type="button" @click="showAdvanced = !showAdvanced">
        {{ showAdvanced ? t("config.hideAdvanced") : t("config.showAdvanced") }}
      </button>

      <div v-if="showAdvanced" class="advanced">
        <div class="grid">
          <label v-if="showMuxKeepalive" class="field">
            <span>{{ t("config.tcpMuxKeepalive") }}</span>
            <input
                v-model.number="form.tcpMuxKeepaliveInterval"
                type="number"
                min="1"
                max="3600"
            />
          </label>
          <label class="field">
            <span>{{ t("config.heartbeatInterval") }}</span>
            <input
                v-model="heartbeatIntervalInput"
                type="number"
                min="0"
                max="3600"
                :placeholder="t('config.optionalEmpty')"
            />
          </label>
          <label class="field">
            <span>{{ t("config.heartbeatTimeout") }}</span>
            <input
                v-model="heartbeatTimeoutInput"
                type="number"
                min="0"
                max="7200"
                :placeholder="t('config.optionalEmpty')"
            />
          </label>
          <label class="field">
            <span>{{ t("config.udpPacketSize") }}</span>
            <input v-model.number="form.udpPacketSize" type="number" min="512" max="65535"/>
          </label>
          <label class="field">
            <span>{{ t("config.tlsServerName") }}</span>
            <input v-model="form.tls.serverName"/>
          </label>
          <PathField
              v-model="form.tls.trustedCaFile"
              :label="t('config.tlsTrustedCa')"
              :placeholder="t('config.pathHint')"
          />
          <PathField
              v-model="form.tls.certFile"
              :label="t('config.tlsCert')"
              :placeholder="t('config.pathHint')"
          />
          <PathField
              v-model="form.tls.keyFile"
              :label="t('config.tlsKey')"
              :placeholder="t('config.pathHint')"
          />
          <label class="switch-row span-2">
            <span class="switch-label">{{ t("config.tlsDisableFirstByte") }}</span>
            <input v-model="form.tls.disableCustomTlsFirstByte" type="checkbox"/>
          </label>
        </div>

        <div v-if="showQuic" class="grid quic-grid">
          <label class="field">
            <span>{{ t("config.quicKeepalive") }}</span>
            <input
                v-model.number="form.quic.keepalivePeriod"
                type="number"
                min="1"
                max="600"
            />
          </label>
          <label class="field">
            <span>{{ t("config.quicIdle") }}</span>
            <input
                v-model.number="form.quic.maxIdleTimeout"
                type="number"
                min="1"
                max="3600"
            />
          </label>
          <label class="field">
            <span>{{ t("config.quicStreams") }}</span>
            <input
                v-model.number="form.quic.maxIncomingStreams"
                type="number"
                min="1"
                max="1000000"
            />
          </label>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.head-actions {
  display: flex;
  align-items: center;
  gap: 0.55rem;
  flex-shrink: 0;
}

.btn-secondary {
  background: transparent;
  color: var(--text);
  border: 1px solid var(--line);
}

.btn-secondary:hover:not(:disabled) {
  background: color-mix(in srgb, var(--muted) 10%, transparent);
  border-color: color-mix(in srgb, var(--muted) 35%, var(--line));
}

.form-card {
  padding: 1.15rem 1.25rem 1.35rem;
  display: grid;
  gap: 1rem;
}

.block-title {
  margin: 0;
  font-size: 0.95rem;
  font-weight: 700;
  color: var(--text);
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

.field > span,
.switch-label {
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

.switch-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  min-height: 2.4rem;
}

.switch-row input[type="checkbox"] {
  width: 1.05rem;
  height: 1.05rem;
  accent-color: var(--accent);
  flex-shrink: 0;
}

.advanced-toggle {
  border: 0;
  background: transparent;
  color: var(--accent);
  font: inherit;
  font-weight: 600;
  font-size: 0.9rem;
  padding: 0;
  cursor: pointer;
  text-align: left;
  width: fit-content;
}

.advanced-toggle:hover {
  text-decoration: underline;
}

.advanced {
  display: grid;
  gap: 1rem;
  padding-top: 0.25rem;
}

.quic-grid {
  padding-top: 0.25rem;
  border-top: 1px solid var(--line);
}

.err {
  margin: 0;
  color: var(--danger);
  font-size: 0.9rem;
}

@media (max-width: 900px) {
  .grid {
    grid-template-columns: 1fr;
  }
}
</style>
