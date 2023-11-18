<script setup lang="ts">
const emit = defineEmits(['signClicked'])

function handleClick(event: Event) {
    (event.target as HTMLInputElement).blur()
    emit('signClicked')
}
</script>

<template>
    <button @click="handleClick"
            class="w-full h-16 p-12 text-xl rounded flex items-center justify-center ring-1 ring-orange-600 bg-orange-700 active:bg-orange-800">±</button>
</template>