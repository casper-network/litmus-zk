use std::fs;

pub(crate) fn get_fixture_content(fname: String) -> String {
    let path = get_path_to_resource(fname);

    fs::read_to_string(path).unwrap()
}

pub(crate) fn get_path_to_resource(fname: String) -> String {
    format!("resources/chain/{fname}")
}

pub(crate) fn get_block_resources_directory() -> fs::ReadDir {
    fs::read_dir(format!("resources/chain/blocks")).unwrap()
}
