use chicken_dance::*;
// use crossterm;
// use std::fs;
use std::net::TcpListener;
use std::thread;
fn main() {

    // Enable raw mode for shouw the ansi
    //crossterm::terminal::enable_raw_mode().unwrap();
    // if is_dir_empty("./chicken_image"){
    //     video_spilt2frame("./chicken.mp4", "./chicken_image");
    // }
    // else if is_dir_empty("./chicken_ascii"){
    //     images2ascii("./chicken_image", "./chicken_ascii",false);
    // }else {
    //     let listener = TcpListener::bind("10.160.131.190:23").expect("Could not bind");
    //     println!("Server Listening...");
    //     for stream in listener.incoming(){
    //         match stream {
    //             Ok(stream) => {
    //                 println!("Connection established From: {}", stream.peer_addr().unwrap());
    //                 thread::spawn(move || {
    //                     send_ascii(stream, "C:\\Users\\mojin\\Desktop\\chicken_ascii");
    //                 });
    //             },
    //             Err(e)=>{
    //                 eprintln!("Failed to accept connection: {}", e);
    //             }
    //         }
    //     }
    // }

    //确保文本文文件夹不为空
    let listener = TcpListener::bind("0.0.0.0:23").expect("Could not bind");
        println!("Server Listening...");
        for stream in listener.incoming(){
            match stream {
                Ok(stream) => {
                    println!("Connection established From: {}", stream.peer_addr().unwrap());
                    thread::spawn(move || {
                        send_ascii(stream, "./chicken_ascii");
                    });
                },
                Err(e)=>{
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
        }
}
// fn is_dir_empty(dir:&str)->bool{
//     let metadata = fs::metadata(dir).unwrap();
//     if metadata.is_dir() {
//         let mut dir_entries = fs::read_dir(dir).unwrap();
//         dir_entries.next().is_none()
//     } else {
//         false
//     }
// }