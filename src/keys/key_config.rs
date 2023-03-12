use std::rc::Rc;

use super::{
    key_list::{GituiKeyEvent, KeysList},
    symbols::KeySymbols,
};

pub type SharedKeyConfig = Rc<KeyConfig>;

pub struct KeyConfig {
    pub keys: KeysList,
    symbols: KeySymbols,
}