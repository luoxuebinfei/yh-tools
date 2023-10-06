<template>
  <div
    class="common-layout h-full static"
    v-if="$router.currentRoute.value.path != '/notify'"
  >
    <el-container class="fixed w-full h-full
    ">
      <el-aside class="h-full w-36">
        <el-menu
          :default-active="activPath"
          class="el-menu-vertical-demo h-full"
          @open="handleOpen"
          @close="handleClose"
          router
        >
          <el-menu-item index="/">
            <el-icon><IconMenu /></el-icon>
            <span>主页</span>
          </el-menu-item>
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
      </el-aside>
      <el-main class="bg-gray-50">
        <router-view> </router-view>
      </el-main>
    </el-container>
  </div>
  <div v-else>
    <router-view> </router-view>
  </div>
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

</style>
