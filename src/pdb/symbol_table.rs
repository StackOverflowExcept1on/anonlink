use iced_x86::{Instruction, SymbolResolver, SymbolResult};
use pdb::{FallibleIterator, Rva};
use pelite::pe64::PeFile;
use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Debug)]
pub struct RemoteSymbolResolver {
    rva2symbol: HashMap<u64, String>,
    symbol2rva: HashMap<String, u64>,
}

impl RemoteSymbolResolver {
    pub fn rva2symbol(&self, rva: &u64) -> Option<&String> {
        self.rva2symbol.get(rva)
    }

    pub fn symbol2rva<T: Into<String>>(&self, symbol: T) -> Option<&u64> {
        self.symbol2rva.get(&symbol.into())
    }
}

impl SymbolResolver for RemoteSymbolResolver {
    fn symbol(
        &mut self,
        _instruction: &Instruction,
        _operand: u32,
        _instruction_operand: Option<u32>,
        address: u64,
        _address_size: u32,
    ) -> Option<SymbolResult> {
        self.rva2symbol(&address)
            .map(|symbol_string| SymbolResult::with_str(address, symbol_string.as_str()))
    }
}

pub fn download_symbols(file: PeFile) -> crate::Result<RemoteSymbolResolver> {
    let debug_data = super::symbol_server::lookup(file)?;
    let pdb_link = debug_data.pdb_link();

    let resp = ureq::get(&pdb_link).call().map_err(Box::new)?;

    let mut buf = Vec::new();
    resp.into_reader().read_to_end(&mut buf)?;

    let cursor = io::Cursor::new(buf);

    let mut pdb = pdb::PDB::open(cursor)?;
    let symbol_table = pdb.global_symbols()?;
    let address_map = pdb.address_map()?;

    let mut rva2symbol = HashMap::new();
    let mut symbol2rva = HashMap::new();

    let mut symbols = symbol_table.iter();
    while let Some(symbol) = symbols.next()? {
        match symbol.parse() {
            Ok(pdb::SymbolData::Public(data)) if data.function => {
                let Rva(value) = data.offset.to_rva(&address_map).unwrap_or_default();

                let rva = value as u64;
                let symbol = data.name.to_string().into_owned();

                rva2symbol.insert(rva, symbol.clone());
                symbol2rva.insert(symbol.clone(), rva);
            }
            _ => {}
        }
    }

    Ok(RemoteSymbolResolver {
        rva2symbol,
        symbol2rva,
    })
}
