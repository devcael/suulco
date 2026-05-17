<template>
  <button
    class="bg-transparent border-none text-fg-subtle cursor-pointer px-1.5 py-0.5 opacity-0 group-hover:opacity-100 transition-opacity duration-150 font-body  whitespace-nowrap"
    @click="$emit('click')"
  >
    <slot />
  </button>
</template>
