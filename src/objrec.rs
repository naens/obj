use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::io::SeekFrom;

#[derive(Debug)]
pub enum RecordType {
    REGINT,
    BLKDEF,
    BLKEND,
    DEBSYM,
    THEADR,
    COMENT,
    MODEND,
    EXTDEF,
    TYPDEF,
    PUBDEF,
    LINNUM,
    LNAMES,
    SEGDEF,
    GRPDEF,
    FIXUPP,
    LEDATA
}

pub struct ObjectRecord {
    pub rtype: RecordType,
    pub data: Vec<u8>
}

fn const_to_type(num: u8) -> RecordType {
    match num {
        0x70 => RecordType::REGINT,
        0x7a => RecordType::BLKDEF,
        0x7c => RecordType::BLKEND,
        0x7e => RecordType::DEBSYM,
        0x80 => RecordType::THEADR,
        0x88 => RecordType::COMENT,
        0x8a => RecordType::MODEND,
        0x8c => RecordType::EXTDEF,
        0x8e => RecordType::TYPDEF,
        0x90 => RecordType::PUBDEF,
        0x94 => RecordType::LINNUM,
        0x96 => RecordType::LNAMES,
        0x98 => RecordType::SEGDEF,
        0x9a => RecordType::GRPDEF,
        0x9c => RecordType::FIXUPP,
        0xa0 => RecordType::LEDATA,
        n => panic!("Bad record type: {:02X}H", n)
    }
}

pub struct ObjReader {
    file: File
}

impl Iterator for ObjReader {
    type Item = ObjectRecord;

    fn next(&mut self) -> Option<ObjectRecord> {

        let mut buf = [0u8; 1];
        if self.file.read(&mut buf).unwrap() == 0 {
            return None;
        }
        let rtype = const_to_type(buf[0]);
        let mut sum: u32 = buf[0] as u32;

        if self.file.read(&mut buf).unwrap() == 0 {
            return None;
        }
        let record_length = buf[0] as usize;
        sum += buf[0] as u32;
    
        if self.file.read(&mut buf).unwrap() == 0 {
            return None;
        }
        let record_length = record_length + (buf[0] as usize) * 0x100;
        sum += buf[0] as u32;

        let mut vec = vec![0u8; record_length - 1];
        let count = self.file.read(vec.as_mut_slice()).unwrap();
        if count < record_length - 1 {
            return None;
        }
        let vecsum: u32 = vec.iter().fold(0u32, |mut s, &v| {s += v as u32; s});
        sum += vecsum;

        if self.file.read(&mut buf).unwrap() == 0 {
            return None;
        }
        sum += buf[0] as u32;

        if sum % 0x100 != 0 {
            panic!("bad checksum in record type {:?}", rtype);
        }

        Some(ObjectRecord { rtype: rtype, data: vec })
    }
}

pub fn make_obj_reader(mut file: File) -> Result<ObjReader, String> {
    /* check first byte is 80 */
    let mut buffer = [0; 1];
    file.read(&mut buffer).unwrap();
    if buffer[0] != 0x80 {
        return Err("bad file".to_string());
    }
    file.seek(SeekFrom::Start(0)).unwrap();
    println!("file ok");

    Ok(ObjReader { file: file })
}
