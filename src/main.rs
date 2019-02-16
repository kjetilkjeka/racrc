use clap::{
    App,
    load_yaml,
};

use std::path::Path;
use std::io::{Read, Write};
use std::fs::File;

mod error;
mod sym_table;

use error::{
    Error,
    ErrorKind,
};

use sym_table::{
    Symbol,
    SymbolTable,
};

fn main() {
    let cli_yaml = load_yaml!("cli.yml");
    let cli_matches = App::from_yaml(cli_yaml).get_matches();

    let input_filepath = Path::new(cli_matches.value_of("input").unwrap());
    let output_filepath = Path::new(cli_matches.value_of("output").unwrap());

    let mut input_file = File::open(input_filepath).unwrap();
    let mut output_file = File::create(output_filepath).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut racr_content_text = String::new();

    input_file.read_to_string(&mut racr_content_text).unwrap();
    let racr_content_tokens = racr_parser::ContentParser::new().parse(&racr_content_text).unwrap();

    let mut racr_file = racr::FileContent {
        content: racr_content_tokens,
    };

    let crate_path = racr::Path{segments: vec![racr::Ident::from("crate")]};

    copy_content_into_symtable(&mut symbol_table, &crate_path, &racr_file.content).unwrap();

    //let mut output_text = RustEwgTarget::default().generate_file(&racr_file);

    //output_file.write_all(output_text.as_bytes()).unwrap();

    println!("Symbol table: {:#?}", symbol_table);
}

/// A procedure for copying content into symtable if the item type allows it.
/// 
/// **Note:**
///  - Fully qualified names will be resolved before adding items to sym_table
///  - Modules will be traversed recursively, depth first.
fn copy_content_into_symtable(sym_table: &mut SymbolTable, path: &racr::Path, content: &[racr::Item]) -> Result<(), Vec<Error>> {
    let mut errors = Vec::new();

    for item in content {
        match item {
            racr::Item::Use(_) => (),
            racr::Item::Device(device) => {
                let mut device = device.clone();
                device.peripherals.iter_mut().for_each(|peripheral| resolve_fully_qualified_name(path, &mut peripheral.path));
                if let Err(e) = sym_table.add_symbol(path.clone(), device.into()) {
                    errors.push(e);
                }
            },
            racr::Item::Peripheral(peripheral) => {
                let mut peripheral = peripheral.clone();
                // TODO: make logic simpler when convinience functions for getting &mut Path is implemented
                for register_slot in peripheral.registers.iter_mut() {
                    match register_slot {
                        racr::RegisterSlot::Single{instance, ..} => {
                            match instance.ty {
                                racr::RegisterType::Single{path: ref mut name} => resolve_fully_qualified_name(path, name),
                                racr::RegisterType::Array{path: ref mut name, ..} => resolve_fully_qualified_name(path, name),
                            }
                        }
                        racr::RegisterSlot::Union{alternatives, ..} => {
                            for instance in alternatives {
                                match instance.ty {
                                    racr::RegisterType::Single{path: ref mut name} => resolve_fully_qualified_name(path, name),
                                    racr::RegisterType::Array{path: ref mut name, ..} => resolve_fully_qualified_name(path, name),
                                }
                            }
                        }
                    }
                }
                if let Err(e) = sym_table.add_symbol(path.clone(), peripheral.into()) {
                    errors.push(e);
                }
            },
            racr::Item::Register(reg) => {
                if let Err(e) = sym_table.add_symbol(path.clone(), reg.clone().into()) {
                    errors.push(e);
                }
            },
            racr::Item::Mod(racr::Module{content: None, ..}) => (),
            racr::Item::Mod(racr::Module{ident, content: Some(module_content) }) => {
                let mut module_path = path.clone();
                module_path.segments.push(ident.clone()); // TODO: make simpler when `Path::extend` is implemented
                if let Err(mut module_errors) = copy_content_into_symtable(sym_table, &module_path, &module_content) {
                    errors.append(&mut module_errors);
                }
            },
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn resolve_fully_qualified_name(current_path: &racr::Path, name: &mut racr::Path) {
    // TODO: Take in --extern and check for those in addition to `crate`
    if name.segments.first().unwrap().to_string() != "crate" { // TODO: fix when impl PartialEq<str> exists
        let mut new_name = current_path.clone();
        new_name.segments.append(&mut name.segments);
        *name = new_name;
    }
}
