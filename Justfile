opendata-quick-dev:
    cargo watch -q -c -w swegov-opendata -x 'run -p swegov-opendata --example quick_dev'

spiders-quick-dev:
    cargo watch -q -c -w opendata-spiders -x 'run -p opendata-spiders --example quick_dev'

quick-dev:
    cargo watch -q -c -w src -w sfs-corpus-core -x 'run -- generate xml data/sfs/output/sfs'

sfs-corpus-watch-test:
    cargo watch -q -c -w sfs-corpus -x 'test -p sfs-corpus'

preprocess-rd-watch-quick-dev:
    cargo watch -q -c -w preprocess-rd -x 'run -p preprocess-rd'

preprocess-rd-watch-test:
    cargo watch -q -c -w preprocess-rd -x 'test -p preprocess-rd'
