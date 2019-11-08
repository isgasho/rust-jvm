use crate::operand::Item;
use crate::string_pool::StringPool;
use crate::utils::*;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ConstantPool(pub Vec<ConstPoolItem>);
impl ConstantPool {
    pub fn new(
        string_map: &mut StringPool,
        inputs: &mut [u8],
        mut index: usize,
        length: usize,
    ) -> (ConstantPool, usize) {
        let mut items = vec![ConstPoolItem::ConstantNull];
        let mut constant_index = 0;
        let constant_pool_length = length - 1;
        while constant_pool_length > constant_index {
            let (tag, update_index) = extract_x_byte_as_usize(inputs, index, 1);

            let (item, update_index) = match ConstPoolTag::from(tag) {
                ConstPoolTag::ConstantClass => {
                    let (item, update_index) =
                        ConstantClass::create_and_update_index(inputs, update_index);
                    (ConstPoolItem::ConstantClass(item), update_index)
                }
                ConstPoolTag::ConstantMethodref => {
                    let (item, update_index) =
                        ConstantMethodref::create_and_update_index(inputs, update_index);
                    (ConstPoolItem::ConstantMethodref(item), update_index)
                }
                ConstPoolTag::ConstantNameAndType => {
                    let (item, update_index) =
                        ConstantNameAndType::create_and_update_index(inputs, update_index);
                    (ConstPoolItem::ConstantNameAndType(item), update_index)
                }
                ConstPoolTag::ConstantUtf8 => {
                    let (item, update_index) =
                        ConstantUtf8::create_and_update_index(string_map, inputs, update_index);
                    (ConstPoolItem::ConstantUtf8(item), update_index)
                }
                ConstPoolTag::ConstantString => {
                    let (item, update_index) =
                        ConstantString::create_and_update_index(inputs, update_index);
                    (ConstPoolItem::ConstantString(item), update_index)
                }
                ConstPoolTag::ConstantFieldref => {
                    let (item, update_index) =
                        ConstantFieldref::create_and_update_index(inputs, update_index);
                    (ConstPoolItem::ConstantFieldref(item), update_index)
                }
                ConstPoolTag::ConstantLong => {
                    let (item, update_index) =
                        ConstantLong::create_and_update_index(inputs, update_index);
                    constant_index += 2;
                    index = update_index;
                    items.push(ConstPoolItem::ConstantLong(item));
                    items.push(ConstPoolItem::ConstantNull);
                    continue;
                }
                ConstPoolTag::ConstantDouble => {
                    let (item, update_index) =
                        ConstantDouble::create_and_update_index(inputs, update_index);
                    constant_index += 2;
                    index = update_index;
                    items.push(ConstPoolItem::ConstantDouble(item));
                    items.push(ConstPoolItem::ConstantNull);
                    continue;
                }
                _ => unimplemented!(
                    "
constant pool purse failed.
current constant pool
{}.
next tag: {}",
                    ConstantPool(items),
                    tag
                ),
            };
            constant_index += 1;
            index = update_index;
            items.push(item);
        }

        (ConstantPool(items), index)
    }

    pub fn get_main_index(&self) -> Option<usize> {
        self.0.iter().position(|item| {
            if let ConstPoolItem::ConstantUtf8(utf8) = item {
                utf8.bytes == vec![0x6D, 0x61, 0x69, 0x6E] // main
            } else {
                false
            }
        })
    }

    pub fn get_clinit_index(&self) -> Option<usize> {
        self.0.iter().position(|item| {
            if let ConstPoolItem::ConstantUtf8(utf8) = item {
                // <clinit>
                utf8.bytes == vec![0x3C, 0x63, 0x6C, 0x69, 0x6E, 0x69, 0x74, 0x3E]
            } else {
                false
            }
        })
    }

    pub fn create_and_set_operand_stack_item(&self, stack: &mut Vec<Item>, index: usize) {
        match self.0.get(index) {
            Some(ref item) => match item {
                // ConstPoolItem::ConstantClass(ConstantClass),
                // ConstPoolItem::ConstantMethodref(ConstantMethodref),
                // ConstPoolItem::ConstantInterfaceMethodref,
                // ConstPoolItem::ConstantNameAndType(ConstantNameAndType),
                ConstPoolItem::ConstantString(ref item) => {
                    stack.push(Item::String(self.get_string(item.string_index)));
                }
                ConstPoolItem::ConstantFieldref(_) => stack.push(Item::Fieldref(index)),
                ConstPoolItem::ConstantUtf8(item) => stack.push(Item::String(item.id)),
                ConstPoolItem::ConstantLong(ref item) => {
                    stack.push(Item::Long(item.high_bytes as i32));
                    stack.push(Item::Long(item.low_bytes as i32));
                }
                ConstPoolItem::ConstantNull => {
                    unreachable!("index: {}. should not come ConstantNull", index)
                }
                _ => unimplemented!("{:?}", item),
            },
            _ => unreachable!("index: {} is not found", index),
        };
    }

