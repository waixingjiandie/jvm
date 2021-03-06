use crate::classfile::constant_pool::ConstantType;
use crate::classfile::ClassFile;
use crate::oop::class::Class;
use crate::oop::field::FieldId;
use crate::oop::method::MethodId;
use crate::oop::OopDesc;
use std::sync::Arc;

pub type U1 = u8;
pub type U2 = u16;
pub type U4 = u32;

pub type FieldIdRef = Arc<FieldId>;
pub type MethodIdRef = Arc<MethodId>;

//引用".class"中的字符串常量值
def_ref!(BytesRef, Vec<u8>);
def_ref!(ConstantPool, Vec<ConstantType>);

def_ref!(ClassFileRef, ClassFile);
def_sync_ref!(ClassRef, Class);
def_sync_ref!(OopRef, OopDesc);
//runtime 字符串分配
def_ptr!(ByteAry, Vec<u8>);
def_ptr!(BoolAry, Vec<u8>);
def_ptr!(CharAry, Vec<u16>);
def_ptr!(ShortAry, Vec<i16>);
def_ptr!(IntAry, Vec<i32>);
def_ptr!(LongAry, Vec<i64>);
def_ptr!(FloatAry, Vec<f32>);
def_ptr!(DoubleAry, Vec<f64>);
