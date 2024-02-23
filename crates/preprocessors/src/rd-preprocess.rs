mod rd_preprocess;

use swegov_opendata_preprocess::PreprocessError;

fn main() -> error_stack::Result<(), PreprocessError> {
    rd_preprocess::main()
}
