<script setup lang="ts">
import {RouterView} from "vue-router";
import {computed} from "vue";
import {useI18n} from "vue-i18n";
import AppSidebar from "@/components/AppSidebar.vue";
import {isTauriRuntime} from "@/api/desktop";
import type {MessageSchema} from "@/i18n";

const {t} = useI18n<{ message: MessageSchema }>();
const showBrowserWarning = computed(() => !isTauriRuntime());
</script>

<template>
  <div class="shell">
    <AppSidebar/>
    <main class="main">
      <p v-if="showBrowserWarning" class="ipc-banner">{{ t("app.browserWarning") }}</p>
      <RouterView/>
    </main>
  </div>
</template>

<style scoped>
.ipc-banner {
  margin: 0 0 1rem;
  padding: 0.75rem 1rem;
  border-radius: var(--radius);
  background: #fff1f0;
  color: #cf1322;
  border: 1px solid #ffa39e;
  font-size: 0.9rem;
}
</style>
