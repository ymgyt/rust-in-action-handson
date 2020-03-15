use libactionkv::ActionKV;

#[cfg(not(target_os = "windows"))]
const USAGE: &'static str = "
Usage:
  akv_mem FIFE get KEY
  akv_mem FIFE delete KEY
  akv_mem FIFE insert KEY VALUE
  akv_mem FIFE update KEY VALUE
";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE).as_ref();
    let key = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = std::path::Path::new(&fname);
    let mut store = ActionKV::open(path).expect("unable to open file");
    store.load().expect("unable to load data");

    match action {
        "get" => {
            match store.get(key).unwrap() {
                None => eprintln!("{:?} not found", key),
                Some(value) => println!("{}", std::str::from_utf8(&value).unwrap()),
            }
        },
        "delete" => store.delete(key).unwrap(),
        "insert" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.insert(key, value).unwrap()
        },
        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.update(key, value).unwrap()
        },
        _ => eprintln!("{}", &USAGE),
    }
}
