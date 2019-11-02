use crate::attribute::code::Code;
use crate::attribute::instruction::Instruction;
use crate::constant::{ConstantNameAndType, ConstantPool};
use crate::field::{BaseType, FieldDescriptor};
use crate::java_class::{custom::Custom, JavaClass};
use crate::operand::Item;
use crate::stackframe::Stackframe;
use crate::utils::{emit_debug_info, read_file};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::mem;
use std::path::Path;

#[derive(Debug)]
pub struct Context<'a> {
    pub class_map: HashMap<String, JavaClass>,
    pub program_count: usize,
    pub stack_frames: Vec<Stackframe>,
    pub root_path: &'a str,
    pub static_fields: StaticFields,
}
pub type ClassMap = HashMap<String, JavaClass>;
// class_name, field_name
pub type StaticFields = HashMap<(String, String), (Item, Item)>;

impl<'a> Context<'a> {
    pub fn new(class_map: ClassMap, class_file: &Custom, root_path: &'a str) -> Context<'a> {
        let mut static_fields = setup_static_fields(&class_map);
        set_static_fields(&class_file, &mut static_fields);

        Context {
            class_map,
            program_count: 0,
            stack_frames: vec![],
            root_path,
            static_fields,
        }
    }

    pub fn run_entry_file(&mut self, class_file: Custom) {
        let entry_method = class_file
            .get_entry_method()
            .expect("add handler in the case failed to find entry method");

        // TBD Perhaps this method is not invoked from super_class
        let super_class_index = class_file.super_class;
        let super_class_ref = class_file.cp_info.get_class_ref(super_class_index);
        let super_class_name = class_file.cp_info.get_utf8(super_class_ref.name_index);
        let stack_frame_item_0 = Item::Classref(super_class_name);

        if let Some(code) = class_file.get_clinit_code() {
            let stack_frame = Stackframe::new(code.max_locals as usize);
            self.stack_frames.push(stack_frame);
            self.call_custom_class_method(&class_file, code);
        }

        let code = entry_method
            .extract_code()
            .expect("should exist code in method");
        let mut stack_frame = Stackframe::new(code.max_locals as usize);
        stack_frame.local_variables.push(stack_frame_item_0);
        self.stack_frames.push(stack_frame);
        self.run_method(&class_file, code);

        self.class_map
            .insert(class_file.this_class_name(), JavaClass::Custom(class_file));
    }

    fn run_method(&mut self, class_file: &Custom, code: &Code) {
        let mut index = 0;
        while let Some(instruction) = code.code.get(index) {
            emit_debug_info(instruction, self.stack_frames.last());
            let (should_finish, update_index) = self.execute(class_file, instruction, index);
            if should_finish {
                break;
            }
            index = update_index + 1;
        }
        self.stack_frames.pop();
    }

    pub fn execute(
        &mut self,
        class_file: &Custom,
        instruction: &Instruction,
        index: usize,
    ) -> (bool, usize) {
        match instruction {
            Instruction::Iadd => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let item = stackframe.operand_stack.iadd();
                stackframe.operand_stack.stack.push(item);
            }
            Instruction::Ladd => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let (first, second) = stackframe.operand_stack.ladd();
                stackframe.operand_stack.stack.push(first);
                stackframe.operand_stack.stack.push(second);
            }
            Instruction::Isub => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let item = stackframe.operand_stack.isub();
                stackframe.operand_stack.stack.push(item);
            }
            Instruction::Lsub => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let (first, second) = stackframe.operand_stack.lsub();
                stackframe.operand_stack.stack.push(first);
                stackframe.operand_stack.stack.push(second);
            }
            Instruction::Imul => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let item = stackframe.operand_stack.imul();
                stackframe.operand_stack.stack.push(item);
            }
            Instruction::Lmul => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let (first, second) = stackframe.operand_stack.lmul();
                stackframe.operand_stack.stack.push(first);
                stackframe.operand_stack.stack.push(second);
            }
            Instruction::Idiv => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let item = stackframe.operand_stack.idiv();
                stackframe.operand_stack.stack.push(item);
            }
            Instruction::Ldiv => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let (first, second) = stackframe.operand_stack.ldiv();
                stackframe.operand_stack.stack.push(first);
                stackframe.operand_stack.stack.push(second);
            }
            Instruction::Irem => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let item = stackframe.operand_stack.irem();
                stackframe.operand_stack.stack.push(item);
            }
            Instruction::Lrem => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let (first, second) = stackframe.operand_stack.lrem();
                stackframe.operand_stack.stack.push(first);
                stackframe.operand_stack.stack.push(second);
            }
            Instruction::IconstN(val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                stackframe.operand_stack.stack.push(Item::Int(*val as i32));
            }
            Instruction::LconstN(val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                stackframe.operand_stack.stack.push(Item::Long(0));
                stackframe.operand_stack.stack.push(Item::Long(*val as i32));
            }
            // maybe need to fix for float or something like that
            Instruction::Bipush(val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                stackframe.operand_stack.stack.push(Item::Int(*val as i32));
            }
            Instruction::Sipush(val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                stackframe.operand_stack.stack.push(Item::Int(*val as i32));
            }
            Instruction::Lookupswitch(vals) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                if let Some(Item::Int(target_key)) = stackframe.operand_stack.stack.pop() {
                    if let Some(jump_pointer) = vals.iter().find(|(optional_key, _)| {
                        if let Some(key) = *optional_key {
                            key == target_key as usize
                        } else {
                            false
                        }
                    }) {
                        return (false, jump_pointer.1);
                    } else {
                        return (false, vals.first().expect("should exist default value").1);
                    }
                } else {
                    unreachable!("should exist operan_item");
                }
            }
            Instruction::Goto(pointer) => {
                return (false, *pointer);
            }
            Instruction::Iinc(index, value) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                if let Some(item) = stackframe.local_variables.get_mut(*index) {
                    if let Item::Int(val) = item {
                        mem::replace(val, *val + *value as i32);
                    }
                }
            }
            Instruction::Lcmp => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let val = stackframe.operand_stack.lcmp();
                stackframe.operand_stack.stack.push(val);
            }
            Instruction::Ifeq(if_val, else_val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let val = stackframe.operand_stack.stack.pop().unwrap();
                let jump_pointer = if val == Item::Int(0) {
                    *if_val
                } else {
                    *else_val
                };
                return (false, jump_pointer);
            }
            Instruction::Ifne(if_val, else_val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let val = stackframe.operand_stack.stack.pop().unwrap();
                let jump_pointer = if val != Item::Int(0) {
                    *if_val
                } else {
                    *else_val
                };
                return (false, jump_pointer);
            }
            Instruction::Iflt(if_val, else_val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let val = stackframe.operand_stack.stack.pop().unwrap();
                let jump_pointer = if val < Item::Int(0) {
                    *if_val
                } else {
                    *else_val
                };
                return (false, jump_pointer);
            }
            Instruction::Ifge(if_val, else_val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let val = stackframe.operand_stack.stack.pop().unwrap();
                let jump_pointer = if val >= Item::Int(0) {
                    *if_val
                } else {
                    *else_val
                };
                return (false, jump_pointer);
            }
            Instruction::Ifgt(if_val, else_val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let val = stackframe.operand_stack.stack.pop().unwrap();
                let jump_pointer = if val > Item::Int(0) {
                    *if_val
                } else {
                    *else_val
                };
                return (false, jump_pointer);
            }
            Instruction::Ifle(if_val, else_val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let val = stackframe.operand_stack.stack.pop().unwrap();
                let jump_pointer = if val <= Item::Int(0) {
                    *if_val
                } else {
                    *else_val
                };
                return (false, jump_pointer);
            }
            Instruction::Ificmpeq(if_val, else_val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let second = stackframe.operand_stack.stack.pop();
                let first = stackframe.operand_stack.stack.pop();
                let jump_pointer = if first == second { *if_val } else { *else_val };
                return (false, jump_pointer);
            }
            Instruction::Ificmpne(if_val, else_val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let second = stackframe.operand_stack.stack.pop();
                let first = stackframe.operand_stack.stack.pop();
                let jump_pointer = if first != second { *if_val } else { *else_val };
                return (false, jump_pointer);
            }
            Instruction::Ificmplt(if_val, else_val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let second = stackframe.operand_stack.stack.pop();
                let first = stackframe.operand_stack.stack.pop();
                let jump_pointer = if first < second { *if_val } else { *else_val };
                return (false, jump_pointer);
            }
            Instruction::Ificmpge(if_val, else_val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let second = stackframe.operand_stack.stack.pop();
                let first = stackframe.operand_stack.stack.pop();
                let jump_pointer = if first >= second { *if_val } else { *else_val };
                return (false, jump_pointer);
            }
            Instruction::Ificmpgt(if_val, else_val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let second = stackframe.operand_stack.stack.pop();
                let first = stackframe.operand_stack.stack.pop();
                let jump_pointer = if first > second { *if_val } else { *else_val };
                return (false, jump_pointer);
            }
            Instruction::Ificmple(if_val, else_val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let second = stackframe.operand_stack.stack.pop();
                let first = stackframe.operand_stack.stack.pop();
                let jump_pointer = if first <= second { *if_val } else { *else_val };
                return (false, jump_pointer);
            }
            Instruction::Iload(index) => {
                self.load_n(*index);
            }
            Instruction::IloadN(index) => {
                self.load_n(*index);
            }
            Instruction::LloadN(index) => {
                let base_index = *index;
                self.load_n(base_index);
                self.load_n(base_index + 1);
            }
            Instruction::Istore(index) => {
                self.store_n(&[*index]);
            }
            Instruction::IstoreN(index) => {
                self.store_n(&[*index]);
            }
            Instruction::LstoreN(index) => {
                let base_index = *index;
                self.store_n(&[base_index + 1, base_index]);
            }
            Instruction::AloadN(index) => {
                self.load_n(*index);
            }
            Instruction::AstoreN(index) => {
                self.store_n(&[*index]);
            }
            Instruction::Putstatic(index) => {
                let this_class_name = class_file.this_class_name();
                let (class_name, field_name) = self.get_class_and_field_name(class_file, *index);
                self.initilize_class_static_info(&this_class_name, &class_name);

                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let (first, second) = match stackframe.operand_stack.stack.pop() {
                    Some(second @ Item::Long(_)) => {
                        let first = stackframe.operand_stack.stack.pop().unwrap();
                        (first, second)
                    }
                    first @ _ => (first.unwrap(), Item::Null),
                };
                self.static_fields
                    .insert((class_name, field_name), (first, second));
            }
            Instruction::Getstatic(index) => {
                let this_class_name = class_file.this_class_name();
                let (class_name, field_name) = self.get_class_and_field_name(class_file, *index);
                self.initilize_class_static_info(&this_class_name, &class_name);

                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let err_message = format!(
                    "Getstatic failed. {}.{} is not found",
                    &class_name, &field_name
                );
                let items = self
                    .static_fields
                    .get_mut(&(class_name, field_name))
                    .expect(&err_message);
                stackframe.operand_stack.stack.push(items.0.clone());
                match items.0 {
                    Item::Long(_) => {
                        stackframe.operand_stack.stack.push(items.1.clone());
                    }
                    _ => {}
                };
            }
            Instruction::Areturn | Instruction::Ireturn => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let item = if let Some(item) = stackframe.operand_stack.stack.pop() {
                    stackframe.operand_stack.stack.clear();
                    item
                } else {
                    unreachable!("should exist return value on operand_stack")
                };
                let length = self.stack_frames.len();
                if let Some(stackframe) = self.stack_frames.get_mut(length - 2) {
                    stackframe.operand_stack.stack.push(item);
                } else {
                    unreachable!("should exist over two stack_frame");
                }
                return (true, index);
            }
            Instruction::Pop => {
                if let Some(stackframe) = self.stack_frames.last_mut() {
                    stackframe.operand_stack.stack.pop();
                }
            }
            Instruction::Dup => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let last = if let Some(last) = stackframe.operand_stack.stack.last() {
                    last.clone()
                } else {
                    unreachable!("should have an item at least");
                };
                stackframe.operand_stack.stack.push(last);
            }
            Instruction::Invokevirtual(index) | Instruction::Invokespecial(index) => {
                let (class_name, name_and_type) = self.get_related_method_info(class_file, *index);
                self.call_method(&class_file, class_name, name_and_type);
            }
            Instruction::Invokestatic(index) => {
                let this_class_name = class_file.this_class_name();
                let (class_name, name_and_type) = self.get_related_method_info(class_file, *index);
                self.initilize_class_static_info(&this_class_name, &class_name);
                self.call_method(&class_file, class_name, name_and_type);
            }
            Instruction::Putfield(index) => {
                let (class_name, field_name) = class_file.cp_info.get_class_and_field_name(*index);
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let operand_stack = &mut stackframe.operand_stack.stack;

                let first = operand_stack
                    .pop()
                    .expect("should exist operand stack item");
                let second = match first {
                    Item::Long(_) => operand_stack
                        .pop()
                        .expect("should exist operand stack item"),
                    _ => Item::Null,
                };

                match operand_stack.pop() {
                    Some(Item::Objectref(object_class_name, mut field_map)) => {
                        assert!(
                            class_name == object_class_name,
                            "should be equal class_name"
                        );
                        field_map.insert(field_name, (first, second));
                        operand_stack.push(Item::Objectref(object_class_name, field_map));
                    }
                    Some(item) => unreachable!("should be Objectref. actual: {}", item),
                    None => unreachable!("should be Objectref. actual: None"),
                };
            }
            Instruction::Getfield(index) => {
                let (class_name, field_name) = class_file.cp_info.get_class_and_field_name(*index);
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let operand_stack = &mut stackframe.operand_stack.stack;
                match operand_stack.pop() {
                    Some(Item::Objectref(object_class_name, field_map)) => {
                        assert!(
                            class_name == object_class_name,
                            "should be equal class_name"
                        );
                        let (first, second) =
                            field_map.get(&field_name).expect("should exist item");
                        match first {
                            Item::Long(val) => {
                                operand_stack.push(Item::Long(*val));
                                operand_stack.push(second.clone());
                            }
                            item @ _ => {
                                operand_stack.push(item.clone());
                            }
                        }
                    }
                    Some(item) => unreachable!("should be Objectref. actual: {}", item),
                    None => unreachable!("should be Objectref. actual: None"),
                };
            }
            Instruction::Ldc(index) => {
                let string_val = class_file.cp_info.get_string(*index);
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                stackframe
                    .operand_stack
                    .stack
                    .push(Item::String(string_val));
            }
            Instruction::Ldc2W(first, second) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                class_file.cp_info.create_and_set_operand_stack_item(
                    &mut stackframe.operand_stack.stack,
                    (*first << 8 | *second) & 0xFFFF,
                );
            }
            Instruction::New(index) => {
                let this_class_name = class_file.this_class_name();
                let class_ref = class_file.cp_info.get_class_ref(*index);
                let class_name = class_file.cp_info.get_utf8(class_ref.name_index);
                self.initilize_class_static_info(&this_class_name, &class_name);
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                if let Some(JavaClass::Custom(target_class)) = self.class_map.get(&class_name) {
                    let mut map = HashMap::new();
                    for field in target_class.fields.iter() {
                        let field_name = target_class.cp_info.get_utf8(field.name_index);
                        let descriptor = target_class.cp_info.get_utf8(field.descriptor_index);
                        let value =
                            create_uninitialized_item(&FieldDescriptor::from(descriptor.as_ref()));
                        map.insert(field_name, value);
                    }

                    stackframe
                        .operand_stack
                        .stack
                        .push(Item::Objectref(class_name, map));
                } else {
                    unreachable!("not come here")
                }
            }
            Instruction::Return => {}
            _ => {}
        };
        (false, index + instruction.counsume_index())
    }

    fn call_method(
        &mut self,
        class_file: &Custom,
        class_name: String,
        name_and_type: &ConstantNameAndType,
    ) {
        let method_name = class_file.cp_info.get_utf8(name_and_type.name_index);
        let method_descriptor = class_file.cp_info.get_utf8(name_and_type.descriptor_index);

        if let Some(mut class) = self.class_map.remove(&class_name) {
            self.call_other_class_method(
                &mut class,
                &class_file.cp_info,
                &method_name,
                &method_descriptor,
            );
            self.class_map.insert(class.this_class_name(), class);
        } else {
            let new_class_file = self.create_custom_class(&class_name);
            let mut new_class_file = JavaClass::Custom(new_class_file);

            self.call_other_class_method(
                &mut new_class_file,
                &class_file.cp_info,
                &method_name,
                &method_descriptor,
            );
            self.class_map.insert(class_name, new_class_file);
        }
    }

    fn initilize_class_static_info(&mut self, this_class_name: &str, class_name: &str) {
        if this_class_name != class_name && self.class_map.get_mut(class_name).is_none() {
            let new_class_file = self.create_custom_class(&class_name);
            if let Some(code) = new_class_file.get_clinit_code() {
                self.call_custom_class_method(&new_class_file, code);
            }

            self.class_map
                .insert(class_name.to_string(), JavaClass::Custom(new_class_file));
        }
    }

    fn create_custom_class(&mut self, class_name: &str) -> Custom {
        let class_name = class_name.to_string() + ".class";
        let class_path = Path::new(self.root_path).join(&class_name);
        let mut buffer = vec![];
        let buffer = read_file(&class_path, &mut buffer).expect(&format!(
            "need to add handler for the case failed to find the class file: {}",
            class_name
        ));
        let (new_class_file, _pc_count) = Custom::new(buffer, 0);
        // TBD should be set initial value
        set_static_fields(&new_class_file, &mut self.static_fields);
        new_class_file
    }

    fn call_other_class_method(
        &mut self,
        class_file: &mut JavaClass,
        caller_cp_info: &ConstantPool,
        method_name: &str,
        method_descriptor: &str,
    ) {
        match class_file {
            JavaClass::BuiltIn(ref mut builtin_class) => {
                let method = builtin_class.methods.get_mut(method_name).expect(&format!(
                    "{} is not found in {}",
                    method_name, builtin_class.class_name
                ));
                let parameter_length = method.parameter_length(&method_descriptor);
                let stack_frame = self.create_new_stack_frame(parameter_length);
                self.stack_frames.push(stack_frame);
                method.execute(&caller_cp_info, &mut self.stack_frames);
            }
            JavaClass::Custom(ref custom_class) => {
                if let Some(method_code) =
                    custom_class.get_method_code_by_string(method_name, method_descriptor)
                {
                    self.call_custom_class_method(custom_class, &method_code);
                }
            }
        }
    }

    fn call_custom_class_method(&mut self, class: &Custom, code: &Code) {
        let local_variable_length = code.max_locals as usize;
        let stack_frame = self.create_new_stack_frame(local_variable_length);
        self.stack_frames.push(stack_frame);
        self.run_method(class, code);
    }

    fn load_n(&mut self, index: usize) {
        let stackframe = self
            .stack_frames
            .last_mut()
            .expect("should exist stack_frame");
        let value = stackframe
            .local_variables
            .get(index)
            .expect("should exist local variable");
        stackframe
            .operand_stack
            .stack
            .push(Item::from(value.clone()));
    }

    fn store_n(&mut self, indexs: &[usize]) {
        let index_size = indexs.len();
        let mut item_vec = Vec::with_capacity(index_size);
        let stackframe = self
            .stack_frames
            .last_mut()
            .expect("should exist stack_frame");
        for i in 0..index_size {
            let item = stackframe
                .operand_stack
                .stack
                .pop()
                .expect("should have item in operand_stack");
            item_vec.push((indexs[i], item));
        }
        item_vec.sort_by(|before, after| {
            if before.0 > after.0 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });

        for (index, item) in item_vec.into_iter() {
            if stackframe.local_variables.get(index).is_some() {
                stackframe.local_variables[index] = Item::from(item);
            } else {
                stackframe.local_variables.insert(index, Item::from(item));
            }
        }
    }

    // (class_name, field_name)
    fn get_class_and_field_name(&mut self, class_file: &Custom, index: usize) -> (String, String) {
        let field_ref = class_file.cp_info.get_field_ref(index);
        let class_ref = class_file.cp_info.get_class_ref(field_ref.class_index);
        let name_and_type = class_file
            .cp_info
            .get_name_and_type(field_ref.name_and_type_index);
        (
            class_file.cp_info.get_utf8(class_ref.name_index),
            class_file.cp_info.get_utf8(name_and_type.name_index),
        )
    }

    fn get_related_method_info<'b>(
        &mut self,
        class_file: &'b Custom,
        index: usize,
    ) -> (String, &'b ConstantNameAndType) {
        let method_ref = class_file.cp_info.get_method_ref(index);
        let class_ref = class_file.cp_info.get_class_ref(method_ref.class_index);
        let name_and_type = class_file
            .cp_info
            .get_name_and_type(method_ref.name_and_type_index);
        let class_name = class_file.cp_info.get_utf8(class_ref.name_index);
        (class_name, name_and_type)
    }

    fn create_new_stack_frame(&mut self, local_variable_length: usize) -> Stackframe {
        let mut new_stack_frame = Stackframe::new(local_variable_length);
        let stackframe = self
            .stack_frames
            .last_mut()
            .expect("should exist stack_frame");

        // TBD need to fix this
        let mut variables: Vec<_> = stackframe
            .operand_stack
            .stack
            .iter()
            .rev()
            .map(|operand_item| Item::from(operand_item.clone()))
            .collect();
        let mut variables = variables.drain(0..local_variable_length).rev().collect();
        new_stack_frame.local_variables.append(&mut variables);

        // TBD need to fix this
        for _ in 0..local_variable_length {
            let _ = stackframe.operand_stack.stack.pop();
        }

        new_stack_frame
    }
}

