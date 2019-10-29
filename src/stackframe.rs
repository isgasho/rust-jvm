use crate::operand::{OperandStack, OperandStackItem};

#[derive(Debug)]
pub enum StackframeItem {
    Null,
    Int(i32),
    Long(i32),
    String(String),
    Classref(String),
    Fieldref(usize),
    Objectref(usize),
}

impl From<OperandStackItem> for StackframeItem {
    fn from(item: OperandStackItem) -> StackframeItem {
        match item {
            OperandStackItem::Int(value) => StackframeItem::Int(value),
            OperandStackItem::Long(value) => StackframeItem::Long(value),
            OperandStackItem::String(value) => StackframeItem::String(value),
            OperandStackItem::Classref(value) => StackframeItem::Classref(value),
            OperandStackItem::Fieldref(index) => StackframeItem::Fieldref(index),
            OperandStackItem::Objectref(index) => StackframeItem::Objectref(index),
            OperandStackItem::Null => StackframeItem::Null,
        }
    }
}

impl From<&OperandStackItem> for StackframeItem {
    fn from(item: &OperandStackItem) -> StackframeItem {
        match item {
            OperandStackItem::Int(value) => StackframeItem::Int(*value),
            OperandStackItem::Long(value) => StackframeItem::Long(*value),
            OperandStackItem::String(value) => StackframeItem::String(value.clone()),
            OperandStackItem::Classref(value) => StackframeItem::Classref(value.clone()),
            OperandStackItem::Fieldref(index) => StackframeItem::Fieldref(*index),
            OperandStackItem::Objectref(index) => StackframeItem::Objectref(*index),
            OperandStackItem::Null => StackframeItem::Null,
        }
    }
}

#[derive(Debug)]
pub struct Stackframe {
    pub local_variables: Vec<StackframeItem>,
    pub operand_stack: OperandStack,
}

impl Stackframe {
    pub fn new(variables_number: usize) -> Self {
        Stackframe {
            local_variables: Vec::with_capacity(variables_number),
            operand_stack: OperandStack::new(),
        }
    }

    pub fn istore(&mut self, operand_stack: &mut OperandStack, index: usize) {
        if let Some(val) = operand_stack.stack.pop() {
            self.local_variables
                .insert(index, StackframeItem::from(val));
        }
    }
}
