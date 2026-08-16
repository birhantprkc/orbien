import {createRouter, createWebHashHistory} from 'vue-router'
import Monitor from '@/views/Monitor.vue'
import Tunnels from '@/views/Tunnels.vue'
import TunnelDetail from '@/views/TunnelDetail.vue'
import Clients from '@/views/Clients.vue'
import ClientDetail from '@/views/ClientDetail.vue'

export const router = createRouter({
    history: createWebHashHistory(import.meta.env.BASE_URL),
    routes: [
        {path: '/', name: 'monitor', component: Monitor},
        {path: '/tunnels', name: 'tunnels', component: Tunnels},
        {path: '/tunnels/:name', name: 'tunnel-detail', component: TunnelDetail},
        {path: '/clients', name: 'clients', component: Clients},
        {path: '/clients/:sessionId', name: 'client-detail', component: ClientDetail},
    ],
})
