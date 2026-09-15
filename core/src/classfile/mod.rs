// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

pub mod constant_pool;

use byteorder::{BigEndian, ReadBytesExt};
use constant_pool::ConstantPool;
use std::io::{Cursor, Read};

pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug, Clone)]
pub struct ParsedCodeAttribute {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
}

impl ParsedCodeAttribute {
    pub fn parse(info: &[u8]) -> Result<Self, String> {
        let mut cursor = Cursor::new(info);
        let max_stack = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
        let max_locals = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
        let code_length = cursor.read_u32::<BigEndian>().map_err(|e| e.to_string())? as usize;
        let mut code = vec![0; code_length];
        cursor.read_exact(&mut code).map_err(|e| e.to_string())?;
        Ok(Self {
            max_stack,
            max_locals,
            code,
        })
    }
}

impl MethodInfo {
    pub fn get_code(&self, cp: &ConstantPool) -> Option<ParsedCodeAttribute> {
        for attr in &self.attributes {
            if let Ok(name) = cp.get_utf8(attr.name_index) {
                if name == "Code" {
                    if let Ok(parsed) = ParsedCodeAttribute::parse(&attr.info) {
                        return Some(parsed);
                    }
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct AttributeInfo {
    pub name_index: u16,
    pub info: Vec<u8>,
}

impl ClassFile {
    pub fn parse(bytes: &[u8]) -> Result<Self, String> {
        let mut cursor = Cursor::new(bytes);
        
        let magic = cursor.read_u32::<BigEndian>().map_err(|e| e.to_string())?;
        if magic != 0xCAFEBABE {
            return Err("Invalid class file magic number (expected 0xCAFEBABE)".to_string());
        }

        let minor_version = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
        let major_version = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;

        let constant_pool_count = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
        let constant_pool = ConstantPool::parse(&mut cursor, constant_pool_count)
            .map_err(|e| format!("Failed to parse constant pool: {}", e))?;

        let access_flags = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
        let this_class = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
        let super_class = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;

        let interfaces_count = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
        let mut interfaces = Vec::with_capacity(interfaces_count as usize);
        for _ in 0..interfaces_count {
            interfaces.push(cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?);
        }

        let fields_count = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
        let mut fields = Vec::with_capacity(fields_count as usize);
        for _ in 0..fields_count {
            fields.push(Self::read_field_info(&mut cursor)?);
        }

        let methods_count = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
        let mut methods = Vec::with_capacity(methods_count as usize);
        for _ in 0..methods_count {
            methods.push(Self::read_method_info(&mut cursor)?);
        }

        let attributes_count = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            attributes.push(Self::read_attribute_info(&mut cursor)?);
        }

        Ok(Self {
            magic,
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        })
    }

    fn read_field_info(cursor: &mut Cursor<&[u8]>) -> Result<FieldInfo, String> {
        let access_flags = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
        let name_index = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
        let descriptor_index = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
        
        let attributes_count = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            attributes.push(Self::read_attribute_info(cursor)?);
        }

        Ok(FieldInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        })
    }

    fn read_method_info(cursor: &mut Cursor<&[u8]>) -> Result<MethodInfo, String> {
        let access_flags = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
        let name_index = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
        let descriptor_index = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
        
        let attributes_count = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            attributes.push(Self::read_attribute_info(cursor)?);
        }

        Ok(MethodInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        })
    }

    fn read_attribute_info(cursor: &mut Cursor<&[u8]>) -> Result<AttributeInfo, String> {
        let name_index = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
        let length = cursor.read_u32::<BigEndian>().map_err(|e| e.to_string())?;
        
        let mut info = vec![0; length as usize];
        cursor.read_exact(&mut info).map_err(|e| e.to_string())?;

        Ok(AttributeInfo { name_index, info })
    }
}
