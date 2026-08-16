export interface MessageSchema {
    nav: {
        menu: string
        monitor: string
        tunnels: string
        clients: string
    }
    actions: {
        themeToLight: string
        themeToDark: string
        locale: string
        github: string
        collapseSidebar: string
        expandSidebar: string
        openMenu: string
        closeMenu: string
    }
    common: {
        notConfigured: string
        enabled: string
        disabled: string
        total: string
        perPage: string
        pagination: string
        prevPage: string
        nextPage: string
    }
    overview: {
        totalClients: string
        onlineClients: string
        tunnels: string
        connections: string
        emptyConfig: string
    }
    monitor: {
        listen: string
        tunnelTypes: string
        tunnelDist: string
        serverConfig: string
        chartTotal: string
        quicPort: string
        kcpPort: string
        tcpMux: string
        tlsForce: string
        httpGwPort: string
        httpsGwPort: string
        rootDomain: string
        maxConnPool: string
        heartbeatTimeout: string
        version: string
    }
    clients: {
        hostname: string
        ip: string
        osFamily: {
            windows: string
            macos: string
            linux: string
            android: string
            freebsd: string
            other: string
        }
        tunnels: string
        connected: string
        disconnected: string
        connections: string
        empty: string
        filter: string
        filterAll: string
        filterEmpty: string
        uptimeSecs: string
        uptimeMins: string
        uptimeHours: string
        uptimeDays: string
        agoSecs: string
        agoMins: string
        agoHours: string
        agoDays: string
        kick: string
        kickConfirm: string
        kickFailed: string
        back: string
        detail: string
        notFound: string
        notFoundDesc: string
        searchTunnels: string
        tunnelsEmpty: string
        tunnelsSearchEmpty: string
    }
    tunnels: {
        port: string
        domain: string
        localAddr: string
        client: string
        empty: string
        traffic: string
        activeConns: string
        filter: string
        filterAll: string
        filterEmpty: string
        back: string
        lastStarted: string
        openClient: string
    }
    traffic: {
        in: string
        out: string
        total: string
        today: string
        network: string
        history: string
        historyAll: string
        range: string
        range24h: string
        range7d: string
        chartType: string
        chartLine: string
        chartBar: string
        loading: string
        failed: string
        empty: string
    }
    status: {
        online: string
        offline: string
    }
    errors: {
        unauthorized: string
        http: string
        api: string
        unknown: string
    }
}
