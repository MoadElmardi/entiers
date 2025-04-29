import matplotlib.pyplot as plt

# Données converties en µs
repetitions = [30, 100, 300, 700, 1000]

# Chiffrement
chiffrement_moyennes = [499.644, 406.708, 446.915, 468.134, 478.798]
chiffrement_ecarts = [98.933, 64.674, 83.721, 94.149, 100.972]

# Opération (conversion ms -> µs)
operation_moyennes = [97.737119 * 1000, 85.711517 * 1000, 90.27329 * 1000, 90.907435 * 1000, 96.545656 * 1000]
operation_ecarts = [9.464731 * 1000, 2.989723 * 1000, 6.688206 * 1000, 6.465817 * 1000, 13.380221 * 1000]

# Déchiffrement
dechiffrement_moyennes = [6.426, 5.257, 5.905, 6.268, 6.483]
dechiffrement_ecarts = [0.995, 0.768, 1.699, 5.282, 3.159]

# Création des 3 graphiques
fig, axes = plt.subplots(3, 1, figsize=(8, 12), sharex=True)

# Graphique 1 : Chiffrement
axes[0].errorbar(repetitions, chiffrement_moyennes, yerr=chiffrement_ecarts, fmt='o-', capsize=5)
axes[0].set_title("Chiffrement")
axes[0].set_ylabel("Temps (µs)")

# Graphique 2 : Opération
axes[1].errorbar(repetitions, operation_moyennes, yerr=operation_ecarts, fmt='o-', capsize=5)
axes[1].set_title("Addition de deux entiers chiffrés")
axes[1].set_ylabel("Temps (µs)")

# Graphique 3 : Déchiffrement
axes[2].errorbar(repetitions, dechiffrement_moyennes, yerr=dechiffrement_ecarts, fmt='o-', capsize=5)
axes[2].set_title("Déchiffrement")
axes[2].set_xlabel("Nombre de répétitions")
axes[2].set_ylabel("Temps (µs)")

# Ajustement des ticks de l'axe X
axes[2].set_xticks(repetitions)
axes[2].set_xticklabels([str(r) for r in repetitions])

# Légende globale
fig.suptitle("Moyennes et écart-types pour différents nombres de répétitions", fontsize=14, y=1.02)

plt.tight_layout()
plt.show()
