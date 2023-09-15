# swegov-opendata-rs

Tool used for collecting SFS (Svensk Författningssamling) from [Riksdagens öppna data](https://data.riksdagen.se).

This workspace contains the binary `fetch-sfs` in the root.

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



# References

- [Riksdagens öppna data dokumentation](https://data.riksdagen.se/dokumentation/)
