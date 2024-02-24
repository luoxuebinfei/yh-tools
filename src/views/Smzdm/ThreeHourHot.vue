<template>
  <div class="common-layout h-full static">
    <el-container class="fixed w-full h-full">
      <el-aside class="h-full w-36"><Menus /></el-aside>
      <el-main class="bg-gray-50 h-full">
        <div class="staic h-full overflow-y-hidden">
          <div class="">
            <div class="mb-1 w-full flex justify-end" v-show="isKeywordShow">
              <el-tooltip
                class="box-item"
                effect="dark"
                content="标题关键词提醒"
                placement="top"
              >
                <el-button
                  id="keywordBell"
                  class="mb-1"
                  style="color: rgb(88, 88, 255)"
                  type="primary"
                  :icon="Bell"
                  text
                  @click="change_keyword('title')"
                />
              </el-tooltip>
              <el-tooltip
                class="box-item"
                effect="dark"
                content="品牌 / 品类关键词提醒"
                placement="top"
              >
                <el-button
                  id="keywordBell"
                  class="mb-1"
                  type="primary"
                  :icon="Bell"
                  text
                  @click="change_keyword('category')"
                />
              </el-tooltip>
              <el-tooltip
                class=""
                effect="dark"
                content="刷新页面"
                placement="top"
              >
                <el-button
                  class=""
                  type="primary"
                  :icon="Refresh"
                  text
                  @click="refreshPage"
                />
              </el-tooltip>
            </div>
            <div class="mb-4" v-show="!isKeywordShow">
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
              <el-button
                @click="change_keyword"
                :icon="ArrowUp"
                size="small"
                text
                >收起</el-button
              >
            </div>
          </div>
          <div class="relative overflow-x-hidden feed-list" style="height: 94%">
            <el-row
              :gutter="10"
              class="bg-neutral-100"
              v-infinite-scroll="load"
              :infinite-scroll-delay="500"
              :infinite-scroll-immediate="false"
            >
              <el-col
                class="relative"
                :xs="{ span: 24 }"
                :sm="{ span: 8 }"
                :md="{ span: 6 }"
                :lg="{ span: 4 }"
                :xl="{ span: 3 }"
                v-for="i in smzdm_list"
                :key="i.article_id"
                ><div
                  class="px-2 py-1 grid-content bg-white border-red-6001 border-21"
                  :style="i.article_border_style"
                >
                  <div class="relative">
                    <!-- 商品图片 -->
                    <a :href="i.article_url" target="_blank">
                      <img :src="i.article_pic_url" alt="" />
                    </a>
                    <!-- 图片右下角的商城标签 -->
                    <a
                      class="absolute bottom-0 right-0 tag-bottom-right"
                      :href="i.article_mall_url"
                      target="_blank"
                      >{{ i.article_mall }}</a
                    >
                  </div>
                  <!-- 标题 -->
                  <div
                    class="text-sm whitespace-normal truncate"
                    style="height: 40px"
                  >
                    <a :href="i.article_url" target="_blank"
                      ><object class="inline" v-show="i.zhifa_tag.name !== ''">
                        <a
                          class="text-xs bg-red-100 text-red-700 px-1 rounded-md"
                          :href="i.zhifa_tag.url"
                          target="_blank"
                          >{{ i.zhifa_tag.name }}</a
                        ></object
                      ><span
                        :title="i.article_title"
                        v-html="i.article_title"
                      ></span
                    ></a>
                  </div>
                  <!-- 价格 -->
                  <div
                    class="text-sm leading-5 text-red-500 truncate whitespace-normal line-clamp-1"
                    style="height: 40px; line-height: 40px"
                  >
                    <a
                      :href="i.article_url"
                      target="_blank"
                      :title="i.article_price"
                      >{{ i.article_price }}</a
                    >
                  </div>
                  <div class="my-1 text-xs">
                    <!-- 作者 -->
                    <div class="float-left">
                      <a
                        class="inline-block align-middle z-avatar-pic"
                        :href="i.article_avatar_url"
                        target="_blank"
                        ><img
                          class="rounded-full"
                          :src="i.article_avatar"
                          alt=""
                      /></a>
                      <a
                        class="inline-block text-center align-middle text-sky-600 whitespace-nowrap truncate"
                        style="max-width: 118px"
                        :href="i.article_avatar_url"
                        target="_blank"
                        >{{ i.article_referrals }}</a
                      >
                    </div>
                    <!-- 发布时间 -->
                    <div class="float-right">{{ i.article_date }}</div>
                    <div></div>
                  </div>
                  <!-- 文章内容 -->
                  <div
                    class="mt-8 mb-2 text-xs text-slate-600 truncate whitespace-normal line-clamp-4"
                    style="height: 64px"
                    :title="i.article_content"
                    v-html="i.article_content"
                  ></div>
                  <div class="">
                    <!-- 评分和评论数量 -->
                    <div
                      class="text-xs inline-block items-stretch float-left"
                      style="line-height: 24px"
                    >
                      <span class="mr-2 self-auto"
                        ><i class="mr-1">值</i>{{ i.article_rating }}</span
                      >
                      <a
                        :href="i.article_url + '#comments'"
                        target="_blank"
                        class=""
                        ><i class="mr-1 self-center"
                          ><el-icon><ChatDotSquare /></el-icon></i
                        ><span class="">{{ i.article_comment }}</span></a
                      >
                    </div>
                    <div class="inline-block float-right">
                      <!-- app扫码 -->
                      <vue-qrcode
                        :value="i.article_url"
                        :options="{ width: 120 }"
                        class="absolute right-3 bottom-11 border-orange-400 border-2"
                        v-show="qrcodeId == i.article_id"
                      ></vue-qrcode>
                      <el-button
                        size="small"
                        @mouseover="overQrcodeStatus(i)"
                        @mouseout="outQrcodeStatus(i)"
                        >APP扫码</el-button
                      >
                    </div>
                    <div></div>
                  </div></div
              ></el-col>
            </el-row>
          </div>
          <el-backtop :right="50" :bottom="50" target=".feed-list" class="" />
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
      >遇到什么值得买机器人验证，请在<a
        href="https://faxian.smzdm.com/h2s0t0f0c1p1/"
        class="text-sky-600"
        target="_blank"
      >
        什么值得买 </a
      >中手动获取<span class="font-semibold"> cookies </span>填入下方输入框中
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
import { nextTick, onMounted, ref } from "vue";
import { Refresh, Bell, ArrowUp, ArrowUpBold } from "@element-plus/icons-vue";
import { ElInput, ElMessageBox } from "element-plus";

