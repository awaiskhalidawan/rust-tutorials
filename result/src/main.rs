use std::fs::File;

fn main() {
    let file_open_result = File::open("hello.txt");       // Try to open a file. The open function will return a Result<File, Error> type.

    let _my_file = match file_open_result {            // Use the match statement to handle the Result type.
        Ok(file) => file,                             // If the file was opened successfully, return the file.  
        Err(error) => {
            panic!("Error opening file: {}", error);  // If there was an error opening the file, panic with the error message.
        }
    };
}
