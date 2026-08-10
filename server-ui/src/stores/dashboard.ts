import {reactive} from 'vue'
import {fetchClients, fetchProxies, fetchSystemInfo, isApiError, type ApiError} from '@/api'
import type {ClientInfo, ProxyInfo, SystemInfo} from '@/types/api'

export type DashboardError =
    | { code: ApiError['code']; params?: Record<string, unknown> }
    | null

const state = reactive({
    info: null as SystemInfo | null,
    clients: [] as ClientInfo[],
    proxies: [] as ProxyInfo[],
    loading: false,
    error: null as DashboardError,
})

export function useDashboardStore() {
    async function refresh() {
        state.loading = true
        state.error = null
        try {
            const [sys, cli, prox] = await Promise.all([
                fetchSystemInfo(),
                fetchClients(),
                fetchProxies(),
            ])
            state.info = sys
            state.clients = cli.items ?? []
            state.proxies = prox.items ?? []
        } catch (e) {
            if (isApiError(e)) {
                state.error = {code: e.code, params: e.params}
            } else {
                state.error = {code: 'unknown'}
            }
        } finally {
            state.loading = false
        }
    }

    return {
        get info() {
            return state.info
        },
        get clients() {
            return state.clients
        },
        get proxies() {
            return state.proxies
        },
        get loading() {
            return state.loading
        },
        get error() {
            return state.error
        },
        refresh,
    }
}
