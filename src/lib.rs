use std::{fs, io::Write, net::TcpStream, path::Path, thread::{sleep, JoinHandle}};
use image::{imageops::FilterType, io::Reader as ImageReader, GenericImageView, Pixel};

/// To use thios function,you must insure that you have installed ffmpeg
pub fn video_spilt2frame(video_path: &str, output_path: &str) {
    let status = std::process::Command::new("ffmpeg")
    .arg("-i")
    .arg(video_path)
    .arg("-r")
    .arg("10") //Frame rate
    .arg(format!("{}/%04d.jpg",output_path))
    .status()
    .expect("failed to execute process");

    if !status.success() {
        eprintln!("Command executed with error code {}", status.code().unwrap_or(1));
    }
}
pub fn images2ascii(images_dir:&str,target_dir:&str,mode:bool){
    let paths = std::fs::read_dir(images_dir).unwrap();
    let mut files = paths.map(|entry| entry.unwrap().path()).collect::<Vec<_>>();
    files.sort();

    let chunk_len = files.len()/8;
    let mut handles = Vec::<JoinHandle<()>>::with_capacity(8);
    if mode{
        println!("正在将图片转换为彩色字符...");
        for (index,some_files) in files.chunks(chunk_len).map(|chunk| chunk.to_vec()).enumerate(){
            let target_dir = target_dir.to_string();
            let handle = std::thread::spawn(move ||{
                for file in some_files{
                    image2color(&file,&target_dir);
                    println!("线程{index}完成一个转换!");
                }
            });
            handles.push(handle);
        }
    }else {
        println!("正在将图片转换为黑白字符...");
        for (index,some_files) in files.chunks(chunk_len).map(|chunk| chunk.to_vec()).enumerate(){
            let target_dir = target_dir.to_string();
            let handle = std::thread::spawn(move ||{
                for file in some_files{
                    image2gray(&file,&target_dir);
                    println!("线程{index}完成一个转换!");
                }
            });
            handles.push(handle);
        }
    }
    handles.into_iter().for_each(|handle| handle.join().unwrap());
}
pub fn read_ascii(ascii_dir:&str){
    let temp = fs::read_dir(ascii_dir).unwrap();
    let mut files = temp.map(|e| e.unwrap().path()).collect::<Vec<_>>();
    files.sort();
    for file in files{
        print!("{}",String::from_utf8(fs::read(file).unwrap()).unwrap());
        std::io::stdout().flush().unwrap();
        print!("\x1b[H");
        sleep(std::time::Duration::from_millis(100));
        //print!("\x1b[2J");
    }
}
pub fn send_ascii(mut stream:TcpStream,ascii_dir:&str){
    let temp = fs::read_dir(ascii_dir).unwrap();
    let mut files = temp.map(|e| e.unwrap().path()).collect::<Vec<_>>();
    files.sort();
    for file in files{
        let content = fs::read_to_string(file).unwrap().replace("\n", "\r\n");
        stream.write_all(content.as_bytes()).expect("Failed to write to stream");
        stream.flush().expect("Failed to flush stream");
        stream.write_all(b"\x1b[H").expect("Failed to write to stream");
        stream.flush().expect("Failed to flush stream");
        sleep(std::time::Duration::from_millis(100));
        //print!("\x1b[2J");
    }
}
fn image2color(image_path:&Path,target_dir:&str){
    let img = ImageReader::open(image_path).unwrap()
    .decode()
    .unwrap();
    let resized = img.resize_exact(150, img.height()*75/img.width(),FilterType::Nearest );
    let mut line = String::with_capacity(2048);
    let mut lines = String::with_capacity(20480);
    for (x,_,pixel) in resized.pixels(){
        let r = pixel.0[0];
        let g = pixel.0[1];
        let b = pixel.0[2];
        let gray = pixel.to_luma();
        let pixel_str = format!("{}{}",rgb2ansi(r, g, b),grayscale2ascii(gray.0[0]));
        line.push_str(&pixel_str);
        if x == resized.width()-1{
            line.push_str("\x1b[0m\n");
            lines.push_str(&line);
            line.clear();
        }   
    }
    let _ = fs::write(Path::new(target_dir).join(format!("{}.ascii",image_path.file_stem().unwrap().to_str().unwrap())), lines);
}
fn image2gray(image_path:&Path,target_dir:&str){
    let img = ImageReader::open(image_path).unwrap()
    .decode()
    .unwrap();
    let resized = img.resize_exact(150, img.height()*75/img.width(),FilterType::Nearest );
    let mut line = String::with_capacity(160);
    let mut lines = String::with_capacity(160*80);
    let gray = resized.to_luma8();
    let mut count = 0;
    for pixel in gray.pixels(){
        let pixel_str = format!("{}",grayscale2ascii(pixel[0]));
        line.push_str(&pixel_str);
        if count == 149{
            line.push_str("\n");
            lines.push_str(&line);
            line.clear();
            count = 0;
        }else {
            count+=1;
        }
    }
    let _ = fs::write(Path::new(target_dir).join(format!("{}.ascii",image_path.file_stem().unwrap().to_str().unwrap())), lines);
}
fn grayscale2ascii(gray:u8)->char{
    match gray{
        0..=15 => ' ',
        16..=31 => '.',
        32..=47 => ':',
        48..=63 => '-',
        64..=79 => '=',
        80..=95 => '+',
        96..=111 => '*',
        112..=127 => '#',
        128..=143 => '%',
        144..=159 => '@',
        160..=175 => 'A',
        176..=191 => 'H',
        192..=207 => 'O',
        208..=223 => 'E',
        224..=239 => '&',
        240..=255 => '$',
    }
}
fn rgb2ansi(r:u8,g:u8,b:u8)->String{
    format!("\x1b[38;2;{};{};{}m", r, g, b)
}