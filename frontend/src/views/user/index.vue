<template>
  <div class="app-container">
    <el-row :gutter="20">
      <el-col :span="8" :xs="24">
        <el-card class="box-card">
          <template #header>
            <div class="clearfix">
              <span>个人信息</span>
            </div>
          </template>
          <div>
            <div class="text-center">
              <div class="user-avatar">
                <img :src="user.avatar" alt="Avatar">
              </div>
              <h3 class="user-name">{{ user.name }}</h3>
              <p class="user-intro">{{ user.introduction || '这个人很懒，什么都没留下...' }}</p>
            </div>
            <ul class="user-info">
              <li><i class="el-icon-user"></i> 用户名 <div class="user-right">{{ user.name }}</div></li>
              <li><i class="el-icon-phone"></i> 手机号码 <div class="user-right">13888888888</div></li>
              <li><i class="el-icon-message"></i> 用户邮箱 <div class="user-right">gemini@google.com</div></li>
              <li>
                <i class="el-icon-s-custom"></i> 所属角色
                <div class="user-right">
                  <el-tag v-for="role in user.roles" :key="role" size="small" style="margin-left: 5px;">{{ role }}</el-tag>
                </div>
              </li>
            </ul>
          </div>
        </el-card>
      </el-col>

      <el-col :span="16" :xs="24">
        <el-card>
          <template #header>
            <div class="clearfix">
              <span>基本资料</span>
            </div>
          </template>
          <el-tabs v-model="activeTab">
            <el-tab-pane label="基本资料" name="userinfo">
              <el-form :model="form" :rules="rules" ref="formRef" label-width="80px">
                <el-form-item label="昵称" prop="name">
                  <el-input v-model="form.name" />
                </el-form-item>
                <el-form-item label="个人简介" prop="introduction">
                  <el-input type="textarea" v-model="form.introduction" />
                </el-form-item>
                <el-form-item>
                  <el-button type="primary" @click="onSubmit">保存</el-button>
                  <el-button @click="onReset">重置</el-button>
                </el-form-item>
              </el-form>
            </el-tab-pane>
            <el-tab-pane label="修改密码" name="resetPwd">
              <el-form :model="pwdForm" :rules="pwdRules" ref="pwdFormRef" label-width="80px">
                <el-form-item label="旧密码" prop="oldPassword">
                  <el-input v-model="pwdForm.oldPassword" placeholder="请输入旧密码" show-password />
                </el-form-item>
                <el-form-item label="新密码" prop="newPassword">
                  <el-input v-model="pwdForm.newPassword" placeholder="请输入新密码" show-password />
                </el-form-item>
                <el-form-item label="确认密码" prop="confirmPassword">
                  <el-input v-model="pwdForm.confirmPassword" placeholder="请确认新密码" show-password />
                </el-form-item>
                <el-form-item>
                  <el-button type="primary" @click="onUpdatePwd">修改密码</el-button>
                </el-form-item>
              </el-form>
            </el-tab-pane>
          </el-tabs>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup>
import { ref, reactive } from 'vue';
import { useUserStore } from '@/stores/user';
import { ElMessage } from 'element-plus';

const userStore = useUserStore();
const user = {
    name: userStore.name,
    avatar: userStore.avatar || 'https://wpimg.wallstcn.com/f778738c-e4f8-4870-b634-56703b4acafe.gif', // Default avatar
    roles: userStore.roles,
    introduction: userStore.introduction
};

const activeTab = ref('userinfo');

// Profile form
const formRef = ref(null);
const form = reactive({
    name: user.name,
    introduction: user.introduction
});
const rules = {
    name: [{ required: true, message: '请输入昵称', trigger: 'blur' }]
};

// Password form
const pwdFormRef = ref(null);
const pwdForm = reactive({
    oldPassword: '',
    newPassword: '',
    confirmPassword: ''
});
const validatePass = (rule, value, callback) => {
    if (value === '') {
        callback(new Error('请输入新密码'));
    } else {
        if (pwdForm.confirmPassword !== '') {
            pwdFormRef.value.validateField('confirmPassword');
        }
        callback();
    }
};
const validatePass2 = (rule, value, callback) => {
    if (value === '') {
        callback(new Error('请再次输入密码'));
    } else if (value !== pwdForm.newPassword) {
        callback(new Error("两次输入密码不一致!"));
    } else {
        callback();
    }
};
const pwdRules = {
    oldPassword: [{ required: true, message: '请输入旧密码', trigger: 'blur' }],
    newPassword: [{ required: true, validator: validatePass, trigger: 'blur' }],
    confirmPassword: [{ required: true, validator: validatePass2, trigger: 'blur' }]
};


const onSubmit = () => {
    formRef.value.validate(valid => {
        if (valid) {
            // Here you would typically call an API to update the user profile
            // For now, just show a success message
            ElMessage.success('个人信息更新成功！');
            // You might want to update the store as well
            userStore.name = form.name;
            userStore.introduction = form.introduction;
        }
    });
};

const onReset = () => {
    form.name = user.name;
    form.introduction = user.introduction;
};

const onUpdatePwd = () => {
    pwdFormRef.value.validate(valid => {
        if (valid) {
            // API call to update password
            ElMessage.success('密码修改成功，下次登录请使用新密码！');
            pwdFormRef.value.resetFields();
        }
    });
};

</script>

<style lang="scss" scoped>
.app-container {
  padding: 20px;
}
.box-card {
  margin-bottom: 20px;
}

.text-center {
  text-align: center;
  padding: 10px 0;
}

.user-avatar {
  margin-bottom: 20px;
  img {
    width: 100px;
    height: 100px;
    border-radius: 50%;
  }
}

.user-name {
    font-weight: 600;
    font-size: 1.5rem;
    margin: 0 0 5px;
}

.user-intro {
    color: #777;
    font-size: 0.9rem;
}

.user-info {
  list-style: none;
  padding-left: 0;
  li {
    border-bottom: 1px solid #f0f3f4;
    padding: 11px 0;
    font-size: 13px;
    .user-right {
      float: right;
      color: #999;
    }
  }
}
</style>
