<template>
  <div class="h-full">
    <div class="mb-2" style="height: 32px">
      <el-row :gutter="20">
        <el-col :span="22">
          <el-input v-model="inputUrl" placeholder="请输入网址" clearable />
        </el-col>
        <el-col :span="2">
          <el-button
            class="inline-block"
            type="primary"
            color="#626aef"
            @click="add"
            >添加</el-button
          >
        </el-col>
      </el-row>
    </div>
    <div class="" style="height: calc(100% - 32px)">
      <ul v-for="item in data" :key="item.url">
        <li class="item" :class="{ 'text-gray-400': item.is_expired,'update': item.is_update }">
          <a class="truncate inline-block" style="width: calc(100% - 80px)" :href="item.url" :title="item.title" target="_blank" @click="edit_item('click',item.url)">{{ item.title }}</a>
          <div class="inline-block float-right">
            <el-button
              type="primary"
              :icon="Delete"
              @click="edit_item('del',item.url)"
            />
          </div>
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";
import { ElMessage } from "element-plus";
import { Delete } from "@element-plus/icons-vue";
import { invoke } from "@tauri-apps/api/tauri";


const inputUrl = ref("");

const data = ref();

// 获取监控列表
const getMonitorList = () => {
  invoke("get_monitor_list").then((res) => {
    data.value = res;
  });
};
getMonitorList();

// 添加监控
const add = () => {
  if (inputUrl.value) {
    // 提取url的id防止重复
    for (let i = 0; i < data.value.length; i++) {
      let match1 = /\/(\d+?)\//gm.exec(data.value[i].url);
      let match2 = /\/(\d+?)\//gm.exec(inputUrl.value);
      console.log(match2, inputUrl.value);
      if (match1[1] == match2[1]) {
        ElMessage({
          message: "请不要重复添加",
          type: "warning",
        });
        inputUrl.value = "";
        return;
      }
    }
    edit_item("add",inputUrl.value);
    inputUrl.value = "";
  }
};

// 修改监控
const edit_item = (command,url) => {
  invoke("edit_monitor", {command: command, url: url }).then((res) => {
    getMonitorList();
  });
};
</script>

<style scoped>
button {
  background-color: #626aef;
}
.item {
  height: 44px;
  line-height: 40px;
  background-color: white;
  border-bottom: 1px solid #f0f0f0;
  padding-left: 20px;
  width: 100%;
}
/* 更新样式 */
.update {
  border: 1px dashed #ff0404;
}
</style>
