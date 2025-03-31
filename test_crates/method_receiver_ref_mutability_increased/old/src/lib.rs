#![no_std] 

pub struct Rectangle {
    length: u32,
    width: u32, 
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.length * self.width
    }  
}