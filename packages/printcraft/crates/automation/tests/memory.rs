//! Byte files exercise exactly the same automation paths as native files.
use printcraft_automation::{Automation, Content};
use serde_json::{Value, json};

fn call(a: &mut Automation, name: &str, args: Value) -> Value {
    match a.call(name, &args).unwrap().remove(0) {
        Content::Json(v) => v,
        other => panic!("expected JSON, got {other:?}"),
    }
}

#[test]
fn memory_create_save_reopen_split_and_isolation() {
    let mut a = Automation::in_memory().with_clock(|| 1_700_000_000);
    let d = call(&mut a, "doc_create", json!({"from":"blank", "pages":3}))["doc"].clone();
    call(&mut a, "doc_save", json!({"doc":d,"path":"nested/out.pdf"}));
    let bytes = a.file_bytes("/nested/./out.pdf").unwrap();
    assert!(bytes.starts_with(b"%PDF-"));
    call(&mut a, "page_rotate", json!({"doc":d,"degrees":90}));
    assert!(a.call("doc_close", &json!({"doc":d})).is_err());
    assert!(call(&mut a, "doc_save", json!({"doc":d}))["incremental"].as_bool().unwrap());
    assert!(a.file_bytes("nested/out.pdf").unwrap().len() > bytes.len());
    call(&mut a, "doc_split", json!({"doc":d,"every":1,"out_dir":"parts"}));
    assert_eq!(a.file_names().unwrap().len(), 4);
    let mut b = Automation::in_memory();
    assert!(b.call("doc_open", &json!({"path":"/nested/out.pdf"})).is_err());
    b.put_file("import.pdf", a.file_bytes("nested/out.pdf").unwrap()).unwrap();
    let reopened = call(&mut b, "doc_open", json!({"path":"import.pdf"}));
    assert_eq!(reopened["pages"], 3);
    assert_eq!(call(&mut b, "doc_info", json!({"doc":reopened["doc"]}))["pages"][0]["rotation"], 90);
}

#[test]
fn memory_paths_cannot_reach_native_files_or_escape_root() {
    let a = Automation::in_memory();
    assert!(a.file_bytes("/etc/passwd").is_err());
    assert!(a.put_file("../escape", vec![]).is_err());
    assert!(a.put_file("/", vec![]).is_err());
    assert!(a.put_file("bad\0name", vec![]).is_err());
    a.put_file("dir/../ok", vec![1, 2, 3]).unwrap();
    assert_eq!(a.file_bytes("/ok").unwrap(), vec![1, 2, 3]);
    assert!(a.put_file("ok", vec![4]).is_err());
    assert_eq!(a.file_bytes("ok").unwrap(), vec![1, 2, 3]);
    assert!(a.with_root(".").is_err());
}
