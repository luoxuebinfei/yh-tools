<template>
  <div class="common-layout h-full static">
    <el-container class="fixed w-full h-full">
      <el-aside class="h-full w-36"><Menus /></el-aside>
      <el-main class="bg-gray-50 h-full">
        <div class="h-full">
          <el-backtop :right="50" :bottom="50" target=".feed" />
          <el-tabs
            v-model="editableTabsValue"
            type="card"
            editable
            class="demo-tabs h-full overflow-x-hidden feed"
            @edit="handleTabsEdit"
            @tab-click="tabClick"
          >
            <el-tab-pane
              v-for="item in editableTabs"
              class="h-full"
              :key="item.name"
              :label="item.title"
              :name="item.name"
              :lazy="true"
            >
              <template #label>
                <span>
                  {{ item.title }}
                  <span @click="chageZhi(item.name)">
                    <el-tooltip
                      class="box-item"
                      effect="dark"
                      :content="switchStatus ? '值率≥50%' : '默认'"
                      placement="top"
                    >
                      <el-icon style="vertical-align: -15%"
                        ><Switch
                      /></el-icon> </el-tooltip
                  ></span>
                </span>
              </template>
              <div class="bg-white h-full">
                <ul class="h-full">
                  <li
                    class="feed-row-wide"
                    v-for="i in searchData"
                    :key="i.article_id"
                  >
                    <div>
                      <div class="z-feed-img">
                        <span
                          :class="{
                            'bg-gray-300':
                              i.article_yh_type === '过期' ||
                              i.article_yh_type === '售罄',
                            'bg-emerald-500': i.article_yh_type === '好价频道',
                            'bg-red-500': ![
                              '过期',
                              '售罄',
                              '好价频道',
                            ].includes(i.article_yh_type),
                          }"
                          class="absolute text-sm h-5 px-1 rounded-r-md text-white"
                          >{{ i.article_yh_type }}</span
                        >
                        <a :href="i.article_url" target="_blank"
                          ><!-- 文章图片 -->
                          <img :src="i.article_pic_url" :alt="i.article_title"
                        /></a>
                      </div>
                      <div class="z-feed-content">
                        <h5
                          class="feed-block-title text-lg truncate"
                          :class="{
                            'text-gray-300':
                              i.article_yh_type === '过期' ||
                              i.article_yh_type === '售罄',
                          }"
                        >
                          <!-- 文章标题 -->
                          <a
                            class="inline"
                            :href="i.article_url"
                            :title="i.article_title"
                            target="_blank"
                            >{{ i.article_title }}<br
                          /></a>
                          <!-- 价格 -->
                          <a
                            class="inline-block"
                            :class="{
                              'text-gray-300':
                                i.article_yh_type === '过期' ||
                                i.article_yh_type === '售罄',
                              'text-red-600': !['过期', '售罄'].includes(
                                i.article_yh_type
                              ),
                            }"
                            :href="i.article_url"
                            :title="i.article_price"
                            target="_blank"
                            >{{ i.article_price }}</a
                          >
                        </h5>
                        <div
                          class="feed-block-descripe-top truncate text-sm text-gray-500"
                        >
                          <!-- 内容 -->
                          {{ i.article_content }}
                        </div>
                        <div class="z-feed-foot z-feed-foot-buy mt-11">
                          <div class="z-feed-foot-l float-left">
                            <span class="price-btn-group mr-4">
                              <!-- 值和不值 -->
                              <span class="price-btn-up mr-4">
                                <span class="unvoted-wrap">
                                  <i class="text-red-600 mr-2">值</i>
                                  <span>{{ i.article_rating }}</span>
                                </span>
                              </span>
                              <span class="price-btn-down">
                                <span class="unvoted-wrap">
                                  <i class="text-gray-400 mr-2">值</i>
                                  <span>{{ i.article_rating_down }}</span>
                                </span>
                              </span>
                            </span>
                            <span class="mr-4">
                              <el-icon><Star /></el-icon>
                              <span class="ml-2">{{ i.article_collect }}</span>
                            </span>
                            <a
                              :href="i.article_url + '#comment'"
                              target="_blank"
                              ><el-icon><ChatDotSquare /></el-icon
                              ><span class="ml-2">{{
                                i.article_comment
                              }}</span></a
                            >
                          </div>
                          <div class="z-feed-foot-r float-right text-sm">
                            <span class="feed-block-extras">
                              {{ i.article_date }}
                              <span class="ml-2">{{ i.article_mall }}</span>
                            </span>
                            <div
                              class="feed-link-btn inline-block relative ml-5"
                            >
                              <!-- app扫码 -->
                              <vue-qrcode
                                :value="i.article_url"
                                :options="{ width: 120 }"
                                class="absolute right-3 bottom-6 border-orange-400 border-2"
                                v-show="qrcodeId == i.article_id"
                              ></vue-qrcode>
                              <el-button
                                size="small"
                                @mouseover="overQrcodeStatus(i)"
                                @mouseout="outQrcodeStatus(i)"
                                >APP扫码</el-button
                              >
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>
                  </li>
                </ul>
              </div>
            </el-tab-pane>
          </el-tabs>
        </div>
      </el-main>
    </el-container>
  </div>

  <el-dialog
    v-model="dialogVisible"
    title="错误"
    width="30%"
    :before-close="dialogHandleClose"
  >
    <span
      >什么值得买返回状态码为<a
        :href="
          'https://developer.mozilla.org/zh-CN/docs/Web/HTTP/Status/' +
          json_err.status
        "
        class="text-sky-600"
        target="_blank"
        >{{ json_err.status }}</a
      >，请尝试从<a
        href="https://faxian.smzdm.com/h2s0t0f0c1p1/"
        class="text-sky-600"
        target="_blank"
      >
        什么值得买 </a
      >中获取<span class="font-semibold"> cookies </span>填入下方输入框中
      <el-input
        v-model="cookiesInput"
        placeholder="请填入新的cookies"
        clearable
    /></span>
    <template #footer>
      <span class="dialog-footer">
        <el-button class="bg-blue-500" type="primary" @click="dialogEvent">
          确定
        </el-button>
      </span>
    </template>
  </el-dialog>
