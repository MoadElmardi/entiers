import matplotlib.pyplot as plt
import numpy as np

# Données pour le tri
tailles = ["FheUint8", "FheUint16", "FheUint32"]
tri_normal = [10.722 / 1000, 5.795 / 1000, 5.542 / 1000]  # conversion µs -> ms
tri_chiffre = [507719.379917, 837181.609582, 1665054.155996]  # secondes -> ms

# Création du graphe
fig, ax = plt.subplots(figsize=(10, 6))

x = np.arange(len(tailles))

# Courbes
ax.plot(x, tri_normal, marker='o', linestyle='-', label="Tri normal", color='blue')
ax.plot(x, tri_chiffre, marker='o', linestyle='-', label="Tri homomorphe", color='red')

# Mise en forme
ax.set_xticks(x)
ax.set_xticklabels(tailles)
ax.set_ylabel("Temps (ms)")
ax.set_title("Comparaison des temps de tri (avec et sans chiffrement) d'un vecteur de taille 128")
ax.legend()
ax.grid(True, which='both', linestyle='--', linewidth=0.5, alpha=0.7)
plt.tight_layout()
plt.show()
