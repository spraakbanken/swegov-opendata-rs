mod sfs_preprocess;

use swegov_opendata_preprocess::PreprocessError;

fn main() -> error_stack::Result<(), PreprocessError> {
    sfs_preprocess::main()
}
