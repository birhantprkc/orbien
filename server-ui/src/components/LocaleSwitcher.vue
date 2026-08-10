<script setup lang="ts">
import {useLocale} from '@/composables/useLocale'
import type {AppLocale} from '@/i18n'

const {t, current, options, switchLocale} = useLocale()

function onChange(e: Event) {
  const value = (e.target as HTMLSelectElement).value as AppLocale
  switchLocale(value)
}
</script>

<template>
  <label class="locale-switch">
    <span class="sr-only">{{ t('actions.locale') }}</span>
    <select
        class="locale-select"
        :value="current"
        :aria-label="t('actions.locale')"
        @change="onChange"
    >
      <option v-for="opt in options" :key="opt.code" :value="opt.code">
        {{ opt.nativeLabel }}
      </option>
    </select>
  </label>
</template>
