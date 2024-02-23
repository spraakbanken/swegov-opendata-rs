#!/usr/bin/env bash

# Run script:
# nohup ./install.sh >> install.out 2>&1 &

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
  echo "------ Clean corpus $corpus ------"
  cd material/$corpus

  # cwb-huffcode -A -r export/cwb.encoded/registry/ ${corpus^^}
  # rm -r export/cwb.encoded/data/*.corpus
  # cwb-compress-rdx -A -r export/cwb.encoded/registry/ ${corpus^^}
  # rm -r export/cwb.encoded/data/*.corpus.{rev,rdx}

  # rm -r sparv-workdir/cwb.install_corpus_marker
  # sparv install cwb:install_corpus

  # Cleanup
  rm -rf logs
  rm -rf export/cwb.vrt
  rm -rf export/cwb.encoded
  rm -rf export/korp.wordpicture
  rm -rf export/stats_export.frequency_list_sbx
  rm -rf export/xml_export.combined
  rm -rf export/xml_export.pretty

  cd ../..
  echo "------ Done cleaning corpus $corpus ------"
done