const smzdm_list = ref<Smzdm_interface[]>([]);

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
  article_comment: number; //评论的数量
  zhifa_tag: ZhifaTag; //诸如“白菜党”，“新品尝鲜”，“绝对值”等标签
  article_border_style: string; //符合关键词的文章边框样式
}

export interface ZhifaTag {
  name: string;
  url: string;
}

// 获取数据
invoke("smzdm_3hhot")
  .then((res) => {
    smzdm_list.value = res as Smzdm_interface[];
    console.log(smzdm_list.value);
  })
  .catch((err) => {
    console.log(err);
    dialogVisible.value = true;
  });
// 下拉获取更多数据
const pageNum = ref(1);
const load = () => {
  if (pageNum.value <= 4) {
    pageNum.value++;
    console.log(pageNum.value);
    invoke("smzdm_3hhot_more", { pagenum: pageNum.value })
      .then((res) => {
        smzdm_list.value = [...smzdm_list.value, ...(res as Smzdm_interface[])];
      })
      .catch((err) => {
        console.log(err);
        dialogVisible.value = true;
      });
  }
};

// 刷新页面
const refreshPage = () => {
  location.reload();
};

// 关键词相关控件
export interface Keyword {
  title: string[];
  category: string[];
}
const keywordStatus = ref(""); //切换关键词
const keywordTags = ref<Keyword>({
  title: [],
  category: [],
}); // 存储获取的关键词
const isKeywordShow = ref(true);
const dynamicTags = ref([""]); // 展示的关键词
const inputVisible = ref(false);
const inputValue = ref("");
const InputRef = ref<InstanceType<typeof ElInput>>();
const change_keyword = (e) => {
  console.log(e);
  // 获取点击的是哪个按钮标签
  keywordStatus.value = e;
  // 将点击的关键词展示
  dynamicTags.value = keywordTags.value[e];
  isKeywordShow.value = !isKeywordShow.value;
};
// 获取关键词
const getKeyword = () => {
  invoke("return_smzdm_keyword").then((res) => {
    console.log(res);
    keywordTags.value = res as Keyword;
    // dynamicTags.value = res as string[];
  });
};
getKeyword();

const handleClose = (tag: string) => {
  dynamicTags.value.splice(dynamicTags.value.indexOf(tag), 1);
  keywordTags.value[keywordStatus.value] = dynamicTags.value;
  invoke("change_smzdm_keyword", { params: keywordTags.value });
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
    keywordTags.value[keywordStatus.value] = dynamicTags.value;
    invoke("change_smzdm_keyword", { params: keywordTags.value });
  }
  inputVisible.value = false;
  inputValue.value = "";
};

// 二维码显示状态
const qrcodeId = ref(0);
const overQrcodeStatus = (e) => {
  qrcodeId.value = e.article_id;
};
const outQrcodeStatus = (e) => {
  qrcodeId.value = 0;
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
</script>

<style scoped>
.el-row {
  margin-bottom: 20px;
}
.el-row:last-child {
  margin-bottom: 0;
}
.el-col {
  border-radius: 4px;
}

.grid-content {
  border-radius: 4px;
  min-height: 36px;
  height: 401px;
  margin-bottom: 14px;
  max-width: 200px;
}

/* 图片右下角的商城标签 */
.tag-bottom-right {
  background-color: hsla(0, 0%, 96%, 0.95);
  padding: 0 14px;
  color: #5188a6;
  height: 34px;
  border-top-left-radius: 8px;
  line-height: 34px;
  font-size: 14px;
}

/* 图标 */
.icon-arrow {
  height: 1ex;
  display: inline-block;
}

a.z-avatar-pic {
  width: 20px;
  height: 20px;
  text-align: center;
  margin-right: 5px;
}
/* 右侧悬浮按钮 */
.sticky-right {
  position: fixed;
  right: 30px;
  bottom: 20px;
  z-index: 100;
}
</style>
