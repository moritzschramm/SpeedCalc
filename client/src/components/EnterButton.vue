<script setup lang="ts">
const props = defineProps<{
    disabled: boolean
}>()

const emit = defineEmits(['enterClicked'])

function handleClick(event: Event) {
    (event.target as HTMLInputElement).blur()
    emit('enterClicked')
}
</script>

<template>
    <button :disabled="props.disabled" @click="handleClick"
            class="w-full h-16 p-12 text-xl rounded flex items-center justify-center ring-1 ring-green-600
                    bg-green-700 active:bg-green-800 disabled:bg-green-900 disabled:ring-green-800">›</button>
</template>