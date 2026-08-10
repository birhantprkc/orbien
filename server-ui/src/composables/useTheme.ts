import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { applyTheme, resolveTheme, toggleTheme, type ThemeMode } from '@/utils/theme'
import type { AppLocale, MessageSchema } from '@/i18n'

function readDomTheme(): ThemeMode {
  const d = document.documentElement.dataset.theme
  return d === 'light' || d === 'dark' ? d : resolveTheme()
}

const mode = ref<ThemeMode>(readDomTheme())

export function useTheme() {
  const { t } = useI18n<{ message: MessageSchema }, AppLocale>()
  const isDark = computed(() => mode.value === 'dark')
  const label = computed(() =>
    mode.value === 'dark' ? t('actions.themeToLight') : t('actions.themeToDark'),
  )
  const hint = computed(() =>
    mode.value === 'dark' ? t('actions.themeLightHint') : t('actions.themeDarkHint'),
  )

  function setTheme(next: ThemeMode) {
    applyTheme(next)
    mode.value = next
  }

  function toggle() {
    mode.value = toggleTheme(mode.value)
  }

  return { mode, isDark, label, hint, setTheme, toggle }
}
