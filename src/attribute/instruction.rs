use crate::utils::{extract_x_byte_as_usize, extract_x_byte_as_vec};
use std::fmt;

#[derive(Debug)]
pub enum Instruction {
    Aconstnull,                                // 0x01
    IconstN(i32),                              // 0x02(-1) - 0x08(5)
    LconstN(usize),                            // 0x09(0) - 0x0a(1)
    FconstN(f32),                              // 0x0b(0) - 0x0d(1)
    DconstN(usize),                            // 0x0e(0) - 0x0f(1)
    Bipush(i32),                               // 0x10
    Sipush(usize),                             // 0x11
    Ldc(usize),                                // 0x12
    Ldc2W(usize, usize),                       // 0x14
    Iload(usize),                              // 0x15
    Aload(usize),                              // 0x19
    IloadN(usize),                             // 0x1a(0) - 0x1d(3)
    LloadN(usize),                             // 0x1e(0) - 0x21(3)
    FloadN(usize),                             // 0x22(0) - 0x25(3)
    DloadN(usize),                             // 0x26(0) - 0x29(3)
    AloadN(usize),                             // 0x2a(0) - 0x2d(3)
    Iaload,                                    // 0x2e
    Laload,                                    // 0x2f
    Aaload,                                    // 0x32
    Baload,                                    // 0x33
    Istore(i32),                               // 0x36
    Astore(usize),                             // 0x3a
    IstoreN(i32),                              // 0x3b(0) - 0x3e(3)
    LstoreN(usize),                            // 0x3f(0) - 0x42(3)
    FstoreN(usize),                            // 0x43(0) - 0x46(3)
    DstoreN(usize),                            // 0x47(0) - 0x4a(3)
    AstoreN(usize),                            // 0x4b(0) - 0x4e(3)
    Iastore,                                   // 0x4f
    Lastore,                                   // 0x50
    Aastore,                                   // 0x53
    Bastore,                                   // 0x54
    Pop,                                       // 0x57
    Dup,                                       // 0x59
    Iadd,                                      // 0x60
    Ladd,                                      // 0x61
    Fadd,                                      // 0x62
    Isub,                                      // 0x64
    Lsub,                                      // 0x65
    Fsub,                                      // 0x66
    Imul,                                      // 0x68
    Lmul,                                      // 0x69
    Fmul,                                      // 0x6a
    Idiv,                                      // 0x6c
    Ldiv,                                      // 0x6d
    Fdiv,                                      // 0x6e
    Irem,                                      // 0x70
    Lrem,                                      // 0x71
    Iinc(usize, usize),                        // 0x84
    Lcmp,                                      // 0x94
    Fcmpg,                                     // 0x95
    Fcmpl,                                     // 0x96
    Ifeq(usize, usize),                        // 0x99
    Ifne(usize, usize),                        // 0x9a
    Iflt(usize, usize),                        // 0x9b
    Ifge(usize, usize),                        // 0x9c
    Ifgt(usize, usize),                        // 0x9d
    Ifle(usize, usize),                        // 0x9e
    Ificmpeq(usize, usize),                    // 0x9f
    Ificmpne(usize, usize),                    // 0xa0
    Ificmplt(usize, usize),                    // 0xa1
    Ificmpge(usize, usize),                    // 0xa2
    Ificmpgt(usize, usize),                    // 0xa3
    Ificmple(usize, usize),                    // 0xa4
    Goto(usize),                               // 0xa7
    Lookupswitch(Vec<(Option<usize>, usize)>), // 0xab
    Ireturn,                                   // 0xac
    Areturn,                                   // 0xb0
    Return,                                    // 0xb1
    Getstatic(usize),                          // 0xb2
    Putstatic(usize),                          // 0xb3
    Getfield(usize),                           // 0xb4
    Putfield(usize),                           // 0xb5
    Invokevirtual(usize),                      // 0xb6
    Invokespecial(usize),                      // 0xb7
    Invokestatic(usize),                       // 0xb8
    New(usize),                                // 0xbb
    Newarray(usize),                           // 0xbc
    Anewarray(usize),                          // 0xbd
    Multianewarray(usize, usize),              // 0xc5
    Noope,                                     // custom command for Ificmple etc.
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Aconstnull => write!(f, "aconst_null"),
            Instruction::IconstN(val) => write!(f, "iconst_{}", val),
            Instruction::LconstN(val) => write!(f, "lconst_{}", val),
            Instruction::FconstN(val) => write!(f, "fconst_{}", val),
            Instruction::DconstN(val) => write!(f, "dconst_{}", val),
            Instruction::Bipush(val) => write!(f, "bipush         {}", val),
            Instruction::Sipush(val) => write!(f, "sipush         {}", val),
            Instruction::Ldc(val) => write!(f, "ldc             #{}", val),
            Instruction::Ldc2W(a, b) => write!(f, "ldc2_w         #{},{}", a, b),
            Instruction::Iload(val) => write!(f, "iload            #{}", val),
            Instruction::Aload(val) => write!(f, "aload            #{}", val),
            Instruction::IloadN(val) => write!(f, "iload_{}", val),
            Instruction::LloadN(val) => write!(f, "lload_{}", val),
            Instruction::FloadN(val) => write!(f, "fload_{}", val),
            Instruction::DloadN(val) => write!(f, "dload_{}", val),
            Instruction::AloadN(val) => write!(f, "aload_{}", val),
            Instruction::Iaload => write!(f, "iaload"),
            Instruction::Laload => write!(f, "laload"),
            Instruction::Aaload => write!(f, "aaload"),
            Instruction::Baload => write!(f, "baload"),
            Instruction::Istore(val) => write!(f, "istore            #{}", val),
            Instruction::Astore(val) => write!(f, "astore            #{}", val),
            Instruction::Aastore => write!(f, "aastore"),
            Instruction::Bastore => write!(f, "bastore"),
            Instruction::IstoreN(val) => write!(f, "istore_{}", val),
            Instruction::LstoreN(val) => write!(f, "lstore_{}", val),
            Instruction::FstoreN(val) => write!(f, "fstore_{}", val),
            Instruction::DstoreN(val) => write!(f, "dstore_{}", val),
            Instruction::Iastore => write!(f, "iastore"),
            Instruction::Lastore => write!(f, "lastore"),
            Instruction::AstoreN(val) => write!(f, "astore_{}", val),
            Instruction::Pop => write!(f, "pop"),
            Instruction::Dup => write!(f, "dup"),
            Instruction::Iadd => write!(f, "iadd"),
            Instruction::Ladd => write!(f, "ladd"),
            Instruction::Fadd => write!(f, "fadd"),
            Instruction::Isub => write!(f, "isub"),
            Instruction::Lsub => write!(f, "lsub"),
            Instruction::Fsub => write!(f, "fsub"),
            Instruction::Imul => write!(f, "imul"),
            Instruction::Lmul => write!(f, "lmul"),
            Instruction::Fmul => write!(f, "fmul"),
            Instruction::Idiv => write!(f, "idiv"),
            Instruction::Ldiv => write!(f, "ldiv"),
            Instruction::Fdiv => write!(f, "fdiv"),
            Instruction::Irem => write!(f, "irem"),
            Instruction::Lrem => write!(f, "lrem"),
            Instruction::Iinc(a, b) => write!(f, "iinc        {}, {}", a, b),
            Instruction::Lcmp => write!(f, "lcmp"),
            Instruction::Fcmpg => write!(f, "fcmpg"),
            Instruction::Fcmpl => write!(f, "fcmpl"),
            Instruction::Ifeq(a, b) => write!(f, "if_eq       {}, {}", a, b),
            Instruction::Ifne(a, b) => write!(f, "if_ne       {}, {}", a, b),
            Instruction::Iflt(a, b) => write!(f, "if_lt       {}, {}", a, b),
            Instruction::Ifge(a, b) => write!(f, "if_ge       {}, {}", a, b),
            Instruction::Ifgt(a, b) => write!(f, "if_gt       {}, {}", a, b),
            Instruction::Ifle(a, b) => write!(f, "if_le       {}, {}", a, b),
            Instruction::Ificmpeq(a, b) => write!(f, "if_icmpeq   {}, {}", a, b),
            Instruction::Ificmpne(a, b) => write!(f, "if_icmpne   {}, {}", a, b),
            Instruction::Ificmplt(a, b) => write!(f, "if_icmplt   {}, {}", a, b),
            Instruction::Ificmpge(a, b) => write!(f, "if_icmpge   {}, {}", a, b),
            Instruction::Ificmpgt(a, b) => write!(f, "if_icmpgt   {}, {}", a, b),
            Instruction::Ificmple(a, b) => write!(f, "if_icmple   {}, {}", a, b),
            Instruction::Goto(val) => write!(f, "goto          {}", val),
            Instruction::Ireturn => write!(f, "ireturn"),
            Instruction::Lookupswitch(vals) => {
                let vals_length = vals.len();
                let mut output_strings = Vec::with_capacity(vals_length);
                for (key, val) in &vals[1..vals_length] {
                    output_strings.push(format!("       {}: {}", key.unwrap(), val));
                }
                output_strings.push(format!("       default: {}", vals.last().unwrap().1));
                write!(
                    f,
                    "lookupswitch {{ // {}
{}
}}",
                    vals_length - 1,
                    output_strings.join("\n")
                )
            }
            Instruction::Areturn => write!(f, "areturn"),
            Instruction::Return => write!(f, "return"),
            Instruction::Getstatic(val) => write!(f, "getstatic       #{}", val),
            Instruction::Putstatic(val) => write!(f, "putstatic       #{}", val),
            Instruction::Getfield(val) => write!(f, "getfield        #{}", val),
            Instruction::Putfield(val) => write!(f, "putfield        #{}", val),
            Instruction::Invokevirtual(val) => write!(f, "invokevirtual   #{}", val),
            Instruction::Invokespecial(val) => write!(f, "invokespecial   #{}", val),
            Instruction::Invokestatic(val) => write!(f, "invokestatic   #{}", val),
            Instruction::New(val) => write!(f, "new            #{}", val),
            Instruction::Newarray(val) => write!(f, "newarray       #{}", val),
            Instruction::Anewarray(val) => write!(f, "anewarray      #{}", val),
            Instruction::Multianewarray(index, dimensions) => {
                write!(f, "multianewarray    #{} {}", index, dimensions)
            }
            Instruction::Noope => write!(f, "noope"),
        }
    }
}

