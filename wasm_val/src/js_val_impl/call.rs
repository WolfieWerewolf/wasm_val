// Copyright 2018 Vladimir Iftodi <Vladimir.Iftodi@gmail.com>. 
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io::Cursor;
use {JsValue, Val, Type, JsSerializable, wasm_ffi};

impl JsValue {
    pub fn call(&self) -> Option<JsValue> {
        match self {
            JsValue{js_type: Type::Function, val: Val::Ref(ref_id)} => {
                let ret_vec = wasm_ffi::call_0(*ref_id);

                Some(JsValue::get_single_val(ret_vec))
            },
            _ => None
        }
    }

    pub fn call_with_arg<S>(&self, arg: S) -> Option<JsValue> where S: JsSerializable {
        match self {
            JsValue{js_type: Type::Function, val: Val::Ref(ref_id)} => {
                let vec = Vec::with_capacity(9);
                let mut cursor = Cursor::new(vec);

                arg.ser(&mut cursor);

                let ret_vec = wasm_ffi::call_1(*ref_id, cursor.into_inner());

                Some(JsValue::get_single_val(ret_vec))
            },
            _ => None
        }
    }

    pub fn call_with_args(&self, args: &[&JsSerializable]) -> Option<JsValue> {
        match self {
            JsValue{js_type: Type::Function, val: Val::Ref(ref_id)} => {
                let alloc_size = args.iter().fold(0_u32, |acc, val| acc + val.size());
                let vec: Vec<u8> = Vec::with_capacity(alloc_size as usize);
                let mut cursor = Cursor::new(vec);

                for arg in args {
                    arg.ser(&mut cursor);
                }

                let ret_vec = wasm_ffi::call_args(*ref_id, args.len() as u32, cursor.into_inner());

                Some(JsValue::get_single_val(ret_vec))
            },
            _ => None
        }
    }

    pub fn call_method(&self, name: &str) -> Option<JsValue> {
        match self.val {
            Val::Ref(ref_id)  => {
                let ret_vec = wasm_ffi::call_method_0(ref_id, name);

                Some(JsValue::get_single_val(ret_vec))
            },
            _ => None
        }
    }

    pub fn call_method_with_arg<S>(&self, name: &str, arg: S) -> Option<JsValue> where S: JsSerializable {
        match self.val {
            Val::Ref(ref_id)  => {
                let vec = Vec::with_capacity(9);
                let mut cursor = Cursor::new(vec);

                arg.ser(&mut cursor);

                let ret_vec = wasm_ffi::call_method_1(ref_id, name, cursor.into_inner());

                Some(JsValue::get_single_val(ret_vec))
            },
            _ => None
        }
    }

    pub fn call_method_with_args(&self, name: &str, args: &[&JsSerializable]) -> Option<JsValue> {
        match self.val {
            Val::Ref(ref_id)  => {
                let alloc_size = args.iter().fold(0_u32, |acc, val| acc + val.size());
                let vec: Vec<u8> = Vec::with_capacity(alloc_size as usize);
                let mut cursor = Cursor::new(vec);

                for arg in args {
                    arg.ser(&mut cursor);
                }

                let ret_vec = wasm_ffi::call_method_args(ref_id, name, args.len() as u32, cursor.into_inner());

                Some(JsValue::get_single_val(ret_vec))
            },
            _ => None
        }
    }
}
