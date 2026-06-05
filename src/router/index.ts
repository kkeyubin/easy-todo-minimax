import { createRouter, createWebHistory } from "vue-router";
import StickyWindow from "../pages/StickyWindow.vue";

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    // 兜底：主进程开窗时先给一个空白页，加载完 StickyWindow 后再 history.replace
    { path: "/", component: { template: "<div></div>" } },
    { path: "/sticky/:id", component: StickyWindow, props: true },
  ],
});