impl Instruction {
    pub fn create_and_push(
        codes: &mut Vec<Instruction>,
        inputs: &[u8],
        index: usize,
        tag: usize,
    ) -> (usize, usize) {
        macro_rules! simple_instruct {
            ($expr:expr) => {
                codes.push($expr);
                return (index, 1);
            };
        }

        match tag {
            // aconst_null
            0x01 => {
                simple_instruct!(Instruction::Aconstnull);
            }
            // iconst_n
            val @ 0x02..=0x08 => {
                simple_instruct!(Instruction::IconstN(val as i32 - 0x03));
            }
            // lconst_n
            val @ 0x09..=0x0a => {
                simple_instruct!(Instruction::LconstN(val - 0x09));
            }
            // fconst_n
            val @ 0x0b..=0x0d => {
                simple_instruct!(Instruction::FconstN((val - 0x0b) as f32));
            }
            // dconst_n
            val @ 0x0e..=0x0f => {
                simple_instruct!(Instruction::DconstN(val - 0x0e));
            }
            // bipush
            0x10 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 1);
                let val = if val > 0x79 {
                    -1 * ((val ^ 0xff) + 1) as i32
                } else {
                    val as i32
                };
                codes.push(Instruction::Bipush(val));
                codes.push(Instruction::Noope);
                (index, 2)
            }
            // sipush
            0x11 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                codes.push(Instruction::Sipush(val));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // ldc
            0x12 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 1);
                codes.push(Instruction::Ldc(val));
                codes.push(Instruction::Noope);
                (index, 2)
            }
            // ldc2_w
            0x14 => {
                let (val, index) = extract_x_byte_as_vec(inputs, index, 2);
                codes.push(Instruction::Ldc2W(val[0] as usize, val[1] as usize));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // iload
            0x15 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 1);
                codes.push(Instruction::Iload(val));
                codes.push(Instruction::Noope);
                (index, 2)
            }
            // aload
            0x19 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 1);
                codes.push(Instruction::Aload(val));
                codes.push(Instruction::Noope);
                (index, 2)
            }
            // iload_n
            val @ 0x1a..=0x1d => {
                simple_instruct!(Instruction::IloadN(val - 0x1a));
            }
            // lload_n
            val @ 0x1e..=0x21 => {
                simple_instruct!(Instruction::LloadN(val - 0x1e));
            }
            // fload_n
            val @ 0x22..=0x25 => {
                simple_instruct!(Instruction::FloadN(val - 0x22));
            }
            // dload_n
            val @ 0x26..=0x29 => {
                simple_instruct!(Instruction::DloadN(val - 0x26));
            }
            // aload_n
            val @ 0x2a..=0x2d => {
                simple_instruct!(Instruction::AloadN(val - 0x2a));
            }
            // iaload
            0x2e => {
                simple_instruct!(Instruction::Iaload);
            }
            // laload
            0x2f => {
                simple_instruct!(Instruction::Laload);
            }
            // iaload
            0x32 => {
                simple_instruct!(Instruction::Aaload);
            }
            // baload
            0x33 => {
                simple_instruct!(Instruction::Baload);
            }
            // istore
            0x36 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 1);
                codes.push(Instruction::Istore(val as i32));
                codes.push(Instruction::Noope);
                (index, 2)
            }
            // astore
            0x3a => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 1);
                codes.push(Instruction::Astore(val));
                codes.push(Instruction::Noope);
                (index, 2)
            }
            // istore_n
            val @ 0x3b..=0x3e => {
                simple_instruct!(Instruction::IstoreN(val as i32 - 0x3b));
            }
            // lstore_n
            val @ 0x3f..=0x42 => {
                simple_instruct!(Instruction::LstoreN(val - 0x3f));
            }
            // fstore_n
            val @ 0x43..=0x46 => {
                simple_instruct!(Instruction::FstoreN(val - 0x43));
            }
            // dstore_n
            val @ 0x47..=0x4a => {
                simple_instruct!(Instruction::DstoreN(val - 0x47));
            }
            // astore_n
            val @ 0x4b..=0x4e => {
                simple_instruct!(Instruction::AstoreN(val - 0x4b));
            }
            // iastore
            0x4f => {
                simple_instruct!(Instruction::Iastore);
            }
            // lastore
            0x50 => {
                simple_instruct!(Instruction::Lastore);
            }
            // aastore
            0x53 => {
                simple_instruct!(Instruction::Aastore);
            }
            // bastore
            0x54 => {
                simple_instruct!(Instruction::Bastore);
            }
            // pop
            0x57 => {
                simple_instruct!(Instruction::Pop);
            }
            // dup
            0x59 => {
                simple_instruct!(Instruction::Dup);
            }
            // iadd
            0x60 => {
                simple_instruct!(Instruction::Iadd);
            }
            // ladd
            0x61 => {
                simple_instruct!(Instruction::Ladd);
            }
            // fadd
            0x62 => {
                simple_instruct!(Instruction::Fadd);
            }
            // isub
            0x64 => {
                simple_instruct!(Instruction::Isub);
            }
            // lsub
            0x65 => {
                simple_instruct!(Instruction::Lsub);
            }
            // fsub
            0x66 => {
                simple_instruct!(Instruction::Fsub);
            }
            // imul
            0x68 => {
                simple_instruct!(Instruction::Imul);
            }
            // lmul
            0x69 => {
                simple_instruct!(Instruction::Lmul);
            }
            // fmul
            0x6a => {
                simple_instruct!(Instruction::Fmul);
            }
            // idiv
            0x6c => {
                simple_instruct!(Instruction::Idiv);
            }
            // ldiv
            0x6d => {
                simple_instruct!(Instruction::Ldiv);
            }
            // fdiv
            0x6e => {
                simple_instruct!(Instruction::Fdiv);
            }
            // irem
            0x70 => {
                simple_instruct!(Instruction::Irem);
            }
            // lrem
            0x71 => {
                simple_instruct!(Instruction::Lrem);
            }
            // iinc
            0x84 => {
                let (val, index) = extract_x_byte_as_vec(inputs, index, 2);
                codes.push(Instruction::Iinc(val[0] as usize, val[1] as usize));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // lcmp
            0x94 => {
                simple_instruct!(Instruction::Lcmp);
            }
            // fcmpg
            0x95 => {
                simple_instruct!(Instruction::Fcmpg);
            }
            // fcmpl
            0x96 => {
                simple_instruct!(Instruction::Fcmpl);
            }
            // ifeq
            0x99 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let code_length = codes.len();
                codes.push(Instruction::Ifeq(
                    (val + code_length - 1) & 0xffff,
                    code_length + 2,
                ));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // ifne
            0x9a => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let code_length = codes.len();
                codes.push(Instruction::Ifne(
                    (val + code_length - 1) & 0xffff,
                    code_length + 2,
                ));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // iflt
            0x9b => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let code_length = codes.len();
                codes.push(Instruction::Iflt(
                    (val + code_length - 1) & 0xffff,
                    code_length + 2,
                ));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // ifge
            0x9c => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let code_length = codes.len();
                codes.push(Instruction::Ifge(
                    (val + code_length - 1) & 0xffff,
                    code_length + 2,
                ));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // ifgt
            0x9d => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let code_length = codes.len();
                codes.push(Instruction::Ifgt(
                    (val + code_length - 1) & 0xffff,
                    code_length + 2,
                ));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // ifle
            0x9e => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let code_length = codes.len();
                codes.push(Instruction::Ifle(
                    (val + code_length - 1) & 0xffff,
                    code_length + 2,
                ));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // if_icmpeq
            0x9f => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let code_length = codes.len();
                codes.push(Instruction::Ificmpeq(
                    (val + code_length - 1) & 0xffff,
                    code_length + 2,
                ));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // if_icmpne
            0xa0 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let code_length = codes.len();
                codes.push(Instruction::Ificmpne(
                    (val + code_length - 1) & 0xffff,
                    code_length + 2,
                ));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // if_icmplt
            0xa1 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let code_length = codes.len();
                codes.push(Instruction::Ificmplt(
                    (val + code_length - 1) & 0xffff,
                    code_length + 2,
                ));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // if_icmpge
            0xa2 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let code_length = codes.len();
                codes.push(Instruction::Ificmpge(
                    (val + code_length - 1) & 0xffff,
                    code_length + 2,
                ));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // if_icmpgt
            0xa3 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let code_length = codes.len();
                codes.push(Instruction::Ificmpgt(
                    (val + code_length - 1) & 0xffff,
                    code_length + 2,
                ));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // if_icmple
            0xa4 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let code_length = codes.len();
                codes.push(Instruction::Ificmple(
                    (val + code_length - 1) & 0xffff,
                    code_length + 2,
                ));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // goto
            0xa7 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let code_length = codes.len();
                codes.push(Instruction::Goto((val + code_length - 1) & 0xFFFF));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // lookupswitch
            0xab => {
                let (offset, index) = extract_x_byte_as_usize(inputs, index, 4);
                // default_value can be used for branch_length
                let (default_value, mut index) = extract_x_byte_as_usize(inputs, index, 4);
                // default + branch_length
                let mut switch_values = Vec::with_capacity(1 + default_value);
                switch_values.push((None, offset + default_value));

                for _ in 0..default_value {
                    let (key, update_index) = extract_x_byte_as_usize(inputs, index, 4);
                    let (val, update_index) = extract_x_byte_as_usize(inputs, update_index, 4);
                    switch_values.push((Some(key), val + default_value));
                    index = update_index
                }
                codes.push(Instruction::Lookupswitch(switch_values));

                let set_length = default_value + 1;
                let switch_instructions_len = set_length * 4 * 2;
                for _ in 0..switch_instructions_len {
                    codes.push(Instruction::Noope)
                }
                (index, switch_instructions_len + 1)
            }
            // ireturn
            0xac => {
                simple_instruct!(Instruction::Ireturn);
            }
            // areturn
            0xb0 => {
                simple_instruct!(Instruction::Areturn);
            }
            // return
            0xb1 => {
                simple_instruct!(Instruction::Return);
            }
            // getstatic
            0xb2 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                codes.push(Instruction::Getstatic(val));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // getstatic
            0xb3 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                codes.push(Instruction::Putstatic(val));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // getfield
            0xb4 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                codes.push(Instruction::Getfield(val));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // putfield
            0xb5 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                codes.push(Instruction::Putfield(val));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // invokevirtual
            0xb6 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                codes.push(Instruction::Invokevirtual(val));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // invokespecial
            0xb7 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                codes.push(Instruction::Invokespecial(val));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // invokestatic
            0xb8 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                codes.push(Instruction::Invokestatic(val));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // new
            0xbb => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                codes.push(Instruction::New(val));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // newarray
            0xbc => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 1);
                codes.push(Instruction::Newarray(val));
                codes.push(Instruction::Noope);
                (index, 2)
            }
            // anewarray
            0xbd => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                codes.push(Instruction::Anewarray(val));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // multianewarray
            0xc5 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let (dimentions, index) = extract_x_byte_as_usize(inputs, index, 1);
                codes.push(Instruction::Multianewarray(val, dimentions));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 4)
            }
            _ => unimplemented!("tag: {:x}", tag),
        }
    }

    pub fn counsume_index(&self) -> usize {
        match self {
            Instruction::Lookupswitch(vals) => vals.len() * 4,
            Instruction::Multianewarray(_, _) => 3,
            Instruction::Ificmple(_, _)
            | Instruction::Getstatic(_)
            | Instruction::Putstatic(_)
            | Instruction::Getfield(_)
            | Instruction::Putfield(_)
            | Instruction::Iinc(_, _)
            | Instruction::Sipush(_)
            | Instruction::Ldc2W(_, _)
            | Instruction::Invokevirtual(_)
            | Instruction::Invokespecial(_)
            | Instruction::Invokestatic(_)
            | Instruction::New(_)
            | Instruction::Anewarray(_) => 2,
            Instruction::Iload(_)
            | Instruction::Aload(_)
            | Instruction::Istore(_)
            | Instruction::Astore(_)
            | Instruction::Bipush(_)
            | Instruction::Newarray(_)
            | Instruction::Ldc(_) => 1,
            Instruction::Aconstnull
            | Instruction::IconstN(_)
            | Instruction::LconstN(_)
            | Instruction::FconstN(_)
            | Instruction::DconstN(_)
            | Instruction::IstoreN(_)
            | Instruction::IloadN(_)
            | Instruction::LstoreN(_)
            | Instruction::FstoreN(_)
            | Instruction::DstoreN(_)
            | Instruction::LloadN(_)
            | Instruction::FloadN(_)
            | Instruction::DloadN(_)
            | Instruction::AstoreN(_)
            | Instruction::AloadN(_)
            | Instruction::Pop
            | Instruction::Dup
            | Instruction::Iadd
            | Instruction::Ladd
            | Instruction::Fadd
            | Instruction::Isub
            | Instruction::Lsub
            | Instruction::Fsub
            | Instruction::Imul
            | Instruction::Lmul
            | Instruction::Fmul
            | Instruction::Idiv
            | Instruction::Ldiv
            | Instruction::Fdiv
            | Instruction::Irem
            | Instruction::Lrem
            | Instruction::Lcmp
            | Instruction::Fcmpg
            | Instruction::Fcmpl
            | Instruction::Ireturn
            | Instruction::Areturn
            | Instruction::Iaload
            | Instruction::Laload
            | Instruction::Aaload
            | Instruction::Baload
            | Instruction::Iastore
            | Instruction::Lastore
            | Instruction::Aastore
            | Instruction::Bastore
            | Instruction::Return => 0,
            instruction => unimplemented!("{}", instruction),
        }
    }
}
