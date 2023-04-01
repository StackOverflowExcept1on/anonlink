use pelite::pe64::debug::{CodeView, Entry};
use pelite::pe64::image::{GUID, IMAGE_DEBUG_CV_INFO_PDB70};
use pelite::pe64::{Pe, PeFile};
use pelite::util::CStr;

#[derive(Debug)]
struct CodeView70<'a> {
    image: &'a IMAGE_DEBUG_CV_INFO_PDB70,
    pdb_file_name: &'a CStr,
}

fn as_code_view70(entry: Entry) -> Option<CodeView70> {
    match entry.as_code_view()? {
        CodeView::Cv70 {
            image,
            pdb_file_name,
        } => Some(CodeView70 {
            image,
            pdb_file_name,
        }),
        _ => None,
    }
}

#[derive(Debug)]
pub struct PossibleDebugData {
    pub filename: String,
    pub hash: String,
}

impl PossibleDebugData {
    pub fn pdb_link(&self) -> String {
        format!(
            "https://msdl.microsoft.com/download/symbols/{filename}/{hash}/{filename}",
            filename = self.filename,
            hash = self.hash
        )
    }
}

pub fn lookup(file: PeFile) -> pelite::Result<PossibleDebugData> {
    let debug = file.debug()?;

    let CodeView70 {
        image,
        pdb_file_name,
    } = debug
        .into_iter()
        .filter_map(|dir| dir.entry().ok().and_then(as_code_view70))
        .next()
        .ok_or(pelite::Error::Invalid)?;

    let mut filename = pdb_file_name.to_string();
    if let Some(pos) = filename.rfind('\\') {
        filename = filename[(pos + 1)..].to_string();
    }

    let GUID {
        Data1: data1,
        Data2: data2,
        Data3: data3,
        Data4: data4,
    } = image.Signature;

    let data4 = u64::from_be_bytes(data4);
    let age = image.Age;

    let hash = format!("{data1:08X}{data2:04X}{data3:04X}{data4:016X}{age:X}");

    Ok(PossibleDebugData { filename, hash })
}
