<script setup lang="ts">
import {computed} from 'vue'
import {formatArch, normalizeOsFamily, type OsFamily} from '@/utils/os'
import {useLocale} from '@/composables/useLocale'
import windowsIcon from '@/assets/icon/windows.svg'
import macosIcon from '@/assets/icon/macos.svg'
import linuxIcon from '@/assets/icon/linux.svg'
import androidIcon from '@/assets/icon/android.svg'
import freebsdIcon from '@/assets/icon/freebsd.svg'
import deviceIcon from '@/assets/icon/device.svg'

const OS_ICONS: Record<OsFamily, string> = {
  windows: windowsIcon,
  macos: macosIcon,
  linux: linuxIcon,
  android: androidIcon,
  freebsd: freebsdIcon,
  other: deviceIcon,
}

const props = withDefaults(
    defineProps<{
      os?: string | null
      arch?: string | null
      size?: 'sm' | 'md'
      showArch?: boolean
      iconOnly?: boolean
      textOnly?: boolean
    }>(),
    {
      os: '',
      arch: '',
      size: 'sm',
      showArch: true,
      iconOnly: false,
      textOnly: false,
    },
)

const {t} = useLocale()

const family = computed(() => normalizeOsFamily(props.os))
const archLabel = computed(() => formatArch(props.arch))
const iconSrc = computed(() => OS_ICONS[family.value])
const isDevice = computed(() => family.value === 'other')

const label = computed(() => {
  switch (family.value) {
    case 'windows':
      return t('clients.osFamily.windows')
    case 'macos':
      return t('clients.osFamily.macos')
    case 'linux':
      return t('clients.osFamily.linux')
    case 'android':
      return t('clients.osFamily.android')
    case 'freebsd':
      return t('clients.osFamily.freebsd')
    default:
      return t('clients.osFamily.other')
  }
})

const tip = computed(() => {
  const parts = [props.os?.trim(), props.arch?.trim()].filter(Boolean)
  return parts.join(' · ') || label.value
})

const text = computed(() => {
  if (props.showArch && archLabel.value) return `${label.value} · ${archLabel.value}`
  return label.value
})
</script>

<template>
  <span
      class="os-badge"
      :class="[family, size, { 'icon-only': iconOnly, 'text-only': textOnly }]"
      :title="tip"
      :aria-label="tip"
  >
    <template v-if="!textOnly">
      <span
          v-if="isDevice"
          class="os-icon os-icon-mask"
          :style="{ '--os-mask': `url(${iconSrc})` }"
          aria-hidden="true"
      />
      <img
          v-else
          class="os-icon"
          :src="iconSrc"
          alt=""
          draggable="false"
      />
    </template>
    <span v-if="!iconOnly" class="os-label">{{ text }}</span>
  </span>
</template>

<style scoped>
.os-badge {
  display: inline-flex;
  align-items: center;
  gap: 0.4rem;
  max-width: 100%;
  min-width: 0;
  padding: 0;
  border: 0;
  background: transparent;
  font-size: 0.8rem;
  font-weight: 550;
  line-height: 1.25;
  color: var(--muted);
  vertical-align: middle;
}

.os-badge.md {
  font-size: 0.84rem;
  gap: 0.45rem;
}

.os-badge.icon-only {
  gap: 0;
}

.os-icon {
  width: 1.05rem;
  height: 1.05rem;
  flex-shrink: 0;
  display: block;
  object-fit: contain;
  user-select: none;
}

.os-badge.md .os-icon {
  width: 1.2rem;
  height: 1.2rem;
}

.os-icon-mask {
  background-color: currentColor;
  color: var(--muted);
  -webkit-mask: var(--os-mask) center / contain no-repeat;
  mask: var(--os-mask) center / contain no-repeat;
}

.os-label {
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
