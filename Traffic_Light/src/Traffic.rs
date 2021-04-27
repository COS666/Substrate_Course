//为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同;
pub enum Traffic_Light{
    Red,
    Green,
    Yellow,
}

pub trait Duration{
    fn get_lamp_time(&self) ->i32;
}

impl Duration for Traffic_Light{
    fn get_lamp_time(&self) ->i32{
        match *self{
            Traffic_Light::Red=>45,
            Traffic_Light::Green=>60,
            Traffic_Light::Yellow=>75,
        }
    }
}

