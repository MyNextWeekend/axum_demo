<script setup>
// 导航栏
import Hamburger from '@/components/Hamburger/index.vue'
import Breadcrumb from '@/components/Breadcrumb/index.vue'
import SvgIcon from '@/components/SvgIcon/index.vue'
import { useMenuStore } from '@/stores/menu'
import HeaderSearch from './HeaderSearch.vue'

const menuStore = useMenuStore()
function toggleSidebar() {
  menuStore.toggleOpened()
}

import { useRouter, useRoute } from 'vue-router'
import { useUserStore } from '@/stores/user'
const userStore = useUserStore()
const router = useRouter()


import { watch } from 'vue'
const route = useRoute()

watch(
  route,
  (to, from) => {
    const { path, fullPath, meta, query, params } = to
    menuStore.addTagView({
      title: meta.title,
      path,
      fullPath,
      query,
      params
    })
  },
  {
    immediate: true
  }
)

async function logout() {
  try {
    await userStore.logout()
    console.log('退出成功')
    router.push({ path: '/login' })
  } catch (err) {
    console.error('退出失败', err)
  }
}
</script>

<template>
  <div class="navbar">
    <hamburger id="hamburger-container" :is-active="menuStore.opened" class="hamburger-container"
      @toggleClick="toggleSidebar" />
    <breadcrumb id="breadcrumb-container" class="breadcrumb-container" />
    <div class="right-menu">
      <HeaderSearch></HeaderSearch>
      <el-dropdown class="avatar-container right-menu-item hover-effect" trigger="click">
        <div class="avatar-wrapper">
          <SvgIcon className="user-avatar" iconClass="planet" />
        </div>
        <template #dropdown>
          <el-dropdown-menu slot="dropdown">
            <router-link to="/">
              <el-dropdown-item> 个人中心 </el-dropdown-item>
            </router-link>
            <router-link to="/">
              <el-dropdown-item> 接入指南 </el-dropdown-item>
            </router-link>
            <a target="_blank" href="https://panjiachen.github.io/vue-element-admin-site/#/">
              <el-dropdown-item> 文档说明 </el-dropdown-item>
            </a>
            <el-dropdown-item divided @click.native="logout">
              <span style="display:block;"> 退出登陆 </span>
            </el-dropdown-item>
          </el-dropdown-menu>
        </template>
      </el-dropdown>
    </div>
  </div>
</template>


<style lang="scss" scoped>
.navbar {
  height: 50px;
  line-height: 50px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  background: #fff;
  border-bottom: 1px solid #e6e6e6;
  padding: 0 1px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.08);
  box-sizing: border-box;

  .hamburger-container {
    cursor: pointer;
    margin-right: 5px;
    display: flex;
    align-items: center;
  }

  .breadcrumb-container {
    flex: 1; // 占满中间空间
    display: flex;
    align-items: start;
  }

  .right-menu {
    display: flex;
    align-items: center;
    padding-right: 36px; // 给右边留空间，可调整大小


    .right-menu-item {
      cursor: pointer;
      display: flex;
      align-items: center;

      &.hover-effect:hover {
        background: #f5f5f5;
        border-radius: 4px;
      }
    }

    .avatar-container {
      .avatar-wrapper {
        position: relative;
        height: 32px;
        width: 32px;
        border-radius: 50%;
        overflow: hidden;
        display: flex;
        align-items: center;
        justify-content: center;

        .user-avatar {
          font-size: 30px; // svg 大小
        }
      }
    }
  }
}
</style>
