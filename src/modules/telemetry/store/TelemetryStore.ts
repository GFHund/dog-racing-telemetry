import { ActiveTelemetryLap } from './../model/ActiveTelemetryLap';
import Database from '@tauri-apps/plugin-sql';
import { defineStore } from 'pinia';
import { computed, Ref, ref } from 'vue';
import { TelemetryFile } from '../model/TelemetryFile';
import { TelemetryLap } from '../model/TelemetryLap';


export const telemetryStore = defineStore('telemetry',() => {
    const openFiles:Ref<Array<TelemetryFile>> = ref([])
    const activeLaps:Ref<Array<ActiveTelemetryLap>> = ref([])
    const getOpenFiles = computed(() => openFiles)
    const getActiveLaps = computed(() => activeLaps)
    async function addOpenFile(file:string){
        const db = await Database.load('sqlite:'+file);
        const laps:TelemetryLap[] = await db.select('SELECT lap FROM data_point GROUP BY lap');
        console.log(laps);
        let newFile:TelemetryFile = {filePath: file,db,laps};
        openFiles.value.push(newFile);
    }
    function removeOpenFile(){
        /*
        file:string
        const index = openFiles.value.indexOf(file);
        if(index === -1){
            return;
        }
        delete openFiles.value[index];
        */
    }
    function addActiveLap(activeTelemetryLap: ActiveTelemetryLap){
        activeLaps.value.push(activeTelemetryLap);
    }
    function removeActiveLap(){
//activeTelemetryLap: ActiveTelemetryLap
    }
    return {openFiles,activeLaps,getOpenFiles,getActiveLaps,addOpenFile,removeOpenFile,addActiveLap,removeActiveLap}
}); 