pub fn set_static_fields(class: &Custom, static_fields: &mut StaticFields) {
    for field in class.fields.iter() {
        let field_name = class.cp_info.get_utf8(field.name_index);
        let value = create_uninitialized_item(&class.get_descriptor(field.descriptor_index));
        static_fields.insert((class.this_class_name(), field_name), value);
    }
}

pub fn setup_static_fields(class_map: &ClassMap) -> StaticFields {
    let mut static_fields = HashMap::new();
    for key in class_map.keys() {
        if let Some(JavaClass::Custom(class)) = class_map.get(key) {
            set_static_fields(&class, &mut static_fields);
        }
    }

    static_fields.insert(
        (String::from("java/lang/System"), String::from("out")),
        (
            Item::Classref(String::from("java/io/PrintStream")),
            Item::Null,
        ),
    );

    static_fields
}

// TBD need to create system to express uninitialized value
pub fn create_uninitialized_item(descriptor: &FieldDescriptor) -> (Item, Item) {
    match descriptor {
        FieldDescriptor::BaseType(BaseType::I) => (Item::Int(0), Item::Null),
        FieldDescriptor::BaseType(BaseType::J) => (Item::Long(0), Item::Long(0)),
        FieldDescriptor::BaseType(BaseType::Z) => (Item::Boolean(true), Item::Null),
        _ => unimplemented!("should implement"),
    }
}
