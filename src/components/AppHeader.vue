<script setup lang="ts">
const nav = [
  {
    key: "sulco",
    label: "sulco",
    to: "/sulco",
    icon: `<svg width="18" height="18" viewBox="0 0 18 18" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path d="M9 14V4M9 4L5 8M9 4L13 8" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>`,
  },
  {
    key: "hoje",
    label: "hoje",
    to: "/hoje",
    icon: `<svg width="18" height="18" viewBox="0 0 18 18" fill="none" xmlns="http://www.w3.org/2000/svg">
      <circle cx="9" cy="9" r="3.2" stroke="currentColor" stroke-width="1.4"/>
      <path d="M9 1.5V3M9 15V16.5M1.5 9H3M15 9H16.5M3.4 3.4L4.5 4.5M13.5 13.5L14.6 14.6M14.6 3.4L13.5 4.5M4.5 13.5L3.4 14.6" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>
    </svg>`,
  },
  {
    key: "inbox",
    label: "inbox",
    to: "/inbox",
    icon: `<svg width="18" height="18" viewBox="0 0 18 18" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect x="2" y="10" width="14" height="6" rx="1.2" stroke="currentColor" stroke-width="1.4"/>
      <path d="M5.5 10L7.5 13H10.5L12.5 10" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round"/>
      <path d="M9 2V9M9 9L6.5 6.5M9 9L11.5 6.5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>`,
  },
  {
    key: "memoria",
    label: "memória",
    to: "/memoria",
    icon: `<svg width="18" height="18" viewBox="0 0 18 18" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path d="M9 2L10.8 6.6L16 7.3L12.2 10.9L13.2 16L9 13.6L4.8 16L5.8 10.9L2 7.3L7.2 6.6L9 2Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round"/>
    </svg>`,
  },
] as const;
</script>

<template>
  <div class="shrink-0">
    <div class="pt-7 pb-4 flex flex-col bg-bg font-body text-fg gap-5">
      <!-- Header -->
      <div class="flex items-center gap-3">
        <img src="../assets/logo/sulco-logo.svg" alt="sulco" class="w-9 h-9 shrink-0" />
        <div class="flex flex-col">
          <span class="font-display text-[28px] text-fg block">sulco</span>
          <span class="font-body text-xs text-fg-muted block">sua fonte da verdade</span>
        </div>
      </div>

      <!-- Nav keymap buttons -->
      <div class="flex gap-2">
        <router-link
          v-for="item in nav"
          :key="item.key"
          :to="item.to"
          custom
          v-slot="{ navigate, isActive }"
        >
          <button
            class="flex-1 flex items-center justify-center gap-2 py-2 rounded-md border cursor-pointer font-body transition-all duration-200 select-none"
            :class="
              isActive
                ? 'bg-interactive border-interactive text-interactive-fg'
                : 'bg-bg-subtle border-line text-fg-subtle hover:text-fg hover:border-line-strong'
            "
            @click="navigate"
          >
            <span v-html="item.icon" class="flex items-center justify-center shrink-0" />
            <span class="text-sm font-body">{{ item.label }}</span>
          </button>
        </router-link>
      </div>
    </div>
  </div>
</template>
