<script setup>
import { reactive, ref } from 'vue'
import { onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { MenuAdd, MenuDelete, MenuUpdate, MenuQuery, MenuInfo } from '@/apis/menu.js'

// 操作模式对应标题
const textMap = reactive({
    update: '修改菜单',
    create: '新增菜单'
})
const dialogStatus = ref("")
const dialogFormVisible = ref(false)
const loading = ref(false)

const listData = ref([])
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

// 重置 infoData（保持响应性）
function resetInfoData() {
    // 初始化 reactive 包裹的对象， 不丢失 响应性
    Object.assign(infoData, {
        id: null,
        parent_: null,
        path: null,
        name: null,
        component: null,
        redirect: null,
        sort: null,
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
}
// 新增
async function createData() {
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
        ElMessage.error('请求出错')
    } finally {
        loading.value = false
    }
}

function handleDelete(row) {
    ElMessageBox.confirm(`确认删除「${row.name}」吗？`, '提示', {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
    })
        .then(async () => {
            try {
                const res = await MenuDelete({ id: row.id })
                if (res.code === 0) {
                    ElMessage.success('删除成功')
                    handleSearch()
                } else {
                    ElMessage.error(res.message || '删除失败')
                }
            } catch (err) {
                console.error(err)
                ElMessage.error('请求出错')
            }
        })
        .catch(() => { })
}

function handleUpdate(row) {
    Object.assign(infoData, row) // copy obj 或者 调用 API 查询详情
    dialogStatus.value = 'update'
    dialogFormVisible.value = true
}

// 更新数据
async function updateData() {
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
        ElMessage.error('请求出错')
    } finally {
        loading.value = false
    }
}
// 查询菜单列表
async function handleSearch() {
    try {
        loading.value = true
        const res = await MenuQuery(queryParams)
        if (res.code === 0) {
            listData.value = res.data || []
        } else {
            ElMessage.error(res.message || '加载失败')
        }
    } catch (err) {
        console.error(err)
        ElMessage.error('请求出错')
    } finally {
        loading.value = false
    }
}

onMounted(() => {
    handleSearch()
})
</script>
<template>
    <div class="content">
        <el-radio-group>
            <el-radio-button label="新增" @click="handleCreate" />
        </el-radio-group>

        <el-table :data="listData" row-key="id">
            <el-table-column fixed prop="name" label="路由名称" />
            <el-table-column prop="component" label="组件标识/路径" />
            <el-table-column prop="meta" label="路由元信息" />
            <el-table-column prop="status" label="状态" />
            <el-table-column prop="remark" label="备注" />
            <el-table-column fixed="right" label="操作" min-width="50">
                <template #default="{ row }">
                    <el-button link type="primary" size="small" @click="handleUpdate(row)">编辑</el-button>
                    <el-button link type="danger" size="small" @click="handleDelete(row)">删除</el-button>
                </template>
            </el-table-column>
        </el-table>

        <el-dialog v-model="dialogFormVisible" :title="textMap[dialogStatus]" width="500px">
            <el-form :model="infoData">
                <div class="dialog-item">
                    <el-form-item label="父节点" label-width="140px">
                        <el-select v-model.number="infoData.parentId" placeholder="请选择">
                            <el-option label="根节点" value="1" />
                            <el-option label="other" value="2" />
                        </el-select>
                    </el-form-item>
                    <el-form-item label="路由路径" label-width="140px">
                        <el-input v-model="infoData.path" autocomplete="off" />
                    </el-form-item>
                    <el-form-item label="菜单名称" label-width="140px">
                        <el-input v-model="infoData.name" autocomplete="off" />
                    </el-form-item>
                    <el-form-item label="vue路径" label-width="140px">
                        <el-input v-model="infoData.component" autocomplete="off" />
                    </el-form-item>
                    <el-form-item label="重定向" label-width="140px">
                        <el-input v-model="infoData.redirect" autocomplete="off" />
                    </el-form-item>
                    <el-form-item label="排序" label-width="140px">
                        <el-input-number v-model="infoData.sort" autocomplete="off" />
                    </el-form-item>
                    <el-form-item label="扩展" label-width="140px">
                        <el-input v-model="infoData.meta" autocomplete="off" />
                    </el-form-item>
                    <el-form-item label="状态" label-width="140px">
                        <el-input-number v-model="infoData.status" autocomplete="off" />
                    </el-form-item>
                    <el-form-item label="备注" label-width="140px">
                        <el-input v-model="infoData.remark" autocomplete="off" />
                    </el-form-item>
                </div>
            </el-form>
            <template #footer>
                <div class="dialog-footer">
                    <el-button @click="dialogFormVisible = false">取消</el-button>
                    <el-button type="primary" @click="dialogStatus === 'create' ? createData() : updateData()">
                        提交
                    </el-button>
                </div>
            </template>
        </el-dialog>
    </div>

</template>


<style lang="scss" scoped>
.content {
    width: 100%;

    .dialog-item {
        width: 400px;
    }
}
</style>