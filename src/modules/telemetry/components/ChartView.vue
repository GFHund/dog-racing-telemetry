<script setup lang="ts">
import { Chart, ChartData, ChartDataset, Point } from 'chart.js';
import { computed, onMounted, useTemplateRef,defineProps, ref, Ref } from 'vue';
import zoomPlugin from 'chartjs-plugin-zoom';
import { ActiveTelemetryLap } from '../model/ActiveTelemetryLap';
import { activeTelemetryLapStore } from '../store/ActiveTelemetryLapStore';
import SelectComponent from './SelectComponent.vue';
import { SelectedObject } from '../model/SelectedObjects';
import _ from 'lodash';


    const telemetryView = useTemplateRef('chart_canvas');
    let chart:Chart|null = null;
    let alStore = activeTelemetryLapStore();
    let selectOptions:Ref<SelectedObject[]> = ref([]);
    let selectedProperty = '';

    const emptyDataset = computed(():ChartData => {
        let data:Point[] = [];
        let xValues = [];
        for(let i = 0.0;i < (2*Math.PI);i+=0.5){
            data.push({
                x:i,y:Math.sin(i)
            });
            xValues.push(i);
        }
        let dataset:ChartDataset[] =  [{
            label:'sin',
            data: data,
            pointStyle:false
        }];
        return {
            labels: xValues,
            datasets: dataset
        }
    });

    onMounted( () => {
        if(telemetryView.value == null){
            return;
        }
        chart = new Chart(telemetryView.value,{
            type:'line',
            data: emptyDataset.value,
            plugins:[zoomPlugin]
        });
    });
    alStore.$subscribe((mutation,state) => {
        console.log('$subscribe');
        console.log(mutation);
        console.log(state);
        if(mutation.storeId == 'activeTelemetry'){
            let keysMerged:string[] = [];
            for(let activeLap of state.activeLaps){
                keysMerged = _.union(keysMerged,activeLap.keys);
            }
            for(let key of keysMerged){
                selectOptions.value.push(<SelectedObject>{ label:key, value:key });
            }
            if(selectOptions.value.length <= 0){
                return;
            }
            selectedProperty = selectOptions.value[0].value;
        }
    });
    alStore.$onAction(
        ({
            name, // name of the action
            store, // store instance, same as `someStore`
            args, // array of parameters passed to the action
            after, // hook after the action returns or resolves
            onError, // hook if the action throws or rejects
        }) => {
            if(name !== 'fetchData'){
                return;
            }
            after(() => {
                if(store.fetchedData.length <= 0){
                    return;
                }
                if(chart == null){
                    console.log('chart == null');
                    return;
                }
                
                let xValues = store.fetchedData[0].getTimings();
                
                
                let datasets = <ChartDataset[]>[];
                for(let data of store.fetchedData){
                    console.log(data);
                    let pointData:Point[] = [];
                    for(let singleTelemetryData of data){
                        //console.log(singleTelemetryData);
                        pointData.push({
                            x:singleTelemetryData.current_time, 
                            y:singleTelemetryData[selectedProperty]})
                    }
                    
                    datasets.push({
                        label:'Lap '+data.getLap(),
                        data: pointData,
                        pointStyle:false
                    })
                    
                }
                console.log(xValues);
                console.log(datasets);
                chart.data = <ChartData>{
                    labels: xValues,
                    datasets: datasets
                }
                chart.update();
            })
        }
  );

  function changeProperty(value:string){

  }
</script>

<template>
    <select-component v-if="selectOptions.length > 0"
                    :default="selectOptions[0]"
                    :options="selectOptions"
                    @change="changeProperty"
                ></select-component>
    <canvas ref="chart_canvas"></canvas>
</template>