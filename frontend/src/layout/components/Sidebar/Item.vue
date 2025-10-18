<script setup>
import { defineProps } from 'vue'
import SvgIcon from '@/components/SvgIcon/index.vue'

defineProps({
  icon: {
    type: String,
    required: true
  },
  title: {
    type: String,
    required: true
  }
})
</script>

<template>
  <SvgIcon v-if="icon" :iconClass="icon" class="sub-el-icon" />
  <span class="title">{{ title }}</span>
</template>

<style lang="scss" scoped>
.sub-el-icon {
  font-size: 18px;
  margin-right: 8px; // 图标和文字间距
  display: flex;
  align-items: center;
}
</style>