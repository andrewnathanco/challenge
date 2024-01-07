import { defineConfig } from "@solidjs/start/config";

export default defineConfig({
  start: {
    server: {
      baseURL: process.env.BASE_PATH || "/",
    },
    ssr: false,
  },
});
