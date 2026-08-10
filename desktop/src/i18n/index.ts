import {createI18n} from "vue-i18n";
import {
    DEFAULT_LOCALE,
    isAppLocale,
    LOCALE_META,
    type AppLocale,
} from "./locales";
import type {MessageSchema} from "./schema";
import enUS from "./messages/en-US";
import zhCN from "./messages/zh-CN";

const STORAGE_KEY = "orbien-desktop-locale";

function detectLocale(): AppLocale {
    try {
        const saved = localStorage.getItem(STORAGE_KEY);
        if (saved && isAppLocale(saved)) return saved;
    } catch {

    }
    const nav = (navigator.language || "").toLowerCase();
    if (nav.startsWith("zh")) return "zh-CN";
    if (nav.startsWith("en")) return "en-US";
    return DEFAULT_LOCALE;
}

const initialLocale = detectLocale();

export const i18n = createI18n<[MessageSchema], AppLocale>({
    legacy: false,
    locale: initialLocale,
    fallbackLocale: "en-US",
    messages: {
        "zh-CN": zhCN,
        "en-US": enUS,
    },
});

export function applyDocumentLocale(locale: AppLocale) {
    document.documentElement.lang = LOCALE_META[locale].htmlLang;
    try {
        localStorage.setItem(STORAGE_KEY, locale);
    } catch {

    }
}

applyDocumentLocale(initialLocale);

export function setLocale(locale: AppLocale) {
    const current = i18n.global.locale as unknown as AppLocale | { value: AppLocale };
    if (typeof current === "object" && current !== null && "value" in current) {
        current.value = locale;
    } else {
        (i18n.global as unknown as { locale: AppLocale }).locale = locale;
    }
    applyDocumentLocale(locale);
}

export {LOCALE_META, SUPPORTED_LOCALES, DEFAULT_LOCALE, isAppLocale} from "./locales";
export type {AppLocale} from "./locales";
export type {MessageSchema} from "./schema";
