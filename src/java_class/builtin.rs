use std::collections::HashMap;

use crate::constant::ConstantPool;
use crate::operand::Item;
use crate::stackframe::Stackframe;
use crate::string_pool::StringPool;

#[derive(Debug)]
pub struct BuiltIn {
    pub class_name: usize,
    pub methods: HashMap<usize, BuiltInMethod>,
}

impl BuiltIn {
    pub fn new(class_name: usize) -> BuiltIn {
        BuiltIn {
            class_name,
            methods: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub enum BuiltInLocal {
    Println,
}

#[derive(Debug)]
pub struct BuiltInMethod {
    pub name: usize,
    pub code_type: BuitlInCodeType,
}

impl BuiltInMethod {
    pub fn new(name: usize, code_type: BuitlInCodeType) -> BuiltInMethod {
        BuiltInMethod { name, code_type }
    }

    pub fn parameter_length(&self, string_map: &mut StringPool, descriptor: usize) -> usize {
        let descriptor = string_map.get_value(&descriptor);
        match self.code_type {
            BuitlInCodeType::Println => match descriptor.as_ref() {
                "(J)V" | "(D)V" => 2,
                _ => 1,
            },
            BuitlInCodeType::JavaLangSystemInit
            | BuitlInCodeType::JavaLangObjectInit
            | BuitlInCodeType::JavaLangObjectToString => 1,
        }
    }

    pub fn execute(
        &mut self,
        string_map: &mut StringPool,
        constant_pool: &ConstantPool,
        stackframes: &mut Vec<Stackframe>,
    ) {
        let mut stackframe = stackframes.pop().expect("should has stack_frame");
        match self.code_type {
            BuitlInCodeType::Println => {
                if let Some(item) = stackframe.local_variables.get(0) {
                    match item {
                        Item::Fieldref(index) => {
                            let value =
                                string_map.get_value(&constant_pool.get_fieldref_as_utf8(*index));
                            println!("{}", value);
                        }
                        Item::String(id) => {
                            let value = string_map.get_value(id);
                            println!("{}", value);
                        }
                        Item::Int(value) => {
                            println!("{}", value);
                        }
                        Item::Long(second) => {
                            if let Some(Item::Long(first)) = stackframe.local_variables.get(1) {
                                println!("{}", ((*first as i64) << 32) as i64 | *second as i64);
                            } else {
                                unreachable!("should exist long second item")
                            }
                            let _ = stackframe.operand_stack.stack.pop();
                        }
                        // TBD should fix to output value correctly
                        Item::Objectref(object_ref) => {
                            println!("objectref: {}", object_ref);
                        }
                        Item::Float(value) => {
                            // TBD need to fix output format
                            println!("{}", value);
                        }
                        _ => unimplemented!(),
                    };
                    let _ = stackframe.operand_stack.stack.pop();
                } else {
                    unreachable!("should have a argument for println")
                }
            }
            BuitlInCodeType::JavaLangSystemInit | BuitlInCodeType::JavaLangObjectInit => {}
            BuitlInCodeType::JavaLangObjectToString => {
                let val = if let Some(Item::Int(val)) = stackframe.local_variables.get(0) {
                    val
                } else {
                    unreachable!("should have a argument for toString")
                };
                let stackframe = stackframes.last_mut().expect("should exist stackframe");
                let string_id = string_map.insert(val.to_string());
                stackframe.operand_stack.stack.push(Item::String(string_id));
            }
        }
    }
}

#[derive(Debug)]
pub enum BuitlInCodeType {
    Println,
    JavaLangObjectInit,
    JavaLangSystemInit,
    JavaLangObjectToString,
}
