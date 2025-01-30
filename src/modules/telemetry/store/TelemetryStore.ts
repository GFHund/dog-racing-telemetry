import { ActiveTelemetryLap } from './../model/ActiveTelemetryLap';
import Database from '@tauri-apps/plugin-sql';
import { defineStore } from 'pinia';
import { computed, Ref, ref } from 'vue';
import { TelemetryFile } from '../model/TelemetryFile';
import { TelemetryLap } from '../model/TelemetryLap';


export const telemetryStore = defineStore('telemetry',() => {
    const openFiles:Ref<Array<TelemetryFile>> = ref([])
    
    async function addOpenFile(file:string){
        const db = await Database.load('sqlite:'+file);
        //const laps:TelemetryLap[] = await db.select('SELECT lap FROM data_point GROUP BY lap');
        await db.execute('CREATE TEMPORARY TABLE lap_table(lap INTEGER,time REAL);')
        await db.execute('INSERT INTO lap_table SELECT json_extract(data,"$.lap") as lap,MAX(json_extract(data,"$.current_time")) as current_time FROM raw_data GROUP BY lap;')
        const laps:TelemetryLap[] = await db.select('SELECT lap FROM lap_table;');
        console.log(laps);
        let newFile:TelemetryFile = new TelemetryFile(file,db,laps);
        openFiles.value.push(newFile);
        await db.close();
    }
    
    return {openFiles,addOpenFile}
}); 