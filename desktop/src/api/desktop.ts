import {invoke as tauriInvoke} from "@tauri-apps/api/core";
import type {ClientConfig, ProxyConfig, ProxyItem} from "./types";

export type {
    ClientConfig,
    ProxyConfig,
    ProxyItem,
    QuicConfig,
    TlsConfig,
} from "./types";
export {
    defaultClientForm,
    defaultProxyForm,
    defaultProxyPlugin,
    defaultProxyTransport,
    defaultQuicConfig,
    defaultTlsConfig,
    joinList,
    normalizeProxyFromServer,
    splitList,
} from "./types";

export interface ClientStatus {
    running: boolean;
    runningSecs: number;
    version: string;
}

export interface RuntimeStats {
    cpuPercent: number;
    memoryMb: number;
    version: string;
}

export function isTauriRuntime(): boolean {
    return (
        typeof window !== "undefined" &&
        !!(window as unknown as { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__
    );
}

async function waitForTauri(timeoutMs = 3000): Promise<boolean> {
    if (isTauriRuntime()) return true;
    const start = Date.now();
    while (Date.now() - start < timeoutMs) {
        await new Promise((r) => setTimeout(r, 50));
        if (isTauriRuntime()) return true;
    }
    return false;
}

async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
    const ok = await waitForTauri();
    if (!ok) {
        throw new Error(
            "Tauri IPC 不可用：请使用桌面窗口（npm run tauri dev），不要直接打开浏览器里的 http://localhost:1420",
        );
    }
    return tauriInvoke<T>(cmd, args);
}

export function getStatus() {
    return invoke<ClientStatus>("get_status");
}

export function getConfig() {
    return invoke<ClientConfig>("get_config");
}

export interface SaveConfigResult {
    config: ClientConfig;
    restarted: boolean;
}

export interface SaveProxiesResult {
    proxies: ProxyItem[];
    restarted: boolean;
}

export function saveClientConfig(config: ClientConfig) {
    return invoke<SaveConfigResult>("save_client_config", {config});
}

export function startClient() {
    return invoke<ClientStatus>("start_client");
}

export function stopClient() {
    return invoke<ClientStatus>("stop_client");
}

export interface LogsSnapshot {
    rev: number;
    lines: string[] | null;
}

export function getLogs(sinceRev = 0) {
    return invoke<LogsSnapshot>("get_logs", {sinceRev});
}

export function clearLogs() {
    return invoke<void>("clear_logs");
}

export function getRuntimeStats() {
    return invoke<RuntimeStats>("get_runtime_stats");
}

export function listProxies() {
    return invoke<ProxyItem[]>("list_proxies");
}

export function saveProxies(proxies: ProxyConfig[]) {
    return invoke<SaveProxiesResult>("save_proxies", {proxies});
}

export interface FileFilter {
    name: string;
    extensions: string[];
}

export interface PickFileOptions {
    title?: string;
    filters?: FileFilter[];
}

export function pickFile(opts?: PickFileOptions) {
    return invoke<string | null>("pick_file", {
        title: opts?.title,
        filters: opts?.filters,
    });
}
