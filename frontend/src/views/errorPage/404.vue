<template>
    <div class="not-found">
        <div class="card" role="main" aria-labelledby="title">
            <h1 id="title" class="code">404</h1>
            <p class="message">抱歉，你访问的页面不存在或已被移走。</p>

            <div class="actions">
                <button class="btn" @click="goHome" aria-label="回到首页">回到首页</button>
                <button class="btn outline" @click="goBack" aria-label="返回上一页">返回上一页</button>
            </div>

            <p class="hint">请检查地址是否输入正确，或使用导航返回。</p>
        </div>

        <!-- 装饰性几何图形，仅 CSS，不是图片 -->
        <div class="dots" aria-hidden="true">
            <span></span><span></span><span></span><span></span><span></span>
        </div>
    </div>
</template>

<script setup>
import { useRouter } from 'vue-router'

const router = useRouter()

function goHome() {
    // 使用 replace 避免回退堆栈中留下这个 404 页
    router.replace({ path: '/' })
}

function goBack() {
    // 如果没有历史记录就回到首页
    if (window.history.length > 1) {
        router.back()
    } else {
        router.replace({ path: '/' })
    }
}
</script>

<style scoped>
/* 布局 */
.not-found {
    min-height: 70vh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 48px 16px;
    background: linear-gradient(180deg, #f6fbff 0%, #ffffff 100%);
    color: #0f1724;
    font-family: Inter, "Helvetica Neue", Arial, sans-serif;
}

/* 卡片 */
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

/* 大号 404 */
.code {
    margin: 0;
    font-size: 96px;
    line-height: 1;
    letter-spacing: -4px;
    font-weight: 800;
    color: #0b1220;
    background: linear-gradient(90deg, #0ea5e9, #7c3aed);
    -webkit-background-clip: text;
    background-clip: text;
    color: transparent;
    text-shadow: 0 6px 30px rgba(124, 58, 237, 0.08);
}

/* 描述信息 */
.message {
    margin: 12px 0 20px;
    font-size: 18px;
    color: #394151;
}

/* 按钮组 */
.actions {
    display: flex;
    gap: 12px;
    justify-content: center;
    margin: 18px 0 10px;
    flex-wrap: wrap;
}

.btn {
    min-width: 140px;
    padding: 10px 18px;
    border-radius: 10px;
    border: none;
    cursor: pointer;
    font-weight: 600;
    font-size: 14px;
    background: linear-gradient(90deg, #4f46e5, #06b6d4);
    color: #fff;
    box-shadow: 0 6px 18px rgba(6, 182, 212, 0.12);
    transition: transform .12s ease, box-shadow .12s ease, opacity .12s;
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

/* 辅助提示 */
.hint {
    margin-top: 14px;
    color: #7b8390;
    font-size: 13px;
}

/* 装饰圆点，仅 CSS */
.dots {
    position: absolute;
    inset: 0;
    pointer-events: none;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 14px;
}

.dots span {
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: linear-gradient(180deg, rgba(124, 58, 237, 0.14), rgba(6, 182, 212, 0.08));
    transform: translateY(40px) scale(0.8);
    opacity: 0;
    animation: float 1800ms ease-in-out infinite;
}

.dots span:nth-child(1) {
    animation-delay: 0ms;
}

.dots span:nth-child(2) {
    animation-delay: 200ms;
    transform: translateY(30px) scale(0.9);
}

.dots span:nth-child(3) {
    animation-delay: 400ms;
    transform: translateY(20px) scale(1);
}

.dots span:nth-child(4) {
    animation-delay: 600ms;
    transform: translateY(30px) scale(0.92);
}

.dots span:nth-child(5) {
    animation-delay: 800ms;
    transform: translateY(40px) scale(0.86);
}

@keyframes float {
    0% {
        transform: translateY(40px) scale(0.85);
        opacity: 0;
    }

    35% {
        transform: translateY(0) scale(1);
        opacity: 1;
    }

    70% {
        transform: translateY(-6px) scale(0.98);
        opacity: 1;
    }

    100% {
        transform: translateY(40px) scale(0.85);
        opacity: 0;
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
