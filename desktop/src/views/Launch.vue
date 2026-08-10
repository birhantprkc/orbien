<script setup lang="ts">
import {computed, onMounted, onUnmounted, ref} from "vue";
import {useRouter} from "vue-router";
import {useI18n} from "vue-i18n";
import {getStatus, startClient, stopClient, type ClientStatus} from "@/api/desktop";
import type {MessageSchema} from "@/i18n";

const router = useRouter();
const {t} = useI18n<{ message: MessageSchema }>();
const status = ref<ClientStatus>({running: false, runningSecs: 0, version: "2.0.0"});
const busy = ref(false);
const error = ref("");

let timer: ReturnType<typeof setInterval> | null = null;

const title = computed(() =>
    status.value.running ? t("launch.running") : t("launch.stopped"),
);
const runningLabel = computed(() => {
  const s = status.value.runningSecs;
  if (s < 60) return t("launch.seconds", {n: s});
  const m = Math.floor(s / 60);
  const r = s % 60;
  return t("launch.minutesSecs", {m, s: r});
});

async function refresh(opts?: { clearError?: boolean }) {
  try {
    status.value = await getStatus();
    if (opts?.clearError) error.value = "";
  } catch (e) {
    error.value = String(e);
  }
}

async function toggle() {
  busy.value = true;
  error.value = "";
  try {
    status.value = status.value.running ? await stopClient() : await startClient();
  } catch (e) {
    error.value = String(e);
  } finally {
    busy.value = false;
    await refresh({clearError: false});
  }
}

onMounted(() => {
  void refresh({clearError: true});
  timer = setInterval(() => void refresh({clearError: false}), 1000);
});

onUnmounted(() => {
  if (timer) clearInterval(timer);
});
</script>

<template>
  <section class="page">
    <header class="page-head">
      <h1 class="page-title">
        <svg viewBox="0 0 24 24" aria-hidden="true">
          <path d="M12 3c4 2 6 6 6 10l-3 1-2 4-2-4-3-1c0-4 2-8 6-10z"/>
          <path d="M9 14l-3 5M15 14l3 5"/>
        </svg>
        {{ t("launch.title") }}
      </h1>
    </header>

    <div class="panel launch-card">
      <div class="hero">
        <div class="orb">
          <div class="blob b1"/>
          <div class="blob b2"/>
          <div class="orb-core">
            <svg viewBox="0 0 24 24" aria-hidden="true">
              <path d="M12 3c4 2 6 6 6 10l-3 1-2 4-2-4-3-1c0-4 2-8 6-10z"/>
              <path d="M9 14l-3 5M15 14l3 5"/>
            </svg>
          </div>
        </div>

        <div class="status-block">
          <div class="status-line">
            <span class="dot" :class="{ on: status.running }"/>
            <strong>{{ title }}</strong>
          </div>
          <div class="meta">
            {{ t("launch.runningTime") }} {{ status.running ? runningLabel : "—" }}
            <button class="btn btn-ghost" type="button" @click="router.push('/logger')">
              {{ t("launch.viewLog") }}
            </button>
          </div>
          <p v-if="error" class="err">{{ error }}</p>
          <button
              class="btn btn-primary stop-btn"
              type="button"
              :disabled="busy"
              @click="toggle"
          >
            {{ status.running ? t("launch.stop") : t("launch.start") }}
          </button>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.launch-card {
  flex: 1;
  display: grid;
  place-items: center;
  padding: 2rem;
  min-height: 520px;
}

.hero {
  display: flex;
  align-items: center;
  gap: 2.5rem;
  flex-wrap: wrap;
  justify-content: center;
}

.orb {
  position: relative;
  width: 220px;
  height: 220px;
  display: grid;
  place-items: center;
}

.blob {
  position: absolute;
  border-radius: 50%;
  filter: blur(2px);
  background: var(--blob-a);
}

.b1 {
  width: 180px;
  height: 180px;
  transform: translate(-18px, 12px);
}

.b2 {
  width: 140px;
  height: 140px;
  background: var(--blob-b);
  transform: translate(28px, -16px);
}

.orb-core {
  position: relative;
  z-index: 1;
  width: 112px;
  height: 112px;
  border-radius: 50%;
  background: #fff;
  box-shadow: 0 10px 30px rgba(59, 130, 246, 0.18);
  display: grid;
  place-items: center;
  color: var(--accent);
}

.orb-core svg {
  width: 2.4rem;
  height: 2.4rem;
  fill: none;
  stroke: currentColor;
  stroke-width: 1.7;
  stroke-linecap: round;
  stroke-linejoin: round;
}

.status-block {
  min-width: 240px;
}

.status-line {
  display: flex;
  align-items: center;
  gap: 0.55rem;
  font-size: 1.35rem;
}

.dot {
  width: 0.9rem;
  height: 0.9rem;
  border-radius: 50%;
  background: #cbd5e1;
  box-shadow: inset 0 0 0 2px #fff;
}

.dot.on {
  background: var(--ok);
}

.meta {
  margin-top: 0.55rem;
  color: var(--accent);
  font-size: 0.95rem;
  display: flex;
  align-items: center;
  gap: 0.35rem;
  flex-wrap: wrap;
}

.stop-btn {
  margin-top: 1.4rem;
  min-width: 220px;
  font-size: 1rem;
}

.err {
  margin: 0.75rem 0 0;
  color: var(--danger);
  font-size: 0.85rem;
}
</style>
