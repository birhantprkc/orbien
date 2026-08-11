<script setup lang="ts">
import ThemeToggle from '@/components/ThemeToggle.vue'
import LocaleSwitcher from '@/components/LocaleSwitcher.vue'
import {useLocale} from '@/composables/useLocale'
import {useSidebar} from '@/composables/useSidebar'
import logoUrl from '@/assets/images/logo.png'
import githubIcon from '@/assets/icon/github.svg?raw'

const GITHUB_URL = 'https://github.com/orbien-org/orbien'

const {t} = useLocale()
const {isMobile, mobileOpen, toggleCollapsed} = useSidebar()
</script>

<template>
  <header class="top">
    <div class="top-left">
      <button
          v-if="isMobile"
          type="button"
          class="icon-btn menu-btn"
          :aria-label="mobileOpen ? t('actions.closeMenu') : t('actions.openMenu')"
          :aria-expanded="mobileOpen"
          @click="toggleCollapsed"
      >
        <svg viewBox="0 0 24 24" aria-hidden="true">
          <path v-if="mobileOpen" d="M6 6l12 12M18 6L6 18"/>
          <path v-else d="M4 7h16M4 12h16M4 17h16"/>
        </svg>
      </button>

      <div class="brand-block">
        <img class="logo-img" :src="logoUrl" alt="Orbien"/>
        <div class="brand-title" aria-label="Orbien">
          <span class="brand-orb">Orb</span><span class="brand-rest">ien</span>
        </div>
      </div>
    </div>

    <div class="actions">
      <a
          class="icon-btn github-link"
          :href="GITHUB_URL"
          target="_blank"
          rel="noopener noreferrer"
          :aria-label="t('actions.github')"
          :title="t('actions.github')"
      >
        <span class="github-icon" aria-hidden="true" v-html="githubIcon"/>
      </a>
      <LocaleSwitcher/>
      <ThemeToggle/>
    </div>
  </header>
</template>

<style scoped>
.github-link {
  text-decoration: none;
  color: var(--text);
}

.github-icon {
  display: inline-grid;
  place-items: center;
  width: 1.15rem;
  height: 1.15rem;
  line-height: 0;
}

.github-icon :deep(svg) {
  width: 100%;
  height: 100%;
  display: block;
  fill: currentColor;
  stroke: none;
}
</style>
