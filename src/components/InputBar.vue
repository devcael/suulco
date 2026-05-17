<script setup lang="ts">
import { ref, inject, computed, watch, onMounted, onUnmounted } from "vue";
import type {
  IInputService,
  InputDestination,
  CreateItemOptions,
} from "../core/services/input_service";
import type { ISulcoService } from "../core/services/sulco_service";
import type { SulcoListItem } from "../core/models";
import SegmentedButton from "./SegmentedButton.vue";
import MemoriaEditorModal from "./MemoriaEditorModal.vue";
import SettingsModal from "./SettingsModal.vue";

const props = withDefaults(
  defineProps<{ initialTarget?: InputDestination }>(),
  { initialTarget: "hoje" },
);

const inputService = inject<IInputService>("inputService")!;
const sulcoService = inject<ISulcoService>("sulcoService")!;

const emit = defineEmits<{
  (e: "submitted", destination: InputDestination): void;
}>();

const input = ref("");
const target = ref<InputDestination>(props.initialTarget);
const textareaRef = ref<HTMLTextAreaElement | null>(null);
const memoriaEditorOpen = ref(false);
const settingsOpen = ref(false);

const targets: { key: InputDestination; label: string }[] = [
  { key: "hoje", label: "hoje" },
  { key: "inbox", label: "inbox" },
  { key: "sulco", label: "sulco" },
  { key: "memoria", label: "memoria" },
];

const placeholders: Record<InputDestination, string> = {
  sulco: "uma intenção...",
  hoje: "o que precisa acontecer hoje...",
  inbox: "uma task para depois...",
  memoria: "",
};

const currentPlaceholder = computed(() => placeholders[target.value]);

const sulcoOptions = ref<SulcoListItem[]>([]);
const selectedSulcoId = ref<number | null>(null);
const sulcoDropdownOpen = ref(false);
const sulcoDropdownRef = ref<HTMLElement | null>(null);

const showSulcoSelect = computed(
  () => target.value === "inbox" || target.value === "sulco",
);

watch(
  () => target.value,
  async (val) => {
    selectedSulcoId.value = null;
    sulcoDropdownOpen.value = false;
    if (val === "inbox" || val === "sulco") {
      sulcoOptions.value = await sulcoService.getActiveSulcoItems();
    }
  },
);

const selectedSulcoLabel = computed(() => {
  if (!selectedSulcoId.value) return "nenhum sulco";
  return (
    sulcoOptions.value.find((s) => s.id === selectedSulcoId.value)?.text ??
    "nenhum sulco"
  );
});

function selectSulco(id: number | null) {
  selectedSulcoId.value = id;
  sulcoDropdownOpen.value = false;
}

function onDocumentClick(e: MouseEvent) {
  if (
    sulcoDropdownRef.value &&
    !sulcoDropdownRef.value.contains(e.target as Node)
  ) {
    sulcoDropdownOpen.value = false;
  }
}

onMounted(() => document.addEventListener("click", onDocumentClick));
onUnmounted(() => document.removeEventListener("click", onDocumentClick));

function setTarget(key: InputDestination) {
  if (key === "memoria") {
    memoriaEditorOpen.value = true;
    return;
  }
  target.value = key;
}

function autoResize() {
  if (!textareaRef.value) return;
  textareaRef.value.style.height = "auto";
  textareaRef.value.style.height = textareaRef.value.scrollHeight + "px";
}

async function handleAdd() {
  const text = input.value.trim();
  if (!text) return;
  const options: CreateItemOptions = {};
  if (selectedSulcoId.value) options.sulcoId = selectedSulcoId.value;
  await inputService.createGlobalItem(target.value, text, options);
  const dest = target.value;
  input.value = "";
  selectedSulcoId.value = null;
  if (textareaRef.value) textareaRef.value.style.height = "auto";
  emit("submitted", dest);
}

function onKeydown(e: KeyboardEvent) {
  if (e.key === "Enter" && !e.shiftKey) {
    e.preventDefault();
    handleAdd();
  }
}
</script>

