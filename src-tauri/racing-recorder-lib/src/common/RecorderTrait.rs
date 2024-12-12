pub struct TelemetryData{
    pub speed:f32,
    pub throttle:f32,
    pub brake:f32,
    pub steering:f32,
    pub current_time:i32,
    pub lap:i32
}

pub trait Recorder{
    fn saveToDatabase(&self){

    }
    fn getData(&self)->TelemetryData;
}