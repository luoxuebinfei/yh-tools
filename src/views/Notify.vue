<template>
  <div
    class="static rounded-lg"
    @mouseenter="changeHoverStatus"
    @mouseleave="changeHoverStatus"
  >
    <!-- <audio src=".\assets\sound\Windows Notify System Generic.wav" autoplay></audio> -->
    <audio id="my-audio" :src="ad" autoplay></audio>
    <audio id="my-video" hidden="true" autoplay></audio>
    <div data-tauri-drag-region class="titlebar glass rounded-t-lg">
      <div class="titlebar-button" id="titlebar-close">
        <el-icon><Close /></el-icon>
      </div>
    </div>
    <div
      class="w-full fixed rounded-b-lg p-2 main glass"
      @click="closeAndOpenUrl"
    >
      <div class="text-sm font-semibold truncate ..." v-html="data.title"></div>
      <div class="text-xs font-sm" v-html="data.content"></div>
      <div v-html="msg"></div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { onBeforeMount, onMounted, ref } from "vue";
import { appWindow, WebviewWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/api/shell";
import { invoke } from "@tauri-apps/api/tauri";

export interface dataIterface {
  cateid: string;
  catename: string;
  comments: number;
  content: string;
  datetime: Date;
  id: number;
  louzhu: string;
  louzhuregtime: null;
  shijianchuo: number;
  shorttime: string;
  title: string;
  url: string;
  yuanurl: string;
}

// 鼠标移入移出
const HoverStatus = ref(false);
// 改变鼠标移入移出状态
const changeHoverStatus = () => {
  HoverStatus.value = !HoverStatus.value;
  invoke("change_hover_status", { param: HoverStatus.value });
};

const msg = ref("");
const data = ref({} as dataIterface);
const lable_name = ref("");

const closeAndOpenUrl = () => {
  const mainWindow = WebviewWindow.getByLabel(lable_name.value);
  if (mainWindow) {
    open(`http://new.xianbao.fun${data.value.url}`);
    invoke("change_hover_status", { param: false });
    mainWindow.close();
  }
};

// 最大最小关闭按钮的事件实现
onMounted(() => {
  document.getElementById("titlebar-close")!.addEventListener("click", () => {
    invoke("change_hover_status", { param: false });
    appWindow.close();
  });
  console.log("onMounted------", document.getElementById("titlebar-close"));
});
onBeforeMount(() => {
  const listen = () => {
    appWindow.listen("body", (event) => {
      console.log(event);
      data.value = event.payload as dataIterface;
    });
  };
  listen();
  const listen2 = () => {
    appWindow.listen("label_name", (event) => {
      lable_name.value = event.payload as string;
    });
  };
  listen2();
  const listen3 = () => {
    appWindow.listen("test", (event) => {
      msg.value = event.payload as string;
    });
  };
  listen3();
});

import { convertFileSrc } from "@tauri-apps/api/tauri";
import { appDataDir, join } from "@tauri-apps/api/path";
const ad = ref("");
const a = async() =>{
  // const appDataDirPath = await appDataDir();
  // const filePath = await join(
  //   appDataDirPath,
  //   "assets/sound/Windows Notify System Generic.wav"
  // );
  const audioUrl = convertFileSrc("src/assets/sound/Windows Notify System Generic.wav");
  ad.value = "src/assets/sound/Windows Notify System Generic.wav";
};
a();
</script>

<style scoped>
.titlebar {
  height: 20px;
  user-select: none;
  display: flex;
  justify-content: flex-end;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  /* background: #eeeeee; */
  z-index: 999;
  cursor: pointer;
}

.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 27px;
  height: 20px;
  cursor: pointer;
  z-index: 999;
}

.titlebar-button:hover {
  background: rgb(206, 206, 206);
}
#titlebar-close:hover {
  background: #e6505c;
}

.main {
  top: 20px;
  cursor: pointer;
  height: calc(100% - 20px);
  overflow: auto;
}
.glass {
  background: rgba(215, 224, 235, 0.9);
  backdrop-filter: blur(25px);
  /* border-radius: 7px; */
}

span:deep(.text-red-600) {
  color: red;
}
</style>
