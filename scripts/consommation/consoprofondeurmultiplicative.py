import matplotlib.pyplot as plt
import numpy as np

# n et profondeur multiplicative = n - 1
n_values = np.array([3, 6, 11, 16, 21, 26, 31, 36, 41, 46, 51])
profondeurs = n_values - 1

# Données de puissance et énergie
puissances = [47.8129, 52.1304, 55.0458, 56.2393, 56.7644, 57.2189,
              56.3798, 56.7015, 57.1941, 57.2855, 57.4954]
energies = [140.2702, 299.8073, 572.5754, 850.2993, 1123.1874, 1399.2994,
            1635.4130, 1916.1097, 2200.6160, 2478.5829, 2756.0912]

# Tracé
fig, ax1 = plt.subplots(figsize=(10, 6))

# Axe Y gauche : puissance
color1 = 'tab:blue'
ax1.set_xlabel('Profondeur multiplicative')
ax1.set_ylabel('Puissance (W)', color=color1)
ax1.plot(profondeurs, puissances, marker='o', color=color1, label='Puissance')
ax1.tick_params(axis='y', labelcolor=color1)

# Afficher uniquement les profondeurs fournies
ax1.set_xticks(profondeurs)
ax1.set_xticklabels(profondeurs)

# Axe Y droit : énergie
ax2 = ax1.twinx()
color2 = 'tab:red'
ax2.set_ylabel('Énergie (J)', color=color2)
ax2.plot(profondeurs, energies, marker='s', color=color2, label='Énergie')
ax2.tick_params(axis='y', labelcolor=color2)

# Définir les limites pour synchroniser les échelles : 1W = 5J
ax1.set_ylim(0, 60)  # Axe gauche (Puissance)
ax2.set_ylim(0, 3000)  # Axe droit (Énergie)

# Grille et titre
plt.title("Consommation selon la profondeur multiplicative (factoriel des FheUint16)")
ax1.grid(True, linestyle='--', alpha=0.5)

fig.tight_layout()
plt.show()
