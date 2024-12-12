use crate::common::RecorderTrait::{Recorder,TelemetryData};
use crate::{AccHandle,acc_init,acc_data};

pub struct Acc{
    handle:AccHandle
}

impl Acc{
    pub fn new()->Acc{
        unsafe{
            let handle = acc_init();
            Acc{handle:handle}
        }
    }
}

impl Recorder for Acc{
    fn getData(&self)->TelemetryData{
        unsafe{
            let data = acc_data(self.handle);
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

unsafe impl Send for Acc{}