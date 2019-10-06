use crate::constant::{ConstPoolItem, ConstantPool};
use crate::utils::{extract_x_byte_as_usize, extract_x_byte_as_vec};

#[derive(Debug)]
pub enum Attribute {
    SourceFile(SourceFile),
    InnerClasses,
    EnclosingMethod,
    SourceDebugExtension,
    BootstrapMethods,
    ConstantValue,
    Code(Code),
    Exceptions,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    AnnotationDefault,
    MethodParameters,
    Synthetic,
    Deprecated,
    Signature,
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    LineNumberTable(LineNumberTable),
    LocalVariableTable,
    LocalVariableTypeTable,
    StackMapTable(StackMapTable),
    RuntimeVisibleTypeAnnotations,
    RuntimeInvisibleTypeAnnotations,
}

impl Attribute {
    pub fn new(
        constant_pool: &ConstantPool,
        inputs: &mut [u8],
        index: usize,
    ) -> (Attribute, usize) {
        let (attribute_name_index, index) = extract_x_byte_as_usize(inputs, index, 2);
        if let ConstPoolItem::ConstantUtf8(item) = &constant_pool.0[attribute_name_index] {
            let attribute_name_index = attribute_name_index as u16;

            let val = String::from_utf8_lossy(item.bytes.as_slice());
            match AttributeTag::from(val.into_owned()) {
                AttributeTag::SourceFile => {
                    let (item, index) = SourceFile::new(inputs, index, attribute_name_index);
                    (Attribute::SourceFile(item), index)
                }
                AttributeTag::LineNumberTable => {
                    let (item, index) = LineNumberTable::new(inputs, index, attribute_name_index);
                    (Attribute::LineNumberTable(item), index)
                }
                AttributeTag::StackMapTable => {
                    let (item, index) = StackMapTable::new(inputs, index, attribute_name_index);
                    (Attribute::StackMapTable(item), index)
                }
                _ => unimplemented!(),
            }
        } else {
            panic!(
                "{:?} is not ConstantUtf8",
                constant_pool.0[attribute_name_index]
            );
        }
    }
}

// this is a custom enum for handling
#[derive(Debug)]
pub enum AttributeTag {
    SourceFile,
    InnerClasses,
    EnclosingMethod,
    SourceDebugExtension,
    BootstrapMethods,
    ConstantValue,
    Code,
    Exceptions,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    AnnotationDefault,
    MethodParameters,
    Synthetic,
    Deprecated,
    Signature,
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    LineNumberTable,
    LocalVariableTable,
    LocalVariableTypeTable,
    StackMapTable,
    RuntimeVisibleTypeAnnotations,
    RuntimeInvisibleTypeAnnotations,
}

