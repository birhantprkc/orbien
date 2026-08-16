import {reactive} from 'vue'
import {fetchClients, fetchTunnels, fetchSystemInfo, isApiError, type ApiError} from '@/api'
import type {ClientInfo, TunnelInfo, SystemInfo} from '@/types/api'

export type DashboardError =
    | { code: ApiError['code']; params?: Record<string, unknown> }
    | null

const state = reactive({
    info: null as SystemInfo | null,
    clients: [] as ClientInfo[],
    tunnels: [] as TunnelInfo[],
    error: null as DashboardError,
})

export function useDashboardStore() {
    async function refresh() {
        state.error = null
        try {
            const [sys, cli, tun] = await Promise.all([
                fetchSystemInfo(),
                fetchClients(),
                fetchTunnels(),
            ])
            state.info = sys
            state.clients = cli.items ?? []
            state.tunnels = tun.items ?? []
        } catch (e) {
            if (isApiError(e)) {
                state.error = {code: e.code, params: e.params}
            } else {
                state.error = {code: 'unknown'}
            }
        }
    }

    return {
        get info() {
            return state.info
        },
        get clients() {
            return state.clients
        },
        get tunnels() {
            return state.tunnels
        },
        get error() {
            return state.error
        },
        refresh,
    }
}
