<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { revealItemInDir } from "@tauri-apps/plugin-opener";

const emit = defineEmits<{
  (e: "close"): void;
}>();

const dbPath = ref("");

onMounted(async () => {
  dbPath.value = await invoke<string>("get_db_path");
});

async function reveal() {
  if (dbPath.value) {
    await revealItemInDir(dbPath.value);
  }
}
</script>

<template>
  <div
    class="fixed inset-0 bg-overlay z-[100] flex items-end justify-center"
    @click="emit('close')"
  >
    <div
      class="bg-bg w-full max-w-[580px] rounded-t-lg px-8 pt-7 pb-10 max-h-[70vh] overflow-y-auto"
      @click.stop
    >
      <div class="font-display text-[20px] text-fg mb-1.5">configurações</div>
      <div class="text-[12px] text-fg-subtle mb-6 ">
        informações do banco de dados local
      </div>

      <div class="mb-2 text-[11px] text-fg-subtle uppercase">
        banco de dados
      </div>
      <div
        class="bg-bg-subtle border border-line rounded-md px-3.5 py-3 font-body text-xs text-fg-secondary break-all mb-5"
      >
        {{ dbPath || "carregando..." }}
      </div>

      <button
        class="w-full py-3 rounded-md border border-line-strong bg-transparent text-fg font-body text-[12px] cursor-pointer mb-2 transition-opacity duration-150 hover:opacity-70"
        @click="reveal"
      >
        abrir no explorador de arquivos
      </button>
      <button
        class="w-full py-[11px] rounded-md border border-line bg-transparent text-fg-subtle font-body text-[12px] cursor-pointer"
        @click="emit('close')"
      >
        fechar
      </button>
    </div>
  </div>
</template>
