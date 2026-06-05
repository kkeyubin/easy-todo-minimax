<template>
  <router-view />
</template>

<script setup lang="ts">
// 根容器。W1 阶段窗口内容由 /sticky/:id 路由渲染。
// 启动恢复逻辑在 main 进程的 WindowManager 里（lib.rs setup hook）。
</script>

<style>
* {
  box-sizing: border-box;
}
html, body, #app {
  margin: 0;
  padding: 0;
  height: 100%;
  font-family: -apple-system, BlinkMacSystemFont, "PingFang SC", "Helvetica Neue", sans-serif;
  font-size: 14px;
  color: #222;
  background: transparent;
  user-select: none;
}
</style>
