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
pub fn extdef(orec: ObjectRecord) { tmp(orec) }
pub fn typdef(orec: ObjectRecord) { tmp(orec) }
pub fn pubdef(orec: ObjectRecord) { tmp(orec) }
pub fn linnum(orec: ObjectRecord) { tmp(orec) }

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

pub fn segdef(orec: ObjectRecord) { tmp(orec) }
pub fn grpdef(orec: ObjectRecord) { tmp(orec) }
pub fn fixupp(orec: ObjectRecord) { tmp(orec) }
pub fn ledata(orec: ObjectRecord) { tmp(orec) }

pub fn tmp(orec: ObjectRecord) {
    print!("record type: {:?}, ", orec.rtype);
    print!("record data:");
    for x in &orec.data {
        print!(" {:02x}", x);
    }
    println!();
}
