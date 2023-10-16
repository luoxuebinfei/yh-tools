<template>
  <div class="static">
    <div class="mb-1 w-full flex justify-end">
      <el-tooltip class="" effect="dark" content="刷新页面" placement="top">
        <el-button
          class=""
          type="primary"
          :icon="Refresh"
          text
          @click="refreshPage"
        />
      </el-tooltip>
    </div>
    <div class="relative">
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
          ><div class="grid-content bg-white border-white border-8">
            <div class="relative">
              <!-- 商品图片 -->
              <a :href="i.article_url" target="_blank">
                <img :src="i.article_pic_url" alt="" />
              </a>
              <!-- 图片右下角的商城标签 -->
              <a
                class="absolute bottom-0 right-0 tag-bottom-right"
                :href="i.article_mall_url"
                >{{ i.article_mall }}</a
              >
            </div>
            <!-- 标题 -->
            <div
              class="text-sm whitespace-normal truncate"
              style="height: 38px"
            >
              <a :href="i.article_url" target="_blank"
                ><object class="inline" v-show="i.zhifa_tag.name !== ''">
                  <a
                    class="text-xs bg-red-100 text-red-700 px-1 rounded-md"
                    :href="i.zhifa_tag.url"
                    target="_blank"
                    >{{ i.zhifa_tag.name }}</a
                  ></object
                >{{ i.article_title }}</a
              >
            </div>
            <!-- 价格 -->
            <div class="text-sm leading-5 text-red-500">
              <a :href="i.article_url" target="_blank">{{ i.article_price }}</a>
            </div>
            <div class="my-1 text-xs">
              <!-- 作者 -->
              <div class="float-left">
                <a
                  class="inline-block align-middle z-avatar-pic"
                  :href="i.article_avatar_url"
                  target="_blank"
                  ><img class="rounded-full" :src="i.article_avatar" alt=""
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
              class="mt-8 text-xs text-slate-600"
              v-html="i.article_content"
            ></div>
            <div>
              <div class="text-xs absolute bottom-4 flex items-stretch">
                <span class="mr-2 self-auto"
                  ><i class="mr-1">值</i>{{ i.article_rating }}</span
                >
                <a :href="i.article_url + '#comments'" target="_blank" class=""
                  ><i class="mr-1 self-center"
                    ><el-icon><ChatDotSquare /></el-icon></i
                  ><span class="">{{ i.article_comment }}</span></a
                >
              </div>
              <div></div>
            </div></div
        ></el-col>
      </el-row>
    </div>
  </div>
</template>
<script lang="ts" setup>
import { invoke } from "@tauri-apps/api/tauri";
import { onMounted, ref } from "vue";
import { Refresh } from "@element-plus/icons-vue";

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
}

export interface ZhifaTag {
  name: string;
  url: string;
}

// 获取数据
invoke("smzdm_3hhot").then((res) => {
  smzdm_list.value = res as Smzdm_interface[];
  console.log(smzdm_list.value);
});
// 下拉获取更多数据
const pageNum = ref(1);
const load = () => {
  if (pageNum.value <= 4) {
    pageNum.value++;
    console.log(pageNum.value);
    invoke("smzdm_3hhot_more", { pagenum: pageNum.value }).then((res) => {
      smzdm_list.value = [...smzdm_list.value, ...(res as Smzdm_interface[])];
    });
  }
};

// 刷新页面
const refreshPage = () => {
  location.reload();
}
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
</style>
