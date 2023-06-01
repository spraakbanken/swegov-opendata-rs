opendata-quick-dev:
    cargo watch -q -c -w swegov-opendata/examples -x 'run -p swegov-opendata --example quick_dev'

quick-dev:
    cargo watch -q -c -w src -w sfs-corpus-core -x 'run -- generate xml data/sfs/output/sfs'
