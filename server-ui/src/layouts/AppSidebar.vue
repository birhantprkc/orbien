<script setup lang="ts">
import { RouterLink, useRoute } from 'vue-router'
import { NAV_ITEMS } from '@/constants/menus'
import { useLocale } from '@/composables/useLocale'
import { useSidebar } from '@/composables/useSidebar'

const { t } = useLocale()
const route = useRoute()
const { collapsed, mobileOpen, isMobile, desktopCollapsed, toggleCollapsed, closeMobile } =
  useSidebar()

function isActive(path: string) {
  if (path === '/') return route.path === '/' || route.path === ''
  return route.path === path || route.path.startsWith(`${path}/`)
}

function onNavigate() {
  if (isMobile.value) closeMobile()
}
</script>

<template>
  <div
    v-if="isMobile && mobileOpen"
    class="sidebar-backdrop"
    aria-hidden="true"
    @click="closeMobile"
  />
  <aside
    class="sidebar"
    :class="{
      'is-collapsed': desktopCollapsed,
      'is-mobile-open': isMobile && mobileOpen,
      'is-mobile': isMobile,
    }"
    :aria-label="t('nav.menu')"
  >
    <nav class="sidebar-nav">
      <RouterLink
        v-for="item in NAV_ITEMS"
        :key="item.name"
        :to="item.path"
        class="side-link"
        :class="{ active: isActive(item.path) }"
        :title="t(`nav.${item.labelKey}`)"
        @click="onNavigate"
      >
        <span class="side-icon" aria-hidden="true">
          <svg v-if="item.icon === 'monitor'" viewBox="0 0 24 24">
            <rect x="3" y="4" width="18" height="12" rx="2" />
            <path d="M8 20h8M12 16v4" />
          </svg>
          <svg v-else-if="item.icon === 'proxies'" viewBox="0 0 24 24">
            <path d="M10 13a5 5 0 0 0 7.5.5l2-2a5 5 0 0 0-7-7l-1.2 1.2" />
            <path d="M14 11a5 5 0 0 0-7.5-.5l-2 2a5 5 0 0 0 7 7l1.2-1.2" />
          </svg>
          <svg v-else viewBox="0 0 24 24">
            <circle cx="12" cy="8" r="3.2" />
            <path d="M5 19c1.8-3.2 4.2-4.8 7-4.8s5.2 1.6 7 4.8" />
          </svg>
        </span>
        <span v-show="!desktopCollapsed" class="side-label">{{ t(`nav.${item.labelKey}`) }}</span>
      </RouterLink>
    </nav>

    <button
      v-if="!isMobile"
      type="button"
      class="sidebar-collapse"
      :aria-label="collapsed ? t('actions.expandSidebar') : t('actions.collapseSidebar')"
      :title="collapsed ? t('actions.expandSidebar') : t('actions.collapseSidebar')"
      @click="toggleCollapsed"
    >
      <svg viewBox="0 0 24 24" aria-hidden="true">
        <path v-if="collapsed" d="M9 6l6 6-6 6" />
        <path v-else d="M15 6l-6 6 6 6" />
      </svg>
    </button>
  </aside>
</template>
