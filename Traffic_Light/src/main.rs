use crate::Traffic::Duration;
use crate::Area::circle;


mod Traffic;
mod Integer_sum;
mod Area;

fn main() {
    //Traffic_Light
    let red=Traffic::Traffic_Light::Red;
    let green=Traffic::Traffic_Light::Green;
    let yellow=Traffic::Traffic_Light::Yellow;
    println!("Duration for red light is {} secs",red.get_lamp_time());
    println!("Duration for green light is {} secs",green.get_lamp_time());
    println!("Duration for yellow light is {} secs",yellow.get_lamp_time());

    //求和函数
    let v=vec![1,2,3,4,5,6,7,8,9,10];
    println!("The sum of Vec is {:?}",Integer_sum::cal_sum(&v).unwrap());

    //打印图形面积
    let triangle=Area::triangle{bottom: 10,height: 10 };
    Area::getArea(&triangle);

    let square=Area::square{length: 5 };
    Area::getArea(&square);

    let circle=Area::circle{radius:5};
    Area::getArea(&circle);
}

