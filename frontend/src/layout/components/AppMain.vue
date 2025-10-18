<script setup>
// 主体内容区
</script>


<template>
  <div class="app-main">
    <section>
      <!-- 添加缓存 -->
      <router-view v-slot="{ Component }">
        <transition name="fade-transform" mode="out-in">
          <keep-alive>
            <component :is="Component" />
          </keep-alive>
        </transition>
      </router-view>
    </section>
  </div>

</template>



<style lang="scss" scoped>
.app-main {
  /* 占满剩余高度 */
  flex: 1;
  min-height: calc(100vh - 84px); // 减去 navbar 高度
  width: 100%;
  position: relative;
  overflow: auto;
  background: #f0f2f5;
  padding: 5px;
  box-sizing: border-box;

  section {
    width: 100%;
    height: 100%;
    background: #fff;
    border-radius: 8px;
    padding: 10px;
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.08);
    box-sizing: border-box;
  }
}

/* 页面切换动效 */
.fade-transform-enter-active,
.fade-transform-leave-active {
  transition: all .3s ease;
}

.fade-transform-enter-from {
  opacity: 0;
  transform: translateX(-10px);
}

.fade-transform-leave-to {
  opacity: 0;
  transform: translateX(10px);
}
</style>
