<script setup lang="ts">
import { ref, onMounted, nextTick } from "vue";

const props = defineProps<{
  currentText: string;
  title: string;
}>();

const emit = defineEmits<{
  (e: "close"): void;
  (e: "save", text: string): void;
}>();

const text = ref("");
const inputRef = ref<HTMLTextAreaElement>();

onMounted(async () => {
  text.value = props.currentText;
  await nextTick();
  inputRef.value?.focus();
  inputRef.value?.select();
});

function save() {
  const trimmed = text.value.trim();
  if (!trimmed || trimmed === props.currentText) {
    emit("close");
    return;
  }
  emit("save", trimmed);
  emit("close");
}

function onKeydown(e: KeyboardEvent) {
  if (e.key === "Enter" && !e.shiftKey) {
    e.preventDefault();
    save();
  }
  if (e.key === "Escape") {
    emit("close");
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
      <div class="font-display text-[20px] text-fg mb-1.5">{{ title }}</div>
      <div class="text-[12px] text-fg-subtle mb-[22px] ">
        edite o texto e confirme
      </div>

      <textarea
        ref="inputRef"
        class="w-full bg-bg-subtle border border-line rounded-md px-3.5 py-[11px] font-display text-lg text-fg outline-none mb-3 transition-[border-color] duration-200 focus:border-accent resize-none min-h-[80px] "
        v-model="text"
        @keydown="onKeydown"
        rows="3"
      />

      <button
        class="w-full py-3 rounded-md border-none bg-interactive text-interactive-fg font-body text-[12px] cursor-pointer mb-2"
        @click="save"
      >
        salvar
      </button>
      <button
        class="w-full py-[11px] rounded-md border border-line bg-transparent text-fg-subtle font-body text-[12px] cursor-pointer"
        @click="emit('close')"
      >
        cancelar
      </button>
    </div>
  </div>
</template>
