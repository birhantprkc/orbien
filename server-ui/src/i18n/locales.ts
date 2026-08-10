export const SUPPORTED_LOCALES = ['zh-CN', 'en-US'] as const
export type AppLocale = (typeof SUPPORTED_LOCALES)[number]
export const DEFAULT_LOCALE: AppLocale = 'zh-CN'
export const LOCALE_META: Record<
    AppLocale,
    { label: string; nativeLabel: string; htmlLang: string }
> = {
    'zh-CN': {label: 'Chinese', nativeLabel: '中文', htmlLang: 'zh-CN'},
    'en-US': {label: 'English', nativeLabel: 'English', htmlLang: 'en'},
}

export function isAppLocale(value: string): value is AppLocale {
    return (SUPPORTED_LOCALES as readonly string[]).includes(value)
}