</template>

<script lang="ts" setup>
import Menus from "@/components/Menus.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { ElMessageBox, TabPaneName } from "element-plus";
import { ref } from "vue";
import { Switch } from "@element-plus/icons-vue";

export interface Smzdm_interface {
  article_id: number; //文章id
  article_url: string; //文章url
  article_pic_url: string; //文章图片url
  article_pic_style: string; //文章图片样式
  article_title: string; //文章标题
  article_price: string; //价格
  article_referrals: string; //作者
  article_avatar: string; //作者头像url
  article_avatar_url: string; //作者主页url
  article_display_date: string; //文章发布时间
  article_date: string; //文章发布时间
  article_content: string; //文章内容
  article_yh_type: string; //文章类型
  article_mall: string; //商城
  article_mall_url: string; //商城url
  article_link: string; //文章链接
  article_rating: number; //"值"的数量
  article_rating_down: number; //"不值"的数量
  article_collect: number; //收藏的数量
  article_comment: number; //评论的数量
  zhifa_tag: ZhifaTag; //诸如“白菜党”，“新品尝鲜”，“绝对值”等标签
  article_border_style: string; //符合关键词的文章边框样式
}

export interface ZhifaTag {
  name: string;
  url: string;
}
//获取数据
const searchData = ref<Smzdm_interface[]>();
const json_err = ref();
const getData = (item, status) => {
  invoke("smzdm_search", { keyword: item, iszhi: status })
    .then((res) => {
      searchData.value = res as Smzdm_interface[];
    })
    .catch((err) => {
      json_err.value = JSON.parse(err);
      dialogVisible.value = true;
    });
};
// getData();

// 二维码显示状态
const qrcodeId = ref(0);
const overQrcodeStatus = (e) => {
  qrcodeId.value = e.article_id;
};
const outQrcodeStatus = (e) => {
  qrcodeId.value = 0;
};

// 关键词
export interface Keyword {
  title: string[];
  category: string[];
}
const keywordTags = ref<Keyword>({
  title: [],
  category: [],
}); // 存储获取的关键词
const getKeyword = () => {
  invoke("return_smzdm_keyword").then((res) => {
    console.log(res);
    keywordTags.value = res as Keyword;
    [...keywordTags.value.title, ...keywordTags.value.category].forEach(
      (item) => {
        editableTabs.value.push({
          title: item,
          name: item,
        });
      }
    );
  });
};
getKeyword();

// 标签页切换
let tabIndex = 2;
const editableTabsValue = ref("");
const editableTabs = ref(
  [] as {
    title: string;
    name: TabPaneName;
  }[]
);

const handleTabsEdit = (
  targetName: TabPaneName | undefined,
  action: "remove" | "add"
) => {
  //   if (action === "add") {
  //     const newTabName = `${++tabIndex}`;
  //     editableTabs.value.push({
  //       title: "New Tab",
  //       name: newTabName,
  //       content: "New Tab content",
  //     });
  //     editableTabsValue.value = newTabName;
  //   } else if (action === "remove") {
  //     const tabs = editableTabs.value;
  //     let activeName = editableTabsValue.value;
  //     if (activeName === targetName) {
  //       tabs.forEach((tab, index) => {
  //         if (tab.name === targetName) {
  //           const nextTab = tabs[index + 1] || tabs[index - 1];
  //           if (nextTab) {
  //             activeName = nextTab.name;
  //           }
  //         }
  //       });
  //     }
  //     editableTabsValue.value = activeName;
  //     editableTabs.value = tabs.filter((tab) => tab.name !== targetName);
  //   }
};
// 标签页点击事件
const tabClick = (tab) => {
  searchData.value = [];
  getData(tab.props.name, switchStatus.value);
};

// 错误提示框
const dialogVisible = ref(false);
const cookiesInput = ref("");

const dialogHandleClose = (done: () => void) => {
  ElMessageBox.confirm("确定关闭?")
    .then(() => {
      invoke("smzdm_write_cookies", { cookies: cookiesInput.value });
      location.reload();
    })
    .catch(() => {
      // catch error
    });
};
const dialogEvent = () => {
  dialogVisible.value = true;
  invoke("smzdm_write_cookies", { cookies: cookiesInput.value });
  location.reload();
};

// 切换是否设置筛选>50%值的选项
const switchStatus = ref(true);
const chageZhi = (item) => {
  console.log(item);
  switchStatus.value = !switchStatus.value;
  getData(item, switchStatus.value);
};
</script>

<style scoped>
/* 每个文章列表的样式 */
.feed-row-wide {
  /* padding-top: 14px; */
  /* padding-bottom: 14px; */
  padding: 14px;
  height: 168px;
  border: 1px solid #ebeef5;
}
/* 图片 */
.z-feed-img {
  width: 140px;
  height: 140px;
  margin-right: 20px;
  float: left;
  position: relative;
}
/* 除图片外的内容 */
.z-feed-content {
  height: 140px;
}
</style>
