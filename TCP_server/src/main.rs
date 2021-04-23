/*第三课
实现要求
1、能正常运行
2、对tcp client （比如telnet等）发过来的消息，可以打印，并做 echo 返回
3、对每一行代码做注释
4、做一次标准的错误处理（模式匹配）
*/

//引入标准库中net的TcpListener和TcpStream
use std::net::{TcpListener,TcpStream};
// 引入标准库内的thread进行线程处理
use std::thread;
// 引用标准库内的io处理错误
use std::io::{self,Read,Write};
// 引入标准库内的time
use std::time;

// 程序入口函数
fn main() -> io::Result<()>{
    // 创建一个Tcp监听，监听8080端口，.unwrap（）可以用？表示
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    // 调用 incoming() 方法接收客户端的链接信息，如果有新的信息进来就会返回一个Result枚举，OK(T:TcpStream)// accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        //用match做模式匹配，来避免错误
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => { /* connection failed */ }
        }
    }
    // 关闭Tcp监听链接
    drop(listener);
    //
    Ok(())
}

// 线程调用的处理函数。
fn handle_client(mut stream: TcpStream) -> io::Result<()>{
    // 定义一个存储用的数组，因为需要后续进行填充值所以声明为可变的 `mut`
    let mut buf= [0; 512];
    //建立一个循环，来反复读取客户端的输入信息，但不采用loop永久循环，暂时循环一千次
    for _ in 0..1000{
        let bytes_read=stream.read(&mut buf)?;
        if bytes_read==0{
            return Ok(());
        }
        //重新写会stream
        stream.write(&buf[..bytes_read])?;
        //利用time包休息一秒钟
        thread::sleep(time::Duration::from_secs(1));
    }
    Ok(())
}


