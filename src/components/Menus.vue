<template>
  <el-menu
    :default-active="activPath"
    class="el-menu-vertical-demo h-full"
    @open="handleOpen"
    @close="handleClose"
    router
  >
    <el-sub-menu index="/yh/xianbaoku">
      <template #title>
        <el-icon><IconMenu /></el-icon>
        <span>优惠信息</span>
      </template>
      <el-menu-item index="/yh/xianbaoku">
        <span>线报酷</span>
      </el-menu-item>
      <el-sub-menu index="/yh/smzdm">
        <template #title>什么值得买</template>
        <el-menu-item index="/yh/smzdm/search">搜索</el-menu-item>
        <el-menu-item index="/yh/smzdm/threeHourHot">三小时热榜</el-menu-item>
        <el-menu-item index="/yh/smzdm/monitoring">页面监控</el-menu-item>
      </el-sub-menu>
    </el-sub-menu>
    <el-menu-item index="/settings">
      <el-icon><Setting /></el-icon>
      <span>设置</span>
    </el-menu-item>
    <el-menu-item
      index="/restart"
      @click="restart"
      class="absolute inset-x-0 bottom-0"
    >
      <el-icon><Refresh /></el-icon>
      <span>重启</span>
    </el-menu-item>
  </el-menu>
  
</template>

<script lang="ts" setup>
import { computed, ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Menu as IconMenu, Setting, Refresh } from "@element-plus/icons-vue";
import { useRoute, useRouter } from "vue-router";
import { relaunch } from "@tauri-apps/api/process";
import { appWindow } from "@tauri-apps/api/window";

const router = useRouter();
const handleOpen = (key: string, keyPath: string[]) => {
  // console.log(key, keyPath);
};
const handleClose = (key: string, keyPath: string[]) => {
  // console.log(key, keyPath);
};
const activPath = computed(() => {
  // console.log(useRoute().path);
  return useRoute().path;
});

const restart = async () => {
  await relaunch();
};

// import { useRoute } from "vue-router";
import { toRaw } from "vue";

const route = useRoute();
console.log(toRaw(route));
</script>

<style scoped>
/* 激活样式 */
.is-active {
  /* 配色不太好 */
  background-color: #ececec;
}
</style>
