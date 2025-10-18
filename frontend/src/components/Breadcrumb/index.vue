<script setup>
import { ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
const route = useRoute()
const router = useRouter()

const levelList = ref([])

watch(
  route, // 监听路由变化
  () => {
    setData()
  },
  { immediate: true } // 立即执行
)

function setData() {
  levelList.value = route.matched.map((item) => ({
    title: item.meta.title,
    path: item.path,
  }))
}

function handleLink(item) {
  if (item.path) {
    // 路由跳转
    router.push(item.path)
  }
}
</script>



<template>
  <el-breadcrumb class="app-breadcrumb" separator="/">
    <transition-group name="breadcrumb">
      <el-breadcrumb-item v-for="(item, index) in levelList" :key="item.path">
        <span v-if="index == levelList.length - 1" class="no-redirect">
          {{ item.title }}
        </span>
        <a v-else @click.prevent="handleLink(item)">{{ item.title }}</a>
      </el-breadcrumb-item>
    </transition-group>
  </el-breadcrumb>
</template>


<style lang="scss" scoped>
.app-breadcrumb.el-breadcrumb {
  display: inline-block;
  font-size: 14px;
  line-height: 50px;
  margin-left: 8px;

  .no-redirect {
    color: #97a8be;
    cursor: text;
  }
}
</style>
