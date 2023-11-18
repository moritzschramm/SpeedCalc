<script setup lang="ts">
import { CalcTask } from './CalcTask'
import NumButton from './components/NumButton.vue'
import SignButton from './components/SignButton.vue'
import EnterButton from './components/EnterButton.vue'

import { ref, watch, onMounted } from 'vue'

const wait = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

const result = ref<number | undefined>()
const resultFeedback = ref<boolean | undefined>()

let calcTask = new CalcTask(`ws://${window.location.host}/calc-task-ws`)

function numberInput(inputNum: number) {
  if (!result.value) {
    result.value = inputNum
  } else if (Math.abs(result.value) < 1000) {
    result.value *= 10
    result.value += result.value < 0 ? -inputNum : inputNum
  }
}

function enterClicked() {
  if (result.value !== undefined) {
    calcTask.submitResult(result.value)
    calcTask.requestNewTask()
    result.value = undefined
  }
}

calcTask.resultAvailable((isCorrect) => {
  resultFeedback.value = isCorrect
  wait(200/*ms*/).then(() => resultFeedback.value = undefined)
})
</script>

<template>
  <div class="bg-neutral-900/30 p-4 rounded sm:shadow-md text-center sm:ring-1 sm:ring-neutral-700">
    <div class="text-3xl p-4 select-none">{{ calcTask.task.value }}</div>
    <div :class="{'bg-neutral-900/30': resultFeedback === undefined, 'bg-orange-800': resultFeedback === false, 'bg-green-800': resultFeedback === true}" 
    class="text-3xl relative w-50 mb-4 mr-8 ml-8 p-2 select-none rounded"><!--bg-neutral-800/90-->
      <span v-if="resultFeedback === undefined" class="text-neutral-600 absolute left-2">=</span> 
      <span class="text-transparent">.</span>{{ result }}<span class="text-transparent">.</span>
      <span v-if="result !== undefined" @click="result = undefined" class="text-4xl text-neutral-200 absolute right-2 rotate-45 -translate-y-0.5 cursor-pointer"> + </span>
    </div>
    <div class="grid grid-cols-3 gap-4">

      <NumButton 
        v-for="num in Array.from(Array(9).keys()).map(x => x + 1)"
        :num="num"
        @numberClicked="numberInput">
      </NumButton>

      <SignButton @signClicked="() => { if (result) result *= -1 }"></SignButton>

      <NumButton :num="0" @numberClicked="numberInput"></NumButton>

      <EnterButton :disabled="result === undefined" @enterClicked="enterClicked"></EnterButton>

    </div>
  </div>
</template>