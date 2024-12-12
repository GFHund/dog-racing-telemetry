<script setup lang="ts">
import { getCurrentWindow  } from '@tauri-apps/api/window';

async function close(){
  await getCurrentWindow().close();
}
</script>

<template>
  <header>
    <ul class="navigation">
      <li><RouterLink to="/">Telemetry</RouterLink></li>
      <li><RouterLink to="/live-data">Live Data</RouterLink></li>
    </ul>
    <div data-tauri-drag-region class="drag-region"></div>
    <div class="controls">
      <div>
        <a class="btn btn-primary">
          <svg class="bi" width="32" height="32" fill="currentColor">
            <use xlink:href="~/bootstrap-icons/bootstrap-icons.svg#dash"/>
          </svg>
        </a>
      </div>
      <div>
        <a class="btn btn-primary">
          <svg class="bi" width="32" height="32" fill="currentColor">
            <use xlink:href="~/bootstrap-icons/bootstrap-icons.svg#plus"/>
          </svg>
        </a>
      </div>
      <div>
        <a class="btn btn-primary" @click="close">
          <svg class="bi" width="32" height="32" fill="currentColor">
            <use xlink:href="~/bootstrap-icons/bootstrap-icons.svg#x"/>
          </svg>
        </a>
      </div>
    </div>
  </header>
  <main class="container">
    <RouterView />
  </main>
</template>

<style lang="scss" scoped>
header{
  display:flex;
}
.drag-region{
  flex-grow:1;
}
.controls{
  display:flex;
  gap:5px;
}
.navigation{
  list-style: none;
  display:flex;
  gap:5px;
  padding:0;
  li{
    background-color:var(--secondary-btn-color);
    padding:5px 10px;
    a{
      color: var(--secondary-btn-font-color);
      text-decoration: none;
    }
  }
}

</style>
<style lang="scss">
@import "node_modules/bootstrap/scss/bootstrap";
:root{
  --bs-body-bg: #444;
  --foreground-box-bg: #666;
  --secondary-btn-color: #a50;
  --secondary-btn-font-color:#fff;
  --primary-btn-color:#222;
}

</style>