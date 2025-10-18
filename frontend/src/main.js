import { createApp } from 'vue'
import App from './App.vue'
import { createPinia } from 'pinia'
import router from './router'
// 权限
import './permission.js'
// element-ui 引入
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
// 注册 icon 组件
import 'virtual:svg-icons-register'

const pinia = createPinia()

createApp(App).use(router).use(pinia).use(ElementPlus).mount('#app')