    pub fn get_name_and_type(&self, index: usize) -> &ConstantNameAndType {
        match self.0.get(index) {
            Some(ConstPoolItem::ConstantNameAndType(ref item)) => item,
            _ => unreachable!(
                "should be ConstantNameAndType. actual {:?}",
                self.0.get(index)
            ),
        }
    }

    pub fn get_class_ref(&self, index: usize) -> &ConstantClass {
        match self.0.get(index) {
            Some(ConstPoolItem::ConstantClass(ref item)) => item,
            _ => unreachable!("should be ConstantClass. actual {:?}", self.0.get(index)),
        }
    }

    pub fn get_class_ref_name(&self, index: usize) -> usize {
        match self.0.get(index) {
            Some(ConstPoolItem::ConstantClass(ref item)) => self.get_utf8(item.name_index),
            _ => unreachable!("should be ConstantClass. actual {:?}", self.0.get(index)),
        }
    }

    // (class_name, field_name)
    pub fn get_class_and_field_name(&self, index: usize) -> (usize, usize) {
        let field = self.get_field_ref(index);
        let name_and_type = self.get_name_and_type(field.name_and_type_index);
        let class_name = self.get_class_ref_name(field.class_index);
        let field_name = self.get_utf8(name_and_type.name_index);
        (class_name, field_name)
    }

    pub fn get_method_ref(&self, index: usize) -> &ConstantMethodref {
        match self.0.get(index) {
            Some(ConstPoolItem::ConstantMethodref(ref item)) => item,
            _ => unreachable!(
                "should be ConstantMethodref. actual {:?}",
                self.0.get(index)
            ),
        }
    }

    pub fn get_field_ref(&self, index: usize) -> &ConstantFieldref {
        match self.0.get(index) {
            Some(ConstPoolItem::ConstantFieldref(ref item)) => item,
            _ => unreachable!("should be ConstantFieldref. actual {:?}", self.0.get(index)),
        }
    }

    pub fn get_string(&self, index: usize) -> usize {
        match self.0.get(index) {
            Some(ConstPoolItem::ConstantString(ref item)) => self.get_utf8(item.string_index),
            _ => unreachable!("should be ConstantString. actual {:?}", self.0.get(index)),
        }
    }

    pub fn get_utf8(&self, index: usize) -> usize {
        match self.0.get(index) {
            Some(ConstPoolItem::ConstantUtf8(item)) => item.id,
            _ => unreachable!("should be ConstantUtf8. actual {:?}", self.0.get(index)),
        }
    }

    pub fn get_utf8_as_string(&self, string_pool: &mut StringPool, index: usize) -> String {
        let id = self.get_utf8(index);
        string_pool.get_value(&id)
    }

    pub fn get_fieldref_as_utf8(&self, index: usize) -> usize {
        match self.0.get(index) {
            Some(ConstPoolItem::ConstantFieldref(ref item)) => {
                let name_and_type = self.get_name_and_type(item.name_and_type_index);
                self.get_utf8(name_and_type.name_index)
            }
            _ => unreachable!("should be ConstantFieldRef. actual {:?}", self.0.get(index)),
        }
    }
}

impl fmt::Display for ConstantPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = Vec::with_capacity(self.0.len());
        for (index, item) in self.0.iter().enumerate() {
            let rw = match item {
                ConstPoolItem::ConstantNull => {
                    continue;
                }
                ConstPoolItem::ConstantMethodref(item) => format!(
                    "  #{} = Methodref        #{}.#{}",
                    index, item.class_index, item.name_and_type_index,
                ),
                ConstPoolItem::ConstantFieldref(item) => format!(
                    "  #{} = Fieldref         #{}.#{}",
                    index, item.class_index, item.name_and_type_index
                ),
                ConstPoolItem::ConstantString(item) => {
                    format!("  #{} = String           #{}", index, item.string_index)
                }
                ConstPoolItem::ConstantClass(item) => {
                    format!("  #{} = Class            #{}", index, item.name_index)
                }
                ConstPoolItem::ConstantUtf8(item) => format!(
                    "  #{} = Utf8             {}",
                    index,
                    String::from_utf8_lossy(item.bytes.as_slice())
                ),
                ConstPoolItem::ConstantNameAndType(item) => format!(
                    "  #{} = NameAndType      #{}:#{}",
                    index, item.name_index, item.descriptor_index
                ),
                ConstPoolItem::ConstantLong(item) => format!(
                    "  #{} = Long             {}l",
                    index,
                    ((item.high_bytes as i64) << 32) as i64 | item.low_bytes as i64
                ),
                ConstPoolItem::ConstantDouble(item) => format!(
                    "  #{} = Double           {}",
                    index,
                    ((item.high_bytes << 8) | item.low_bytes) & 0xFFFF
                ),
                _ => unimplemented!(),
            };
            result.push(rw);
        }
        write!(f, "{}", result.join("\n"))
    }
}