<template>
  <div class="pt-4 pb-9 border-t border-line bg-bg sticky bottom-0 z-10">
    <div class="flex items-center gap-2 mb-2.5">
      <div class="flex gap-1.5 flex-1">
        <SegmentedButton
          v-for="t in targets"
          :key="t.key"
          class="text-xs px-[11px] py-1 rounded-sm border font-body transition-all duration-150 cursor-pointer"
          :class="
            target === t.key
              ? 'bg-interactive text-accent border-interactive'
              : 'bg-transparent text-fg-subtle border-line'
          "
          @click="setTarget(t.key)"
        >
          {{ t.label }}
        </SegmentedButton>
      </div>

      <div v-if="showSulcoSelect" class="relative" ref="sulcoDropdownRef">
        <button
          class="text-xs px-[11px] py-1 rounded-sm border font-body transition-all duration-150 cursor-pointer bg-transparent text-fg-subtle border-line max-w-[140px] truncate"
          @click.stop="sulcoDropdownOpen = !sulcoDropdownOpen"
        >
          {{ selectedSulcoLabel }}
        </button>
        <div
          v-if="sulcoDropdownOpen"
          class="absolute bottom-full right-0 mb-1 bg-bg border border-line rounded-md py-1 min-w-[160px] z-10"
        >
          <button
            class="w-full text-left text-xs px-3 py-1.5 font-body text-fg-subtle hover:bg-bg-subtle transition-colors cursor-pointer"
            @click="selectSulco(null)"
          >
            nenhum
          </button>
          <button
            v-for="s in sulcoOptions"
            :key="s.id"
            class="w-full text-left text-xs px-3 py-1.5 font-body transition-colors cursor-pointer truncate"
            :class="
              selectedSulcoId === s.id
                ? 'text-fg bg-bg-subtle'
                : 'text-fg-subtle hover:bg-bg-subtle'
            "
            @click="selectSulco(s.id)"
          >
            {{ s.text }}
          </button>
          <p
            v-if="sulcoOptions.length === 0"
            class="text-xs px-3 py-1.5 text-fg-subtle italic font-body"
          >
            nenhum sulco
          </p>
        </div>
      </div>

      <button
        class="w-6 h-6 rounded-sm border border-line bg-transparent text-fg-subtle cursor-pointer flex items-center justify-center shrink-0 transition-all duration-150 hover:border-fg-subtle hover:text-fg"
        @click="settingsOpen = true"
        title="configurações"
      >
        <svg width="11" height="11" viewBox="0 0 11 11" fill="none">
          <circle
            cx="5.5"
            cy="5.5"
            r="1.5"
            stroke="currentColor"
            stroke-width="1.1"
          />
          <path
            d="M5.5 1v1M5.5 9v1M1 5.5h1M9 5.5h1M2.4 2.4l.7.7M7.9 7.9l.7.7M7.9 2.4l-.7.7M2.4 7.9l.7-.7"
            stroke="currentColor"
            stroke-width="1.1"
            stroke-linecap="round"
          />
        </svg>
      </button>
    </div>

    <div class="flex gap-2.5 items-end">
      <textarea
        ref="textareaRef"
        class="flex-1 bg-bg-subtle border border-line rounded-md px-3.5 py-[11px] font-body text-base text-fg outline-none resize-none min-h-[44px] overflow-hidden transition-[border-color] duration-200 focus:border-accent placeholder:text-fg-subtle placeholder:italic placeholder:text-[13px]"
        :value="input"
        @input="
          input = ($event.target as HTMLTextAreaElement).value;
          autoResize();
        "
        @keydown="onKeydown"
        :placeholder="currentPlaceholder"
        rows="1"
      />
      <button
        class="w-[44px] h-[44px] rounded-md bg-interactive border-none cursor-pointer flex items-center justify-center shrink-0 transition-opacity duration-150 hover:opacity-[0.78] active:scale-[0.97]"
        @click="handleAdd"
      >
        <svg width="15" height="15" viewBox="0 0 15 15" fill="none">
          <path
            d="M7.5 12.5V2.5M7.5 2.5L3 7M7.5 2.5L12 7"
            stroke="#F0EAE0"
            stroke-width="1.3"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
      </button>
    </div>
  </div>

  <MemoriaEditorModal
    v-if="memoriaEditorOpen"
    @close="memoriaEditorOpen = false"
    @submitted="
      memoriaEditorOpen = false;
      emit('submitted', 'memoria');
    "
  />

  <SettingsModal v-if="settingsOpen" @close="settingsOpen = false" />
</template>
