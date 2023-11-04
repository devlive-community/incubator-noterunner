import {createApp} from "vue";
import "./styles.css";
import App from "./App.vue";
import router from "./router";
import {createIcons} from "./fontawesome.ts";

const app = createApp(App);
// Import fa icons
createIcons(app);
app.use(router);
app.mount("#app");
