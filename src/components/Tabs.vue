<script setup lang="ts">
export interface Tab {
  value: string | number;
  label: string;
}

const props = defineProps<{
  tabs: Tab[];
  modelValue: string | number;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: string | number): void;
  (e: "change", value: string | number): void;
}>();

const selectTab = (tabValue: string | number): void => {
  if (tabValue !== props.modelValue) {
    emit("update:modelValue", tabValue);
    emit("change", tabValue);
  }
};
</script>

<template>
  <div class="w-full">
    <div class="border-b border-gray-200 dark:border-gray-700">
      <nav
        class="flex space-x-8 -mb-px overflow-x-auto scrollbar-none"
        aria-label="Tabs"
      >
        <button
          v-for="tab in tabs"
          :key="tab.value"
          :class="[
            'whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm transition-all duration-200 ease-in-out focus:outline-none',
            modelValue === tab.value
              ? 'border-indigo-600 text-indigo-600 dark:border-indigo-400 dark:text-indigo-400'
              : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 dark:text-gray-400 dark:hover:text-gray-300 dark:hover:border-gray-600',
          ]"
          :aria-current="modelValue === tab.value ? 'page' : undefined"
          @click="selectTab(tab.value)"
        >
          {{ tab.label }}
        </button>
      </nav>
    </div>

    <div class="mt-6">
      <slot :active-tab="modelValue" />
    </div>
  </div>
</template>
