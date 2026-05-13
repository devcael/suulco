import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import "./assets/css/tailwind.css";
import { ServicesPlugin } from "./core/plugins/services";

const app = createApp(App);
app.use(router);
app.use(ServicesPlugin);
app.mount("#app");
