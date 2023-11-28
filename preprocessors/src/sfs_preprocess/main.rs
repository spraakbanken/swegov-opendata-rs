use std::path::Path;

use preprocessors::shared::prepare_and_run;
use swegov_opendata_preprocess::{
    preprocess_sfs::{preprocess_sfs_corpus, PreprocessSfsCorpuraOptions},
    PreprocessError,
};

pub fn main() -> error_stack::Result<(), PreprocessError> {
    let args = std::env::args();
    let mut args = args.skip(1);
    let input: String = args.next().expect("`INPUT` required");
    let output = args.next().expect("`OUTPUT` required");
    prepare_and_run("preprocess-sfs", || {
        preprocess_sfs_corpus(PreprocessSfsCorpuraOptions {
            input: Path::new(&input),
            output: Path::new(&output),
        })
    })
}
