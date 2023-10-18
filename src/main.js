import { createApp } from "vue";
// import "./styles.css";
import App from "./App.vue";
import ElementPlus from "element-plus";
import "element-plus/dist/index.css";
import * as ElementPlusIconsVue from "@element-plus/icons-vue";
import "./tailwind.css";
import router from "./router";
import VueQrcode from '@chenfengyuan/vue-qrcode';

const app = createApp(App);
app.use(ElementPlus);
app.use(router);
app.component(VueQrcode.name, VueQrcode);
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component);
}
app.mount("#app");
