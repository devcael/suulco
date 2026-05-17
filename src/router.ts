import { createRouter, createWebHistory } from "vue-router";
import NorteView from "./views/NorteView.vue";
import HojeView from "./views/HojeView.vue";
import InboxView from "./views/InboxView.vue";
import MemoriaView from "./views/MemoriaView.vue";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/", redirect: "/hoje" },
    { path: "/sulco", component: NorteView },
    { path: "/hoje", component: HojeView },
    { path: "/inbox", component: InboxView },
    { path: "/memoria", component: MemoriaView },
  ],
});

export default router;
