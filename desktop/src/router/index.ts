import {createRouter, createWebHashHistory} from "vue-router";
import Launch from "@/views/Launch.vue";
import Proxy from "@/views/Proxy.vue";
import Config from "@/views/Config.vue";
import Logger from "@/views/Logger.vue";

const router = createRouter({

    history: createWebHashHistory(),
    routes: [
        {path: "/", redirect: "/launch"},
        {path: "/launch", name: "launch", component: Launch},
        {path: "/proxy", name: "proxy", component: Proxy},
        {path: "/config", name: "config", component: Config},
        {path: "/logger", name: "logger", component: Logger},
    ],
});

export default router;
