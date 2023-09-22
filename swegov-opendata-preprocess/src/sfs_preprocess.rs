use std::{
    fs,
    io::{self, Read},
    path::Path,
    time::Instant,
};

use error_stack::ResultExt;
use swegov_opendata::DokumentStatusPage;
use swegov_opendata_preprocess::{
    preprocess::sfs::{Corpus, Text},
    PreprocessError,
};

use tracing_subscriber::EnvFilter;

fn main() {
    let start = Instant::now();
    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::fmt()
        .json()
        // .with_span_list(true)
        // .with_current_span(true)
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("sfs_preprocess=info,info"))
                .expect("telemetry: Creating EnvFilter"),
        )
        // .with_writer(io::stderr)
        // .with_writer(file_appender)
        .finish();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).expect("telemetry: setting subscriber");

    let res = try_main();
    let time_elapsed = start.elapsed();
    tracing::info!(?time_elapsed, "elapsed time");
    eprintln!("running time: {:?}", time_elapsed);
    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}

fn try_main() -> error_stack::Result<(), PreprocessError> {
    let args = std::env::args();
    let mut args = args.skip(1);
    let input = args.next().expect("`INPUT` required");
    let output = args.next().expect("`OUTPUT` required");

    let num_files_read = walk_and_build_sparv_xml(&input, &output)?;
    tracing::info!(count = num_files_read, "files read");
    Ok(())
}

#[tracing::instrument]
pub fn walk_and_build_sparv_xml(
    input: &str,
    output: &str,
) -> error_stack::Result<usize, PreprocessError> {
    eprintln!("Reading from '{}' ...", input);

    let mut num_files_read = 0;
    for year in fs::read_dir(input).change_context(PreprocessError)? {
        let year = year.change_context(PreprocessError)?;
        match build_sparv_xml_from_year(year.path().as_path(), Path::new(output)) {
            Ok(num_files_read_for_year) => {
                tracing::info!(
                    count = num_files_read_for_year,
                    ?year,
                    "files read for year"
                );
                num_files_read += num_files_read_for_year;
            }
            Err(error) => {
                tracing::error!(?year, error = ?error, "Error when processing year");
                continue;
            }
        };
    }
    Ok(num_files_read)
}

#[tracing::instrument]
pub fn build_sparv_xml_from_year(
    path: &Path,
    output_base: &Path,
) -> error_stack::Result<usize, PreprocessError> {
    fs::create_dir_all(output_base).change_context(PreprocessError)?;
    let mut corpus = Corpus::new("sfs");
    let mut num_files_read = 0;
    for file_path in fs::read_dir(path).change_context(PreprocessError)? {
        let file_path = file_path.change_context(PreprocessError)?;
        // tracing::event!(
        //     Level::INFO,
        //     file_path = ?file_path,
        //     "reading a file"
        // );
        tracing::debug!("reading text from {}", file_path.path().display());
        let text = read_text(file_path.path().as_path())?;

        tracing::debug!("adding text to corpus");
        corpus.add_text(text);
        num_files_read += 1;
    }

    let year_str = path.display().to_string();
    let year = year_str.rsplit_once('/').unwrap().1;
    let year_filename = format!("sfs-{year}.xml");
    let year_filename = output_base.join(year_filename);
    // tracing::event!("filename = {:#?}", year_filename);
    // tracing::debug!("sfs_year = {:#?}", sfs_year);
    tracing::debug!("writing corpus");
    write_corpus(&corpus, &year_filename).attach_printable_lazy(|| {
        format!("error when writing corpus to '{}'", year_filename.display())
    })?;
    Ok(num_files_read)
}

#[tracing::instrument]
pub fn read_text(path: &Path) -> error_stack::Result<Text, PreprocessError> {
    let file = fs::File::open(path).change_context(PreprocessError)?;
    let file_reader = io::BufReader::new(file);
    let mut reader = flate2::read::GzDecoder::new(file_reader);

    tracing::debug!("[main.read_text] parsing JSON");
    let mut text = String::new();
    reader
        .read_to_string(&mut text)
        .change_context(PreprocessError)?;
    let DokumentStatusPage { dokumentstatus } = serde_json::from_str(&text)
        // .map_err(|err| {
        //     // tracing::debug!("text={}", text);
        //     err
        // })
        .change_context(PreprocessError)
        .attach_printable_lazy(|| format!("Failed to deserialize file: {}", path.display()))?;
    // let DokumentStatusPage { dokumentstatus } = dokumentstatus;
    tracing::debug!("[main.read_text] create Text");
    Ok(Text::try_from(dokumentstatus)
        .change_context(PreprocessError)
        .attach_printable_lazy(|| format!("Failed to parse Text from '{}'", path.display()))?)
}

#[tracing::instrument(skip(corpus))]
pub fn write_corpus(corpus: &Corpus, path: &Path) -> error_stack::Result<(), PreprocessError> {
    // let mut buffer = String::new();
    // quick_xml::se::to_writer(&mut buffer, corpus)?;
    let file = fs::File::create(path)
        .change_context(PreprocessError)
        .attach_printable_lazy(|| format!("failed to create {}", path.display()))?;
    let mut writer = io::BufWriter::new(file);

    tracing::debug!("building elem from corpus");
    let elem = corpus.build_elem();
    tracing::debug!("built elem from corpus, writing ...");
    // dbg!(&elem);
    elem.write_to(&mut writer).change_context(PreprocessError)?;
    // file.write_all(&buffer.as_bytes())?;
    Ok(())
}
