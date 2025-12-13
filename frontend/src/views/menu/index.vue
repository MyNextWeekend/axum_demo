<script setup>
import { reactive, ref, nextTick } from 'vue'
import { onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { MenuAdd, MenuDelete, MenuUpdate, MenuQuery } from '@/apis/menu.js'
import { MENU_STATUS } from '@/enums/common'
import { enumToLabel } from '@/utils/enumUtils'
import { Plus } from '@element-plus/icons-vue'

// 操作模式对应标题
const textMap = reactive({
    update: '修改菜单',
    create: '新增菜单'
})
const dialogStatus = ref("")
const dialogFormVisible = ref(false)
const loading = ref(false)
const formRef = ref(null)

const listData = ref([])
const menuTree = ref([])
const infoData = reactive({
    id: null,
    parentId: null,
    path: null,
    name: null,
    component: null,
    redirect: null,
    sort: null,
    meta: null,
    status: null,
    remark: null,
})
// 查询参数
const queryParams = reactive({
    filters: [],
    logic: 'and',
    page: 0,
    size: 20,
    sorts: []
})
const rules = {
    name: [{ required: true, message: '请输入菜单名称', trigger: 'blur' }],
    path: [{ required: true, message: '请输入路径', trigger: 'blur' }],
    status: [{ required: true, message: '请选择状态', trigger: 'change' }]
}

// 重置 infoData（保持响应性）
function resetInfoData() {
    // 初始化 reactive 包裹的对象， 不丢失 响应性
    Object.assign(infoData, {
        id: null,
        parentId: null,
        path: null,
        name: null,
        component: null,
        redirect: null,
        sort: 0,
        meta: null,
        status: null,
        remark: null,
    })
}
// 打开创建弹窗
function handleCreate() {
    resetInfoData()
    dialogStatus.value = 'create'
    dialogFormVisible.value = true
    // 等弹窗渲染后清除验证状态
    nextTick(() => {
        formRef.value?.clearValidate()
    })
}
// 新增
async function createData() {
    formRef.value.validate(async valid => {
        if (!valid) return
        try {
            loading.value = true
            const res = await MenuAdd(infoData)
            if (res.code === 0) {
                ElMessage.success('新增成功')
                dialogFormVisible.value = false
                handleSearch()
            } else {
                ElMessage.error(res.message || '新增失败')
            }
        } catch (err) {
            console.error(err)
        } finally {
            loading.value = false
        }
    })
}

function handleDelete(row) {
    ElMessageBox.confirm(
        `确认删除「${row.menu.name}」吗？`,
        '提示',
        {
            confirmButtonText: '确定',
            cancelButtonText: '取消',
            type: 'warning',
        }
    ).then(async () => {
        try {
            const res = await MenuDelete({ id: row.menu.id })
            if (res.code === 0) {
                ElMessage.success('删除成功')
                handleSearch()
            } else {
                ElMessage.error(res.message || '删除失败')
            }
        } catch (err) {
            console.error(err)
        }
    })
        .catch(() => { })
}

function handleUpdate(row) {
    Object.assign(infoData, row.menu) // copy obj 或者 调用 API 查询详情
    dialogStatus.value = 'update'
    dialogFormVisible.value = true
    // 等弹窗渲染后清除验证状态
    nextTick(() => {
        formRef.value?.clearValidate()
    })
}

// 更新数据
async function updateData() {
    formRef.value.validate(async valid => {
        if (!valid) return
        try {
            loading.value = true
            const res = await MenuUpdate(infoData)
            if (res.code === 0) {
                ElMessage.success('修改成功')
                dialogFormVisible.value = false
                handleSearch()
            } else {
                ElMessage.error(res.message || '修改失败')
            }
        } catch (err) {
            console.error(err)
        } finally {
            loading.value = false
        }
    })
}
// 查询菜单列表
async function handleSearch() {
    try {
        loading.value = true
        const res = await MenuQuery(queryParams)
        if (res.code === 0) {
            listData.value = res.data || []
            menuTree.value = [
                { id: 0, label: '根节点', children: res.data } // 手动加一个根节点
            ]
        } else {
            ElMessage.error(res.message || '加载失败')
        }
    } catch (err) {
        console.error(err)
    } finally {
        loading.value = false
    }
}

onMounted(() => {
    handleSearch()
})
</script>
<template>
    <div class="app-container">
        <el-card shadow="never">
            <div class="toolbar">
                <el-button type="primary" :icon="Plus" @click="handleCreate">新增</el-button>
            </div>

            <el-table v-loading="loading" :data="listData" row-key="menu.id" border stripe>
                <el-table-column fixed prop="menu.name" label="路由名称" />
                <el-table-column prop="menu.component" label="组件标识/路径" width="200"/>
                <el-table-column prop="menu.meta" label="路由元信息" />
                 <el-table-column prop="menu.sort" label="排序" width="80" align="center"/>
                <el-table-column prop="menu.status" label="状态" width="100" align="center">
                    <template #default="{ row }">
                        <el-tag :type="row.menu.status === 1 ? 'success' : 'info'">
                            {{ enumToLabel(MENU_STATUS, row.menu.status) }}
                        </el-tag>
                    </template>
                </el-table-column>
                <el-table-column prop="menu.remark" label="备注" />
                <el-table-column fixed="right" label="操作" min-width="120" align="center">
                    <template #default="{ row }">
                        <el-button link type="primary" size="small" @click="handleUpdate(row)">编辑</el-button>
                        <el-button link type="danger" size="small" @click="handleDelete(row)">删除</el-button>
                    </template>
                </el-table-column>
            </el-table>
        </el-card>

        <el-dialog v-model="dialogFormVisible" :title="textMap[dialogStatus]" width="600px" :close-on-click-modal="false">
            <el-form :model="infoData" :rules="rules" ref="formRef" label-width="100px">
                <el-form-item label="父节点" prop="parentId">
                    <el-tree-select v-model="infoData.parentId" :data="menuTree"
                        :props="{ value: 'id', label: 'label', children: 'children' }" placeholder="请选择父节点"
                        check-strictly :render-after-expand="false" :default-expand-all="true" clearable style="width: 100%"/>
                </el-form-item>
                <el-form-item label="菜单名称" prop="name">
                    <el-input v-model="infoData.name" placeholder="请输入菜单名称" autocomplete="off" />
                </el-form-item>
                <el-form-item label="路由路径" prop="path">
                    <el-input v-model="infoData.path" placeholder="请输入路由路径" autocomplete="off" />
                </el-form-item>
                <el-form-item label="Vue组件路径" prop="component">
                    <el-input v-model="infoData.component" placeholder="例如：views/user/index.vue" autocomplete="off" />
                </el-form-item>
                <el-form-item label="重定向" prop="redirect">
                    <el-input v-model="infoData.redirect" placeholder="请输入重定向路径" autocomplete="off" />
                </el-form-item>
                <el-form-item label="排序" prop="sort">
                    <el-input-number v-model="infoData.sort" :min="0" controls-position="right" autocomplete="off" style="width: 30%"/>
                </el-form-item>
                <el-form-item label="扩展" prop="meta">
                    <el-input v-model="infoData.meta" placeholder='例如：{"icon": "user"}' autocomplete="off" />
                </el-form-item>
                <el-form-item label="状态" prop="status">
                    <el-select v-model="infoData.status" placeholder="请选择状态" style="width: 100%">
                        <el-option v-for="item in MENU_STATUS" :key="item.value" :label="item.label"
                            :value="item.value" />
                    </el-select>
                </el-form-item>
                <el-form-item label="备注" prop="remark">
                    <el-input v-model="infoData.remark" type="textarea" autocomplete="off" />
                </el-form-item>
            </el-form>
            <template #footer>
                <div class="dialog-footer">
                    <el-button @click="dialogFormVisible = false">取消</el-button>
                    <el-button type="primary" :loading="loading" @click="dialogStatus === 'create' ? createData() : updateData()">
                        提交
                    </el-button>
                </div>
            </template>
        </el-dialog>
    </div>
</template>


<style lang="scss" scoped>
.app-container {
  padding: 10px;
}

.toolbar {
    margin-bottom: 20px;
}

</style>
