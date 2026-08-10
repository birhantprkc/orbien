<script setup lang="ts">
import {computed} from "vue";
import {RouterLink, useRoute} from "vue-router";
import {useI18n} from "vue-i18n";
import logoUrl from "@/assets/logo.png";
import {
  LOCALE_META,
  setLocale,
  SUPPORTED_LOCALES,
  type AppLocale,
  type MessageSchema,
} from "@/i18n";

const route = useRoute();
const {t, locale} = useI18n<{ message: MessageSchema }, AppLocale>();

const items = computed(() => [
  {to: "/launch", label: t("nav.launch"), icon: "rocket" as const},
  {to: "/proxy", label: t("nav.proxy"), icon: "cloud" as const},
  {to: "/config", label: t("nav.config"), icon: "gear" as const},
  {to: "/logger", label: t("nav.logger"), icon: "doc" as const},
]);

function onLocaleChange(e: Event) {
  const value = (e.target as HTMLSelectElement).value as AppLocale;
  setLocale(value);
}
</script>

<template>
  <aside class="sidebar">
    <div class="brand">
      <img class="logo" :src="logoUrl" :alt="t('app.brand')"/>
      <div class="brand-name" :aria-label="t('app.brand')">
        <span class="brand-base">Orbi</span><span class="brand-accent">en</span>
      </div>
    </div>

    <nav class="nav">
      <RouterLink
          v-for="item in items"
          :key="item.to"
          :to="item.to"
          class="nav-item"
          :class="{ active: route.path === item.to }"
          :title="item.label"
      >
        <svg v-if="item.icon === 'rocket'" viewBox="0 0 24 24" aria-hidden="true">
          <path d="M12 3c4 2 6 6 6 10l-3 1-2 4-2-4-3-1c0-4 2-8 6-10z"/>
          <path d="M9 14l-3 5M15 14l3 5"/>
        </svg>
        <svg v-else-if="item.icon === 'cloud'" viewBox="0 0 24 24" aria-hidden="true">
          <path d="M7 18h10a4 4 0 0 0 .3-8 5.5 5.5 0 0 0-10.6 1.5A3.5 3.5 0 0 0 7 18z"/>
        </svg>
        <svg v-else-if="item.icon === 'gear'" viewBox="0 0 24 24" aria-hidden="true">
          <circle cx="12" cy="12" r="3"/>
          <path
              d="M19.4 15a1.7 1.7 0 0 0 .3 1.8l.1.1a2 2 0 1 1-2.8 2.8l-.1-.1a1.7 1.7 0 0 0-1.8-.3 1.7 1.7 0 0 0-1 1.5V21a2 2 0 1 1-4 0v-.1a1.7 1.7 0 0 0-1-1.5 1.7 1.7 0 0 0-1.8.3l-.1.1a2 2 0 1 1-2.8-2.8l.1-.1a1.7 1.7 0 0 0 .3-1.8 1.7 1.7 0 0 0-1.5-1H3a2 2 0 1 1 0-4h.1a1.7 1.7 0 0 0 1.5-1 1.7 1.7 0 0 0-.3-1.8l-.1-.1a2 2 0 1 1 2.8-2.8l.1.1a1.7 1.7 0 0 0 1.8.3H9a1.7 1.7 0 0 0 1-1.5V3a2 2 0 1 1 4 0v.1a1.7 1.7 0 0 0 1 1.5 1.7 1.7 0 0 0 1.8-.3l.1-.1a2 2 0 1 1 2.8 2.8l-.1.1a1.7 1.7 0 0 0-.3 1.8V9c.3.6.9 1 1.5 1H21a2 2 0 1 1 0 4h-.1a1.7 1.7 0 0 0-1.5 1z"
          />
        </svg>
        <svg v-else viewBox="0 0 24 24" aria-hidden="true">
          <path d="M7 3h7l3 3v15H7z"/>
          <path d="M14 3v4h4M9 12h6M9 16h6"/>
        </svg>
      </RouterLink>
    </nav>

    <div class="footer">
      <label class="locale">
        <span class="sr-only">{{ t("locale.label") }}</span>
        <select
            class="locale-select"
            :value="locale"
            :aria-label="t('locale.label')"
            @change="onLocaleChange"
        >
          <option v-for="code in SUPPORTED_LOCALES" :key="code" :value="code">
            {{ LOCALE_META[code].nativeLabel }}
          </option>
        </select>
      </label>
    </div>
  </aside>
</template>

<style scoped>
.sidebar {
  display: flex;
  flex-direction: column;
  align-items: center;
  background: var(--chrome);
  border-right: 1px solid var(--line);
  padding: 0.85rem 0.45rem 0.75rem;
  height: 100vh;
}

.brand {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.35rem;
  margin-bottom: 1rem;
}

.logo {
  width: 34px;
  height: 34px;
  object-fit: contain;
  display: block;
}

.brand-name {
  font-size: 0.68rem;
  font-weight: 700;
  letter-spacing: 0.02em;
  line-height: 1;
}

.brand-base {
  color: var(--text);
}

.brand-accent {
  background: var(--brand-grad);
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
  -webkit-text-fill-color: transparent;
}

.nav {
  display: flex;
  flex-direction: column;
  gap: 0.45rem;
  flex: 1;
  width: 100%;
  align-items: center;
}

.nav-item {
  width: 44px;
  height: 44px;
  border-radius: var(--radius);
  display: grid;
  place-items: center;
  color: var(--nav-idle);
  text-decoration: none;
}

.nav-item svg {
  width: 1.35rem;
  height: 1.35rem;
  fill: none;
  stroke: currentColor;
  stroke-width: 1.7;
  stroke-linecap: round;
  stroke-linejoin: round;
}

.nav-item:hover {
  background: var(--accent-soft);
  color: var(--accent);
}

.nav-item.active {
  background: var(--accent-soft);
  color: var(--accent);
}

.footer {
  width: 100%;
  padding-top: 0.5rem;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.locale {
  width: 100%;
  display: flex;
  justify-content: center;
}

.locale-select {
  appearance: none;
  box-sizing: border-box;
  width: 3.4rem;
  border: 1px solid var(--line);
  background: var(--panel);
  color: var(--text-secondary);
  border-radius: var(--radius);
  padding: 0.2rem 0.15rem;
  font-size: 0.68rem;
  font-weight: 600;
  text-align: center;
  cursor: pointer;
}

.locale-select:hover,
.locale-select:focus {
  border-color: var(--accent-muted);
  outline: none;
}

.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}
</style>
