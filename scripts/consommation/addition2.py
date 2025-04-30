# Re-import necessary packages after code execution state reset
import matplotlib.pyplot as plt

# Données avec trois outils
types = ["FheUint8", "FheUint16", "FheUint32", "FheUint64", "FheUint128"]

# time -v (kB)
rss_time_kb = [134940, 136692, 136644, 138308, 141928]

# pmap (2e colonne dans chaque ligne) — mémoire utilisée réellement (Resident)
rss_pmap_kb = [135992, 136936, 137620, 138288, 140908]

# valgrind massif (heap peak)
rss_valgrind_kb = [129331, 129740, 130662, 132300, 136908]

# Conversion en MB
rss_time_mb = [x / 1024 for x in rss_time_kb]
rss_pmap_mb = [x / 1024 for x in rss_pmap_kb]
rss_valgrind_mb = [x / 1024 for x in rss_valgrind_kb]

# Création du graphique
plt.figure(figsize=(10, 6))
plt.plot(types, rss_time_mb, marker='o', label='time -v')
plt.plot(types, rss_pmap_mb, marker='s', label='pmap')
plt.plot(types, rss_valgrind_mb, marker='^', label='valgrind massif')

plt.xlabel("Type d'entier FHE (FheUintN)")
plt.ylabel("Mémoire max (MB)")
plt.title("Comparaison de la mémoire utilisée selon l'outil")
plt.ylim(60, 200)
plt.grid(True)
plt.legend()
plt.tight_layout()
plt.show()
