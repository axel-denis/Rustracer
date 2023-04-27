//
// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// ppm interface module
//

use std::fs::File;
use std::io::{BufWriter, Write};

pub struct PPMInterface {
    buffer: &[u8],

}

impl PPMInterface {
    pub fn new(file_path: String) -> Self {
        let file = File::create(file_path).unwrap();
        PPMInterface { file }
    }

    fn create_header(&self, width: u32, height: u32) -> String {
        format!("P6\n{} {}\n255\n", width, height)
    }

    pub fn write(&mut self, width: u32, height: u32, content: Vec<u8>) {
        let header = self.create_header(width, height);
        let mut writer = BufWriter::new(&self.file);

        writer.write_all(header.as_bytes()).unwrap();
        writer.write_all(&content).unwrap();
    }
}
