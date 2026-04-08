import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import wasm from "vite-plugin-wasm";
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
  ],
  worker: {
    format: "es",
    plugins: () => [
      wasm(),
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
  css: {
    preprocessorOptions: {
        scss: {
            quietDeps: true,
            silenceDeprecations: ['color-functions'],
        }
    },
  }
});
