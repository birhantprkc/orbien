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
    listen: string
    quicPort: number
    kcpPort: number
    httpGwPort: number
    httpsGwPort: number
    rootDomain: string
    tcpMux: boolean
    tlsForce: boolean
    maxConnPool: number
    heartbeatTimeout: number
}

export interface SystemStatus {
    clientCounts: number
    totalClientCounts: number
    tunnelTypeCount: Record<string, number>
    activeConns: number
    totalTrafficIn: number
    totalTrafficOut: number
}

export interface ClientInfo {
    sessionId: string
    user: string
    hostname: string
    os: string
    arch: string
    clientIP: string
    version: string
    tunnelCount: number
    activeConns: number
    connectedSecs: number
    status: string
}

export interface TunnelInfo {
    name: string
    type: string
    remoteAddr: string
    localAddr: string
    sessionId: string
    status: string
    todayTrafficIn: number
    todayTrafficOut: number
    activeConns: number
    lastStartTime?: string
}

export interface TunnelTrafficPoint {
    date: string
    trafficIn: number
    trafficOut: number
}

export interface TunnelTrafficResp {
    name: string
    unit: 'bytes' | string
    granularity: 'day' | string
    history: TunnelTrafficPoint[]
}
