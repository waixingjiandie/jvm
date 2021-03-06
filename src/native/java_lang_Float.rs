#![allow(non_snake_case)]

use crate::native::{new_fn, JNIEnv, JNINativeMethod, JNIResult};
use crate::oop::OopDesc;
use crate::runtime::JavaThread;
use crate::types::OopRef;
use crate::util;

pub fn get_native_methods() -> Vec<JNINativeMethod> {
    vec![new_fn(
        "floatToRawIntBits",
        "(F)I",
        Box::new(jvm_floatToRawIntBits),
    )]
}

fn jvm_floatToRawIntBits(_jt: &mut JavaThread, _env: JNIEnv, args: Vec<OopRef>) -> JNIResult {
    let arg0 = args.get(0).unwrap();
    let v = util::oop::extract_float(arg0.clone());
    let v = v.to_bits().to_be_bytes();
    let v = i32::from_be_bytes([v[0], v[1], v[2], v[3]]);
    Ok(Some(OopDesc::new_int(v)))
}
