use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::hash::Hash;
use phf::PhfHash;
use phf_shared::FmtConst;


pub fn generate_map<Key>(
    name: &str,
    output_path: &str,
    key_type: &str,
    value_type: &str,
    map: &[(Key, &str)],
    imports: Option<&str>,
)
where
    Key: Hash + PhfHash + Eq + FmtConst
{
    let path = Path::new(output_path);
    let mut file = BufWriter::new(File::create(&path).unwrap());

    if let Some(imports) = imports {
        writeln!(
            &mut file,
            "{}\n",
            imports,
        ).unwrap();
    }

    write!(
        &mut file,
        "pub static {name}: ::phf::Map<{key}, {value}> = ",
        name = name,
        key = key_type,
        value = value_type,
    ).unwrap();

    let mut builder = phf_codegen::Map::new();

    for (key, value) in map {
        builder.entry(key, value);
    }

    writeln!(&mut file, "{};", builder.build()).unwrap();
}
