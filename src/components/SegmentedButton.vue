<template>
  <button
    class="text-xs px-[11px] py-1 rounded-sm border font-body transition-all duration-150 cursor-pointer"
    :class="active
      ? 'bg-interactive text-interactive-fg border-interactive'
      : 'bg-transparent text-fg-subtle border-line'"
    @click="$emit('click')"
  >
    <slot />
  </button>
</template>

<script setup>
defineProps({ active: Boolean })
defineEmits(['click'])
</script>
