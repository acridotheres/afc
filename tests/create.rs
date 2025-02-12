use afc::{File, FileSource};
use dh::recommended::*;

#[test]
fn create_000() {
    let mut source = dh::data::read_ref(&[8u8; 31]);
    let mut target = dh::file::open_rw("./tests/samples/c000.afc").unwrap();

    let mut files = vec![FileSource(
        File {
            path: "file1".to_string(),
            offset: 0,
            length: 31,
        },
        &mut source,
    )];

    afc::create(&mut files, &mut target, 1024).unwrap();

    target.rewind().unwrap();

    let list = afc::list(&mut target).unwrap();

    assert_eq!(list.len(), 1);
    assert_eq!(list[0].path, "file1");
    assert_eq!(list[0].length, 31);

    let mut output = dh::data::write_new(list[0].length);
    afc::extract(
        &mut FileSource(list[0].clone(), &mut target),
        &mut output,
        1024,
    )
    .unwrap();

    assert_eq!(dh::data::close_ref(source), &dh::data::close(output));
}
