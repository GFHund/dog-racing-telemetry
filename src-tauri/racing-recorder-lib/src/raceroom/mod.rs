use crate::common::RecorderTrait::{Recorder,TelemetryData};
use crate::{RrHandle,rr_init,rr_data};

pub struct RaceRoom{
    handle: RrHandle
}

impl RaceRoom{
    pub fn new()->RaceRoom{
        unsafe{
            let handle = rr_init();
            RaceRoom{handle:handle}
        }
    }
}

impl Recorder for RaceRoom{
    fn getData(&self)->TelemetryData{
        unsafe{
            let data = rr_data(self.handle);
            TelemetryData{
                speed: data.speed,
                steering: data.steerAngle,
                throttle: data.gas,
                brake: data.brake,
                current_time: data.currentTime,
                lap: data.lap
            }
        }
    }
}

unsafe impl Send for RaceRoom{}