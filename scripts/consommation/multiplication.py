import matplotlib.pyplot as plt

# Données corrigées pour affichage uniforme de X (catégoriel) et Y en MB
x_labels = ["FheUint8", "FheUint16", "FheUint32", "FheUint64", "FheUint128"]
y_values_kb = [135696, 137520, 143604, 168616, 269428]
# time -v (kB)
rss_time_kb = [134940, 136692, 136644, 138308, 141928]
y_values_mb = [val / 1024 for val in y_values_kb]  # conversion en MB

# Conversion en MB
rss_time_mb = [x / 1024 for x in rss_time_kb]

# Création du graphique avec axe X équidistant
plt.figure(figsize=(10, 6))
plt.plot(x_labels, y_values_mb, marker='o', label='multiplication')
plt.plot(x_labels, rss_time_mb, marker='^', label='addition')

plt.xlabel("Type d'entier FHE (FheUintN)")
plt.ylabel("Mémoire RSS max (MB)")
plt.title("Consommation mémoire lors de la multiplication et l'addition de deux FheUintN mesurée par la commande time avec l'option -v")
plt.ylim(100, 300)
plt.grid(True)
plt.tight_layout()
plt.show()
