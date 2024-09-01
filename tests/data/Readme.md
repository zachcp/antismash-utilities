# Example Data


## Mibig

## Antismash Json


```sh
# https://www.ncbi.nlm.nih.gov/datasets/genome/GCF_005519465.1/
# submit to AS
# https://antismash.secondarymetabolites.org/upload/bacteria-ca2986de-f172-4d68-b7f2-07f0493e4eb4/genomic.json
# downaload as genomic.json

json_typegen tests/data/genomic.json  -o src/antismash.rs -n AntismashJSON

cat genomic.json | fx '.records[0].modules' > modules.json
cat genomic.json | fx '.records[0].modules["antismash.detection.hmm_detection"]' > hmm_detection.json
cat genomic.json | fx '.records[0].modules["antismash.detection.cluster_hmmer"]' > cluster_hmmer.json
cat genomic.json | fx '.records[0].modules["antismash.detection.genefunctions"]' > genefunctions.json
cat genomic.json | fx '.records[0].modules["antismash.detection.nrps_pks_domains"]' > nrps_pks_domains.json
cat genomic.json | fx '.records[0].modules["antismash.detection.tigrfam"]' > tigrfam.json
cat genomic.json | fx '.records[0].modules["antismash.modules.active_site_finder"]' > active_site_finder.json
cat genomic.json | fx '.records[0].modules["antismash.modules.cluster_compare"]' > cluster_compare.json
cat genomic.json | fx '.records[0].modules["antismash.modules.clusterblast"]' > clusterblast.json
cat genomic.json | fx '.records[0].modules["antismash.modules.lanthipeptides"]' > lanthipeptides.json
cat genomic.json | fx '.records[0].modules["antismash.modules.lassopeptides"]' > lassopeptides.json
cat genomic.json | fx '.records[0].modules["antismash.modules.nrps_pks"]' > nrps_pks.json
cat genomic.json | fx '.records[0].modules["antismash.modules.pfam2go"]' > pfam2go.json
cat genomic.json | fx '.records[0].modules["antismash.modules.rrefinder"]' > rrefinder.json
cat genomic.json | fx '.records[0].modules["antismash.modules.sactipeptides"]' > sactipeptides.json
cat genomic.json | fx '.records[0].modules["antismash.modules.t2pks"]' > t2pks.json
cat genomic.json | fx '.records[0].modules["antismash.modules.tfbs_finder"]' > tfbs_finder.json
cat genomic.json | fx '.records[0].modules["antismash.modules.thiopeptides"]' > thiopeptides.json
cat genomic.json | fx '.records[0].modules["antismash.modules.tta"]' > tta.json

json_typegen tests/data/modules.json  -o src/antismash_mods.rs -n Module
json_typegen tests/data/hmm_detection.json  -o src/antismash_hmm_detection.rs -n HmmDetection
json_typegen tests/data/cluster_hmmer.json  -o src/antismash_cluster_hmmer.rs -n ClusterHmm
json_typegen tests/data/genefunctions.json  -o src/antismash_genefunctions.rs -n GeneFunction
json_typegen tests/data/nrps_pks_domains.json  -o src/antismash_nrps_pks_domains.rs -n NrpsPksDomains
json_typegen tests/data/tigrfam.json  -o src/antismash_tigrfam.rs -n Tigrfam
json_typegen tests/data/active_site_finder.json  -o src/antismash_active_site_finder.rs -n ActiveSiteFinder
json_typegen tests/data/cluster_compare.json  -o src/antismash_cluster_compare.rs -n ClusterCompare
json_typegen tests/data/clusterblast.json  -o src/antismash_clusterblast.rs -n Clusterblast
json_typegen tests/data/lanthipeptides.json  -o src/antismash_lanthipeptides.rs -n Lanthipeptides
json_typegen tests/data/lassopeptides.json  -o src/antismash_lassopeptides.rs -n Lassopeptides
json_typegen tests/data/nrps_pks.json  -o src/antismash_nrps_pks.rs -n NrpsPks
json_typegen tests/data/pfam2go.json  -o src/antismash_pfam2go.rs -n Pfam2go
json_typegen tests/data/rrefinder.json  -o src/antismash_rrefinder.rs -n Rrefinder
json_typegen tests/data/sactipeptides.json  -o src/antismash_sactipeptides.rs -n Sactipeptides
json_typegen tests/data/t2pks.json  -o src/antismash_t2pks.rs -n T2pks
json_typegen tests/data/tfbs_finder.json  -o src/antismash_tfbs_finder.rs -n TfbsFinder
json_typegen tests/data/thiopeptides.json  -o src/antismash_thiopeptides.rs -n Thiopeptides
json_typegen tests/data/tta.json  -o src/antismash_tta.rs -n Tta

```
