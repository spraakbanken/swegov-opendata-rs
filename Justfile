opendata-quick-dev:
    cargo watch -q -c -w swegov-opendata -x 'run -p swegov-opendata --example quick_dev'

sfs-corpus-quick-dev:
    cargo watch -q -c -w sfs-corpus-2 -x 'run -p sfs-corpus-2 --example quick_dev'

spiders-quick-dev:
    cargo watch -q -c -w opendata-spiders -x 'run -p opendata-spiders --example quick_dev'

quick-dev:
    cargo watch -q -c -w src -w sfs-corpus-core -x 'run -- generate xml data/sfs/output/sfs'
