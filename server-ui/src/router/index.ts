import {createRouter, createWebHashHistory} from 'vue-router'
import Monitor from '@/views/Monitor.vue'
import Proxies from '@/views/Proxies.vue'
import ProxyDetail from '@/views/ProxyDetail.vue'
import Clients from '@/views/Clients.vue'
import ClientDetail from '@/views/ClientDetail.vue'

export const router = createRouter({
    history: createWebHashHistory(import.meta.env.BASE_URL),
    routes: [
        {path: '/', name: 'monitor', component: Monitor},
        {path: '/proxies', name: 'proxies', component: Proxies},
        {path: '/proxies/:name', name: 'proxy-detail', component: ProxyDetail},
        {path: '/clients', name: 'clients', component: Clients},
        {path: '/clients/:runId', name: 'client-detail', component: ClientDetail},
        {path: '/overview', redirect: '/'},
    ],
})
