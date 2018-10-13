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

pub fn modend(orec: ObjectRecord) {
    println!("Module End Record (MODEND)");
    println!("==========================");

    let module_type = orec.data[0];
    if module_type & 0x80 != 0 {
        println!("Main program module");
    }
    if module_type & 0x40 != 0 {
        println!("Contains a start address");
        if module_type & 1 == 1 {
            println!("Start address contains a relocatable address reference");
        }
        let end_data = orec.data[1];
        let mut i = 2;
        print!("\tFrame: ");
        if end_data & 0x80 != 0 {	/* frame specified explicitly */
            let frame_thread = (end_data >> 4) & 7;
            print!("thread={} ", frame_thread);
        } else {			/* frame in thread */
            let frame_method = (end_data >> 4) & 7;
            print!("method={} ", frame_method);
            if frame_method != 5 {
                let frame_datum = orec.data[i];
                print!(", datum={}", frame_datum);
                i = i + 1;
            }
        }
        println!();
        print!("\tTarget: ");
        if end_data & 0x08 != 0 {	/* target specified explicitly */
            let target_thread = end_data & 3;
            print!("thread={}", target_thread);
        } else {			/* target in thread */
            let target_datum = orec.data[i];
            print!("datum={}, ", target_datum);
            i = i + 1;
            let target_method = end_data & 7;
            print!("method={} ", target_method);
            if target_method < 3 {
                let target_displacement = (orec.data[i] as u32)
                        + 256*(orec.data[i+1] as u32);
                print!("displacement={}", target_displacement);
            }
        }
    }
    println!();
}

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

pub fn fixupp(orec: ObjectRecord) {
    println!("Fixup Record (FIXUPP)");
    println!("=====================");

    let mut i = 0;
    while i < orec.data.len() {
        if orec.data[i] & 0x80 != 0 {	/* fixup field */
            print!("Fixup field: ");
            let locat = (orec.data[i+1] as u32) + 256*(orec.data[i] as u32);
            let m = (locat >>14 ) & 1;
            let loc = (locat >> 10) & 7;
            let data = locat & 0x3ff;
            if m == 1 {
                print!("segment-relative, ");
            } else {
                print!("self-relative, ");
            }
            println!("loc={}, data={:03x}", loc, data);

            let fixdat = orec.data[i+2];
            i = i + 3;
            let f = fixdat >> 7;
            print!("\tFrame: ");
            if f == 1 {
                let frame_thread = (fixdat >> 4) & 7;
                print!("thread={} ", frame_thread);
            } else {
                let frame_method = (fixdat >> 4) & 7;
                print!("method={} ", frame_method);
                if frame_method != 5 {
                    let frame_datum = orec.data[i];
                    print!(", datum={}", frame_datum);
                    i = i + 1;
                }
            }
            println!();
            print!("\tTarget: ");
            let t = (fixdat >> 3) & 1;
            if t == 1 {
                let target_thread = fixdat & 3;
                print!("thread={}", target_thread);
            } else {
                let target_datum = orec.data[i];
                print!("datum={}, ", target_datum);
                i = i + 1;
                let target_method = fixdat & 7;
                print!("method={} ", target_method);
                if target_method < 3 {
                    let target_displacement = (orec.data[i] as u32)
                            + 256*(orec.data[i+1] as u32);
                    print!("displacement={}", target_displacement);
                    i = i + 2;
                }
            }
            println!();
        } else {		/* thread field */
            let data = orec.data[i];
            let number = data & 3;
            let method = (data >> 2) & 7;
            let d = match data >> 6 { 1 => "frame", _ => "target" };
            let index = orec.data[i+1];
            println!("Thread field: {} thread: {}, method: {}, index: {}",
                d, number, method, index);
            i = i + 2;
        }
    }
    
    println!();
}

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
