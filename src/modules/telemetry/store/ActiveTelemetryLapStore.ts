import { defineStore } from "pinia";
import { Ref, ref } from "vue";
import { ActiveTelemetryLap } from "../model/ActiveTelemetryLap";
import { TelemetryDataCollection } from "../model/TelemetryDataCollection";
import Database from "@tauri-apps/plugin-sql";
import { RefSymbol } from "@vue/reactivity";


export const activeTelemetryLapStore = defineStore('activeTelemetry',() => {
    const activeLaps:Ref<ActiveTelemetryLap[]> = ref([]);
    let fetchedData:Ref<TelemetryDataCollection[]> = ref([]);

    async function fetchData(){
        let newFetchedData = <TelemetryDataCollection[]>[];
        for(let activeLap of activeLaps.value){
            const db = await Database.load('sqlite:'+activeLap.file);
            let data = <{data:string}[]>await db.select('SELECT data FROM raw_data WHERE json_extract(data,\"$.lap\") = $1',[activeLap.lap]);
            let singleRoundData = new TelemetryDataCollection(activeLap.lap);
            for(let telemetryData of data){
                //console.log(telemetryData);
                let dataObj = JSON.parse(telemetryData.data);
                singleRoundData.push(dataObj);
            }
            await db.close();
            newFetchedData.push(singleRoundData);
        }
        //fetchedData = ref(newFetchedData);
        fetchedData.value = newFetchedData;
        console.log("fetch finished");
    }

    return {activeLaps,fetchedData,fetchData};
})