<template>
<div>
    <p>Counter value: {{ counter_value }}</p>
    <button v-on:click="startCounter()">Start counting</button>
    <button v-on:click="addValue(10)">Add 10</button>
    <button v-on:click="invoke('pause_counter')">Pause</button>
    <button v-on:click="stopCounter()">Reset</button>
</div>
</template>

<script setup lang="ts">
import { ref } from "vue"
import { invoke } from "@tauri-apps/api/tauri"
import { listen } from '@tauri-apps/api/event'

const counter_value = ref(0)

listen<{ value: number }>("counter_updated", (event) => {
  counter_value.value = event.payload.value
})

async function startCounter() {
  await invoke("start_counter")
}

async function stopCounter() {
  await invoke("reset_counter")
}

async function addValue(value: number) {
  await invoke("add_value", { value: value })
}

</script>
