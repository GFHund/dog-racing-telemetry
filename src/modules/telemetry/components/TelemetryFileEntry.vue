<script lang="ts" setup>
import { ActiveTelemetryLap } from '../model/activeTelemetryLap';
import { TelemetryFile } from '../model/telemetryFile';
import { telemetryStore } from '../store/telemetryStore';
import TelemetryFileLapEntry from './TelemetryFileLapEntry.vue';

const props = defineProps({
    laps: {
        type: TelemetryFile,
        required: true
    }
});
const store = telemetryStore();
function onSelected(selected){
    if(selected.activated){
        store.addActiveLap(new ActiveTelemetryLap(props.laps.filePath,selected.lap));
    } else {
        store.removeActiveLap(new ActiveTelemetryLap(props.laps.filePath,selected.lap));
    }
    
}
</script>
<template>
    <span>{{ props.laps.filePath }}</span>
    <table>
        <tr v-for="lap in props.laps.laps" :key="'lap_'+lap.lap">
            <TelemetryFileLapEntry :lap="lap" @selected="onSelected"></TelemetryFileLapEntry>
        </tr>
    </table>
</template>