#[derive(Debug, PartialEq)]
pub enum ConstPoolTag {
    ConstantNull = 0, // custom tag for index 0
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

impl From<usize> for ConstPoolTag {
    fn from(num: usize) -> ConstPoolTag {
        match num {
            0 => ConstPoolTag::ConstantNull,
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

#[derive(Debug, PartialEq)]
pub enum ConstPoolItem {
    ConstantNull,
    ConstantClass(ConstantClass),
    ConstantFieldref(ConstantFieldref),
    ConstantMethodref(ConstantMethodref),
    ConstantInterfaceMethodref,
    ConstantString(ConstantString),
    ConstantInteger,
    ConstantFloat,
    ConstantLong(ConstantLong),
    ConstantDouble(ConstantDouble),
    ConstantNameAndType(ConstantNameAndType),
    ConstantUtf8(ConstantUtf8),
    ConstantMethodHandle,
    ConstantMethodType,
    ConstantInvokeDynamic,
}

#[derive(Debug, PartialEq)]
pub struct ConstantLong {
    pub tag: ConstPoolTag,
    pub high_bytes: i32, // u4
    pub low_bytes: i32,  // u4
}

impl ConstantLong {
    pub fn create_and_update_index(inputs: &mut [u8], index: usize) -> (ConstantLong, usize) {
        let (high_bytes, index) = extract_x_byte_as_usize(inputs, index, 4);
        let (low_bytes, index) = extract_x_byte_as_usize(inputs, index, 4);
        (
            ConstantLong {
                tag: ConstPoolTag::ConstantString,
                high_bytes: high_bytes as i32,
                low_bytes: low_bytes as i32,
            },
            index,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct ConstantDouble {
    pub tag: ConstPoolTag,
    pub high_bytes: usize, // u4
    pub low_bytes: usize,  // u4
}

impl ConstantDouble {
    pub fn create_and_update_index(inputs: &mut [u8], index: usize) -> (ConstantDouble, usize) {
        let (high_bytes, index) = extract_x_byte_as_usize(inputs, index, 4);
        let (low_bytes, index) = extract_x_byte_as_usize(inputs, index, 4);
        (
            ConstantDouble {
                tag: ConstPoolTag::ConstantString,
                high_bytes,
                low_bytes,
            },
            index,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct ConstantString {
    pub tag: ConstPoolTag,
    pub string_index: usize, // u2
}

impl ConstantString {
    pub fn create_and_update_index(inputs: &mut [u8], index: usize) -> (ConstantString, usize) {
        let (string_index, index) = extract_x_byte_as_usize(inputs, index, 2);
        (
            ConstantString {
                tag: ConstPoolTag::ConstantString,
                string_index,
            },
            index,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct ConstantFieldref {
    pub tag: ConstPoolTag,
    pub class_index: usize,         // u2
    pub name_and_type_index: usize, // u2
}

impl ConstantFieldref {
    pub fn create_and_update_index(inputs: &mut [u8], index: usize) -> (ConstantFieldref, usize) {
        let (class_index, index) = extract_x_byte_as_usize(inputs, index, 2);
        let (name_and_type_index, index) = extract_x_byte_as_usize(inputs, index, 2);
        (
            ConstantFieldref {
                tag: ConstPoolTag::ConstantFieldref,
                class_index,
                name_and_type_index,
            },
            index,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct ConstantNameAndType {
    pub tag: ConstPoolTag,
    pub name_index: usize,       // u2
    pub descriptor_index: usize, // u2
}

impl ConstantNameAndType {
    pub fn create_and_update_index(
        inputs: &mut [u8],
        index: usize,
    ) -> (ConstantNameAndType, usize) {
        let (name_index, index) = extract_x_byte_as_usize(inputs, index, 2);
        let (descriptor_index, index) = extract_x_byte_as_usize(inputs, index, 2);

        (
            ConstantNameAndType {
                tag: ConstPoolTag::ConstantNameAndType,
                name_index,
                descriptor_index,
            },
            index,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct ConstantClass {
    pub tag: ConstPoolTag,
    pub name_index: usize, // u2
}

impl ConstantClass {
    pub fn create_and_update_index(inputs: &mut [u8], index: usize) -> (ConstantClass, usize) {
        let (name_index, index) = extract_x_byte_as_usize(inputs, index, 2);
        (
            ConstantClass {
                tag: ConstPoolTag::ConstantClass,
                name_index,
            },
            index,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct ConstantMethodref {
    pub tag: ConstPoolTag,
    pub class_index: usize,         // u2
    pub name_and_type_index: usize, // u2
}

impl ConstantMethodref {
    pub fn create_and_update_index(inputs: &mut [u8], index: usize) -> (ConstantMethodref, usize) {
        let (class_index, index) = extract_x_byte_as_usize(inputs, index, 2);
        let (name_and_type_index, index) = extract_x_byte_as_usize(inputs, index, 2);

        (
            ConstantMethodref {
                tag: ConstPoolTag::ConstantMethodref,
                class_index,
                name_and_type_index,
            },
            index,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct ConstantUtf8 {
    pub id: usize, // custom value
    pub tag: ConstPoolTag,
    pub length: usize, // u2
    pub bytes: Vec<u8>,
}

impl ConstantUtf8 {
    pub fn create_and_update_index(
        string_map: &mut StringPool,
        inputs: &mut [u8],
        index: usize,
    ) -> (ConstantUtf8, usize) {
        let (utf8_length, index) = extract_x_byte_as_usize(inputs, index, 2);
        let (bytes, index) = extract_x_byte_as_vec(inputs, index, utf8_length);
        let value = String::from_utf8_lossy(bytes.as_slice());
        let id = string_map.insert(value.to_string());

        (
            ConstantUtf8 {
                id,
                tag: ConstPoolTag::ConstantUtf8,
                length: utf8_length,
                bytes,
            },
            index,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn constant_pool_constant_methodref() {
        let mut inputs = vec![
            0x0a, // class
            0x00, 0x0a, // class_index
            0x00, 0x0b, // name_and_type_index
        ];

        let result = ConstantPool::new(&mut StringPool::new(), &mut inputs, 0, 2);

        assert_eq!(
            result,
            (
                ConstantPool(vec![
                    ConstPoolItem::ConstantNull,
                    ConstPoolItem::ConstantMethodref(ConstantMethodref {
                        tag: ConstPoolTag::ConstantMethodref,
                        class_index: 0x0a,
                        name_and_type_index: 0x0b
                    })
                ]),
                inputs.len()
            )
        );
    }

    #[test]
    fn constant_pool_constant_class() {
        let mut inputs = vec![
            0x07, // class
            0x00, 0x0b, // name_index
        ];
        let result = ConstantPool::new(&mut StringPool::new(), &mut inputs, 0, 2);

        assert_eq!(
            result,
            (
                ConstantPool(vec![
                    ConstPoolItem::ConstantNull,
                    ConstPoolItem::ConstantClass(ConstantClass {
                        tag: ConstPoolTag::ConstantClass,
                        name_index: 0x0b
                    })
                ]),
                inputs.len()
            )
        );
    }

    // #[test]
    // fn constant_pool_utf8() {
    //     let mut inputs = [
    //         0x01, // utf8
    //         0x00, 0x0A, // length
    //         0x53, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x46, 0x69, 0x6C, 0x65, // bytes(SourceFile)
    //     ];
    //     let result = ConstantPool::new(&mut inputs, 0, 2);

    //     assert_eq!(
    //         result,
    //         (
    //             ConstantPool(vec![
    //                 ConstPoolItem::ConstantNull,
    //                 ConstPoolItem::ConstantUtf8(ConstantUtf8 {

    //                     tag: ConstPoolTag::ConstantUtf8,
    //                     length: 0x0a,
    //                     bytes: vec![0x53, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x46, 0x69, 0x6C, 0x65]
    //                 })
    //             ]),
    //             inputs.len()
    //         )
    //     );
    // }

    #[test]
    fn constant_pool_name_and_type() {
        let mut inputs = vec![
            0x0c, // name_and_type
            0x00, 0x0a, // name_index
            0x00, 0x0b, // descriptor_index
        ];
        let result = ConstantPool::new(&mut StringPool::new(), &mut inputs, 0, 2);

        assert_eq!(
            result,
            (
                ConstantPool(vec![
                    ConstPoolItem::ConstantNull,
                    ConstPoolItem::ConstantNameAndType(ConstantNameAndType {
                        tag: ConstPoolTag::ConstantNameAndType,
                        name_index: 0x0a,
                        descriptor_index: 0x0b
                    })
                ]),
                inputs.len()
            )
        );
    }

    #[test]
    fn constant_pool_name_and_type_print() {
        let mut inputs = vec![
            0x0c, // name_and_type
            0x00, 0x0a, // name_index
            0x00, 0x0b, // descriptor_index
        ];
        let (constant_pool, _) = ConstantPool::new(&mut StringPool::new(), &mut inputs, 0, 2);

        assert_eq!(
            format!("{}", constant_pool),
            "  #1 = NameAndType      #10:#11"
        );
    }
}
