import { defineConfig } from "@solidjs/start/config";

export default defineConfig({
  base: process.env.BASE_PATH || "/",
  build: {
    assetsDir: process.env.BASE_PATH?.substring(1) || "/",
  },
});
