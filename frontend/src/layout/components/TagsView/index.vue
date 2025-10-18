<script setup>
// 标签页
import { ref, reactive, watch } from 'vue'
import { useRoute } from 'vue-router'
import SvgIcon from '@/components/SvgIcon/index.vue'
import { useMenuStore } from '@/stores/menu.js'
import ContextMenum from './ContextMenum.vue'

// 控制右键菜单显示隐藏
const visible = ref(false)
// 右键菜单位置
const floatMenu = reactive({
  top: 0,
  left: 0
})
const route = useRoute()
const menuStore = useMenuStore()

const isActive = (tag) => {
  return tag.path === route.path
}

const selectIndex = ref(0)
// 右键菜单
const openMenu = (event, index) => {
  const { clientX: left, clientY: top } = event
  floatMenu.top = top + 'px'
  floatMenu.left = left + 'px'
  selectIndex.value = index
  visible.value = true
}
const closeMenu = () => {
  visible.value = false
}

watch(visible, (value) => {
  if (value) {
    document.addEventListener('click', closeMenu)
  } else {
    document.removeEventListener('click', closeMenu)
  }
})

</script>

<template>
  <div id="tags-view-container" class="tags-view-container">
    <el-scrollbar class="tags-view-wrapper">
      <router-link v-for="(item, index) in menuStore.tagsViewList" :key="item.fullPath" :to="{ path: item.fullPath }"
        class="tags-view-item" :class="isActive(item) ? 'active' : ''" @contextmenu.prevent="openMenu($event, index)">
        {{ item.title }}
        <SvgIcon iconClass="close" className="close" @click.prevent.stop="menuStore.removeTagView(index, 'index')">
        </SvgIcon>
      </router-link>
    </el-scrollbar>
    <context-menum v-show="visible" :style="floatMenu" :index="selectIndex" class="contextmenu"></context-menum>
  </div>
</template>


<style lang="scss" scoped>
.tags-view-container {
  display: flex;
  height: 34px;
  width: 100%;
  background: #fff;
  border-bottom: 1px solid #d8dce5;
  box-shadow: 0 1px 3px 0 rgba(0, 0, 0, .12), 0 0 3px 0 rgba(0, 0, 0, .04);

  .tags-view-wrapper {
    .tags-view-item {
      display: inline-block;
      position: relative;
      cursor: pointer;
      height: 26px;
      line-height: 26px;
      border: 1px solid #d8dce5;
      color: #495060;
      background: #fff;
      padding: 0 8px;
      font-size: 12px;
      margin-left: 5px;
      margin-top: 4px;

      &:first-of-type {
        margin-left: 15px;
      }

      &:last-of-type {
        margin-right: 15px;
      }

      &.active {
        background-color: #42b983;
        color: #fff;
        border-color: #42b983;
      }
    }

    .close {
      width: 7px;
      margin-left: 5px;
    }
  }

  .contextmenu {
    margin: 0;
    background: #fff;
    z-index: 3000;
    position: absolute;
    list-style-type: none;
    padding: 5px 0;
    border-radius: 4px;
    font-size: 12px;
    font-weight: 400;
    color: #333;
    box-shadow: 2px 2px 3px 0 rgba(0, 0, 0, .3);

    li {
      margin: 0;
      padding: 7px 16px;
      cursor: pointer;

      &:hover {
        background: #eee;
      }
    }
  }
}
</style>

<style lang="scss">
//reset element css of el-icon-close
.tags-view-wrapper {
  .tags-view-item {
    .el-icon-close {
      width: 16px;
      height: 16px;
      vertical-align: 2px;
      border-radius: 50%;
      text-align: center;
      transition: all .3s cubic-bezier(.645, .045, .355, 1);
      transform-origin: 100% 50%;

      &:before {
        transform: scale(.6);
        display: inline-block;
        vertical-align: -3px;
      }

      &:hover {
        background-color: #b4bccc;
        color: #fff;
      }
    }
  }
}
</style>
