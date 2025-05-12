#!/bin/bash

# === CONFIGURATION ===
REPETITIONS=10
#BINAIRE="./target/release/addition"
#BINAIRE="./target/release/multiplication"
#BINAIRE="./target/release/division"
#BINAIRE="./target/release/factoriel"
#BINAIRE="./target/release/comparaison"
#BINAIRE="./target/release/max"
#BINAIRE="./target/release/selectionconditionnelle"
BINAIRE="./target/release/securite"
P_IDLE=3.097  # Puissance mesurée à l'état idle en Watts (J/s)

# === DONNÉES ===
declare -a P_NET_LIST
declare -a E_NET_LIST

echo "Commande : $BINAIRE"

echo "Mesure nette d'énergie sur $REPETITIONS exécutions..."
echo "Puissance idle fixée à ${P_IDLE} W"

for i in $(seq 1 $REPETITIONS); do
    echo "Itération $i..."

    OUT=$(sudo perf stat -a -e power/energy-pkg/ $BINAIRE 2>&1)

    # Extraction des Joules et du temps écoulé
    JOULES=$(echo "$OUT" | grep "Joules" | awk '{print $1}' | sed 's/[^0-9,.-]//g' | tr ',' '.')
    TIME=$(echo "$OUT" | grep "seconds time elapsed" | awk '{print $1}' | tr ',' '.')

    if [[ -n "$JOULES" && -n "$TIME" ]]; then
        P_TOTAL=$(echo "scale=6; $JOULES / $TIME" | bc -l)
        P_NET=$(echo "scale=6; $P_TOTAL - $P_IDLE" | bc -l)
        E_NET=$(echo "scale=6; $P_NET * $TIME" | bc -l)

        echo "  Énergie brute : $JOULES J, Temps : $TIME s"
        echo "  => Puissance nette : $P_NET W, Énergie nette : $E_NET J"

        P_NET_LIST+=($P_NET)
        E_NET_LIST+=($E_NET)
    else
        echo "  ⚠️ Erreur de mesure."
    fi
done

# === ANALYSE STATISTIQUE ===

# Moyenne et écart-type
function moyenne_ecart_type() {
    local -n valeurs=$1
    local total=0
    for v in "${valeurs[@]}"; do
        total=$(echo "$total + $v" | bc -l)
    done
    local moy=$(echo "$total / ${#valeurs[@]}" | bc -l)

    local sumsq=0
    for v in "${valeurs[@]}"; do
        local diff=$(echo "$v - $moy" | bc -l)
        local sq=$(echo "$diff * $diff" | bc -l)
        sumsq=$(echo "$sumsq + $sq" | bc -l)
    done

    local ecart=$(echo "scale=6; sqrt($sumsq / ${#valeurs[@]})" | bc -l)

    echo "$moy $ecart"
}

read MOY_PNET ECART_PNET < <(moyenne_ecart_type P_NET_LIST)
read MOY_ENET ECART_ENET < <(moyenne_ecart_type E_NET_LIST)

echo "----------------------------------------"
echo "Résultats moyens sur $REPETITIONS exécutions :"
echo "Puissance nette moyenne : $MOY_PNET W (± $ECART_PNET)"
echo "Énergie nette moyenne   : $MOY_ENET J (± $ECART_ENET)"
