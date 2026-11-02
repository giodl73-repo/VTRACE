import {defineConfig} from '@playwright/test';
export default defineConfig({testDir:'tests',testMatch:'*.spec.js',workers:1,timeout:30000,
  use:{baseURL:process.env.VTRACE_URL??'http://127.0.0.1:8772/VTRACE/',headless:true,trace:'retain-on-failure',screenshot:'only-on-failure'},
  webServer:process.env.VTRACE_URL?undefined:{command:'python -m http.server 8772 --directory ../../target/package-prototype --bind 127.0.0.1',url:'http://127.0.0.1:8772/VTRACE/',reuseExistingServer:false},
});
