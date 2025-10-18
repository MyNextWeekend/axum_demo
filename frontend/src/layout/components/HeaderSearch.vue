<script setup>
import { ref, watch, computed } from 'vue'
import SvgIcon from '@/components/SvgIcon/index.vue'
import { useUserStore } from '@/stores/user.js'
import { useRouter } from 'vue-router'
import Fuse from 'fuse.js'

const router = useRouter()
const userStore = useUserStore()

const isShow = ref(false)
const searchRef = ref(null)
const searchOption = ref([])
const search = ref('')

const onShowClick = () => {
    isShow.value = !isShow.value
    searchRef.value.focus() // 光标自动激活
}


function generateRoutes(routes, prefixTitle = []) {
    let res = []
    for (const router of routes) {
        let title
        if (router.meta) {
            title = [...prefixTitle, router.meta.title]
            res.push({
                path: router.path,
                title: title
            })
        } else {
            title = prefixTitle
        }
        if (router.children) {
            const tempRoutes = generateRoutes(router.children, title)
            if (tempRoutes.length >= 1) {
                res = [...res, ...tempRoutes]
            }
        }
    }
    return res
}

let fuse;

function querySearch(query) {
    if (query !== '') {
        searchOption.value = fuse.search(query)
    } else {
        searchOption.value = []
    }
}

const initFuse = (data) => {
    fuse = new Fuse(data, {
        shouldSort: true,
        threshold: 0.4,
        location: 0,
        distance: 100,
        maxPatternLength: 32,
        minMatchCharLength: 1,
        keys: [{
            name: 'title',
            weight: 0.7
        }, {
            name: 'path',
            weight: 0.3
        }]
    })
}

const selectChange = (val) => {
    router.push(val.path)
}

watch(isShow, (val) => {
    if (val) {
        document.body.addEventListener('click', searchClose)
    }
})

const searchClose = () => {
    isShow.value = false
    searchRef.value.blur()
    searchOption.value = []
    search.value = ''
}

const searchDataPool = computed(() => {
    return generateRoutes(userStore.routes)
})
initFuse(searchDataPool.value)
</script>

<template>
    <div :class="{ show: isShow }" class="header-search">
        <SvgIcon iconClass="search" className="search" @click.stop="onShowClick" />
        <el-select ref="searchRef" v-model="search" :remote-method="querySearch" filterable default-first-option remote
            placeholder="菜单搜索" class="header-search-select" @change="selectChange">
            <el-option v-for="opt in searchOption" :key="opt.item.path" :value="opt.item"
                :label="opt.item.title.join(' > ')" />
        </el-select>
    </div>

</template>

<style lang="scss" scoped>
.header-search {
    display: flex;
    height: 100%;
    align-items: center;
    justify-content: center;

    .search {
        height: 25px;
        width: 25px;

    }

    .header-search-select {
        transition: width 0.2s;
        font-size: 18px;
        width: 0;
        overflow: hidden;
        border-radius: 0;
        background: transparent;
        display: inline-block;

        :deep(.el-select__wrapper) {
            border: 0;
            border-radius: 0;
            padding-left: 0;
            padding-right: 0;
            box-shadow: none;
            vertical-align: middle;
            border-bottom: 1px solid #d9d9d9;
        }
    }

    &.show {
        .header-search-select {
            width: 210px;
            margin-left: 10px;
        }
    }


}
</style>