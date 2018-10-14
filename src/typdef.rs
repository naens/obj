use std::str;

enum Leaf {
    Number(u16),
    String(String),
    Index(u8),
    Null
}

struct LeafReader <'a> {
    vec: &'a [u8],
    index: usize,
    leaf_counter: i32
}

impl <'a, 'b> LeafReader <'a> {

    /* before every 8 leaves: skip the EN byte */
    fn check_index(&mut self) {
        if self.leaf_counter % 8 == 0 {
            self.index = self.index +1
        }
    }

    fn read_null(&mut self) -> Leaf {
        self.index = self.index + 1;
        self.leaf_counter = self.leaf_counter + 1;
        Leaf::Null
    }

    fn read_index(&mut self) -> Leaf {
        let result = self.vec[self.index + 1];
        self.index = self.index + 2;
        self.leaf_counter = self.leaf_counter + 1;
        Leaf::Index(result)
    }

    fn read_u8(&mut self) -> Leaf {
        let result = self.vec[self.index] as u16;
        self.index = self.index + 1;
        self.leaf_counter = self.leaf_counter + 1;
        Leaf::Number(result)
    }

    fn read_u16(&mut self) -> Leaf {
        let result = (self.vec[self.index+1] as u16) + 256*(self.vec[self.index+2] as u16);
        self.index = self.index + 3;
        self.leaf_counter = self.leaf_counter + 1;
        Leaf::Number(result)
    }

    fn read_string(&mut self) -> Leaf {
        let length = self.vec[self.index + 1] as usize;
        let result = str::from_utf8(&self.vec[self.index+2..self.index+2+length]).unwrap();
        self.index = self.index + 2 + length;
        self.leaf_counter = self.leaf_counter + 1;
        Leaf::String(result.to_string())
    }

}

impl <'a> Iterator for LeafReader <'a> {
    type Item = Leaf;

    fn next(&mut self) -> Option<Leaf> {

        if self.index < self.vec.len() {
            self.check_index();
            match self.vec[self.index] {
                0 ... 0x7f => Some(self.read_u8()),
                0x80 => Some(self.read_null()),
                0x81 => Some(self.read_u16()),
                0x82 => Some(self.read_string()),
                0x83 => Some(self.read_index()),
                _ => panic!("bad byte: {}", self.vec[self.index])
            }
        } else {
            None
        }

    }
}

fn make_leaf_reader(vec: &[u8]) -> LeafReader {
    LeafReader {
        vec: vec,
        index: 0,
        leaf_counter: 0
    }
}

fn typdef_label(mut leaf_reader: LeafReader) -> String {
    let leaf = leaf_reader.next().unwrap();
    match leaf {
        Leaf::Null => {},
        _ => panic!("bad leaf")
    }
    let ret = leaf_reader.next().unwrap();
    let retstr = match ret {
        Leaf::Number(0x72) => "long",
        Leaf::Number(0x73) => "short",
        _ => "unknown"
    };
    format!("label ({})", retstr)
}

fn typdef_procedure(mut leaf_reader: LeafReader) -> String {
    let null_leaf = leaf_reader.next().unwrap();
    match null_leaf {
        Leaf::Null => {},
        _ => panic!("bad leaf")
    }
    let typ_leaf = leaf_reader.next().unwrap();
    let typstr = match typ_leaf {
        Leaf::Index(index) => format!("T{} ", index),
        Leaf::Null => format!(""),
        _ => panic!("unknown")
    };
    let ret_leaf = leaf_reader.next().unwrap();
    let retstr = match ret_leaf {
        Leaf::Number(0x72) => "long",
        Leaf::Number(0x73) => "short",
        _ => "unknown"
    };
    let num_leaf = leaf_reader.next().unwrap();
    let num = match num_leaf {
        Leaf::Number(num) => num,
        _ => panic!("unknown")
    };
    if num > 0 {
        let lst_leaf = leaf_reader.next().unwrap();
        let lst_index = match lst_leaf {
            Leaf::Index(index) => index,
            _ => panic!("unknown")
        };
        format!("procedure ({}, L{}) {}{}", num, lst_index, typstr, retstr)
    } else {
        format!("procedure ({}) {}{}", num, typstr, retstr)
    }
}

fn typdef_parameter(mut leaf_reader: LeafReader) -> String {
    let index_leaf = leaf_reader.next().unwrap();
    let type_index = match index_leaf {
        Leaf::Index(index) => index,
        _ => panic!("unknown")
    };
    format!("parameter T{}", type_index)
}

fn typdef_array(mut leaf_reader: LeafReader) -> String {
    let length_leaf = leaf_reader.next().unwrap();
    let length = match length_leaf {
        Leaf::Number(length) => length,
        _ => panic!("unknown")
    };
    let index_leaf = leaf_reader.next().unwrap();
    let type_index = match index_leaf {
        Leaf::Index(index) => index,
        _ => panic!("unknown")
    };
    format!("array ({}-bit) T{}", length, type_index)
}

fn typdef_struct(mut leaf_reader: LeafReader) -> String {
    let length_leaf = leaf_reader.next().unwrap();
    let length = match length_leaf {
        Leaf::Number(length) => length,
        _ => panic!("unknown")
    };
    let num_leaf = leaf_reader.next().unwrap();
    let num = match num_leaf {
        Leaf::Number(num) => num,
        _ => panic!("unknown")
    };
    let type_leaf = leaf_reader.next().unwrap();
    let type_index = match type_leaf {
        Leaf::Index(index) => index,
        _ => panic!("unknown")
    };
    let name_leaf = leaf_reader.next().unwrap();
    let name_index = match name_leaf {
        Leaf::Index(index) => index,
        _ => panic!("unknown")
    };
    format!("structure ({}-bit, {}) Types:L{} Names:L{}",
            length, num, type_index, name_index)
}

