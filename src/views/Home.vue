<template>
  <!-- <el-button @click="test">点击</el-button> -->
  <div class="h-full">
    <el-auto-resizer>
      <template #default="{ height, width }">
        <el-table-v2
          :columns="columns"
          :data="data"
          :width="width"
          :height="height"
          :row-event-handlers="columnEvents"
          :row-class="rowClass"
          fixed
        />
      </template>
    </el-auto-resizer>
  </div>
</template>

<script lang="ts" setup>
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/api/notification";
import { WebviewWindow, appWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";
import { open } from "@tauri-apps/api/shell";


const data = ref([]);

const columns = ref([
  {
    key: "title",
    dataKey: "title",
    title: "标题",
    width: 470,
  },
  {
    key: "comments",
    dataKey: "comments",
    title: "评论数",
    width: 60,
  },
  {
    key: "shorttime",
    dataKey: "shorttime",
    title: "时间",
    width: 60,
  },

  // 添加其他列定义
]);
const columnEvents = {
  onclick: (e) => {
    // const webview = new WebviewWindow("theUniqueLabel", {
    //   url: `http://new.xianbao.fun/${e.rowData.url}`,
    //   title: e.rowData.title,
    //   width: 1200,
    //   height: 800,
    // });
    open(`http://new.xianbao.fun/${e.rowData.url}`);
  },
};

const get_data = async () => {
  invoke("get_data");
  var unlisten = appWindow.listen("listen_data", (event) => {
    console.log(new Date(Date.parse(new Date().toString())));
    const nowtime = new Date().getTime();
    data.value = [...(event.payload as never[]), ...data.value];
    if (data.value.length > 20) {
      data.value = data.value.slice(0, 20);
    }
    console.log(event);
  });
};
get_data();

const rowClass = () => {
  return "";
};

const test = async () => {
  let permissionGranted = await isPermissionGranted();
  if (!permissionGranted) {
    const permission = await requestPermission();
    permissionGranted = permission === "granted";
  }
  if (permissionGranted) {
    sendNotification({
      title: "TAURI",
      body: "Tauri is awesome!",
      sound: "default",
    });
  }
};
</script>

<style>
div.el-table-v2__row:hover {
  cursor: pointer;
}
</style>
