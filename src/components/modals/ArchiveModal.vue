<script setup lang="ts">
import { inject } from "vue";
import type { ISulcoService } from "../../core/services/sulco_service";

const props = defineProps<{
  sulcoId: number;
  sulcoText: string;
}>();

const emit = defineEmits<{
  (e: "close"): void;
  (e: "archived"): void;
}>();

const sulcoService = inject<ISulcoService>("sulcoService")!;

async function confirm() {
  await sulcoService.archiveSulcoItem(props.sulcoId);
  emit("archived");
  emit("close");
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
      <div class="font-display text-[20px] text-fg mb-1.5">arquivar intenção</div>
      <div class="text-[12px] text-fg-subtle tracking-wide mb-7 leading-normal">
        "{{ sulcoText }}" vai para os arquivados. você pode restaurar quando quiser.
      </div>

      <button
        class="w-full py-3 rounded-md border border-danger-border bg-transparent text-danger font-body text-[12px] tracking-wide cursor-pointer mb-2"
        @click="confirm"
      >
        sim, arquivar
      </button>
      <button
        class="w-full py-[11px] rounded-md border border-line bg-transparent text-fg-subtle font-body text-[12px] tracking-wide cursor-pointer"
        @click="emit('close')"
      >
        cancelar
      </button>
    </div>
  </div>
</template>
