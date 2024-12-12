<script setup lang="ts">
    import { Child, ChildProcess, Command } from "@tauri-apps/plugin-shell";
    import WebSocket, { Message } from "@tauri-apps/plugin-websocket";
    import Database from '@tauri-apps/plugin-sql';
    import { ref } from "vue";

    let isRecording = ref(false);
    let game = ref('acc');
    let intervalNumber = 0;
    let wsServer:Child|null = null;//ChildProcess<string>|null = null;
    let ws:WebSocket|null = null;
    let db:Database|null = null;
    let output = '';
    let speed = ref(0.0);
    let throttle = ref(0.0);
    let brake = ref(0.0);
    let steering = ref(0.0);

    async function toggleRecording(){
        isRecording.value = !isRecording.value;
        if(isRecording.value){
            /*
            intervalNumber = setInterval(function(){
            console.log("hello World");
            },200)
            */
            //let result = await Command.sidecar("ws-server").spawn();
            let result = await Command.create("ws-server").spawn();
            //= await Command.create("ws-server").execute();
            console.log(result);
            wsServer = result;
            const now = Date.now();
            let day = now.getDate();
            let month = now.getMonth();
            let year = now.getFullYear();
            let hour = now.getHours();
            let minute = now.getMinutes();
            let filename = ''+year+'_'+month+'_'+day+'_'+hour+'_'+minute+'.sqlite';
            db = await Database.load('sqlite:'+filename);
            await db.execute('CREATE TABLE IF NOT EXISTS raw_data('+
            'id INTEGER PRIMARY KEY AUTOINCREMENT,'+
            'lap INTEGER,'+
            'speed REAL,'+
            'gas REAL,'+
            'throttle REAL,'+
            'break REAL,'+
            'steering REAL,'+
            'current_time INTEGER'+
            ');');

            ws = await WebSocket.connect('ws://127.0.0.1:9002');
            ws.addListener((msg: Message) => {
                console.log('msg.data type:', typeof msg.data);
                if(typeof msg.data == "string"){

                    let o = JSON.parse(msg.data);
                    console.log('Received Message:', o);
                    speed.value = o.speed;
                    throttle.value = o.throttle;
                    brake.value = o.brake;
                    steering.value = o.steering;
                    db?.execute('INSERT INTO raw_data(lap,speed,throttle,break,steering,current_time) VALUES($1,$2,$3,$5,$6)',
                        [o.lap,o.speed,o.throttle,o.break,o.steering,o.current_time]
                    ).then(()=>{});
                }
            });
            let initialPackage = {
                command: 'record',
                game: game.value
            }
            await ws.send(JSON.stringify(initialPackage));           
        } else {
            //clearInterval(intervalNumber);
            let closePackage = {
                command: 'close'
            };
            await db?.close();
            await ws?.send(JSON.stringify(closePackage));
            if(wsServer){
                wsServer.kill();
            }
        }
    }
    
</script>

<template>
    <div class="data-grid">
        <div>
            <div class="label">Speed</div>
            <div class="value">{{speed}}</div>
        </div>
        <div>
            <div class="label">Throttle</div>
            <div class="value">{{throttle}}</div>
        </div>
        <div>
            <div class="label">Break</div>
            <div class="value">{{brake}}</div>
        </div>
        <div>
            <div class="label">Steering</div>
            <div class="value">{{steering}}</div>
        </div>
    </div>
    <div class="controls" v-if="!isRecording">
        <label>
            Game
            <select v-model="game">
                <option value="acc">ACC</option>
                <option value="rr">RaceRoom</option>
            </select>
        </label>
        <button type="button" @click="toggleRecording">Start</button>
    </div>
    <div v-else>
        Recording Started <button type="button" @click="toggleRecording">Stop</button>
    </div>
    <div>
        
    </div>
</template>

<style lang="scss" scoped>
    .data-grid{
        display:grid;
        grid-template-columns: 1fr 1fr 1fr;
        gap:15px;
        margin-bottom:15px;
        & > div{
            background-color:var(--foreground-box-bg);
            padding:15px;
        }
    }
</style>