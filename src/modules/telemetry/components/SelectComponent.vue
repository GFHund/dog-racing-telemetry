<script lang="ts" setup>
import { ref } from 'vue';
import { SelectedObject } from '../model/SelectedObjects';

    const props = defineProps({
        options: {
            type:Array<SelectedObject>,
            required:true
        },
        default: {
            type:SelectedObject,
            required:true
        },
        multi: {
            type: Boolean,
            default: false,
            required:false
        }
    });
    //const selected = defineModel<SelectedObject>();
    let selected = ref(props.default);
    let hideWindow = ref(true);
    function onChanged(){

    }
    function toggleWindow(){
        hideWindow.value = !hideWindow.value;
    }
</script>

<template>
    <div class="select-container">
        <a class="btn btn-primary d-block" @click="toggleWindow">{{ selected.label }}</a>
        <div class="select-window" :class="{'d-none':hideWindow}">
            <template v-if="props.multi">
                <label v-for="option in props.options" :key="option.value">
                    <input type="checkbox" :value="option.value" @change="onChanged"> {{ option.label }}
                </label>
            </template>
            <template v-else>
                <label v-for="option in props.options" :key="option.value">
                    <input type="radio" :value="option.value" @change="onChanged"> {{ option.label }}
                </label>
            </template>
        </div>
    </div>
</template>

<style lang="scss" scoped>
    .select-container{
        position:relative;
    }
    .select-window{
        position:absolute;
        top:100%;
        left:0;
        label{
            display:block;
        }
    }
</style>