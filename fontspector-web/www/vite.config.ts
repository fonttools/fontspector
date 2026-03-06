import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import path from "path";
import { viteStaticCopy } from "vite-plugin-static-copy";

export default defineConfig({
  plugins: [
    vue(),
    viteStaticCopy({
      targets: [
        {
          src: "node_modules/harfbuzzjs/hb.wasm",
          dest: ".",
        },
      ],
    }),
    wasm(),
    topLevelAwait(),
  ],
  worker: {
    format: "es",
    plugins: () => [
      wasm(),
      topLevelAwait(),
    ],
  },
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
      "pkg": path.resolve(__dirname, "../pkg"),
    },
  },
  server: {
    fs: {
      allow: [".."],
    },
  },
});
