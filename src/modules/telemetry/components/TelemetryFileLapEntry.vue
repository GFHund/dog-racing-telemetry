<script setup lang="ts">
import { ref,defineProps, defineEmits } from 'vue';
import { TelemetryLap } from '../model/TelemetryLap';
    const props = defineProps({
        lap:{
            type:TelemetryLap,
            required: true
        }
    });
    const emit = defineEmits(['selected']);
    const selected = ref(false);
    function onChanged(value:Event){
        let targetElement = <HTMLInputElement|null>value.target;
        if(targetElement == null){
            return;
        }
        emit('selected',{lap:props.lap.lap,activated:targetElement.checked});
    }
</script>
<template>
    <td><input type="checkbox" :checked="selected" @change="onChanged"></td>
    <td>{{ lap.lap }}</td>    
</template>