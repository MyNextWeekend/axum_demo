<template>
  <div class="dashboard-container">
    <!-- Top row with stat cards -->
    <el-row :gutter="20" class="panel-group">
      <el-col :xs="12" :sm="12" :lg="6" class="card-panel-col">
        <div class="card-panel" @click="handleCardClick('new-users')">
          <div class="card-panel-icon-wrapper icon-people">
            <svg-icon icon-class="user" class-name="card-panel-icon" />
          </div>
          <div class="card-panel-description">
            <div class="card-panel-text">New Users</div>
            <span class="card-panel-num">102,400</span>
          </div>
        </div>
      </el-col>
      <el-col :xs="12" :sm="12" :lg="6" class="card-panel-col">
        <div class="card-panel" @click="handleCardClick('messages')">
          <div class="card-panel-icon-wrapper icon-message">
            <svg-icon icon-class="game" class-name="card-panel-icon" />
          </div>
          <div class="card-panel-description">
            <div class="card-panel-text">Messages</div>
            <span class="card-panel-num">81,212</span>
          </div>
        </div>
      </el-col>
      <el-col :xs="12" :sm="12" :lg="6" class="card-panel-col">
        <div class="card-panel" @click="handleCardClick('purchases')">
          <div class="card-panel-icon-wrapper icon-money">
             <svg-icon icon-class="wallet" class-name="card-panel-icon" />
          </div>
          <div class="card-panel-description">
            <div class="card-panel-text">Purchases</div>
            <span class="card-panel-num">92,880</span>
          </div>
        </div>
      </el-col>
      <el-col :xs="12" :sm="12" :lg="6" class="card-panel-col">
        <div class="card-panel" @click="handleCardClick('shoppings')">
          <div class="card-panel-icon-wrapper icon-shopping">
            <svg-icon icon-class="computer" class-name="card-panel-icon" />
          </div>
          <div class="card-panel-description">
            <div class="card-panel-text">Shoppings</div>
            <span class="card-panel-num">13,600</span>
          </div>
        </div>
      </el-col>
    </el-row>

    <!-- Second row for a larger chart or component -->
    <el-row>
        <el-col :span="24">
            <el-card>
                <template #header>
                    Welcome
                </template>
                <div>
                    <p>Welcome to your new and cool dashboard!</p>
                    <p>You can add more components here, like charts or activity feeds.</p>
                </div>
            </el-card>
        </el-col>
    </el-row>
  </div>
</template>

<script setup>
import SvgIcon from '@/components/SvgIcon/index.vue'

const handleCardClick = (cardName) => {
  console.log(`${cardName} card clicked`);
  // You can add navigation or other actions here
}
</script>

<style lang="scss" scoped>
.dashboard-container {
  padding: 32px;
  background-color: #f0f2f5;
  position: relative;
}

.panel-group {
  margin-bottom: 20px;

  .card-panel-col {
    margin-bottom: 32px;
  }

  .card-panel {
    height: 108px;
    cursor: pointer;
    font-size: 12px;
    position: relative;
    overflow: hidden;
    color: #666;
    background: #fff;
    box-shadow: 4px 4px 40px rgba(0, 0, 0, .05);
    border-color: rgba(0, 0, 0, .05);
    border-radius: 6px;

    &:hover {
      .card-panel-icon-wrapper {
        color: #fff;
      }

      .icon-people {
        background: #40c9c6;
      }

      .icon-message {
        background: #36a3f7;
      }

      .icon-money {
        background: #f4516c;
      }

      .icon-shopping {
        background: #34bfa3
      }
    }

    .card-panel-icon-wrapper {
      float: left;
      margin: 14px 0 0 14px;
      padding: 16px;
      transition: all 0.38s ease-out;
      border-radius: 6px;
    }
    
    .icon-people {
      color: #40c9c6;
    }

    .icon-message {
      color: #36a3f7;
    }

    .icon-money {
      color: #f4516c;
    }

    .icon-shopping {
      color: #34bfa3;
    }

    .card-panel-icon {
      float: left;
      font-size: 48px;
    }

    .card-panel-description {
      float: right;
      font-weight: bold;
      margin: 26px;
      margin-left: 0px;

      .card-panel-text {
        line-height: 18px;
        color: rgba(0, 0, 0, 0.45);
        font-size: 16px;
        margin-bottom: 12px;
      }

      .card-panel-num {
        font-size: 20px;
      }
    }
  }
}

@media (max-width:550px) {
  .card-panel-description {
    display: none;
  }

  .card-panel-icon-wrapper {
    float: none !important;
    width: 100%;
    height: 100%;
    margin: 0 !important;

    .svg-icon {
      display: block;
      margin: 14px auto !important;
      float: none !important;
    }
  }
}
</style>
