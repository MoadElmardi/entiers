#!/bin/bash

# === CONFIGURATION ===
BINAIRE="./target/release/addition"  # nom du binaire TFHE à exécuter
REPETITIONS=10
CSV_FILE="resultats_energie.csv"

# === INITIALISATION ===
echo "iteration,energie_joules,temps_secondes" > "$CSV_FILE"
ENERGIES=()
TEMPS=()

echo "Mesure de la consommation énergétique sur $REPETITIONS répétitions..."

for i in $(seq 1 $REPETITIONS); do
    echo "Lancement $i..."

    # Exécution avec perf et capture de la sortie
    OUTPUT=$(sudo perf stat -e power/energy-pkg:u "$BINAIRE" 2>&1)

    # Extraction des valeurs
    JOULES=$(echo "$OUTPUT" | grep -i "Joules" | awk '{print $1}' | tr , .)
    TEMPS_EXEC=$(echo "$OUTPUT" | grep "seconds time elapsed" | awk '{print $1}' | tr , .)

    if [[ -n "$JOULES" && -n "$TEMPS_EXEC" ]]; then
        ENERGIES+=($JOULES)
        TEMPS+=($TEMPS_EXEC)
        echo "$i,$JOULES,$TEMPS_EXEC" >> "$CSV_FILE"
    else
        echo "Erreur de mesure à l'itération $i"
    fi
done

# === CALCUL STATISTIQUE ===
SUM=0
for val in "${ENERGIES[@]}"; do
    SUM=$(echo "$SUM + $val" | bc -l)
done
MOYENNE=$(echo "$SUM / ${#ENERGIES[@]}" | bc -l)

# Écart-type
SQUARE_DIFFS=0
for val in "${ENERGIES[@]}"; do
    DIFF=$(echo "$val - $MOYENNE" | bc -l)
    SQUARE=$(echo "$DIFF * $DIFF" | bc -l)
    SQUARE_DIFFS=$(echo "$SQUARE_DIFFS + $SQUARE" | bc -l)
done
VARIANCE=$(echo "$SQUARE_DIFFS / ${#ENERGIES[@]}" | bc -l)
ECART_TYPE=$(echo "scale=6; sqrt($VARIANCE)" | bc -l)

echo "---------------------------"
echo "Moyenne énergie : $MOYENNE J"
echo "Écart-type      : $ECART_TYPE J"
echo "Résultats exportés vers : $CSV_FILE"
