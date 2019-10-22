use crate::operand::{OperandStack, OperandStackItem};

#[derive(Debug)]
pub enum StackframeItem {
    Null,
    I32(i32),
    Long(i64),
    String(String),
    Utf8(usize),
    Classref(usize),
    Fieldref(usize),
}

impl From<OperandStackItem> for StackframeItem {
    fn from(item: OperandStackItem) -> StackframeItem {
        match item {
            OperandStackItem::I32(value) => StackframeItem::I32(value),
            OperandStackItem::Long(value) => StackframeItem::Long(value),
            OperandStackItem::String(value) => StackframeItem::String(value),
            OperandStackItem::Utf8(index) => StackframeItem::Utf8(index),
            OperandStackItem::Classref(index) => StackframeItem::Classref(index),
            OperandStackItem::Fieldref(index) => StackframeItem::Fieldref(index),
            OperandStackItem::Null => StackframeItem::Null,
        }
    }
}

#[derive(Debug)]
pub struct Stackframe {
    pub local_variables: Vec<StackframeItem>,
}

impl Stackframe {
    pub fn new(variables_number: usize) -> Self {
        Stackframe {
            local_variables: Vec::with_capacity(variables_number),
        }
    }

    pub fn istore(&mut self, operand_stack: &mut OperandStack, index: usize) {
        if let Some(val) = operand_stack.stack.pop() {
            self.local_variables
                .insert(index, StackframeItem::from(val));
        }
    }
}
