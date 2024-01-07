import { defineConfig } from "@solidjs/start/config";

export default defineConfig({
  base: process.env.BASE_URL || "/",
  start: {
    server: {
      baseURL: process.env.BASE_URL || "/",
    },
  },
});
