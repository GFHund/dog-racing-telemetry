import { createMemoryHistory,createRouter, RouteRecordRaw } from "vue-router";
import TelemetryView from "./modules/telemetry/pages/TelemetryView.vue";
import TelemetrySingleChartView from "./modules/telemetry/pages/TelemetrySingleChartView.vue";
import TelemetryMultiChartView from "./modules/telemetry/pages/TelemetryMultiChartView.vue";
import LiveDataView from "./modules/liveData/pages/LiveDataView.vue";
export const routes:RouteRecordRaw[] = [
    {
        path: '/',
        name:'telemetry',
        component: TelemetryView,
        children:[
            {
                path:'',
                redirect:'single-chart'
            },
            {
                path: 'single-chart',
                name: 'single-chart',
                component: TelemetrySingleChartView
            },
            {
                path: 'multi-chart',
                name: 'multi-chart',
                component: TelemetryMultiChartView
            }
        ]
    },
    {
        path: '/live-data',
        name: 'liveData',
        component: LiveDataView
    }
];

export const router = createRouter({
    history: createMemoryHistory(),
    routes
});