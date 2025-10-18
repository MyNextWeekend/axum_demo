<template>
    <div class="unauthorized">
        <div class="card" role="main" aria-labelledby="title">
            <h1 id="title" class="code">401</h1>
            <p class="message">你没有权限访问此页面。</p>

            <div class="actions">
                <button class="btn" @click="goLogin" aria-label="前往登录">前往登录</button>
                <button class="btn outline" @click="goBack" aria-label="返回上一页">返回上一页</button>
            </div>

            <p class="hint">如果你认为这是错误，请联系管理员或重新登录。</p>
        </div>

        <!-- CSS 装饰元素 -->
        <div class="bars" aria-hidden="true">
            <span></span><span></span><span></span><span></span>
        </div>
    </div>
</template>

<script setup>
import { useRouter } from 'vue-router'

const router = useRouter()

function goLogin() {
    router.replace({ path: '/login' })
}

function goBack() {
    if (window.history.length > 1) {
        router.back()
    } else {
        router.replace({ path: '/' })
    }
}
</script>

<style scoped>
.unauthorized {
    min-height: 70vh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 48px 16px;
    background: linear-gradient(180deg, #fefcfb 0%, #ffffff 100%);
    color: #111827;
    font-family: Inter, "Helvetica Neue", Arial, sans-serif;
}

/* 卡片布局 */
.card {
    width: 100%;
    max-width: 720px;
    text-align: center;
    background: white;
    border-radius: 14px;
    padding: 40px 32px;
    box-shadow: 0 8px 30px rgba(15, 23, 36, 0.08);
    position: relative;
    overflow: hidden;
}

/* 标题数字 */
.code {
    margin: 0;
    font-size: 96px;
    font-weight: 800;
    letter-spacing: -4px;
    background: linear-gradient(90deg, #f97316, #facc15);
    -webkit-background-clip: text;
    background-clip: text;
    color: transparent;
    text-shadow: 0 6px 30px rgba(249, 115, 22, 0.1);
}

/* 描述信息 */
.message {
    margin: 12px 0 20px;
    font-size: 18px;
    color: #394151;
}

/* 按钮 */
.actions {
    display: flex;
    justify-content: center;
    gap: 12px;
    flex-wrap: wrap;
    margin: 18px 0 10px;
}

.btn {
    min-width: 140px;
    padding: 10px 18px;
    border-radius: 10px;
    border: none;
    cursor: pointer;
    font-weight: 600;
    font-size: 14px;
    background: linear-gradient(90deg, #f97316, #f59e0b);
    color: #fff;
    box-shadow: 0 6px 18px rgba(249, 115, 22, 0.15);
    transition: transform .12s ease, box-shadow .12s ease;
}

.btn:hover {
    transform: translateY(-3px);
}

.btn:active {
    transform: translateY(-1px);
}

.btn.outline {
    background: transparent;
    color: #374151;
    border: 1px solid rgba(55, 65, 81, 0.08);
    box-shadow: none;
}

.btn.outline:hover {
    box-shadow: 0 6px 18px rgba(15, 23, 36, 0.04);
}

/* 提示信息 */
.hint {
    margin-top: 14px;
    color: #7b8390;
    font-size: 13px;
}

/* 装饰条 */
.bars {
    position: absolute;
    inset: 0;
    pointer-events: none;
    display: flex;
    justify-content: center;
    align-items: flex-end;
    gap: 8px;
}

.bars span {
    width: 6px;
    height: 20px;
    border-radius: 4px;
    background: linear-gradient(180deg, rgba(249, 115, 22, 0.2), rgba(245, 158, 11, 0.1));
    animation: pulse 1.4s infinite ease-in-out;
}

.bars span:nth-child(2) {
    animation-delay: 0.15s;
}

.bars span:nth-child(3) {
    animation-delay: 0.3s;
}

.bars span:nth-child(4) {
    animation-delay: 0.45s;
}

@keyframes pulse {

    0%,
    100% {
        height: 20px;
        opacity: 0.6;
    }

    50% {
        height: 40px;
        opacity: 1;
    }
}

/* 响应式 */
@media (max-width: 520px) {
    .code {
        font-size: 64px;
    }

    .card {
        padding: 28px 18px;
    }

    .btn {
        min-width: 120px;
    }
}
</style>
