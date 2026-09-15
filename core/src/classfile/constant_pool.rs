// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

use byteorder::{BigEndian, ReadBytesExt};
use std::io::{self, Cursor, Read};

#[derive(Debug, Clone)]
pub enum Constant {
    Utf8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class { name_index: u16 },
    String { string_index: u16 },
    Fieldref { class_index: u16, name_and_type_index: u16 },
    Methodref { class_index: u16, name_and_type_index: u16 },
    InterfaceMethodref { class_index: u16, name_and_type_index: u16 },
    NameAndType { name_index: u16, descriptor_index: u16 },
    MethodHandle { reference_kind: u8, reference_index: u16 },
    MethodType { descriptor_index: u16 },
    InvokeDynamic { bootstrap_method_attr_index: u16, name_and_type_index: u16 },
    Unused,
}

pub struct ConstantPool {
    pub constants: Vec<Constant>,
}

impl ConstantPool {
    pub fn parse(cursor: &mut Cursor<&[u8]>, count: u16) -> io::Result<Self> {
        let mut constants = Vec::with_capacity(count as usize);
        // Constant pool is 1-indexed; slot 0 is empty
        constants.push(Constant::Unused);

        let mut i = 1;
        while i < count {
            let tag = cursor.read_u8()?;
            let constant = match tag {
                1 => { // CONSTANT_Utf8
                    let length = cursor.read_u16::<BigEndian>()?;
                    let mut bytes = vec![0; length as usize];
                    cursor.read_exact(&mut bytes)?;
                    let s = String::from_utf8(bytes)
                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                    Constant::Utf8(s)
                }
                3 => Constant::Integer(cursor.read_i32::<BigEndian>()?),
                4 => Constant::Float(cursor.read_f32::<BigEndian>()?),
                5 => { // CONSTANT_Long (takes 2 slots)
                    let val = cursor.read_i64::<BigEndian>()?;
                    constants.push(Constant::Long(val));
                    constants.push(Constant::Unused); // Slot index placeholder
                    i += 2;
                    continue;
                }
                6 => { // CONSTANT_Double (takes 2 slots)
                    let val = cursor.read_f64::<BigEndian>()?;
                    constants.push(Constant::Double(val));
                    constants.push(Constant::Unused); // Slot index placeholder
                    i += 2;
                    continue;
                }
                7 => Constant::Class { name_index: cursor.read_u16::<BigEndian>()? },
                8 => Constant::String { string_index: cursor.read_u16::<BigEndian>()? },
                9 => Constant::Fieldref {
                    class_index: cursor.read_u16::<BigEndian>()?,
                    name_and_type_index: cursor.read_u16::<BigEndian>()?,
                },
                10 => Constant::Methodref {
                    class_index: cursor.read_u16::<BigEndian>()?,
                    name_and_type_index: cursor.read_u16::<BigEndian>()?,
                },
                11 => Constant::InterfaceMethodref {
                    class_index: cursor.read_u16::<BigEndian>()?,
                    name_and_type_index: cursor.read_u16::<BigEndian>()?,
                },
                12 => Constant::NameAndType {
                    name_index: cursor.read_u16::<BigEndian>()?,
                    descriptor_index: cursor.read_u16::<BigEndian>()?,
                },
                15 => Constant::MethodHandle {
                    reference_kind: cursor.read_u8()?,
                    reference_index: cursor.read_u16::<BigEndian>()?,
                },
                16 => Constant::MethodType { descriptor_index: cursor.read_u16::<BigEndian>()? },
                18 => Constant::InvokeDynamic {
                    bootstrap_method_attr_index: cursor.read_u16::<BigEndian>()?,
                    name_and_type_index: cursor.read_u16::<BigEndian>()?,
                },
                _ => return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Unknown constant pool tag: {}", tag),
                )),
            };

            constants.push(constant);
            i += 1;
        }

        Ok(Self { constants })
    }

    pub fn get_utf8(&self, index: u16) -> Result<&str, String> {
        match self.constants.get(index as usize) {
            Some(Constant::Utf8(ref s)) => Ok(s.as_str()),
            _ => Err(format!("Invalid constant pool UTF8 index: {}", index)),
        }
    }

    pub fn get_class_name(&self, index: u16) -> Result<&str, String> {
        match self.constants.get(index as usize) {
            Some(Constant::Class { name_index }) => self.get_utf8(*name_index),
            _ => Err(format!("Invalid constant pool Class index: {}", index)),
        }
    }

    pub fn get_name_and_type(&self, index: u16) -> Result<(&str, &str), String> {
        match self.constants.get(index as usize) {
            Some(Constant::NameAndType { name_index, descriptor_index }) => {
                let name = self.get_utf8(*name_index)?;
                let descriptor = self.get_utf8(*descriptor_index)?;
                Ok((name, descriptor))
            }
            _ => Err(format!("Invalid constant pool NameAndType index: {}", index)),
        }
    }

    pub fn get_method_ref(&self, index: u16) -> Result<(&str, &str, &str), String> {
        match self.constants.get(index as usize) {
            Some(Constant::Methodref { class_index, name_and_type_index }) => {
                let class_name = self.get_class_name(*class_index)?;
                let (name, descriptor) = self.get_name_and_type(*name_and_type_index)?;
                Ok((class_name, name, descriptor))
            }
            _ => Err(format!("Invalid constant pool Methodref index: {}", index)),
        }
    }

    pub fn get_field_ref(&self, index: u16) -> Result<(&str, &str, &str), String> {
        match self.constants.get(index as usize) {
            Some(Constant::Fieldref { class_index, name_and_type_index }) => {
                let class_name = self.get_class_name(*class_index)?;
                let (name, descriptor) = self.get_name_and_type(*name_and_type_index)?;
                Ok((class_name, name, descriptor))
            }
            _ => Err(format!("Invalid constant pool Fieldref index: {}", index)),
        }
    }
}
