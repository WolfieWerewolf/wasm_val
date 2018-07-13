// Copyright 2018 Vladimir Iftodi <Vladimir.Iftodi@gmail.com>. 
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io::Cursor;
use ser_constants::TypeTag;

use byteorder::{LittleEndian, WriteBytesExt,};

pub trait JsSerializable {
    fn size(&self) -> u8;
    fn ser(&self, cursor: &mut Cursor<Vec<u8>>) -> ();
}

impl JsSerializable for bool {
    fn size(&self) -> u8 { 1 }

    fn ser(&self, cursor: &mut Cursor<Vec<u8>>) {
        match self {
            false => cursor.write_u8(TypeTag::BoolFalse as u8).unwrap(),
            true => cursor.write_u8(TypeTag::BoolTrue as u8).unwrap(),
        }
    }
}

impl JsSerializable for i8 {
    fn size(&self) -> u8 { 2 }

    fn ser(&self, cursor: &mut Cursor<Vec<u8>>) {
        cursor.write_u8(TypeTag::Int8 as u8).unwrap();
        cursor.write_i8(*self).unwrap();
    }
}

impl JsSerializable for u8 {
    fn size(&self) -> u8 { 2 }

    fn ser(&self, cursor: &mut Cursor<Vec<u8>>) {
        cursor.write_u8(TypeTag::UInt8 as u8).unwrap();
        cursor.write_u8(*self).unwrap();
    }
}

impl JsSerializable for i16 {
    fn size(&self) -> u8 { 3 }

    fn ser(&self, cursor: &mut Cursor<Vec<u8>>) {
        cursor.write_u8(TypeTag::Int16 as u8).unwrap();
        cursor.write_i16::<LittleEndian>(*self).unwrap();
    }
}

impl JsSerializable for u16 {
    fn size(&self) -> u8 { 3 }

    fn ser(&self, cursor: &mut Cursor<Vec<u8>>) {
        cursor.write_u8(TypeTag::UInt16 as u8).unwrap();
        cursor.write_u16::<LittleEndian>(*self).unwrap();
    }
}

impl JsSerializable for i32 {
    fn size(&self) -> u8 { 5 }

    fn ser(&self, cursor: &mut Cursor<Vec<u8>>) {
        cursor.write_u8(TypeTag::Int32 as u8).unwrap();
        cursor.write_i32::<LittleEndian>(*self).unwrap();
    }
}

impl JsSerializable for u32 {
    fn size(&self) -> u8 { 5 }

    fn ser(&self, cursor: &mut Cursor<Vec<u8>>) {
        cursor.write_u8(TypeTag::UInt32 as u8).unwrap();
        cursor.write_u32::<LittleEndian>(*self).unwrap();
    }
}


impl<'a> JsSerializable for &'a str {
    fn size(&self) -> u8 { 9 }

    fn ser(&self, cursor: &mut Cursor<Vec<u8>>) {
        cursor.write_u8(TypeTag::String as u8).unwrap();
        cursor.write_u32::<LittleEndian>(self.len() as u32).unwrap();
        cursor.write_u32::<LittleEndian>(self.as_ptr() as u32).unwrap();
    }
}