<script lang="ts" setup>
import { ref,defineEmits, defineProps, useTemplateRef} from 'vue';
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
    const events = defineEmits(['change']);
    const radioInputs = useTemplateRef('input_radio');
    const checkboxInputs = useTemplateRef('input_checkbox');
    //const selected = defineModel<SelectedObject>();
    let selected = ref({...props.default});
    let hideWindow = ref(true);
    function onChanged(ev:Event){
        const target:HTMLInputElement = ev.target;
        if(target.type === 'radio'){
            for(let rad of radioInputs.value){
                if(rad.checked){
                    events('change',rad.value);
                    selected.value.label = rad.parentElement.innerText;
                    selected.value.value = rad.value;
                    break;
                }
            }
            
        } else {
            let selectedItems = [];
            for(let rad of checkboxInputs.value){
                if(rad.checked){
                    selectedItems.push(rad.value);
                }
            }
            events('change',selectedItems);
        }
        hideWindow.value = true;
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
                    <input type="checkbox" name="select-check" :value="option.value" @change="onChanged" ref="input_checkbox"> {{ option.label }}
                </label>
            </template>
            <template v-else>
                <label v-for="option in props.options" :key="option.value">
                    <input type="radio" name="select-radio" :value="option.value" @change="onChanged" ref="input_radio"> {{ option.label }}
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
        z-index:50;
        top:100%;
        left:0;
        width:100%;
        color:white;
        background-color:var(--bs-blue);
        label{
            display:block;
        }
    }
</style>