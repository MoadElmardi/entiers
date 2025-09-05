#!/usr/bin/env bash

# Script pour mesurer le temps d'exécution moyen d'un programme Rust sur 100 runs

# Nom du fichier binaire
BIN="target/release/operation_temps"

# Nombre d'exécutions
runs=30

total_ns=0

# Boucle d'exécution et mesure
for ((i=1; i<=runs; i++)); do
    echo "Itération $i"
    start=$(date +%s%N)
    ./$BIN
    end=$(date +%s%N)
    elapsed=$((end - start))
    total_ns=$((total_ns + elapsed))
done

# Calcul de la moyenne
average_ns=$((total_ns / runs))
# Conversion en millisecondes (float avec awk)
average_ms=$(awk "BEGIN {printf \"%.3f\", $average_ns/1000000}")

echo "- Résultat sur $runs exécutions :"
echo "- Moyenne : $average_ns nanosecondes ($average_ms ms)"