fn typdef_scalar(mut leaf_reader: LeafReader) -> String {
    let length_leaf = leaf_reader.next().unwrap();
    let length = match length_leaf {
        Leaf::Number(length) => length,
        _ => panic!("unknown")
    };
    let type_leaf = leaf_reader.next().unwrap();
    let type_string = match type_leaf {
        Leaf::Number(0x7c) => "u",
        Leaf::Number(0x7d) => "i",
        Leaf::Number(0x7e) => "r",
        _ => panic!("unknown")
    };
    format!("{}{}", type_string, length)
}

fn typdef_list(leaf_reader: LeafReader) -> String {
    let mut result: String = "list (".to_owned();
    let mut first = true;
    for leaf in leaf_reader {
        if !first {
            result.push_str(", ");
        }
        first = false;
        let item_string = match leaf {
            Leaf::Number(num) => format!("number:{}", num),
            Leaf::String(str) => format!("'{}'", str),
            Leaf::Index(index) => format!("index:{}", index),
            Leaf::Null => format!("null")
        };
        result.push_str(&item_string);
    }
    result.push_str(")");
    return result;
}

fn typdef_by_number(leaf_reader: LeafReader, n: u16) -> String {
    match n {
        0x71 => typdef_label(leaf_reader),
        0x74 => typdef_procedure(leaf_reader),
        0x75 => typdef_parameter(leaf_reader),
        0x77 => typdef_array(leaf_reader),
        0x79 => typdef_struct(leaf_reader),
        0x7b => typdef_scalar(leaf_reader),
        0x7f => typdef_list(leaf_reader),
        _ => format!("???")
    }
}

pub fn typdef_to_string(vec: &[u8]) -> String {
    let mut leaf_reader = make_leaf_reader(vec);
    let leaf = leaf_reader.next().unwrap();
    match leaf {
        Leaf::Number(n) => typdef_by_number(leaf_reader, n),
        Leaf::String(s) => format!("string: {}", s),
        Leaf::Index(i) => format!("index: {}", i),
        Leaf::Null => format!("null")
    }
}

#[test]
fn null() {
    assert_eq!(typdef_to_string(&[0x00, 0x80]), "null");
}

#[test]
fn label() {
    assert_eq!(typdef_to_string(&[0x00, 0x71, 0x80, 0x73]), "label (short)");
}

#[test]
fn procedure() {
    assert_eq!(typdef_to_string(&[0x00, 0x74, 0x80, 0x80, 0x73, 0x03, 0x83, 0x13]),
            "procedure (3, L19) short");
    assert_eq!(typdef_to_string(&[0x00, 0x74, 0x80, 0x83, 0x10, 0x73, 0x01, 0x83, 0x22]),
            "procedure (1, L34) T16 short");
    assert_eq!(typdef_to_string(&[0x00, 0x74, 0x80, 0x80, 0x73, 0x00]),
            "procedure (0) short");
}

#[test]
fn parameter() {
    assert_eq!(typdef_to_string(&[0x00, 0x75, 0x83, 0x04]), "parameter T4");
}

#[test]
fn array() {
    assert_eq!(typdef_to_string(&[0x00, 0x77, 0x60, 0x83, 0x27]),
            "array (96-bit) T39");
    assert_eq!(typdef_to_string(&[0x00, 0x77, 0x81, 0xc0, 0x12, 0x83, 0x02]),
            "array (4800-bit) T2");
    assert_eq!(typdef_to_string(&[0x00, 0x77, 0x30, 0x83, 0x02]),
            "array (48-bit) T2");
    assert_eq!(typdef_to_string(&[0x00, 0x77, 0x00, 0x83, 0x02]),
            "array (0-bit) T2");
}

#[test]
fn structure() {
    assert_eq!(typdef_to_string(&[0x00, 0x79, 0x40, 0x02, 0x83, 0x19, 0x83, 0x1a]),
            "structure (64-bit, 2) Types:L25 Names:L26");
    assert_eq!(typdef_to_string(&[0x00, 0x79, 0x81, 0xb8, 0x01, 0x01,
            0x83, 0x23, 0x83, 0x24]),
            "structure (440-bit, 1) Types:L35 Names:L36");
}

#[test]
fn scalar() {
    assert_eq!(typdef_to_string(&[0x00, 0x7b, 0x10, 0x7c]), "u16");
    assert_eq!(typdef_to_string(&[0x00, 0x7b, 0x08, 0x7c]), "u8");
    assert_eq!(typdef_to_string(&[0x00, 0x7b, 0x10, 0x7d]), "i16");
    assert_eq!(typdef_to_string(&[0x00, 0x7b, 0x20, 0x7e]), "r32");
}

#[test]
fn list() {
    assert_eq!(typdef_to_string(&[0x00, 0x7f,
            0x82, 0x08, 0x4c, 0x41, 0x53, 0x54, 0x4e, 0x41, 0x4d, 0x45,
            0x82, 0x09, 0x46, 0x49, 0x52, 0x53, 0x54, 0x4e, 0x41, 0x4d, 0x45,
            0x82, 0x02, 0x4d, 0x49,
            0x82, 0x06, 0x41, 0x4d, 0x4f, 0x55, 0x4e, 0x54]),
            "list ('LASTNAME', 'FIRSTNAME', 'MI', 'AMOUNT')");
    assert_eq!(typdef_to_string(&[0x00, 0x7f,
            0x83, 0x05, 0x83, 0x06, 0x83, 0x06, 0x83, 0x05, 0x83, 0x06]),
            "list (index:5, index:6, index:6, index:5, index:6)");
    assert_eq!(typdef_to_string(&[0x00, 0x7f, 0x83, 0x22]), "list (index:34)");
}
