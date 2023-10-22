import {
  createRouter,
  createWebHistory,
  createWebHashHistory,
} from "vue-router";
import { defineAsyncComponent } from "vue";

// 静态导入页面
import Xianbaoku from "../views/Xianbaoku.vue";
import Settings from "../views/Setting.vue";
import Notify from "../views/Notify.vue";
import ThreeHourHot from "../views/Smzdm/ThreeHourHot.vue";
import SearchKeyword from "../views/Smzdm/Search.vue";

const router = createRouter({
  // history: createWebHashHistory(),  // hash 模式
  history: createWebHistory(), // history 模式
  routes: [
    {
      path: "/",
      redirect:"/yh/xianbaoku",
      children: [
        {
          path: "/yh/xianbaoku",
          name: "xianbaoku",
          component: Xianbaoku,
          meta: {
            title: "线报库",
          },
        },
        {
          path: "/yh/smzdm/search",
          name: "search",
          component: SearchKeyword,
          meta: {
            title: "搜索",
            data:[
              {
                name:"referrer",
                content:"no-referrer"
              }
            ]
          },
        },
        {
          path: "/yh/smzdm/threehourhot",
          name: "threehourhot",
          component: ThreeHourHot,
          meta: {
            title: "三小时热榜",
            data:[
              {
                name:"referrer",
                content:"no-referrer"
              }
            ]
            
          },
          // meta: {
          //   "referrer":"no-referrer",
          // }
        },
      ],
    },
    {
      path: "/settings",
      name: "settings",
      component: Settings,
      meta: {
        title: "设置页",
      },
    },
    {
      path: "/notify",
      name: "notify",
      component: Notify,
      meta: {
        title: "通知页",
      },
    },
    {
      path: "/*",
      redirect: "/",
    },
  ],
});

// 全局路由守卫
router.beforeEach((to, from, next) => {
  // console.log(to, from)
  if (to.meta.title) {
    document.title = `${to.meta.title}`;
  }
  writer(to);
  next();
});

router.afterEach((to, from) => {
  // console.log(to, from)
  console.log("afterEach");
});

export default router;

//router/index.js
const writer = (to) => {
  //首先找到head里的meta
  const deleArr = [];
  document.head.childNodes.forEach(item => {
    switch (item.tagName) {
      case "META":
        deleArr.push(item);
        break;
      case "TITLE":
        //顺便设置title
        document.title = to.meta.title || to.name;
        break;
    }
  });
 
  //删除原来的meta
  deleArr.forEach(item => {
    document.head.removeChild(item);
  })
 
  //添加想要的meta（全局）
  const metas = document.createElement("META");
  const creatArr = [
    { charset: "utf-8" },
    { "http-equiv": "X-UA-Compatible", content: "IE=edge" },
    //视图缩放
    { name: "viewport", content: "width=device-width,initial-scale=1.0, minimum-scale=1.0, maximum-scale=1.0, user-scalable=no,minimal-ui" }
  ];
 
  //将单个路由设定的meta加到creatArr数组中
  //局部meta和全局meta重复时，局部meta替换全局meta
  const tmpArr = to.meta.data ? to.meta.data.concat() : [];
 
  if (tmpArr.length > 0) {
    to.meta.data.forEach((item, index) => {
      creatArr.forEach((ele, ind) => {
        if (Object.keys(item)[0] == Object.keys(ele)[0]) {
          creatArr[ind] = JSON.parse(JSON.stringify(item));
          tmpArr.splice(index, 1);
        }
      });
    });
  }
 
  //生成合并后的数组
  const eleArr = creatArr.concat(tmpArr);
 
  //将设定的值写入文档片段
  const creatFrag = document.createDocumentFragment();
  eleArr.forEach(ele => {
    creatFrag.append(metas.cloneNode());
    Object.entries(ele).forEach(item => {
      creatFrag.lastChild.setAttribute(item[0], item[1]);
    });
  });
 
  //将文档片段写入head
  document.head.prepend(creatFrag);
}