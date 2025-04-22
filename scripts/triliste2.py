import matplotlib.pyplot as plt
import numpy as np

# Données
tailles_vecteurs = [8, 16, 32, 64, 128, 256, 512, 1028, 2048]
tri_normal_vecteur = [0.489, 0.779, 1.469, 2.638, 5.795, 12.461, 27.672, 75.591, 136.271]  # en µs
tri_chiffre_vecteur = [
    11175.285251,
    37140.943489,
    114341.592514,
    311997.227768,
    837181.609582,
    2143524.898918,
    5373237.146841,
    12982072.423754,
    31102769.062904
]  # en ms

# Conversion du tri chiffré en heures (ms -> h)
tri_chiffre_vecteur_h = [v / 1000 / 3600 for v in tri_chiffre_vecteur]

# Création du graphique
fig, ax1 = plt.subplots(figsize=(10, 6))
x = np.arange(len(tailles_vecteurs))

# Axe gauche - tri normal (µs)
color1 = 'tab:blue'
ax1.set_xlabel("Taille du vecteur")
ax1.set_ylabel("Tri normal (µs)", color=color1)
ax1.plot(x, tri_normal_vecteur, marker='o', linestyle='-', color=color1, label="Tri normal")
ax1.tick_params(axis='y', labelcolor=color1)
ax1.set_xticks(x)
ax1.set_xticklabels(tailles_vecteurs)

# Axe droit - tri homomorphe (heures)
color2 = 'tab:red'
ax2 = ax1.twinx()
ax2.set_ylabel("Tri homomorphe (heures)", color=color2)
ax2.plot(x, tri_chiffre_vecteur_h, marker='o', linestyle='-', color=color2, label="Tri homomorphe")
ax2.tick_params(axis='y', labelcolor=color2)

ax1.grid(True, linestyle='--', alpha=0.6)
ax2.grid(True, linestyle='--', alpha=0.6)

# Titre global
fig.suptitle("Temps de tri normal vs homomorphe selon la taille du vecteur (FheUint16)")

plt.tight_layout()
plt.show()
