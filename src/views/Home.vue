<template>
  <!-- <el-button @click="test">点击</el-button> -->
  <div class="h-full">
    <div class="Container">
      <div>
        <div
          class="text-sm text-slate-500 italic"
          v-show="listen_time.length !== 0 && isKeywordShow"
        >
          上次更新时间: {{ listen_time }}
        </div>
      </div>
      <el-tooltip
        class="box-item"
        effect="dark"
        content="关键词提醒"
        placement="top"
      >
        <el-button
          id="keywordBell"
          class="mb-1"
          type="primary"
          :icon="Bell"
          size="small"
          v-show="isKeywordShow"
          @click="change_keyword"
        />
      </el-tooltip>
    </div>
    <div class="mb-1" v-show="!isKeywordShow">
      <el-tag
        v-for="tag in dynamicTags"
        :key="tag"
        class="mx-1"
        closable
        :disable-transitions="false"
        @close="handleClose(tag)"
      >
        {{ tag }}
      </el-tag>
      <el-input
        v-if="inputVisible"
        ref="InputRef"
        v-model="inputValue"
        class="ml-1 w-20"
        size="small"
        @keyup.enter="handleInputConfirm"
        @blur="handleInputConfirm"
      />
      <el-button
        v-else
        class="button-new-tag ml-1"
        size="small"
        @click="showInput"
      >
        + 关键词
      </el-button>
      <el-button @click="change_keyword" :icon="ArrowUp" size="small" text
        >收起</el-button
      >
    </div>
    <div style="height: calc(100% - 22px)">
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
import { ref, nextTick,onBeforeUnmount } from "vue";
import { open } from "@tauri-apps/api/shell";
import { ElInput } from "element-plus";
import { Bell, ArrowUp } from "@element-plus/icons-vue";

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
    open(`http://new.xianbao.fun/${e.rowData.url}`);
  },
};

const listen_time = ref("");
const listen_timestamp = ref(0);
const get_data = async () => {
  invoke("get_data");
  appWindow.listen("listen_data", (event) => {
    // console.log(new Date(Date.parse(new Date().toString())));
    const nowtime = new Date().getTime();
    data.value = [...(event.payload as never[]), ...data.value];
    if (data.value.length > 20) {
      data.value = data.value.slice(0, 20);
    }
    // console.log(event);
  });
  appWindow.listen("listen_data_time", (event) => {
    listen_timestamp.value = event.payload as number;
    const date = new Date(event.payload as number);
    const year = date.getFullYear();
    const month = (date.getMonth() + 1).toString().padStart(2, "0"); // 添加前导零
    const day = date.getDate().toString().padStart(2, "0"); // 添加前导零
    const hour = date.getHours().toString().padStart(2, "0"); // 添加前导零
    const minute = date.getMinutes().toString().padStart(2, "0"); // 添加前导零
    const second = date.getSeconds().toString().padStart(2, "0"); // 添加前导零

    const formattedDateTime = `${year}-${month}-${day} ${hour}:${minute}:${second}`;
    listen_time.value = formattedDateTime;
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

// 关键词展示
const isKeywordShow = ref(true);
const change_keyword = () => {
  isKeywordShow.value = !isKeywordShow.value;
};

// 关键词相关
const inputValue = ref("");
const dynamicTags = ref([""]);
const inputVisible = ref(false);
const InputRef = ref<InstanceType<typeof ElInput>>();
// 获取关键词
const getKeyword = () => {
  invoke("return_keyword").then((res) => {
    console.log(res);
    dynamicTags.value = res as string[];
  });
};
getKeyword();

const handleClose = (tag: string) => {
  dynamicTags.value.splice(dynamicTags.value.indexOf(tag), 1);
  invoke("change_keyword", { params: dynamicTags.value });
};

const showInput = () => {
  inputVisible.value = true;
  nextTick(() => {
    InputRef.value!.input!.focus();
  });
};

const handleInputConfirm = () => {
  if (inputValue.value) {
    dynamicTags.value.push(inputValue.value);
    invoke("change_keyword", { params: dynamicTags.value });
  }
  inputVisible.value = false;
  inputValue.value = "";
};

// 设置一个定时器，每隔3分钟检查后台服务是否正常
// let timerId = setTimeout(() => {
//   let timerId = setInterval(() => {
//     // 获取当前的时间戳
//     let now_timestamp = Date.parse(new Date().toString());
//     console.log(now_timestamp);
//     if (now_timestamp - listen_timestamp.value > 170000) {
//       console.log("检测到后台服务异常");
//       invoke("check_xianbao_server").then(() => {
//         invoke("get_data");
//       });
//     }
//   }, 1000);
//   return timerId;
// }, 1000 * 10);

// onBeforeUnmount(() => {
//   clearInterval(timerId);
// });
</script>

<style scoped>
div.el-table-v2__row:hover {
  cursor: pointer;
}
.Container {
  display: flex;
  justify-content: space-between;
}
/*关键词按钮*/
#keywordBell {
  background-color: #409eff;
  height: calc(100% - 22px);
}
</style>
