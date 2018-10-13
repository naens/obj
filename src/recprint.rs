use std::str;

use ::objrec::*;

pub fn regint(orec: ObjectRecord) { tmp(orec) }
pub fn blkdef(orec: ObjectRecord) { tmp(orec) }
pub fn blkend(orec: ObjectRecord) { tmp(orec) }
pub fn debsym(orec: ObjectRecord) { tmp(orec) }

pub fn theadr(orec: ObjectRecord) {
    let name = str::from_utf8(&orec.data[1..]).unwrap();
    println!("Translator Header Record (THEADR)");
    println!("=================================");
    println!("Object Module Name: {}", name);
    println!();
}

pub fn coment(orec: ObjectRecord) { tmp(orec) }
pub fn modend(orec: ObjectRecord) { tmp(orec) }

pub fn extdef(orec: ObjectRecord) {
    println!("External Names Definition Record (EXTDEF)");
    println!("=========================================");
    let mut p = &orec.data[..];
    while !p.is_empty() {
        let length = p[0] as usize;
        let str = &p[0..length+1];
        let type_index = p[length+1] as usize;
        let name = str::from_utf8(&str[1..]).unwrap();
        print!("Name: {}", name);
        if type_index > 0 {
            print!(", type index: {}", type_index);
        }
        println!();
        p = &p[length+2..];
    }
    println!();
}

pub fn typdef(orec: ObjectRecord) { tmp(orec) }

pub fn pubdef(orec: ObjectRecord) {
    println!("Public Names Definition Record (PUBDEF)");
    println!("=======================================");

    let base_group_index = orec.data[0];
    println!("Base group index: {}", base_group_index);

    let base_segment_index = orec.data[1];
    println!("Base segment index: {}", base_segment_index);

    let i;
    if base_segment_index == 0 {
        let base_frame = (orec.data[2] as u32) + 256*(orec.data[3] as u32);
        println!("Base frame: {}", base_frame);
        i = 4;
    } else {
        i = 2;
    }

    let mut p = &orec.data[i..];
    while !p.is_empty() {
        let length = p[0] as usize;
        let name = str::from_utf8(&p[1..length+1]).unwrap();
        let public_offset = (p[length+1] as u32) + 256*(p[length+2] as u32);
        let type_index = p[length+3] as usize;
        print!("Name: {}, public offset: {}", name, public_offset);
        if type_index > 0 {
            print!(", type index: {}", type_index);
        }
        println!();
        p = &p[length+4..];
    }
    println!();
}

pub fn linnum(orec: ObjectRecord) {
    println!("Line Numbers Record (LINNUM)");
    println!("===========================");

    let base_group_index = orec.data[0];
    println!("Base group index: {}", base_group_index);

    let base_segment_index = orec.data[1];
    println!("Base segment index: {}", base_segment_index);

    let mut i = 2;
    while i < orec.data.len() {
        let line = (orec.data[i] as u32) + 256 * (orec.data[i+1] as u32);
        let offset = (orec.data[i+2] as u32) + 256 * (orec.data[i+3] as u32);
        println!("Line: {}, offset: {}", line, offset);
        i = i + 4;
    }
    println!();
}

pub fn lnames(orec: ObjectRecord) { 
    println!("List of Names Record (LNAMES)");
    println!("=============================");
    let mut p = &orec.data[..];
    let mut count = 0;
    while !p.is_empty() {
        count = count + 1;
        let length = p[0] as usize;
        let s: &[u8] = &p[0..length+1];
        let name = str::from_utf8(&s[1..]).unwrap();
        println!("{}: {:?}", count, name);
        p = &p[length+1..];
    }
    println!();
}

pub fn segdef(orec: ObjectRecord) {
    println!("Segment Definition Record (SEGDEF)");
    println!("=================================");
    let acbp = orec.data[0];
    let a = (acbp & 0xe0) >> 5;
    let c = (acbp & 0x1c) >> 2;
    let b1 = (acbp & 0x02) >> 1;               /* b1 because of joe bug */
    let p = acbp & 0x01;

    let alignment = match a {
        0 => "absolute",
        1 => "relocatable, byte aligned",
        2 => "relocatable, word aligned",
        3 => "relocatable, paragraph (16-byte) aligned",
        4 => "relocatable, aligned on a page boundary",
        5 => "relocatable, double-word aligned",
        6 => "not supported",
        7 => "not definied",
        _ => "<bad value>"
    };
    println!("Alignment: {}", alignment);

    let combination = match c {
        0 => "private",
        1 | 6 => "common",
        2 | 4 | 7 => "public",
        5 => "stack",
        _ => "<bad value>"
    };
    println!("Combination: {}", combination);

    println!("Big: {}", if b1 == 1 { "true" } else {"false" });

    println!("P: {}", if p == 1 { "true" } else {"false" });

    /* frame number and offset when A is 0 */
    let i;
    if a == 0 {
        let frame_number = (orec.data[1] as u32) + 256*(orec.data[2] as u32);
        let offset = orec.data[3];
        println!("Frame number: {}", frame_number);
        println!("Offset: {}", offset);
        i = 4;
    } else {
        i = 1;
    }
    let segment_length = (orec.data[i] as u32) + 256*(orec.data[i+1] as u32);
    println!("Segment length: {}", segment_length);

    let segment_name_index = orec.data[i+2];
    println!("Segment name index: {}", segment_name_index);

    let class_name_index = orec.data[i+3];
    println!("Class name index: {}", class_name_index);

    let overlay_name_index = orec.data[i+4];
    println!("Overlay name index: {}", overlay_name_index);

    println!();
}

pub fn grpdef(orec: ObjectRecord) { tmp(orec) }
pub fn fixupp(orec: ObjectRecord) { tmp(orec) }

pub fn ledata(orec: ObjectRecord) {
    println!("Logical Enumerated Data Record (LEDATA)");
    println!("=======================================");

    let segment_index = orec.data[0];
    println!("Segment index: {}", segment_index);

    let data_offset = (orec.data[1] as u32) + 256*(orec.data[2] as u32);
    println!("Enumerated data offset: {}", data_offset);

    let mut i = 0;
    print!("Data: ");
    while i+3 < orec.data.len() {
        if i > 0 && i % 16 == 0 {
            print!("\n      ");
        }
        print!(" {:02x}", orec.data[i+3]);
        i = i + 1;
    }
    println!();
    println!();
}

pub fn tmp(orec: ObjectRecord) {
    print!("record type: {:?}, ", orec.rtype);
    print!("record data:");
    for x in &orec.data {
        print!(" {:02x}", x);
    }
    println!();
}
