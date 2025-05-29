import matplotlib.pyplot as plt
import numpy as np

# n et profondeur multiplicative = n - 1
security_levels = [128, 64, 40]

# Données de puissance et énergie
power_means = [35.9830, 34.1709, 34.2363]
energy_means = [44.6738, 43.8020, 42.2848]

# Tracé
fig, ax1 = plt.subplots(figsize=(10, 6))

# Axe Y gauche : puissance
color1 = 'tab:blue'
ax1.set_xlabel('Niveau de sécurité (bits)')
ax1.set_ylabel('Puissance (W)', color=color1)
ax1.plot(security_levels, power_means, marker='o', color=color1, label='Puissance')
ax1.tick_params(axis='y', labelcolor=color1)

# Afficher uniquement les niveaux de sécurité fournis
ax1.set_xticks(security_levels)
ax1.set_xticklabels(security_levels)

# Axe Y droit : énergie
ax2 = ax1.twinx()
color2 = 'tab:red'
ax2.set_ylabel('Énergie (J)', color=color2)
ax2.plot(security_levels, energy_means, marker='s', color=color2, label='Énergie')
ax2.tick_params(axis='y', labelcolor=color2)

# Définir les limites pour synchroniser les échelles : 1W = 5J
ax1.set_ylim(0, 50)  # Axe gauche (Puissance)
ax2.set_ylim(0, 50)  # Axe droit (Énergie)

# Grille et titre
plt.title("Consommation selon le niveau de sécurité (addition de deux FheUint16)")
ax1.grid(True, linestyle='--', alpha=0.5)

fig.tight_layout()
plt.show()
