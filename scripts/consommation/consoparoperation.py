import matplotlib.pyplot as plt
import numpy as np

# Données
opérations = [
    "Addition", "Multiplication", "Division",
    "Égalité", "Inégalité", ">", "≥",
    "Max", "if_else_then"
]

puissances = [
    36.1822, 43.3891, 47.4502,
    33.5353, 33.4007, 33.7542, 33.8287,
    37.3778, 36.4022
]

energies = [
    44.8436, 86.9786, 322.1636,
    40.0557, 39.8170, 40.1977, 40.3490,
    50.8146, 45.2151
]

x = np.arange(len(opérations))
largeur = 0.35

# Création du graphe
fig, ax1 = plt.subplots(figsize=(12, 6))

# Axe Y gauche : puissance
bar1 = ax1.bar(x - largeur/2, puissances, largeur, label="Puissance (W)", color='skyblue')
ax1.set_ylabel('Puissance (W)', color='skyblue')
ax1.tick_params(axis='y', labelcolor='skyblue')

# Axe Y droit : énergie
ax2 = ax1.twinx()
bar2 = ax2.bar(x + largeur/2, energies, largeur, label="Énergie (J)", color='salmon')
ax2.set_ylabel('Énergie (J)', color='salmon')
ax2.tick_params(axis='y', labelcolor='salmon')

# Définir les limites pour synchroniser les échelles : 1W = 5J
ax1.set_ylim(0, 350)  # Axe gauche (Puissance)
ax2.set_ylim(0, 350)  # Axe droit (Énergie)

# Configuration X
ax1.set_xticks(x)
ax1.set_xticklabels(opérations, rotation=30, ha="right")
ax1.set_title("Consommation énergétique par opération (FheUint16, 10 répétitions)")

# Grille et légende
ax1.grid(axis='y', linestyle='--', alpha=0.5)
fig.tight_layout()
plt.show()
