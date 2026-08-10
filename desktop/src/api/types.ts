export interface ProxyTransportOptions {
  bandwidthLimit: string;
  bandwidthLimitMode: string;
  proxyProtocolVersion: string;
}

export interface ProxyPluginConfig {
  type: string;
  localAddr: string;
  crtPath: string;
  keyPath: string;
  hostHeaderRewrite: string;
}

export interface ProxyConfig {
  name: string;
  proxyType: string;
  localIp: string;
  localPort: number;
  remotePort: number;
  customDomains: string[];
  subdomain: string;
  locations: string[];
  httpUser: string;
  httpPassword: string;
  hostHeaderRewrite: string;
  routeByHttpUser: string;
  transport: ProxyTransportOptions;
  plugin?: ProxyPluginConfig | null;
}

export interface ProxyItem {
  name: string;
  proxyType: string;
  local: string;
  remote: string;
  copyValue: string;
}

export interface TlsConfig {
  enable: boolean;
  certFile: string;
  keyFile: string;
  trustedCaFile: string;
  serverName: string;
  disableCustomTlsFirstByte: boolean;
}

export interface QuicConfig {
  keepalivePeriod: number;
  maxIdleTimeout: number;
  maxIncomingStreams: number;
}

export interface ClientConfig {
  serverAddr: string;
  serverPort: number;
  user: string;
  token: string;
  udpPacketSize: number;
  protocol: string;
  poolCount: number;
  tcpMux: boolean;
  tcpMuxKeepaliveInterval: number;
  heartbeatInterval: number;
  heartbeatTimeout: number;
  tls: TlsConfig;
  quic: QuicConfig;
  orbienPath: string;
  proxies: ProxyConfig[];
}

export function defaultTlsConfig(): TlsConfig {
  return {
    enable: true,
    certFile: "",
    keyFile: "",
    trustedCaFile: "",
    serverName: "",
    disableCustomTlsFirstByte: true,
  };
}

export function defaultQuicConfig(): QuicConfig {
  return {
    keepalivePeriod: 10,
    maxIdleTimeout: 30,
    maxIncomingStreams: 100000,
  };
}

export function defaultProxyTransport(): ProxyTransportOptions {
  return {
    bandwidthLimit: "",
    bandwidthLimitMode: "client",
    proxyProtocolVersion: "",
  };
}

export function defaultProxyPlugin(): ProxyPluginConfig {
  return {
    type: "https2http",
    localAddr: "127.0.0.1:80",
    crtPath: "",
    keyPath: "",
    hostHeaderRewrite: "",
  };
}

export function defaultProxyForm(type: string = "tcp"): ProxyConfig {
  const base: ProxyConfig = {
    name: "",
    proxyType: type,
    localIp: "127.0.0.1",
    localPort: type === "https" ? 443 : type === "udp" ? 12001 : 8080,
    remotePort: type === "udp" ? 7001 : 6000,
    customDomains: [],
    subdomain: "",
    locations: [],
    httpUser: "",
    httpPassword: "",
    hostHeaderRewrite: "",
    routeByHttpUser: "",
    transport: defaultProxyTransport(),
    plugin: null,
  };
  if (type === "http") {
    base.localPort = 80;
    base.remotePort = 0;
  }
  if (type === "https") {
    base.remotePort = 0;
  }
  return base;
}

export function defaultClientForm(): Omit<ClientConfig, "proxies"> {
  return {
    serverAddr: "127.0.0.1",
    serverPort: 9527,
    user: "",
    token: "",
    udpPacketSize: 1500,
    protocol: "tcp",
    poolCount: 1,
    tcpMux: true,
    tcpMuxKeepaliveInterval: 30,
    heartbeatInterval: -1,
    heartbeatTimeout: -1,
    tls: defaultTlsConfig(),
    quic: defaultQuicConfig(),
    orbienPath: "",
  };
}

export function splitList(raw: string): string[] {
  return raw
    .split(/[\n,]+/)
    .map((s) => s.trim())
    .filter(Boolean);
}

export function joinList(items: string[] | undefined): string {
  return (items ?? []).join(", ");
}

export function normalizeProxyFromServer(p: ProxyConfig): ProxyConfig {
  return {
    ...defaultProxyForm(p.proxyType || "tcp"),
    ...p,
    customDomains: p.customDomains ?? [],
    locations: p.locations ?? [],
    transport: { ...defaultProxyTransport(), ...(p.transport ?? {}) },
    plugin: p.plugin?.type
      ? { ...defaultProxyPlugin(), ...p.plugin }
      : null,
  };
}
