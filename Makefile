.PHONY: opendata-quick-dev
opendata-quick-dev:
	cargo watch -q -c -w swegov-opendata -x 'run -p swegov-opendata --example quick_dev'

.PHONY: sfs-corpus-quick-dev
sfs-corpus-quick-dev:
	cargo watch -q -c -w sfs-corpus-2 -x 'run -p sfs-corpus-2 --example quick_dev'

.PHONY: spiders-quick-dev
spiders-quick-dev:
	cargo watch -q -c -w opendata-spiders -x 'run -p opendata-spiders --example quick_dev'

.PHONY: quick-dev
quick-dev:
	cargo watch -q -c -w src -w sfs-corpus-core -x 'run -- generate xml data/sfs/output/sfs'

.PHONY: sfs-preprocess2-quick-dev
sfs-preprocess2-quick-dev:
	cargo watch -q -c -w swegov-opendata-preprocess -x 'run -p swegov-opendata-preprocess --bin sfs-preprocess2 -- data/data_raw/sfs/sfs data/sfs-corpus'
