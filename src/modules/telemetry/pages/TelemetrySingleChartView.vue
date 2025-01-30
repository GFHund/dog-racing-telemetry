<script setup lang="ts">



import { telemetryStore } from '../store/telemetryStore';
import ChartView from '../components/ChartView.vue';
import Database from '@tauri-apps/plugin-sql';


const store = telemetryStore();

store.$subscribe((mutation,state) => {
    if(mutation.storeId === 'activeLaps'){
        for(let activeLap of state.activeLaps){
            reciveData(activeLap.file,activeLap.lap).then((data) => {

            });
        }
    }
});



async function reciveData(file:string,lap:number){
    const db = await Database.load('sqlite:'+file);
    db.select("SELECT json_extract(data,\"$.\") FROM raw_data");
}
</script>

<template>
    
    <chart-view></chart-view>
</template>