<script setup lang="ts">
import IconBadge, {type IconTone} from '@/components/IconBadge.vue'
import type {AppIconName} from '@/components/AppIcon.vue'

withDefaults(
    defineProps<{
      label: string
      hint?: string
      icon?: AppIconName
      tone?: IconTone
    }>(),
    {hint: '', tone: 'blue'},
)
</script>

<template>
  <div class="stat-card card">
    <div class="stat-top">
      <div class="stat-copy">
        <div class="k">{{ label }}</div>
        <div class="v">
          <slot/>
        </div>
      </div>
      <IconBadge v-if="icon" :name="icon" :tone="tone" class="stat-icon"/>
    </div>
    <div v-if="hint || $slots.hint" class="stat-hint">
      <slot name="hint">{{ hint }}</slot>
    </div>
  </div>
</template>

<style scoped>
.stat-card {
  min-height: 5.5rem;
  display: flex;
  flex-direction: column;
}

.stat-top {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 0.75rem;
}

.stat-copy {
  min-width: 0;
  flex: 1;
}

.stat-copy .k {
  margin-bottom: 0.4rem;
}

.stat-copy .v {
  font-variant-numeric: tabular-nums;
  line-height: 1.2;
}

.stat-icon {
  margin-top: 0.05rem;
}

.stat-hint {
  margin-top: 0.45rem;
  color: var(--muted);
  font-size: 0.78rem;
}
</style>
