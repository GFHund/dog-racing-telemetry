export class TelemetryDataCollection{
    public constructor(private lap:number,private dataObj:Object[] = []){}

    getTimings():number[]{
        let ret = <number[]>[];
        for(let obj of this.dataObj){
            if('current_time' in obj === false){
                continue;
            }
            if(!Number.isSafeInteger(obj.current_time)){
                continue;
            }
            let lap = <number>obj.current_time;
            ret.push(lap);
        }
        return ret;
    }

    getPropertyies(property:string):number[]{
        let ret = <number[]>[];
        for(let obj of this.dataObj){
            if(property in obj === false){
                continue;
            }
            // @ts-ignore
            if(Number.isFinite(obj[property]) === false){
                continue;
            }
            // @ts-ignore
            let propertyValue = <number>obj[property];
            ret.push(propertyValue);
        }
        return ret;
    }

    push(data:Object){
        this.dataObj.push(data);
    }

    getLap(){
        return this.lap;
    }

    [Symbol.iterator](){
        let i = 0;
        let dataObj = this.dataObj;
        return {
            next(){
                if(i < dataObj.length){

                    let ret = {value: dataObj[i], done:false};
                    i++;
                    return ret;
                }
                return {value:undefined, done:true};
            }
        }
    }
}