<script setup lang="ts">
import {computed, nextTick, onMounted, onUnmounted, ref, watch} from "vue";
import {useI18n} from "vue-i18n";
import {clearLogs, getLogs} from "@/api/desktop";
import type {MessageSchema} from "@/i18n";

const {t} = useI18n<{ message: MessageSchema }>();
const tab = ref<"app" | "client">("app");
const query = ref("");
const queryDebounced = ref("");
const lines = ref<string[]>([]);
const autoScroll = ref(true);
const scroller = ref<HTMLElement | null>(null);

let logsRev = 0;
let timer: ReturnType<typeof setInterval> | null = null;
let queryTimer: ReturnType<typeof setTimeout> | null = null;
let refreshInFlight = false;
let scrollPending = false;

function isOrbienLine(line: string) {
  return line.startsWith("[orbien]") || line.startsWith("[orbien:err]");
}

function levelClass(line: string): string {
  if (line.startsWith("[error]") || line.startsWith("[orbien:err]")) return "lvl-error";
  if (line.startsWith("[warn]")) return "lvl-warn";
  if (line.startsWith("[info]") || line.startsWith("[orbien]")) return "lvl-info";
  if (line.startsWith("[debug]")) return "lvl-debug";

  const head = line.slice(0, 160).toLowerCase();
  if (head.includes("error")) return "lvl-error";
  if (head.includes("warn")) return "lvl-warn";
  if (head.includes("info")) return "lvl-info";
  if (head.includes("debug")) return "lvl-debug";
  return "lvl-default";
}

interface DisplayLine {
  text: string;
  cls: string;
}

const displayLines = computed<DisplayLine[]>(() => {
  const wantOrbien = tab.value === "client";
  const q = queryDebounced.value;
  const out: DisplayLine[] = [];
  for (const line of lines.value) {
    if (isOrbienLine(line) !== wantOrbien) continue;
    if (q && !line.toLowerCase().includes(q)) continue;
    out.push({text: line, cls: levelClass(line)});
  }
  return out;
});

async function refresh() {
  if (refreshInFlight) return;
  if (typeof document !== "undefined" && document.hidden) return;
  refreshInFlight = true;
  try {
    const snap = await getLogs(logsRev);
    logsRev = snap.rev;
    if (snap.lines) {
      lines.value = snap.lines;
    }
  } catch {
    logsRev = 0;
    lines.value = ["[warn] Tauri IPC unavailable — open via `npm run tauri dev`"];
  } finally {
    refreshInFlight = false;
  }
}

async function onClear() {
  try {
    await clearLogs();
  } catch {
    /* ignore */
  }
  logsRev = 0;
  await refresh();
}

function scheduleScroll() {
  if (!autoScroll.value || scrollPending) return;
  scrollPending = true;
  requestAnimationFrame(() => {
    scrollPending = false;
    if (!autoScroll.value || !scroller.value) return;
    scroller.value.scrollTop = scroller.value.scrollHeight;
  });
}

watch(query, (v) => {
  if (queryTimer) clearTimeout(queryTimer);
  queryTimer = setTimeout(() => {
    queryDebounced.value = v.trim().toLowerCase();
  }, 150);
});

watch([displayLines, autoScroll], async () => {
  if (!autoScroll.value) return;
  await nextTick();
  scheduleScroll();
});

onMounted(() => {
  void refresh();
  timer = setInterval(() => void refresh(), 1000);
});

onUnmounted(() => {
  if (timer) clearInterval(timer);
  if (queryTimer) clearTimeout(queryTimer);
});
</script>

<template>
  <section class="page logger-page">
    <header class="page-head">
      <h1 class="page-title">
        <svg viewBox="0 0 24 24" aria-hidden="true">
          <path d="M7 3h7l3 3v15H7z"/>
          <path d="M14 3v4h4M9 12h6M9 16h6"/>
        </svg>
        {{ t("logger.title") }}
      </h1>
    </header>

    <div class="tabs">
      <button type="button" :class="{ active: tab === 'app' }" @click="tab = 'app'">
        {{ t("logger.appLog") }}
      </button>
      <button type="button" :class="{ active: tab === 'client' }" @click="tab = 'client'">
        {{ t("logger.orbienLog") }}
      </button>
    </div>

    <div class="panel console">
      <div class="toolbar">
        <input v-model="query" type="text" :placeholder="t('common.searchLogs')"/>
        <label class="auto">
          <input v-model="autoScroll" type="checkbox"/>
          {{ t("common.autoScroll") }}
        </label>
        <button class="btn btn-ghost" type="button" @click="onClear">
          {{ t("common.clear") }}
        </button>
      </div>

      <div ref="scroller" class="log-body">
        <div class="log-content">
          <div v-if="!displayLines.length" class="muted">{{ t("common.emptyLogs") }}</div>
          <div
              v-for="(row, i) in displayLines"
              :key="i"
              class="log-line"
              :class="row.cls"
          >
            {{ row.text }}
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.logger-page {
  flex: 1;
  min-height: 0;
  overflow: hidden;
}

.logger-page .page-head {
  flex-shrink: 0;
}

.tabs {
  display: flex;
  flex-shrink: 0;
  gap: 1.25rem;
  border-bottom: 1px solid var(--line);
}

.tabs button {
  border: none;
  background: transparent;
  color: var(--muted);
  padding: 0.45rem 0.1rem 0.7rem;
  cursor: pointer;
  font-weight: 600;
  border-bottom: 2px solid transparent;
}

.tabs button.active {
  color: var(--accent);
  border-bottom-color: var(--accent);
}

.console {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background: #0f172a;
  border: none;
}

.toolbar {
  display: flex;
  flex-shrink: 0;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem 0.9rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
}

.toolbar input[type="text"] {
  flex: 1;
  border: 1px solid rgba(255, 255, 255, 0.12);
  background: rgba(255, 255, 255, 0.04);
  color: #e8e8f0;
  border-radius: var(--radius);
  padding: 0.45rem 0.7rem;
}

.auto {
  color: #94a3b8;
  font-size: 0.82rem;
  display: inline-flex;
  align-items: center;
  gap: 0.35rem;
  white-space: nowrap;
}

.log-body {
  flex: 1;
  min-height: 0;
  overflow: auto;
  overscroll-behavior: contain;
}

.log-content {
  padding: 0.9rem 1rem 1.2rem;
  font-size: 0.82rem;
  line-height: 1.55;
  font-family: "SF Mono", Menlo, Monaco, "PingFang SC", "Hiragino Sans GB", "Microsoft YaHei",
  ui-monospace, monospace;
}

.log-line {
  white-space: pre-wrap;
  word-break: break-word;
  content-visibility: auto;
  contain-intrinsic-size: auto 1.55em;
}

.lvl-error {
  color: #fca5a5;
}

.lvl-warn {
  color: #fcd34d;
}

.lvl-info {
  color: #7dd3fc;
}

.lvl-debug {
  color: #94a3b8;
}

.lvl-default {
  color: #cbd5e1;
}

.muted {
  color: #94a3b8;
}

.log-body::-webkit-scrollbar {
  width: 10px;
}

.log-body::-webkit-scrollbar-track {
  background: transparent;
}

.log-body::-webkit-scrollbar-thumb {
  background: rgba(148, 163, 184, 0.35);
  border-radius: 999px;
  border: 2px solid transparent;
  background-clip: padding-box;
}

.log-body::-webkit-scrollbar-thumb:hover {
  background: rgba(148, 163, 184, 0.55);
  background-clip: padding-box;
  border: 2px solid transparent;
}
</style>
