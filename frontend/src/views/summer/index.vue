<template>
  <div class="app-container">
    <el-card shadow="never">
      <!-- Search Form -->
      <el-form :model="queryParams" ref="queryForm" :inline="true" label-width="68px">
        <el-form-item label="用户名" prop="username">
          <el-input
            v-model="queryParams.username"
            placeholder="请输入用户名"
            clearable
            @keyup.enter="handleQuery"
          />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" :icon="Search" @click="handleQuery">搜索</el-button>
          <el-button :icon="Refresh" @click="resetQuery">重置</el-button>
        </el-form-item>
      </el-form>

      <!-- Action Toolbar -->
      <el-row :gutter="10" class="mb8">
        <el-col :span="1.5">
          <el-button
            type="primary"
            plain
            :icon="Plus"
            @click="handleCreate"
          >新增</el-button>
        </el-col>
      </el-row>

      <!-- User Table -->
      <el-table v-loading="loading" :data="userList" @sort-change="handleSortChange">
        <el-table-column label="ID" prop="id" sortable="custom" width="100" />
        <el-table-column label="用户名" prop="username" sortable="custom" />
        <el-table-column label="邮箱" prop="email" />
        <el-table-column label="状态" prop="status" sortable="custom">
           <template #default="{ row }">
              <el-tag :type="row.status === 1 ? 'success' : 'info'">
                {{ row.status === 1 ? '正常' : '禁用' }}
              </el-tag>
            </template>
        </el-table-column>
        <el-table-column label="创建时间" prop="created_at" sortable="custom">
            <template #default="{ row }">
                <span>{{ new Date(row.created_at).toLocaleString() }}</span>
            </template>
        </el-table-column>
        <el-table-column label="操作" width="180" class-name="small-padding fixed-width">
          <template #default="{ row }">
            <el-button link type="primary" :icon="Edit" @click="handleUpdate(row)">编辑</el-button>
            <el-button link type="danger" :icon="Delete" @click="handleDelete(row)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>

      <!-- Pagination -->
      <el-pagination
        v-show="total > 0"
        :total="total"
        v-model:page="queryParams.page"
        v-model:limit="queryParams.size"
        @pagination="getList"
      />
    </el-card>

    <!-- Add/Edit Dialog -->
    <el-dialog :title="dialog.title" v-model="dialog.visible" width="600px" append-to-body>
      <el-form ref="form" :model="form" :rules="rules" label-width="80px">
        <el-form-item label="用户名" prop="username">
          <el-input v-model="form.username" placeholder="请输入用户名" />
        </el-form-item>
        <el-form-item label="邮箱" prop="email">
          <el-input v-model="form.email" placeholder="请输入邮箱" />
        </el-form-item>
        <el-form-item label="密码" prop="password" v-if="dialog.status === 'create'">
          <el-input v-model="form.password" placeholder="请输入密码" type="password" />
        </el-form-item>
        <el-form-item label="状态" prop="status">
          <el-radio-group v-model="form.status">
            <el-radio :label="1">正常</el-radio>
            <el-radio :label="0">禁用</el-radio>
          </el-radio-group>
        </el-form-item>
      </el-form>
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="dialog.visible = false">取 消</el-button>
          <el-button type="primary" @click="submitForm">确 定</el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue';
import { ElMessage, ElMessageBox } from 'element-plus';
import { UserQuery, UserAdd, UserUpdate, UserDelete } from '@/apis/user.js';
import { Search, Refresh, Plus, Edit, Delete } from '@element-plus/icons-vue';

// Reactive State
const loading = ref(true);
const userList = ref([]);
const total = ref(0);

const queryParams = reactive({
  page: 1,
  size: 10,
  username: '',
  sorts: []
});

const dialog = reactive({
  visible: false,
  title: '',
  status: 'create'
});

const form = ref({
  id: null,
  username: '',
  email: '',
  password: '',
  status: 1,
});

const rules = {
  username: [{ required: true, message: '用户名不能为空', trigger: 'blur' }],
  email: [{ required: true, message: '邮箱不能为空', trigger: 'blur' }, { type: 'email', message: '请输入正确的邮箱地址', trigger: ['blur', 'change'] }],
  password: [{ required: true, message: '密码不能为空', trigger: 'blur' }]
};

// Functions
async function getList() {
  loading.value = true;
  const params = {
    page: queryParams.page -1,
    size: queryParams.size,
    filters: [],
    sorts: queryParams.sorts
  }
  if(queryParams.username) {
    params.filters.push({
        name: 'username',
        value: queryParams.username
    })
  }

  const res = await UserQuery(params);
  if (res.code === 0 && res.data) {
    userList.value = res.data.list;
    total.value = res.data.total;
  } else {
    userList.value = [];
    total.value = 0;
  }
  loading.value = false;
}

function handleQuery() {
  queryParams.page = 1;
  getList();
}

function resetQuery() {
  queryParams.page = 1;
  queryParams.username = '';
  queryParams.sorts = [];
  handleQuery();
}

function handleSortChange(data) {
  const { prop, order } = data;
  queryParams.sorts = order ? [{ name: prop, sort: order === 'ascending' ? 'asc' : 'desc' }] : [];
  getList();
}

function resetForm() {
  form.value = {
    id: null,
    username: '',
    email: '',
    password: '',
    status: 1,
  };
}

function handleCreate() {
  resetForm();
  dialog.status = 'create';
  dialog.title = '添加用户';
  dialog.visible = true;
}

function handleUpdate(row) {
  resetForm();
  Object.assign(form.value, row);
  dialog.status = 'update';
  dialog.title = '修改用户';
  dialog.visible = true;
}

function submitForm() {
  if (dialog.status === 'create') {
    createData();
  } else {
    updateData();
  }
}

async function createData() {
  const res = await UserAdd(form.value);
  if (res.code === 0) {
    ElMessage.success('新增成功');
    dialog.visible = false;
    getList();
  } else {
    ElMessage.error(res.message || '新增失败');
  }
}

async function updateData() {
  const res = await UserUpdate(form.value);
  if (res.code === 0) {
    ElMessage.success('修改成功');
    dialog.visible = false;
    getList();
  } else {
    ElMessage.error(res.message || '修改失败');
  }
}

function handleDelete(row) {
  ElMessageBox.confirm(`确认删除用户「${row.username}」吗？`, '提示', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning',
  }).then(async () => {
    const res = await UserDelete({ id: row.id });
    if (res.code === 0) {
      ElMessage.success('删除成功');
      getList();
    } else {
      ElMessage.error(res.message || '删除失败');
    }
  }).catch(() => {});
}

// Lifecycle Hook
onMounted(() => {
  getList();
});
</script>

<style lang="scss" scoped>
.app-container {
  padding: 20px;
}
.mb8 {
  margin-bottom: 8px;
}
</style>
