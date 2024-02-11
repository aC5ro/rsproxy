use std::{env, io::Write};
use std::fs::OpenOptions;
use pelite::{pe::{Pe, PeFile}, FileMap};
fn main() {
    let args : Vec<String> = env::args().skip(1).collect();
    let pe_path = &args[0];
    let out_path = &args[1];
    let special_add = &args[2];
    let mut out_file = OpenOptions::new()
    .create(true)
    .append(true)
    .write(true)
    .open(out_path).expect("Failed to open output file.");

    let pe_map = FileMap::open(pe_path).expect("Failed to read PE into memory.");
    let pe = PeFile::from_bytes(&pe_map).expect("Failed to get PE object.");
    let exports = pe.exports().expect("Failed to get Exports.");
    let query = exports.by().expect("Failed to LoL");

    for res in query.iter_names(){
        
        if let (Ok(name) , Ok(_)) = res {
            let pragma_name = pe_path.strip_suffix(".dll").expect("Failed to strip .dll.").replace(r"\", r"\\");
            let comment = format!("#pragma comment(linker , \"/export:{}={}{}.{}\")\n",name, pragma_name,special_add,name);
            out_file.write(comment.as_bytes()).expect("Failed to write pragma comment.");
            println!("[+] Proxied {}" , name);
        }
    }

}
