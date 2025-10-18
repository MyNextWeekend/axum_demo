<script setup>
import { defineProps } from 'vue';

const props = defineProps({
    index: {
        type: Number,
        required: true
    }
})

import { useMenuStore } from '@/stores/menu.js';
import { useRouter } from 'vue-router'

const router = useRouter()
const mentStore = useMenuStore()

const refresh = () => {
    router.go(0)
}

</script>


<template>
    <ul class="context-menu">
        <li class="context-menu-item" @click="refresh">刷新</li>
        <li class="context-menu-item" @click="mentStore.removeTagView(props.index, 'right')">关闭右侧</li>
        <li class="context-menu-item" @click="mentStore.removeTagView(props.index, 'other')">关闭其他</li>
        <li class="context-menu-item" @click="mentStore.removeTagView(props.index, 'all')">关闭全部</li>
    </ul>

</template>

<style lang="scss" scoped>
.context-menu {
    position: fixed;
    z-index: 1000;
    width: 70px;
    background: #fff;
    box-shadow: 0 2px 12px 0 rgba(0, 0, 0, .1);
    border-radius: 4px;
    padding: 5px 0;
    list-style: none; //  去掉默认小圆点
    margin: 0; //  去掉默认外边距
    padding-left: 0; //  去掉左侧缩进

    .context-menu-item {
        padding: 5px 5px;
        font-size: 13px;
        color: #333;
        cursor: pointer;

        &:hover {
            background-color: #f5f5f5;
        }
    }
}
</style>