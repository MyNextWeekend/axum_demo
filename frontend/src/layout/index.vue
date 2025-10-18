<script setup>
import AppMain from './components/AppMain.vue'
import Navbar from './components/Navbar.vue'
import Sidebar from './components/Sidebar/index.vue'
import TagsView from './components/TagsView/index.vue'

import { useMenuStore } from '@/stores/menu'

const menuStore = useMenuStore()

</script>


<template>
  <div class="app-wrapper" :class="[menuStore.opened ? 'openSidebar' : 'hideSidebar']">
    <sidebar class="sidebar-container" />
    <div class="main-container">
      <div class="header">
        <navbar />
        <tags-view />
      </div>
      <app-main />
    </div>
  </div>
</template>


<style lang="scss" scoped>
.app-wrapper {
  display: flex;
  width: 100%;
  height: 100vh; // 占满整个视口高度
  overflow: hidden;

  // 打开侧边栏
  &.openSidebar {
    .sidebar-container {
      width: 190px; // 展开宽度
    }

    .main-container {
      margin-left: 190px;
    }
  }

  // 收起侧边栏
  &.hideSidebar {
    .sidebar-container {
      width: 54px; // 收缩宽度
    }

    .main-container {
      margin-left: 54px;
    }
  }
}

.sidebar-container {
  position: fixed;
  top: 0;
  left: 0;
  bottom: 0;
  background: #304156; // 深色背景
  transition: width 0.3s ease;
  overflow: hidden;
}

.main-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  transition: margin-left 0.3s ease;
  min-height: 100vh;
  background: #f0f2f5;
}

.header {
  height: 84px;
  display: flex;
  flex-direction: column;
  background: #fff;
  border-bottom: 1px solid #e0e0e0;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.08);
}

app-main {
  flex: 1; // 占满剩余空间
  overflow: auto;
  padding: 20px;
  background: #fff;
}
</style>