impl From<String> for AttributeTag {
    fn from(input: String) -> AttributeTag {
        match input.as_str() {
            "SourceFile" => AttributeTag::SourceFile,
            "InnerClasses" => AttributeTag::InnerClasses,
            "EnclosingMethod" => AttributeTag::EnclosingMethod,
            "SourceDebugExtension" => AttributeTag::SourceDebugExtension,
            "BootstrapMethods" => AttributeTag::BootstrapMethods,
            "ConstantValue" => AttributeTag::ConstantValue,
            "Code" => AttributeTag::Code,
            "Exceptions" => AttributeTag::Exceptions,
            "RuntimeVisibleParameterAnnotations" => {
                AttributeTag::RuntimeVisibleParameterAnnotations
            }
            "RuntimeInvisibleParameterAnnotations" => {
                AttributeTag::RuntimeInvisibleParameterAnnotations
            }
            "AnnotationDefault" => AttributeTag::AnnotationDefault,
            "MethodParameters" => AttributeTag::MethodParameters,
            "Synthetic" => AttributeTag::Synthetic,
            "Deprecated" => AttributeTag::Deprecated,
            "Signature" => AttributeTag::Signature,
            "RuntimeVisibleAnnotations" => AttributeTag::RuntimeVisibleAnnotations,
            "RuntimeInvisibleAnnotations" => AttributeTag::RuntimeInvisibleAnnotations,
            "LineNumberTable" => AttributeTag::LineNumberTable,
            "LocalVariableTable" => AttributeTag::LocalVariableTable,
            "LocalVariableTypeTable" => AttributeTag::LocalVariableTypeTable,
            "StackMapTable" => AttributeTag::StackMapTable,
            "RuntimeVisibleTypeAnnotations" => AttributeTag::RuntimeVisibleTypeAnnotations,
            "RuntimeInvisibleTypeAnnotations" => AttributeTag::RuntimeInvisibleTypeAnnotations,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct SourceFile {
    pub attribute_name_index: u16, // u2
    pub attribute_length: u32,     // u4
    pub sourcefile_index: u16,     // u2
}

impl SourceFile {
    pub fn new(inputs: &mut [u8], index: usize, attribute_name_index: u16) -> (SourceFile, usize) {
        let (attribute_length, index) = extract_x_byte_as_usize(inputs, index, 4);
        let attribute_length = attribute_length as u32;

        let (sourcefile_index, index) = extract_x_byte_as_usize(inputs, index, 2);
        let sourcefile_index = sourcefile_index as u16;

        let source_file = SourceFile {
            attribute_name_index,
            attribute_length,
            sourcefile_index,
        };
        (source_file, index)
    }
}

#[derive(Debug)]
pub struct Code {
    pub attribute_name_index: u16, // u2
    pub attribute_length: u32,     // u4
    pub max_stack: u16,            // u2
    pub max_locals: u16,           // u2
    pub code_length: u32,          // u4
    pub code: Vec<Instruction>,
    pub exception_table_length: u16, // u2
    pub exception_table: Vec<ExceptionTableItem>,
    pub attributes_count: u16, // u2
    pub attribute_info: Vec<Attribute>,
}

impl Code {
    pub fn new(
        constant_pool: &mut ConstantPool,
        inputs: &mut [u8],
        index: usize,
        attribute_name_index: u16,
    ) -> (Code, usize) {
        let (attribute_length, index) = extract_x_byte_as_usize(inputs, index, 4);
        let attribute_length = attribute_length as u32;

        let (max_stack, index) = extract_x_byte_as_usize(inputs, index, 2);
        let max_stack = max_stack as u16;

        let (max_locals, index) = extract_x_byte_as_usize(inputs, index, 2);
        let max_locals = max_locals as u16;

        let (code_length, mut index) = extract_x_byte_as_usize(inputs, index, 4);
        let code_length = code_length as u32;

        let mut code = Vec::with_capacity(code_length as usize);
        for _ in 0..code_length {
            let (tag, update_index) = extract_x_byte_as_usize(inputs, index, 1);

            let (instruction, update_index) = Instruction::new(inputs, update_index, tag);

            index = update_index;
            code.push(instruction);
        }

        let (exception_table_length, index) = extract_x_byte_as_usize(inputs, index, 4);
        let exception_table_length = exception_table_length as u16;
        let exception_table = Vec::with_capacity(exception_table_length as usize);

        let (attributes_count, mut index) = extract_x_byte_as_usize(inputs, index, 4);
        let attributes_count = attributes_count as u16;
        let mut attribute_info = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            let (attribute, update_index) = Attribute::new(constant_pool, inputs, index);
            index = update_index;
            attribute_info.push(attribute);
        }

        (
            Code {
                attribute_name_index,
                attribute_length,
                max_stack,
                max_locals,
                code_length,
                code,
                exception_table_length,
                exception_table,
                attributes_count,
                attribute_info,
            },
            index,
        )
    }
}

#[derive(Debug)]
pub struct ExceptionTableItem {
    pub start_pc: u16,   //u2
    pub end_pc: u16,     //u2
    pub handler_pc: u16, //u2
    pub catch_type: u16, //u2
}

#[derive(Debug)]
pub enum Instruction {
    IconstN(usize),         // 0x02(-1) - 0x08(5)
    Ldc(usize),             // 0x12
    IloadN(usize),          // 0x1a(0) - 0x1d(3)
    AloadN(usize),          // 0x2a(0) - 0x2d(3)
    IstoreN(usize),         // 0x3b(0) - 0x3e(3)
    Iadd,                   // 0x60
    Ificmple(usize, usize), // 0xa4
    Return,                 // 0xac
    Getfield(usize),        // 0xb4
    Invokevirtual(usize),   // 0xb6
    Invokespecial(usize),   // 0xb7
    Getstatic(usize),       // 0xb2
}

impl Instruction {
    pub fn new(inputs: &mut [u8], index: usize, tag: usize) -> (Instruction, usize) {
        match tag {
            // aload_n
            val @ 0x2a..0x2d => (Instruction::AloadN(val - 0x2a), index),
            // ldc
            0x12 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                (Instruction::Ldc(val), index)
            }
            // if_icmple
            0xa4 => {
                let (val, index) = extract_x_byte_as_vec(inputs, index, 2);
                (
                    Instruction::Ificmple(val[0] as usize, val[1] as usize),
                    index,
                )
            }
            // invokevirtual
            0xb6 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 4);
                (Instruction::Invokevirtual(val), index)
            }
            // invokespecial
            0xb7 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 4);
                (Instruction::Invokespecial(val), index)
            }
            // getstatic
            0xb2 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 4);
                (Instruction::Getstatic(val), index)
            }
            // getfield
            0xb4 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 4);
                (Instruction::Getfield(val), index)
            }
            // iadd
            0x60 => (Instruction::Iadd, index),
            // return
            0xac => (Instruction::Return, index),
            // iload_n
            val @ 0x1a..0x1d => (Instruction::IloadN(val - 0x1a), index),
            // iload_n
            val @ 0x02..0x08 => (Instruction::IconstN(val - 0x03), index),
            // istore_n
            val @ 0x3b..0x3e => (Instruction::IstoreN(val - 0x3b), index),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct LineNumberTable {
    pub attribute_name_index: u16,     // u2
    pub attribute_length: u32,         // u4
    pub line_number_table_length: u16, // u2
    pub line_number_tables: Vec<LineNumberTableItem>,
}

