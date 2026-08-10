export interface NavItem {
    name: 'monitor' | 'proxies' | 'clients'
    path: string
    labelKey: 'monitor' | 'proxies' | 'clients'
    icon: 'monitor' | 'proxies' | 'clients'
}

export const NAV_ITEMS: readonly NavItem[] = [
    {name: 'monitor', path: '/', labelKey: 'monitor', icon: 'monitor'},
    {name: 'proxies', path: '/proxies', labelKey: 'proxies', icon: 'proxies'},
    {name: 'clients', path: '/clients', labelKey: 'clients', icon: 'clients'},
]
