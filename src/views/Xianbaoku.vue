<template>
  <!-- <el-button @click="test">点击</el-button> -->
  <div class="common-layout h-full static">
    <el-container class="fixed w-full h-full">
      <el-aside class="h-full w-36">
        <Menus />
      </el-aside>
      <el-main class="bg-gray-50 h-full">
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
      </el-main>
    </el-container>
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
import { ref, nextTick, onBeforeUnmount } from "vue";
import { open } from "@tauri-apps/api/shell";
import { ElInput } from "element-plus";
import { Bell, ArrowUp } from "@element-plus/icons-vue";
import Menus from "@/components/Menus.vue";
import Database from "tauri-plugin-sql-api";

const data = ref([]);

const columns = ref([
  {
    key: "title",
    dataKey: "title",
    title: "标题",
    width: 700,
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
  onclick: (e: { rowData: { url: any; }; }) => {
    open(`http://new.xianbao.fun${e.rowData.url}`);
  },
};

const listen_time = ref("");
const listen_timestamp = ref(0);
const get_data = async () => {
  invoke("get_data");
  appWindow.listen("listen_data", (event: { payload: never[]; }) => {
    // console.log(new Date(Date.parse(new Date().toString())));
    const nowtime = new Date().getTime();
    data.value = [...(event.payload as never[]), ...data.value];
    if (data.value.length > 20) {
      data.value = data.value.slice(0, 20);
    }
    // console.log(event);
  });
  appWindow.listen("listen_data_time", (event: { payload: number; }) => {
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
const getKeyword = async() => {
  const db = await Database.load("sqlite:database.db");
  const res: any[] = await db.select("SELECT keyword FROM keywords WHERE belong = 'xianbaoku'");
  dynamicTags.value = res.map((item: { keyword: any; }) => item.keyword);
};
getKeyword();

const handleClose = async(tag: string) => {
  dynamicTags.value.splice(dynamicTags.value.indexOf(tag), 1);
  const db = await Database.load("sqlite:database.db");
  await db.execute("DELETE FROM keywords WHERE keyword = ? AND belong = 'xianbaoku'", [tag]);
};

const showInput = () => {
  inputVisible.value = true;
  nextTick(() => {
    InputRef.value!.input!.focus();
  });
};

const handleInputConfirm = async() => {
  if (inputValue.value) {
    dynamicTags.value.push(inputValue.value);
    const db = await Database.load("sqlite:database.db");
    await db.execute("INSERT INTO keywords (keyword, belong) VALUES (?, 'xianbaoku')", [inputValue.value]);
  }
  inputVisible.value = false;
  inputValue.value = "";
};

const listen_sever = () => {
  appWindow.listen("xianbao_server_close", (event: any) => {
    location.reload();
  });
};
listen_sever();
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
