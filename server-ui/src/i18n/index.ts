import {createI18n} from 'vue-i18n'
import {
    DEFAULT_LOCALE,
    isAppLocale,
    LOCALE_META,
    type AppLocale,
} from './locales'
import type {MessageSchema} from './schema'
import enUS from './messages/en-US'
import zhCN from './messages/zh-CN'

const STORAGE_KEY = 'orbien-server-ui-locale'

const messages = {
    'en-US': enUS,
    'zh-CN': zhCN,
} satisfies Record<AppLocale, MessageSchema>

function detectBrowserLocale(): AppLocale {
    const langs = navigator.languages?.length
        ? navigator.languages
        : [navigator.language]
    for (const raw of langs) {
        if (isAppLocale(raw)) return raw
        const short = raw.toLowerCase()
        if (short.startsWith('zh')) return 'zh-CN'
        if (short.startsWith('en')) return 'en-US'
    }
    return DEFAULT_LOCALE
}

export function resolveLocale(preferred?: string | null): AppLocale {
    if (preferred && isAppLocale(preferred)) return preferred
    const stored = localStorage.getItem(STORAGE_KEY)
    if (stored && isAppLocale(stored)) return stored
    return detectBrowserLocale()
}

export function applyDocumentLocale(locale: AppLocale) {
    document.documentElement.lang = LOCALE_META[locale].htmlLang
    localStorage.setItem(STORAGE_KEY, locale)
}

const initialLocale = resolveLocale()

export const i18n = createI18n({
    legacy: false,
    locale: initialLocale,
    fallbackLocale: 'en-US' satisfies AppLocale,
    messages,
})

applyDocumentLocale(initialLocale)

export function setLocale(locale: AppLocale) {
    i18n.global.locale.value = locale
    applyDocumentLocale(locale)
}

export type {AppLocale, MessageSchema}
export {LOCALE_META, SUPPORTED_LOCALES, DEFAULT_LOCALE, isAppLocale} from './locales'
