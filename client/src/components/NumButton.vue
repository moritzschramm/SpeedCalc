<script setup lang="ts">
const props = defineProps<{
  num: number
}>()

const emit = defineEmits(['numberClicked'])

function handleClick(event: Event) {
  (event.target as HTMLInputElement).blur()
  emit('numberClicked', props.num)
}
</script>

<template>
    <button @click="handleClick"
            class="w-full h-16 p-12 text-xl rounded flex items-center justify-center ring-1 ring-neutral-600 select-none
                    bg-neutral-700 active:bg-neutral-800">
        {{ num }}
    </button>
</template>