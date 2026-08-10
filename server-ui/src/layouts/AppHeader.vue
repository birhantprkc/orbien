<script setup lang="ts">
import ThemeToggle from '@/components/ThemeToggle.vue'
import LocaleSwitcher from '@/components/LocaleSwitcher.vue'
import {useLocale} from '@/composables/useLocale'
import {useSidebar} from '@/composables/useSidebar'
import logoUrl from '@/assets/images/logo.png'

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
      <LocaleSwitcher/>
      <ThemeToggle/>
    </div>
  </header>
</template>
