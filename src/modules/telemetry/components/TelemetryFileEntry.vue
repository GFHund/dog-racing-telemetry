<script lang="ts" setup>
import Database from '@tauri-apps/plugin-sql';
import { ActiveTelemetryLap } from '../model/activeTelemetryLap';
import { TelemetryFile } from '../model/telemetryFile';
import TelemetryFileLapEntry from './TelemetryFileLapEntry.vue';
import { activeTelemetryLapStore } from '../store/ActiveTelemetryLapStore';

const props = defineProps({
    laps: {
        type: TelemetryFile,
        required: true
    }
});

const activeLapsStore = activeTelemetryLapStore();
function onSelected(selected:{lap:number,activated:boolean}){
    console.log(selected);
    if(selected.activated){
        //store.addActiveLap(new ActiveTelemetryLap(props.laps.filePath,selected.lap));
        getProperties(props.laps.filePath,selected.lap).then((keys) => {
            activeLapsStore.activeLaps.push(new ActiveTelemetryLap(props.laps.filePath,selected.lap,keys));
            activeLapsStore.fetchData();
        })
        
    } else {
        let index = 0;
        for(let lap of activeLapsStore.activeLaps){
            if(lap.lap === selected.lap && lap.file === props.laps.filePath){
                break;
            }
            index++;
        }
        activeLapsStore.activeLaps.splice(index,1);
    }
    
}
async function getProperties(file:string,lap:number):Promise<string[]>{
    const db = await Database.load('sqlite:'+file);
    console.log(lap);
    const json = <{data:string}[]>await db.select("SELECT data FROM raw_data WHERE json_extract(data,\"$.lap\") = $1 LIMIT 1",[lap]);
    console.log(json);
    if(json.length <= 0){
        return [];
    }
    console.log(json[0].data);
    let dataObj = <Object>JSON.parse(json[0].data);
    return Object.keys(dataObj);
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