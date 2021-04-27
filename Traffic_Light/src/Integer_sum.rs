//实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]， 返回类型为Option<u32>，溢出时返回None;

pub fn cal_sum(vec:&Vec<u32>)->Option<u32>{
    let mut sumV:u32=0;
    for i in vec{
        match sumV.checked_add(*i) {
            Some(s) => {sumV = s;}
            None => {return None;}
        };
    }
    Some(sumV)
}