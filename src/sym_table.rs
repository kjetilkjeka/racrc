//! The RACRC symbol table.
//! 
//! Exists for multiple reasons:
//!  - Detecting if multiple symbols are defined at the full qualified path
//!  - Store information about definitions and modules that can be accessed from the path

use std::rc::Rc;
use std::collections::HashMap;

use crate::error::Error;
use crate::error::ErrorKind;

#[derive(Debug)]
pub(crate) struct SymbolTable {
    table: HashMap<racr::Path, Symbol>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Symbol {
    Device(Rc<racr::DeviceDefinition>),
    Peripheral(Rc<racr::PeripheralDefinition>),
    Register(Rc<racr::RegisterDefinition>),
}

impl SymbolTable {
    pub(crate) fn new() -> Self {
        SymbolTable{
            table: HashMap::new(),
        }
    }

    /// Adds a symbol to the symbol table
    /// 
    /// Path should be the qualified path without the item name.
    /// The item name will be added from the symbol.
    /// 
    /// Returns an error if symbol already exists.
    pub(crate) fn add_symbol(&mut self, mut path: racr::Path, sym: Symbol) -> Result<(), Error> {
        if !self.table.contains_key(&path) {
            path.segments.push(sym.ident().clone());
            assert!(self.table.insert(path, sym).is_none());
            Ok(())
        } else {
            Err(Error::new(ErrorKind::RedefinedSymbol))
        }
    }

    pub(crate) fn contains_symbol(&self, fully_qualified_name: &racr::Path) -> bool {
        self.table.contains_key(fully_qualified_name)
    }

    /// Returns the symbol corresponding to `path` if any exists.
    pub(crate) fn get(&self, fully_qualified_name: &racr::Path) -> Option<Symbol> {
        self.table.get(fully_qualified_name).map(|x| x.clone())
    }

}

impl Symbol {
    fn ident(&self) -> &racr::Ident {
        match self {
            Symbol::Device(device) => &device.ident,
            Symbol::Peripheral(per) => &per.ident,
            Symbol::Register(reg) => &reg.ident,
        }
    }
}

impl From<racr::DeviceDefinition> for Symbol {
    fn from(s: racr::DeviceDefinition) -> Symbol {
        Symbol::Device(Rc::new(s))
    }
}

impl From<racr::PeripheralDefinition> for Symbol {
    fn from(s: racr::PeripheralDefinition) -> Symbol {
        Symbol::Peripheral(Rc::new(s))
    }
}

impl From<racr::RegisterDefinition> for Symbol {
    fn from(s: racr::RegisterDefinition) -> Symbol {
        Symbol::Register(Rc::new(s))
    }
}
