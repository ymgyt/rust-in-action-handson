use libactionkv::ActionKV;
use std::collections::HashMap;

#[cfg(not(target_os = "windows"))]
const USAGE: &'static str = "
Usage:
  akv_mem FIFE get KEY
  akv_mem FIFE delete KEY
  akv_mem FIFE insert KEY VALUE
  akv_mem FIFE update KEY VALUE
";

fn store_index_on_disk(a: &mut ActionKV, index_key: &[u8]) {
    a.index.remove(index_key);
    let index_as_bytes = bincode::serialize(&a.index).unwrap();
    a.index = HashMap::new();
    a.insert(index_key, &index_as_bytes).unwrap();
}

fn main() {
    const INDEX_KEY: &[u8] = b"+index";

    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE).as_ref();
    let key = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = std::path::Path::new(&fname);
    let mut a = ActionKV::open(path).expect("unable to open file");
    a.load().expect("unable to load data");
    store_index_on_disk(&mut a, INDEX_KEY);

    match action {
        "get" => {
            let index_as_bytes = a.get(&INDEX_KEY).unwrap().unwrap();
            let index: HashMap<Vec<u8>, u64> = bincode::deserialize(&index_as_bytes).unwrap();

            match index.get(key) {
                None => eprintln!("{:?} not found", key),
                Some(value) => println!("{:?}", value),
            }
        },
        "delete" => a.delete(key).unwrap(),
        "insert" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            a.insert(key, value).unwrap()
        },
        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            a.update(key, value).unwrap()
        },
        _ => eprintln!("{}", &USAGE),
    }
}
