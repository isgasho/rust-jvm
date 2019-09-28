mod operand;
use operand::OperandStackItem;

mod stackframe;
use stackframe::StarckframeItem;

mod order;
use order::{Opecode, Order};

mod context;
use crate::context::{ConstantPool, ProgramContext};

mod utils;
use crate::utils::read_file;

#[derive(Debug)]
struct Interface;
#[derive(Debug)]
struct Field;
#[derive(Debug)]
struct Method;
#[derive(Debug)]
struct Attribute;

#[derive(Debug)]
struct ClassFile {
    magic: u16,                 // u4
    minor_version: u8,          // u2
    major_version: u8,          // u2
    constant_pool_count: u8,    // u2
    cp_info: ConstantPool,      // cp_info        constant_pool[constant_pool_count-1];
    access_flags: u8,           // u2
    this_class: u8,             // u2
    super_class: u8,            // u2
    interfaces_count: u8,       // u2
    interfaces: Vec<Interface>, // u2             interfaces[interfaces_count];
    fields_count: u8,           // u2
    fields: Vec<Field>,         // field_info     fields[fields_count];
    methods_count: u8,          // u2
    methods: Vec<Method>,       // method_info    methods[methods_count];
    attributes_count: u8,       // u2
    attributes: Vec<Attribute>, // attribute_info attributes[attributes_count];
}

#[derive(Debug)]
pub enum ConstPoolTag {
    ConstantClass = 7,
    ConstantFieldref = 9,
    ConstantMethodref = 10,
    ConstantInterfaceMethodref = 11,
    ConstantString = 8,
    ConstantInteger = 3,
    ConstantFloat = 4,
    ConstantLong = 5,
    ConstantDouble = 6,
    ConstantNameAndType = 12,
    ConstantUtf8 = 1,
    ConstantMethodHandle = 15,
    ConstantMethodType = 16,
    ConstantInvokeDynamic = 18,
}

impl From<u8> for ConstPoolTag {
    fn from(num: u8) -> ConstPoolTag {
        match num {
            7 => ConstPoolTag::ConstantClass,
            9 => ConstPoolTag::ConstantFieldref,
            10 => ConstPoolTag::ConstantMethodref,
            11 => ConstPoolTag::ConstantInterfaceMethodref,
            8 => ConstPoolTag::ConstantString,
            3 => ConstPoolTag::ConstantInteger,
            4 => ConstPoolTag::ConstantFloat,
            5 => ConstPoolTag::ConstantLong,
            6 => ConstPoolTag::ConstantDouble,
            12 => ConstPoolTag::ConstantNameAndType,
            1 => ConstPoolTag::ConstantUtf8,
            15 => ConstPoolTag::ConstantMethodHandle,
            16 => ConstPoolTag::ConstantMethodType,
            18 => ConstPoolTag::ConstantInvokeDynamic,
            _ => panic!("failed to convert {} to ConstPoolTag", num),
        }
    }
}

fn main() {
    let mut program_context = ProgramContext::new(vec![
        Order::new(Opecode::Iconst, OperandStackItem::I32(1)),
        Order::new(Opecode::Iconst, OperandStackItem::I32(2)),
        Order::new(Opecode::Iadd, OperandStackItem::I32(2)),
    ]);
    program_context.executes_programs();

    // operand_stack.iconst(OperandStackItem::I32(1));
    // stackframe.istore(&mut operand_stack, 0);

    // operand_stack.bipush(OperandStackItem::I32(1));
    // operand_stack.bipush(OperandStackItem::I32(2));
    // let result = operand_stack.iadd();
}

/*
* 1 + 2;
*/
// bipush 1
// bipush 2
// iadd

/*
 *  int i;
 *  i = 0;
 */
//  iconst_0
//  istore_1
//
