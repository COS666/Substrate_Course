//实现一个打印图形面积的函数，它接收一个可以计算面积的类型 作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束

pub struct triangle{
    pub(crate) bottom:u32,
    pub(crate) height:u32,
}

pub struct square{
    pub(crate) length:u32,
}

pub struct circle{
    pub(crate) radius:u32,
}

pub trait calculatearea{
    fn cal_area(&self);
}

impl calculatearea for triangle{
    fn cal_area(&self){
        let area=self.bottom*self.height;
        println!("三角形面积={}",area)
    }
}

impl calculatearea for square{
    fn cal_area(&self){
        let area=self.length*self.length;
        println!("正方形面积={}",area)
    }
}

impl calculatearea for circle{
    fn cal_area(&self){
        let area=3*self.radius*self.radius;
        println!("圆面积={}",area)
    }
}

pub fn getArea<T:calculatearea>(graphics: &T){
    graphics.cal_area();
}