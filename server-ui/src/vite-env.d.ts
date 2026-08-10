/// <reference types="vite/client" />

import type {MessageSchema} from './i18n/schema'
import type {AppLocale} from './i18n/locales'

declare module 'vue-i18n' {
    export interface DefineLocaleMessage extends MessageSchema {
    }
}

declare module '@vue/runtime-core' {
    interface ComponentCustomProperties {
        $t: (key: string, ...args: unknown[]) => string
        $i18n: { locale: AppLocale }
    }
}

export {}
