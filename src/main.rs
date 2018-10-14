use std::env;
use std::fs::File;
use std::path::Path;

pub mod recprint;

pub mod objrec;

pub mod typdef;

fn main() {
    /* get arguments */
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);
    if args.len() != 2 {
        println!("usage {} <filename>", &args[0]);
        std::process::exit(1);
    }
    let path = Path::new(&args[1]);
    let display = path.display();

    /* open the file */
    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {:?}", display),
        Ok(file) => file,
    };

    let obj_reader = objrec::make_obj_reader(file).unwrap();

    /* read */
    for orec in obj_reader {
        match orec.rtype {
            objrec::RecordType::REGINT => recprint::regint(orec),
            objrec::RecordType::BLKDEF => recprint::blkdef(orec),
            objrec::RecordType::BLKEND => recprint::blkend(orec),
            objrec::RecordType::DEBSYM => recprint::debsym(orec),
            objrec::RecordType::THEADR => recprint::theadr(orec),
            objrec::RecordType::COMENT => recprint::coment(orec),
            objrec::RecordType::MODEND => recprint::modend(orec),
            objrec::RecordType::EXTDEF => recprint::extdef(orec),
            objrec::RecordType::TYPDEF => recprint::typdef(orec),
            objrec::RecordType::PUBDEF => recprint::pubdef(orec),
            objrec::RecordType::LINNUM => recprint::linnum(orec),
            objrec::RecordType::LNAMES => recprint::lnames(orec),
            objrec::RecordType::SEGDEF => recprint::segdef(orec),
            objrec::RecordType::GRPDEF => recprint::grpdef(orec),
            objrec::RecordType::FIXUPP => recprint::fixupp(orec),
            objrec::RecordType::LEDATA => recprint::ledata(orec)
        }
    }
}
