import matplotlib.pyplot as plt
import numpy as np

# Types
types = ["FheUint8", "FheUint16", "FheUint32", "FheUint64", "FheUint128"]
x = np.arange(len(types))

# Données de puissance (W)
puissance_add = [33.6853, 36.1822, 39.1882, 43.9929, 48.9671]
puissance_mul = [36.4623, 43.3891, 55.4810, 62.6618, 65.3364]
puissance_div = [40.4130, 47.4502, 54.3859, 59.1415, 62.8798]

# Données d'énergie (J)
energie_add = [39.5387, 44.8436, 56.7409, 80.6018, 128.5702]
energie_mul = [48.9345, 86.9786, 247.8686, 897.0712, 3535.2612]
energie_div = [117.4240, 322.1636, 1119.8097, 4262.2348, 17120.0102]

# === Graphe 1 : Puissance ===
plt.figure(figsize=(10, 6))
plt.plot(x, puissance_add, marker='o', label='Addition')
plt.plot(x, puissance_mul, marker='o', label='Multiplication')
plt.plot(x, puissance_div, marker='o', label='Division')
plt.xticks(x, types)
plt.ylim(0, 70)
plt.ylabel("Puissance (W)")
plt.title("Puissance moyenne par type de chiffrement")
plt.grid(True, linestyle='--', alpha=0.5)
plt.legend()
plt.tight_layout()
plt.show()

# === Graphe 2 : Énergie ===
plt.figure(figsize=(10, 6))
plt.plot(x, energie_add, marker='o', label='Addition')
plt.plot(x, energie_mul, marker='o', label='Multiplication')
plt.plot(x, energie_div, marker='o', label='Division')
plt.xticks(x, types)
plt.ylabel("Énergie (J)")
plt.title("Énergie moyenne par type de chiffrement")
plt.grid(True, linestyle='--', alpha=0.5)
plt.legend()
plt.tight_layout()
plt.show()
