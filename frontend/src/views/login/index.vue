<script setup>
import { ref } from "vue";
let username = ref('admin')
let password = ref('admin')

import { useUserStore } from '@/stores/user.js'
import { useRouter, useRoute } from 'vue-router'

const router = useRouter()
const route = useRoute()
const userStore = useUserStore()

async function handleLogin() {
  try {
    await userStore.login({
      username: username.value,
      password: password.value
    })
    router.push({ path: route.query.redirect || '/' })
  } catch (err) {
    console.error('登录失败', err)
  }
}
</script>


<template>
  <div class="login-container">
    <div class="login-card">
      <h2>XX平台</h2>
      <div class="login-form">
        <div class="form-item">
          <input id="username" v-model="username" type="text" placeholder="用户名" />
        </div>
        <div class="form-item">
          <input id="password" v-model="password" type="password" placeholder="密码" />
        </div>
        <button @click="handleLogin">登录</button>
      </div>
    </div>
  </div>
</template>


<style scoped>
/* 彩色旋转背景 */
.login-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100vh;
  background: linear-gradient(270deg, #ff4e50, #f9d423, #24c6dc, #6a82fb, #ff4e50);
  background-size: 1000% 1000%;
  animation: gradientRotate 20s ease infinite;
}

@keyframes gradientRotate {
  0% {
    background-position: 0% 50%;
  }

  50% {
    background-position: 100% 50%;
  }

  100% {
    background-position: 0% 50%;
  }
}

/* 登录卡片样式 */
.login-card {
  background: rgba(255, 255, 255, 0.95);
  padding: 40px 30px;
  border-radius: 16px;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.3);
  width: 360px;
  text-align: center;
  animation: fadeIn 0.6s ease;
}

.login-card h2 {
  margin-bottom: 30px;
  font-size: 26px;
  color: #333;
  font-weight: 600;
}

.login-form .form-item {
  margin-bottom: 20px;
}

.login-form input {
  width: 100%;
  padding: 12px 15px;
  border: 1px solid #ddd;
  border-radius: 10px;
  font-size: 14px;
  transition: all 0.3s;
}

.login-form input:focus {
  border-color: #6a82fb;
  box-shadow: 0 0 10px rgba(106, 130, 251, 0.4);
  outline: none;
}

.login-form button {
  width: 100%;
  padding: 12px;
  background: #6a82fb;
  border: none;
  border-radius: 10px;
  color: #fff;
  font-size: 16px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s;
}

.login-form button:hover {
  background: #5b6fd1;
  transform: translateY(-2px);
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(-20px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>