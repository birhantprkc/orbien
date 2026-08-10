<script setup lang="ts">
import {useLocale} from '@/composables/useLocale'

withDefaults(
    defineProps<{

      empty?: boolean
    }>(),
    {empty: true},
)

const {t} = useLocale()
</script>

<template>
  <span v-if="empty" class="empty-text">{{ t('common.notConfigured') }}</span>
  <span v-else class="filled-text"><slot/></span>
</template>

<style scoped>
.empty-text {
  color: var(--muted);
  font-style: normal;
  font-weight: 500;
  letter-spacing: 0.02em;
}

.filled-text {
  color: var(--text);
  font-family: 'IBM Plex Mono', ui-monospace, monospace;
  font-size: 0.92rem;
  word-break: break-all;
}
</style>
