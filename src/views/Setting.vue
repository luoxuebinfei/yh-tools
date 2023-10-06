<template>
    <div>
        设置页面
        <el-button @click="test">点击</el-button>
    </div>
</template>

<script lang="ts" setup>
import { invoke } from "@tauri-apps/api/tauri";
import { WebviewWindow, appWindow } from "@tauri-apps/api/window";
const test =()=>{
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
}

</script>