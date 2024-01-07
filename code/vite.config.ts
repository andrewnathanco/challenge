import { defineConfig } from "@solidjs/start/config";

export default defineConfig({
  base: process.env.BASE_PATH || "/",
  start: {
    server: {
      baseURL: process.env.BASE_PATH || "/",
    },
  },
});
