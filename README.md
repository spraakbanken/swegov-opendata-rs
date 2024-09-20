# swegov-opendata-rs

Tools used for collecting SFS (Svensk Författningssamling) from [Riksdagens öppna data](https://data.riksdagen.se).

[![MIT licensed][mit-badge]][mit-url]

[![Maturity badge - level 1][scorecard-badge]][scorecard-url]

[![CI(check)][actions-check-badge]][actions-check-url]
[![CI(scheduled)][actions-scheduled-badge]][actions-scheduled-url]
[![CI(test)][actions-test-badge]][actions-test-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE
[actions-check-badge]: https://github.com/spraakbanken/swegov-opendata-rs/actions/workflows/check.yml/badge.svg
[actions-check-url]: https://github.com/spraakbanken/swegov-opendata-rs/actions?query=workflow%3Acheck+branch%3Amain
[actions-scheduled-badge]: https://github.com/spraakbanken/swegov-opendata-rs/actions/workflows/scheduled.yml/badge.svg
[actions-scheduled-url]: https://github.com/spraakbanken/swegov-opendata-rs/actions?query=workflow%3Ascheduled+branch%3Amain
[actions-test-badge]: https://github.com/spraakbanken/swegov-opendata-rs/actions/workflows/test.yml/badge.svg
[actions-test-url]: https://github.com/spraakbanken/swegov-opendata-rs/actions?query=workflow%3Atest+branch%3Amain
[scorecard-badge]: https://img.shields.io/badge/Maturity-Level%201%20--%20New%20Project-yellow.svg
[scorecard-url]: https://github.com/spraakbanken/getting-started/blob/main/scorecard.md


## fetch-sfs

Binary to run for collecting SFS.

Uses [`webcrawler`](#webcrawler) and [`opendata-spider`](#opendata-spiders).

Takes roughly 1 hour to fetch all SFS data.

## opendata-spiders

Lives in [`opendata-spider`](./opendata-spiders/).

Uses [`swegov-opendata`](#swegov-opendata).

### sfs

Contains concrete spider for collecting [SFS](./opendata-spiders/src/sfs.rs).

This spider spawns urls that searches for documents of type `SFS` in 20 years spans, using the `data.riksdagen.se/dokumentlista` path.

These lists are scraped for `dok_id` to scrape documents and `nasta_sida` to scrape next page in the `dokumentlista`.

All fetched pages are stored to disk in JSON-format, except for the pages with html fragments, that are stored as-is. The documents are grouped by year.

This spider handles the following inconsistencies in the api.

- Fetching the data in JSON format sometimes doesn't include text field.
    - Instead the documents are fetch with xml and translated to JSON in the process step.
- `data.riksdagen.se/dokument/<dok_id>` is supposed to get the document with `dok_id`.
    - sometimes, an empty document with no data is returned
    - sometimes, the `html` field of a document is returned
    - for both problems above , the path `data.riksdagen.se/dokumentstatus/<dok_id>` is needed
-




## sfs-corpus
Uses [`swegov-opendata`](#swegov-opendata).
Build corpus files for processing with sparv.

## swegov-opendata

Data model for the documents and document lists from riksdagens öppna data with [`serde`](https://serde.rs) serialization and deserialization.

## webcrawler
Lives in [`webcrawler`](./webcrawler/).

Generic web crawler that defines an interface for spiders.

The spiders work in 2 steps,
- scraping an url for new urls and/or data
- processing the fetched data



## References

- [Riksdagens öppna data dokumentation](https://data.riksdagen.se/dokumentation/)

## MSRV Policy

The MSRV (Minimum Supported Rust Version) is fixed for a given minor (1.x)
version. However it can be increased when bumping minor versions, i.e. going
from 1.0 to 1.1 allows us to increase the MSRV. Users unable to increase their
Rust version can use an older minor version instead. Below is a list of swegov-opendata-rs versions
and their MSRV:

 * v0.1: Rust 1.70.

Note however that swegov-opendata-rs also has dependencies, which might have different MSRV
policies. We try to stick to the above policy when updating dependencies, but
this is not always possible.
