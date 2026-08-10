<script lang="ts">
export type AppIconName =
    | 'globe'
    | 'server'
    | 'monitor'
    | 'link'
    | 'bolt'
    | 'signal'
    | 'shield'
    | 'mux'
    | 'users'
    | 'user'
    | 'proxies'
    | 'tag'
    | 'waves'
    | 'arrow-down'
    | 'arrow-up'
    | 'kick'
</script>

<script setup lang="ts">
defineProps<{
  name: AppIconName
}>()
</script>

<template>
  <svg viewBox="0 0 24 24" aria-hidden="true">
    <template v-if="name === 'globe'">
      <circle cx="12" cy="12" r="9"/>
      <path d="M3 12h18M12 3a14 14 0 0 1 0 18M12 3a14 14 0 0 0 0 18"/>
    </template>
    <template v-else-if="name === 'server'">
      <rect x="4" y="4" width="16" height="6" rx="1.5"/>
      <rect x="4" y="14" width="16" height="6" rx="1.5"/>
      <path d="M8 7h.01M8 17h.01"/>
    </template>
    <template v-else-if="name === 'monitor'">
      <rect x="3" y="4" width="18" height="12" rx="2"/>
      <path d="M8 20h8M12 16v4"/>
    </template>
    <template v-else-if="name === 'link'">
      <path d="M10 13a5 5 0 0 0 7.5.5l2-2a5 5 0 0 0-7-7l-1.2 1.2"/>
      <path d="M14 11a5 5 0 0 0-7.5-.5l-2 2a5 5 0 0 0 7 7l1.2-1.2"/>
    </template>
    <template v-else-if="name === 'bolt'">
      <path d="M13 3 5 13h6l-1 8 9-12h-6l1-6z"/>
    </template>
    <template v-else-if="name === 'signal'">
      <path d="M5 18v-2M9 18v-5M13 18V8M17 18V5"/>
    </template>
    <template v-else-if="name === 'shield'">
      <path d="M12 3 5 6v5c0 5 3.2 8.4 7 9.5 3.8-1.1 7-4.5 7-9.5V6l-7-3z"/>
    </template>
    <template v-else-if="name === 'users'">
      <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/>
      <circle cx="9" cy="7" r="3.2"/>
      <path d="M22 21v-2a4 4 0 0 0-3-3.87M16 3.13a4 4 0 0 1 0 7.75"/>
    </template>
    <template v-else-if="name === 'user'">
      <circle cx="12" cy="8" r="3.2"/>
      <path d="M5 19c1.8-3.2 4.2-4.8 7-4.8s5.2 1.6 7 4.8"/>
    </template>
    <template v-else-if="name === 'proxies'">
      <circle cx="6" cy="12" r="2.5"/>
      <circle cx="18" cy="7" r="2.5"/>
      <circle cx="18" cy="17" r="2.5"/>
      <path d="M8.5 11.2 15.5 8.2M8.5 12.8l7 3.8"/>
    </template>
    <template v-else-if="name === 'tag'">
      <path d="M20 12 12 4H5a1 1 0 0 0-1 1v7l8 8 8-8z"/>
      <circle cx="8.5" cy="8.5" r="1.2"/>
    </template>
    <template v-else-if="name === 'waves'">
      <path d="M2 8c2.5 2.5 5.5 2.5 8 0s5.5-2.5 8 0 5.5 2.5 8 0"/>
      <path d="M2 14c2.5 2.5 5.5 2.5 8 0s5.5-2.5 8 0 5.5 2.5 8 0"/>
      <path d="M2 20c2.5 2.5 5.5 2.5 8 0s5.5-2.5 8 0 5.5 2.5 8 0"/>
    </template>
    <template v-else-if="name === 'arrow-down'">
      <path d="M12 3v12"/>
      <path d="m7 10 5 5 5-5"/>
      <path d="M5 19h14"/>
    </template>
    <template v-else-if="name === 'arrow-up'">
      <path d="M12 21V9"/>
      <path d="m7 14 5-5 5 5"/>
      <path d="M5 5h14"/>
    </template>
    <template v-else-if="name === 'kick'">
      <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/>
      <path d="M16 17l5-5-5-5"/>
      <path d="M21 12H9"/>
    </template>
    <template v-else>
      <rect x="4" y="4" width="16" height="16" rx="3"/>
      <path d="M8 12h8M12 8v8"/>
    </template>
  </svg>
</template>

<style scoped>
svg {
  width: 1em;
  height: 1em;
  fill: none;
  stroke: currentColor;
  stroke-width: 1.7;
  stroke-linecap: round;
  stroke-linejoin: round;
  display: block;
}
</style>
