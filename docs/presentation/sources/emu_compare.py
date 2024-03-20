from matplotlib import pyplot as plt

results: dict[str, int] = {"QEMU": 30525, "Kopycat": 4302, "Unicorn": 26751, "MARS": 5648, "SPIM": 23418, "Rush": 0}
results = dict(sorted(results.items(), key=lambda item: item[1]))
plt.rcdefaults()
plt.bar(results.keys(), results.values(), align="center")

def addlabels(x, y):
    for i in range(len(x)):
        plt.text(i, y[i], y[i], ha="center")
        
addlabels(results.keys(), list(results.values()))
plt.xlabel("Эмуляторы")
plt.xticks(rotation=30)
plt.ylabel("Быстродействие, IPS")
plt.tight_layout()
plt.savefig("./emu_compare.jpg", dpi=600)
