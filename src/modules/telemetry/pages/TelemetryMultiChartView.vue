<script setup lang="ts">
import  {Chart, ChartData, ChartDataset, Point } from 'chart.js/auto';
import zoomPlugin from 'chartjs-plugin-zoom';
import { computed, onMounted, useTemplateRef } from 'vue';



let chart_objs:Chart[] = [];
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
        data: data
    }];
    return {
        labels: xValues,
        datasets: dataset
    }
});
onMounted(() => {
    
})

</script>

<template>
    <chart-view v-for="n in 5" :key="'chart_'+n"></chart-view>
    
</template>