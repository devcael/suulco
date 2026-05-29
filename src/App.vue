<script setup lang="ts">
import { RouterView } from "vue-router";
import Navbar from "./components/Navbar.vue";
</script>

<template>
  <div class="flex flex-row h-full overflow-hidden">
    <Navbar />
    <main class="flex-1 min-h-0 overflow-y-auto">
      <RouterView />
    </main>
  </div>
</template>
