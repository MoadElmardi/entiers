import matplotlib.pyplot as plt
import re

# Données fournies
schemes = [
    "PARAM_MESSAGE_1_CARRY_1_KS_PBS_32_BITS",
    "V1_0_PARAM_MESSAGE_1_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
    "V1_0_PARAM_MESSAGE_1_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
    "V1_0_PARAM_MESSAGE_2_CARRY_0_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
    "V1_0_PARAM_MESSAGE_2_CARRY_1_KS_PBS_GAUSSIAN_2M128",
    "V1_0_PARAM_MESSAGE_2_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
    "V1_0_PARAM_MESSAGE_3_CARRY_0_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
    "V1_0_PARAM_MESSAGE_3_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
    "V1_0_PARAM_MESSAGE_3_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128",
    "V1_0_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128"
]
# Max RSS en kB, converti en MB
max_rss_kb = [131712, 131840, 161104, 171648, 131712, 483200, 132096, 140164, 404680, 1040768]
max_rss_mb = [x / 1024 for x in max_rss_kb]

# Valgrind déjà en MB
valgrind_mb = [123.8, 124.6, 151.9, 163.6, 124.4, 465.6, 124.4, 131.5, 388.8, 1007.7]

# Extraction des bits message et carry
labels = []
for s in schemes:
    m = re.search(r"PARAM_MESSAGE_(\d+)_CARRY_(\d+)", s)
    if m:
        labels.append(f"m={m.group(1)}, c={m.group(2)}")
    else:
        labels.append("N/A")

# Plot
x = range(len(labels))
width = 0.35

fig, ax = plt.subplots(figsize=(10, 5))
ax.bar([i - width/2 for i in x], max_rss_mb, width, label="time -v")
ax.bar([i + width/2 for i in x], valgrind_mb, width, label="Valgrind")

ax.set_xticks(x)
ax.set_xticklabels(labels, rotation=45, ha="right")
ax.set_ylabel("Usage (MB)")
ax.set_title("Comparaison des coûts en mémoire selon les espaces message et carry")
ax.legend()
ax.set_axisbelow(True)
plt.grid(linestyle='--', linewidth=0.5, alpha=0.7)
plt.tight_layout()
plt.show()
