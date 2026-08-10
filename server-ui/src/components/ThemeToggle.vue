<script setup lang="ts">
import {useTheme} from '@/composables/useTheme'

const {isDark, label, toggle} = useTheme()
</script>

<template>
  <button
      type="button"
      class="theme-toggle"
      :aria-label="label"
      :title="label"
      @click="toggle"
  >
    <svg v-if="isDark" viewBox="0 0 24 24" aria-hidden="true">
      <circle cx="12" cy="12" r="4"/>
      <path
          d="M12 2v2M12 20v2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M2 12h2M20 12h2M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4"
      />
    </svg>
    <svg v-else viewBox="0 0 24 24" aria-hidden="true">
      <path d="M21 14.5A8.5 8.5 0 0 1 9.5 3 7 7 0 1 0 21 14.5z"/>
    </svg>
  </button>
</template>
