pub mod helpers;
use std::{fs::{self, File}, io::{Error, Write}};

#[derive(Debug)]
pub struct Labyrinth{
    delmitter: char,
    width: u16,
    height: u16,
    data: Vec<u8>
}
impl Labyrinth {
    fn serialize_bin(&self) -> Vec<u8>{
        let mut data: [u8; 5] = [0; 5];
        data[0] = self.delmitter as u8;
        data[1] = (self.width >> 8) as u8;
        data[2] = self.width as u8;
        data[3] = (self.height >> 8) as u8;
        data[4] = self.height as u8;
        let mut data: Vec<u8> = data.to_vec();
        for i in &self.data{
            data.push(*i);
        }
        return data.to_vec();
    }
    fn deserialize_bin(data: &Vec<u8>) -> Labyrinth{
        let mut object = Labyrinth{
            delmitter: '.',
            width: 0,
            height: 0,
            data: Vec::new(),
        };
        object.delmitter = data[0] as char;
        object.width = ((data[1] as u16) << 8) ^ (data[2] as u16);
        object.height = ((data[3] as u16) << 8) ^ (data[4] as u16);
        object.data = data[5..].to_vec();
        return object;
    }
    fn deserialize_text(data: &String) -> Result<Labyrinth, &'static str>{
        let content: Vec<String> = data.lines().map(|val| {val.to_string()}).collect();
        let mut object = Labyrinth{
            delmitter: content[0].chars().collect::<Vec<char>>()[0],
            width: content[0].len() as u16,
            height: content.len() as u16,
            data: Vec::new(),
        };
        
        let mut content = content.join("");
        let content_len = content.len();
        if content_len % 8 != 0{
            content.push_str(" ".repeat(8 - (content_len % 8)).as_str())
        }
        let content_len = content.len();
        for i in 0..content_len/8{
            let buf = &content[i*8..(i+1)*8];
            let mut buf_num: u8 = 0;
            for (index, j) in buf.chars().enumerate(){
                if j == object.delmitter{
                    buf_num ^= 1 << index;
                }
            }  
            object.data.push(buf_num);
        }
        Ok(object)
    }
    pub fn to_text_file(&self, path: &String) -> Result<(), &'static str>{
        let mut file: File; 
        match File::create(path){
            Ok(res) => file = res,
            Err(_) => return Err("Failed to create file!")
        }
        let mut result: String = String::from("");
        for i in &self.data{
            for j in 0..8{
                if helpers::get_bit(i.to_owned() as usize, j) == 1{
                    result = result + &String::from(self.delmitter);
                }
                else{
                    result.push_str(" ");
                }
            }
        }
        for i in 0..self.height{
            match writeln!(file, "{}", &result.as_str()[(self.width*i) as usize..(self.width*(i+1)) as usize]){
                Ok(_) => {}
                Err(_) => return Err("Failed to write text!"),
            }
        }

        Ok(())
    }
    pub fn from_text_file(filename: &String) -> Result<Labyrinth, &'static str>{
        let content: String;
        match fs::read_to_string(&filename){
            Ok(data) => content = data,
            Err(_) => return Err("Failed to read file!"),
        }
        return Labyrinth::deserialize_text(&content);
    }
    pub fn from_bin_file(filename: &String) -> Result<Labyrinth, &'static str>{
        let content: Vec<u8>;
        match fs::read(&filename){
            Ok(data) => content = data,
            Err(_) => return Err("Failed to read file!"),
        }
        return Ok(Labyrinth::deserialize_bin(&content));
    }
    pub fn to_bin_file(&self, filename: &String) -> Result<(), &'static str>{
        let file_res: Result<File, Error> = File::create(&filename);
        let mut file: File;
        match file_res {
            Ok(res) => file = res,
            Err(_) => return Err("Can not create file"),
        }
        let data: Vec<u8> = self.serialize_bin();
        let _ = file.write(&data);
        Ok(())
    }
}
