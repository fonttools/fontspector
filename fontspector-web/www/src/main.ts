import { createApp } from "vue";
import App from "./App.vue";
import "./main.scss";

// @ts-ignore
import hbjs from "harfbuzzjs";

hbjs.then((HarfbuzzJs: any) => {
  // @ts-ignore
  window.hbjs = HarfbuzzJs;
  createApp(App).mount("#app");
});
