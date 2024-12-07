use std::{env::current_dir, fs::read_to_string, path::Path};

fn load_input_intern(file_path: &Path) -> String {
    match read_to_string(file_path) {
        Ok(s) => s,
        Err(e) => panic!(
            "error reading input '{}': {}",
            file_path.as_os_str().to_str().unwrap(),
            e
        ),
    }
}

pub fn load_input(input_file: &str) -> String {
    let mut file_path = current_dir().unwrap();
    file_path.push("testdata");
    file_path.push(input_file);

    load_input_intern(&file_path)
}

pub fn load_data(input_file: &str, outputs_file: &str) -> (String, String, String) {
    let mut file_path = current_dir().unwrap();
    file_path.push("testdata");
    file_path.push(input_file);

    let input = load_input_intern(&file_path);

    file_path.pop();
    file_path.push(outputs_file);

    let output_lines = match read_to_string(&file_path) {
        Ok(s) => s,
        Err(e) => panic!(
            "error reading input '{}': {}",
            file_path.as_mut_os_string().to_str().unwrap(),
            e
        ),
    };

    let mut output_lines = output_lines.lines();
    let output1 = output_lines.next().unwrap().to_string();
    let output2 = output_lines.next().unwrap().to_string();

    (input, output1, output2)
}
