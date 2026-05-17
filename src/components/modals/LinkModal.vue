<script setup lang="ts">
import { ref, inject, onMounted } from "vue";
import type { ISulcoService } from "../../core/services/sulco_service";
import type { IHojeService } from "../../core/services/hoje_service";
import type { SulcoLinkOption } from "../../core/models";

const props = defineProps<{
  taskId: number;
}>();

const emit = defineEmits<{
  (e: "close"): void;
  (e: "linked"): void;
}>();

const sulcoService = inject<ISulcoService>("sulcoService")!;
const hojeService = inject<IHojeService>("hojeService")!;

const options = ref<SulcoLinkOption[]>([]);

async function load() {
  options.value = await sulcoService.getSulcoLinkOptions(props.taskId);
}

async function link(sulcoId: number | null) {
  await hojeService.linkTaskToSulco(props.taskId, sulcoId);
  emit("linked");
  emit("close");
}

onMounted(load);
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
      <div class="font-display text-[20px] text-fg mb-1.5">vincular ao sulco</div>
      <div class="text-[12px] text-fg-subtle mb-[22px] ">
        qual intenção essa task serve?
      </div>

      <div class="flex flex-col gap-0.5">
        <div
          v-for="opt in options"
          :key="opt.id"
          class="flex items-center gap-3.5 py-3.5 border-b border-line cursor-pointer transition-opacity duration-150 hover:opacity-70 last:border-b-0"
          @click="link(opt.id)"
        >
          <div
            class="w-[7px] h-[7px] border border-fg-subtle rotate-45 shrink-0"
            :class="opt.isCurrentlyLinked ? 'bg-accent border-accent' : ''"
          />
          <span class="font-display text-xl text-fg flex-1">{{ opt.text }}</span>
        </div>

        <div
          class="flex items-center gap-3.5 py-3.5 cursor-pointer transition-opacity duration-150 hover:opacity-70"
          @click="link(null)"
        >
          <div class="w-[7px] h-[7px] border border-fg-subtle rotate-45 shrink-0" />
          <span class="font-display text-xl text-fg-subtle italic flex-1">nenhum</span>
        </div>
      </div>

      <button
        class="mt-5 w-full py-[11px] rounded-md border border-line bg-transparent text-fg-subtle font-body text-[12px] cursor-pointer"
        @click="emit('close')"
      >
        cancelar
      </button>
    </div>
  </div>
</template>
