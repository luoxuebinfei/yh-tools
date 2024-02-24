<template>
  <div class="common-layout h-full static">
    <el-container class="fixed w-full h-full">
      <el-aside class="h-full w-36"><Menus /></el-aside>
      <el-main class="bg-gray-50 h-full">
        <div>
          设置页面
          <el-button @click="test">点击</el-button>
          <el-button @click="test2">测试2</el-button>
        </div>
      </el-main>
    </el-container>
  </div>
</template>

<script lang="ts" setup>
import Menus from "@/components/Menus.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { WebviewWindow, appWindow } from "@tauri-apps/api/window";
const test = () => {
  // const webview = new WebviewWindow("my-label", {
  //     url: "/notify",
  //   });
  //   webview.once("tauri://created", function () {
  //     // webview window successfully created
  //   });
  //   webview.once("tauri://error", function (e) {
  //     // an error happened creating the webview window
  //     console.log(e);
  //   });
  invoke("create_window");
};
const test2 = () => {
  invoke("test2");
};
</script>
