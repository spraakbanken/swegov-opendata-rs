#!/usr/bin/env bash

# Start preloader (if not running already):
# nohup sparv preload -j8 > preloader.out 2>&1 &
#
# Run script:
# nohup ./annotate.sh > annotate.out 2>&1 &

# Increase chance for a process to be finished when machine runs out of memory
echo 500 > /proc/$$/oom_score_adj

corpora="
rd-bet
rd-ds
rd-eun
rd-flista
rd-fpm
rd-frsrdg
rd-ip
rd-kammakt
rd-kom
rd-mot
rd-ovr
rd-prop
rd-prot
rd-rskr
rd-samtr
rd-skfr
rd-sou
rd-tlista
rd-utr
rd-utsk
rd-yttr
"

for corpus in $corpora
do
  echo ""
  echo "------ Running corpus $corpus ------"
  cd material/$corpus
  rm -rf export/sbx_strix/*
  sparv run sbx_strix:config
  sparv install sbx_strix:install_config
  # rm -rf logs

  # sparv run -j8 --socket ../sparv.socket -k
  # rm -rf logs export/xml_export.preserved_format
  # cp -r export/xml_export.pretty/* /export/res/lb/korpus/strix-xml/$corpus/
  cd ../..
  echo "------ Done running corpus $corpus ------"
done
