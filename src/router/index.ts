import { createRouter, createWebHistory } from "vue-router";
import MainWindow from "../pages/MainWindow.vue";
import StickyWindow from "../pages/StickyWindow.vue";

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/", component: MainWindow },
    { path: "/sticky/:id", component: StickyWindow, props: true },
  ],
});
