# Réimporter les bibliothèques après le reset
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

# Redéfinir les données
binary_operations_data = {
    "FheUint8": {
        "Addition": 92.297693,
        "Soustraction": 93.202893,
        "Multiplication": 189.800295,
        "Division": 1296.846253,
        "ET binaire": 32.876169,
        "OU binaire": 33.048385,
        "XOR binaire": 32.189504,
        "Égalité": 52.288028,
        "Inégalité": 51.694383,
        "Supérieur": 50.693836,
        "Supérieur ou égal": 50.516097,
    },
    "FheUint16": {
        "Addition": 159.315901,
        "Soustraction": 164.781039,
        "Multiplication": 806.418305,
        "Division": 4738.001629,
        "ET binaire": 59.992443,
        "OU binaire": 58.797078,
        "XOR binaire": 58.519008,
        "Égalité": 105.805724,
        "Inégalité": 100.835244,
        "Supérieur": 98.766179,
        "Supérieur ou égal": 98.239399,
    },
    "FheUint32": {
        "Addition": 349.194552,
        "Soustraction": 371.017134,
        "Multiplication": 3401.475257,
        "Division": 18336.281913,
        "ET binaire": 119.742028,
        "OU binaire": 118.730902,
        "XOR binaire": 117.874755,
        "Égalité": 190.399072,
        "Inégalité": 179.709295,
        "Supérieur": 205.709455,
        "Supérieur ou égal": 201.332309,
    },
    "FheUint64": {
        "Addition": 841.3218,
        "Soustraction": 873.508673,
        "Multiplication": 14562.70346,
        "Division": 78236.159062,
        "ET binaire": 269.920398,
        "OU binaire": 271.226977,
        "XOR binaire": 269.248274,
        "Égalité": 402.135556,
        "Inégalité": 385.062165,
        "Supérieur": 441.766627,
        "Supérieur ou égal": 436.257638,
    },
    "FheUint128": {
        "Addition": 1643.96842,
        "Soustraction": 1668.927962,
        "Multiplication": 56621.668454,
        "Division": 287619.345882,
        "ET binaire": 499.984006,
        "OU binaire": 499.115527,
        "XOR binaire": 499.97837,
        "Égalité": 681.460455,
        "Inégalité": 655.54825,
        "Supérieur": 1005.764136,
        "Supérieur ou égal": 1000.023832,
    },
    "FheUint256": {
        "Addition": 3567.151456,
        "Soustraction": 3521.901218,
        "Multiplication": 194023.101801,
        "ET binaire": 888.566696,
        "OU binaire": 885.444396,
        "XOR binaire": 887.520996,
        "Égalité": 1168.027244,
        "Inégalité": 1150.647008,
        "Supérieur": 1947.630923,
        "Supérieur ou égal": 1982.936872,
    },
    "FheUint512": {
        "Addition": 7784.081203,
        "Soustraction": 7470.255288,
        "Multiplication": 762147.148907,
        "ET binaire": 1770.527361,
        "OU binaire": 1791.392894,
        "XOR binaire": 1790.479154,
        "Égalité": 2281.092306,
        "Inégalité": 2248.39477,
        "Supérieur": 4357.999199,
        "Supérieur ou égal": 4340.45795,
    }
}

# Création du DataFrame et conversion en minutes
df_binary = pd.DataFrame(binary_operations_data)
df_binary_min = df_binary / 1000 / 60  # ms to min

# Création du graphique
fig, ax = plt.subplots(figsize=(16, 9))

operations = df_binary.index.tolist()
types = df_binary.columns.tolist()
x = np.arange(len(operations))
width = 0.1
offsets = np.linspace(-0.35, 0.35, len(types))
colors = plt.cm.get_cmap('tab10', len(types))
bar_tops_min = [[] for _ in range(len(operations))]

# Tracer les barres
for i, t in enumerate(types):
    values = df_binary_min[t]
    positions = x + offsets[i]
    bars = ax.bar(positions, values, width, label=t, color=colors(i))
    for j, bar in enumerate(bars):
        if not np.isnan(bar.get_height()):
            bar_tops_min[j].append((bar.get_x() + bar.get_width() / 2, bar.get_height()))

# Tracer les courbes en pointillés
for group in bar_tops_min:
    group = sorted(group, key=lambda p: p[0])
    xs, ys = zip(*group)
    ax.plot(xs, ys, color='black', linestyle='--', marker='o', linewidth=1, label="Évolution par opération")

# Nettoyage de la légende
handles, labels = ax.get_legend_handles_labels()
seen = set()
unique_handles_labels = [(h, l) for h, l in zip(handles, labels) if not (l in seen or seen.add(l))]

# Mise en forme
ax.set_xticks(x)
ax.set_xticklabels(operations, rotation=45, ha="right")
ax.set_ylabel("Temps moyen (minutes)")
ax.set_title("Temps d'exécution des opérations binaires homomorphes par type de FheUint (en minutes)")
ax.legend(*zip(*unique_handles_labels), title="Légende")
ax.grid(True, which='both', linestyle='--', linewidth=0.5, alpha=0.7)
plt.tight_layout()
plt.show()
