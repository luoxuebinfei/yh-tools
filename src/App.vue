<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Menus from "./components/Menus.vue";
import Notify_ from "./views/Notify.vue";

</script>

<template>
  <div id="app" class="rounded-md">
    <router-view></router-view>
  </div>
  <!-- <div v-else><Notify_ /></div> -->
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
