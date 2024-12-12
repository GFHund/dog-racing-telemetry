import Database from "@tauri-apps/plugin-sql";
import { TelemetryLap } from "./TelemetryLap";

export class TelemetryFile{
    
    constructor(public filePath:string,public db:Database,public laps:TelemetryLap[]){}
}