import { createRouter, createWebHistory } from "vue-router";
import StickyWindow from "../pages/StickyWindow.vue";

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/", component: { template: "<div></div>" } },
    { path: "/sticky/:id", component: StickyWindow, props: true },
  ],
});
