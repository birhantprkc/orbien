export interface ApiResponse<T> {
    code: number
    msg: string
    data: T
}

export interface Page<T> {
    total: number
    page: number
    pageSize: number
    items: T[]
}

export interface SystemInfo {
    version: string
    config: SystemConfig
    status: SystemStatus
}

export interface SystemConfig {
    bindAddr: string
    bindPort: number
    quicBindPort: number
    kcpBindPort: number
    vhostHTTPPort: number
    vhostHTTPSPort: number
    subDomainHost: string
    tcpMux: boolean
    tlsForce: boolean
    maxPoolCount: number
    heartbeatTimeout: number
}

export interface SystemStatus {
    clientCounts: number
    totalClientCounts: number
    proxyTypeCount: Record<string, number>
    curConns: number
    totalTrafficIn: number
    totalTrafficOut: number
}

export interface ClientInfo {
    runId: string
    user: string
    hostname: string
    clientIP: string
    version: string
    proxyCount: number
    curConns: number
    connectedSecs: number
    status: string
}

export interface ProxyInfo {
    name: string
    type: string
    remoteAddr: string
    localAddr: string
    clientId: string
    status: string
    todayTrafficIn: number
    todayTrafficOut: number
    curConns: number
    lastStartTime?: string
}

export interface ProxyTrafficPoint {
    date: string
    trafficIn: number
    trafficOut: number
}

export interface ProxyTrafficResp {
    name: string
    unit: 'bytes' | string
    granularity: 'day' | string
    history: ProxyTrafficPoint[]
}
