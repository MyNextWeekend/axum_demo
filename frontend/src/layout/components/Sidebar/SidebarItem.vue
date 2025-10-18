<script setup>
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import Item from './Item.vue'

// 接收父组件传来的 props
const prop = defineProps({
  item: {
    type: Object,
    required: true
  },
  basePath: {
    type: String,
    default: ''
  }
})

const route = useRoute()
const router = useRouter()

// 当前激活菜单
const activePath = computed(() => route.path)

// 点击菜单跳转
function handleClick(path) {
  console.log("跳转路由至: ", path)
  router.push(path)
}
</script>

<template>
  <!-- 子菜单 只有一个 且 item 没有 meta属性 -->
  <el-menu-item v-if="item.children?.length === 1 && item.meta === undefined" :index="item.children[0]?.path"
    @click="handleClick(item.children[0]?.path || '')">
    <item v-if="item.children[0].meta?.title" :icon="item.children[0].meta?.icon" :title="item.children[0].meta?.title">
    </item>
  </el-menu-item>

  <!-- 没有 子菜单 -->
  <el-menu-item v-else-if="item.children?.length === 0 && item.meta !== undefined" :index="item.path"
    @click="handleClick(item.path || '')">
    <item v-if="item.meta?.title" :icon="item.meta?.icon" :title="item.meta?.title">
    </item>
  </el-menu-item>

  <!-- 剩余有 子菜单 的情况 -->
  <el-sub-menu v-else-if="item.children?.length" :index="item.path">
    <template #title>
      <item v-if="item.meta?.title" :icon="item.meta?.icon" :title="item.meta?.title"></item>
    </template>
    <sidebar-item v-for="child in item.children" :key="child.path" :item="child"></sidebar-item>
  </el-sub-menu>


</template>
