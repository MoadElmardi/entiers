import matplotlib.pyplot as plt

# Données corrigées
profondeur_multiplicative = [1, 2, 5, 10, 20]  # profondeur associée aux factoriels 2, 3, 6, 11, 21
valgrind_mib = [127.8, 128.4, 129.0, 129.1, 129.1]
valgrind_mb = [mib * 1.048576 for mib in valgrind_mib]  # Conversion MiB → MB

time_kb = [137860, 138588, 139192, 139512, 140012]
time_mb = [kb / 1000 for kb in time_kb]  # Conversion KB → MB

# Tracé
plt.figure(figsize=(10, 6))
plt.plot(profondeur_multiplicative, time_mb, label='time -v (MB)', marker='o')
plt.plot(profondeur_multiplicative, valgrind_mb, label='Valgrind (MB)', marker='^', color='green')

plt.xlabel("Profondeur multiplicative")
plt.ylabel("Consommation mémoire (MB)")
plt.title("Consommation mémoire du factoriel selon la profondeur multiplicative")
plt.grid(True)
plt.legend()
plt.ylim(100, 150)
plt.xticks(profondeur_multiplicative)  # valeurs X exactes
plt.tight_layout()
plt.show()