impl LineNumberTable {
    pub fn new(
        inputs: &mut [u8],
        index: usize,
        attribute_name_index: u16,
    ) -> (LineNumberTable, usize) {
        let (attribute_length, index) = extract_x_byte_as_usize(inputs, index, 4);
        let attribute_length = attribute_length as u32;

        let (line_number_table_length, mut index) = extract_x_byte_as_usize(inputs, index, 2);
        let line_number_table_length = line_number_table_length as u16;

        let mut line_number_tables = Vec::with_capacity(line_number_table_length as usize);

        for _ in 0..line_number_table_length {
            let (start_pc, update_index) = extract_x_byte_as_usize(inputs, index, 2);
            let start_pc = start_pc as u16;

            let (line_number, update_index) = extract_x_byte_as_usize(inputs, update_index, 2);
            let line_number = line_number as u16;

            line_number_tables.push(LineNumberTableItem {
                start_pc,
                line_number,
            });
            index = update_index;
        }

        (
            LineNumberTable {
                attribute_name_index,
                attribute_length,
                line_number_table_length,
                line_number_tables,
            },
            index,
        )
    }
}

#[derive(Debug)]
pub struct LineNumberTableItem {
    pub start_pc: u16,    // u2
    pub line_number: u16, // u2
}

#[derive(Debug)]
pub struct StackMapTable {
    attribute_name_index: u16, // u2
    attribute_length: u32,     // u4
    number_of_entries: u16,    // u2
    stack_map_frame: Vec<StackMapFrame>,
}

impl StackMapTable {
    pub fn new(
        inputs: &mut [u8],
        index: usize,
        attribute_name_index: u16,
    ) -> (StackMapTable, usize) {
        let (attribute_length, index) = extract_x_byte_as_usize(inputs, index, 4);
        let attribute_length = attribute_length as u32;

        let (number_of_entries, mut index) = extract_x_byte_as_usize(inputs, index, 2);
        let number_of_entries = number_of_entries as u16;
        let mut stack_map_frame = Vec::with_capacity(number_of_entries as usize);

        for _ in 0..number_of_entries {
            let (frame, update_index) = StackMapFrame::new(inputs, index);
            stack_map_frame.push(frame);
            index = update_index;
        }
        (
            StackMapTable {
                attribute_name_index,
                attribute_length,
                number_of_entries,
                stack_map_frame,
            },
            index,
        )
    }
}

#[derive(Debug)]
pub enum StackMapFrame {
    SameFrame(usize),
    SameLocals1StackItemFrame,
    SameLocals1StackItemFrameExtended,
    ChopFrame,
    SameFrameExtended,
    AppendFrame,
    FullFrame,
}

impl StackMapFrame {
    pub fn new(inputs: &mut [u8], index: usize) -> (StackMapFrame, usize) {
        let (tag, index) = extract_x_byte_as_usize(inputs, index, 1);
        match tag {
            0..63 => (StackMapFrame::SameFrame(tag), index),
            _ => unimplemented!(),
        }
    }
}
