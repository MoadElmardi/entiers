import numpy as np
import pandas as pd
import matplotlib.pyplot as plt

# Redéfinir les données après le reset
data = {
    "FheUint16": {
        "Chiffrement": (832.991, 162.573),
        "Shift": (7.232, 5.329),
        "Addition constante": (120617.264, 4819.445),
        "Multiplication constante": (42323.265, 2039.232),
        "Casting": (5.959, 2.659),
        "ET binaire": (14.736, 5.916),
        "OU binaire": (2.912, 0.873),
        "NOT": (11.053, 3.623),
        "Déchiffrement": (13.492, 3.721),
    },
    "FheUint32": {
        "Chiffrement": (1436.9, 136.057),
        "Shift": (12.704, 7.203),
        "Addition constante": (219452.607, 13631.646),
        "Multiplication constante": (72966.084, 5640.959),
        "Casting": (9.259, 1.735),
        "ET binaire": (22.758, 4.005),
        "OU binaire": (2.701, 0.299),
        "NOT": (17.239, 2.276),
        "Déchiffrement": (22.702, 4.029),
    },
    "FheUint64": {
        "Chiffrement": (2907.695, 236.386),
        "Shift": (32.13, 13.669),
        "Addition constante": (491712.197, 27830.265),
        "Multiplication constante": (161336.802, 14045.654),
        "Casting": (28.166, 3.324),
        "ET binaire": (52.438, 8.299),
        "OU binaire": (3.556, 0.382),
        "NOT": (38.661, 4.941),
        "Déchiffrement": (50.581, 5.447),
    },
    "FheUint128": {
        "Chiffrement": (5915.26, 506.985),
        "Shift": (91.697, 35.558),
        "Addition constante": (1099972.889, 116122.29),
        "Multiplication constante": (355397.237, 54879.944),
        "Casting": (83.541, 32.107),
        "ET binaire": (242.881, 79.171),
        "OU binaire": (4.972, 2.793),
        "NOT": (111.932, 29.221),
        "Déchiffrement": (102.801, 29.884),
    },
    "FheUint256": {
        "Chiffrement": (11529.584, 740.022),
        "Shift": (211.564, 103.259),
        "Addition constante": (2193878.011, 76241.797),
        "Multiplication constante": (683005.009, 28700.365),
        "Casting": (208.783, 44.336),
        "ET binaire": (604.782, 135.392),
        "OU binaire": (5.32, 0.776),
        "NOT": (253.671, 42.117),
        "Déchiffrement": (55.916, 15.65),
    },
    "FheUint512": {
        "Chiffrement": (28489.648, 4915.348),
        "Shift": (740.504, 423.01),
        "Addition constante": (5858141.563, 979344.317),
        "Multiplication constante": (1679207.772, 286501.323),
        "Casting": (586.609, 85.257),
        "ET binaire": (1864.558, 606.701),
        "OU binaire": (10.65, 7.85),
        "NOT": (677.793, 193.98),
        "Déchiffrement": (66.564, 24.506),
    },
    "FheUint1024": {
        "Chiffrement": (56104.658, 6974.097),
        "Shift": (3093.332, 601.536),
        "Addition constante": (12105080.288, 1245048.01),
        "Multiplication constante": (3357857.584, 357884.028),
        "Casting": (1234.497, 207.646),
        "ET binaire": (3903.601, 721.452),
        "OU binaire": (11.394, 4.951),
        "NOT": (1690.97, 255.094),
        "Déchiffrement": (66.118, 28.381),
    },
    "FheUint2048": {
        "Chiffrement": (110419.555, 25963.023),
        "Shift": (6562.523, 2546.279),
        "Addition constante": (23864846.124, 4025606.726),
        "Multiplication constante": (7578268.466, 1393017.853),
        "Casting": (2413.889, 907.523),
        "ET binaire": (10779.234, 2492.716),
        "OU binaire": (12.544, 5.657),
        "NOT": (3998.976, 791.477),
        "Déchiffrement": (99.48, 153.836),
    },
}

# Création d'un DataFrame pour visualisation
df = pd.DataFrame({k: {op: val[0] for op, val in v.items()} for k, v in data.items()})
#import ace_tools as tools; tools.display_dataframe_to_user(name="Temps moyens des opérations (µs)", dataframe=df)

# Liste des opérations et types
operations = list(data["FheUint16"].keys())
types = list(data.keys())

# Conversion des temps moyens en millisecondes pour les barres
df_ns = df * 1000  # convert µs to ms

# Position des barres
x = np.arange(len(operations))
width = 0.1  # largeur de chaque barre
offsets = np.linspace(-0.35, 0.35, len(types))

# Liste des opérations à exclure
excluded_ops_ns = ["Addition constante", "Multiplication constante", "Chiffrement"]
filtered_ops_ns = [op for op in operations if op not in excluded_ops_ns]
x_ns = np.arange(len(filtered_ops_ns))

# Création du graphe
fig, ax = plt.subplots(figsize=(14, 8))

# Couleurs et stockage des coordonnées pour les lignes
colors = plt.cm.get_cmap('tab10', len(types))
line_coords = {op: [] for op in operations}

# Tracer les barres et sauvegarder les hauteurs des barres par opération
bar_centers_ns = [[] for _ in range(len(filtered_ops_ns))]

for i, t in enumerate(types):
    values = df_ns[t].loc[filtered_ops_ns]
    positions = x_ns + offsets[i]
    bars = ax.bar(positions, values, width, label=t, color=colors(i))
    for j, bar in enumerate(bars):
        bar_centers_ns[j].append((bar.get_x() + bar.get_width() / 2, bar.get_height()))

# Tracer les courbes en pointillés noirs
for group in bar_centers_ns:
    group = sorted(group, key=lambda p: p[0])
    xs, ys = zip(*group)
    ax.plot(xs, ys, color='black', linestyle='--', marker='o', linewidth=1, label="Évolution par opération")

# Empêche la répétition de la légende des courbes
handles, labels = ax.get_legend_handles_labels()
seen = set()
unique_handles_labels = [(h, l) for h, l in zip(handles, labels) if not (l in seen or seen.add(l))]


# Ajustements
ax.set_xticks(x_ns)
ax.set_xticklabels(filtered_ops_ns, rotation=45, ha="right")
ax.set_ylabel("Temps moyen (ns)")
ax.set_title("Temps d'exécution des opérations unaires par type de FheUint (en ns)")
ax.legend(*zip(*unique_handles_labels), title="Légende")
plt.tight_layout()
plt.show()
