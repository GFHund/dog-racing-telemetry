<script setup lang="ts">
import { ref } from 'vue';
import { TelemetryLap } from '../model/TelemetryLap';
    const props = defineProps({
        lap:{
            type:TelemetryLap,
            required: true
        }
    });
    const emit = defineEmits(['selected']);
    const selected = ref(false);
    function onChanged(value:any){
        emit('selected',{lap:props.lap,activated:value});
    }
</script>
<template>
    <td><input type="checkbox" :checked="selected" @change="onChanged"></td>
    <td>{{ lap.lap }}</td>    
</template>