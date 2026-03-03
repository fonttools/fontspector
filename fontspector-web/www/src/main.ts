import { createApp } from "vue";
import App from "./App.vue";
import "bootstrap/dist/css/bootstrap.css";
import "bootstrap";
// @ts-ignore
import hbjs from "harfbuzzjs";

hbjs.then((HarfbuzzJs: any) => {
  // @ts-ignore
  window.hbjs = HarfbuzzJs;
  createApp(App).mount("#app");
});
