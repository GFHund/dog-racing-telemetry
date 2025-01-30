<script setup lang="ts">
    import { open } from '@tauri-apps/plugin-dialog';
    import { telemetryStore } from '../store/telemetryStore';
    import TelemetryFileEntry from '../components/TelemetryFileEntry.vue';
    import SelectComponent from '../components/SelectComponent.vue';
import { useRoute, useRouter } from 'vue-router';

    const store = telemetryStore();
    const router = useRouter();

    async function openFile(){
        const file = await open({
            multiple: false,
            directory: false,
            canCreateDirectories:true,
            filters:[
                {extensions: ['sqlite3', 'sqlite'], name:'telemetry file'}
            ]
        });
        if(file !== null){
            store.addOpenFile(file);
        }
    }

    function changeView(value:string){
        console.log(value);
        router.push({name:value});
    }
</script>

<template>
    <div class="row">
        <div class="col">
            <div class="laps-header">
                <div class="laps-open-file" @click="openFile">
                    <svg class="bi" width="20" height="20" fill="currentColor">
                        <use xlink:href="~/bootstrap-icons/bootstrap-icons.svg#file-earmark-plus"/>
                    </svg>
                </div>
            </div>
            <div class="file-list">
                <div v-for="file in store.openFiles" :key="file.filePath">
                    <TelemetryFileEntry :laps="file"></TelemetryFileEntry>
                </div>
            </div>
        </div>
        <div class="col">
            <div class="chart-view-header">
                
                <select-component 
                    :default="{label:'Single Diagram',value:'single-chart'}"
                    :options="[
                        {label:'Single Diagram',value:'single-chart'},
                        {label:'Multi Diagram',value:'multi-chart'},
                ]" @change="changeView"></select-component>
                <!--
                <select-component
                    :default="{label:'Throttle',value:'throttle'}"
                    :options="[
                        {label:'Throttle',value:'throttle'},
                        {label:'Break',value:'break'},
                        {label:'Steering',value:'steering'},
                    ]"
                ></select-component>
                -->
            </div>
            
            <RouterView />
        </div>
    </div>
</template>
<style lang="scss" scoped>
.laps-header{
    background-color:var(--primary-btn-color);
}
.laps-open-file{
    padding:5px 10px;
    svg{
        color:var(--bs-white);
    }
}
.file-list{
    
}
</style>