from matplotlib import pyplot as plt

results: dict[str, int] = {"QEMU": 15826, "Kopycat": 5052, "Unicorn": 8172, "MARS": 4302, "SPIM": 7084, "Rush": 9324}
results = dict(sorted(results.items(), key=lambda item: item[1]))
plt.rcdefaults()
plt.bar(results.keys(), results.values(), align="center")

def addlabels(x, y):
    for i in range(len(x)):
        plt.text(i, y[i], y[i], ha="center", fontsize=15)
        
addlabels(results.keys(), list(results.values()))
plt.xlabel("Эмуляторы", fontsize=12)
# plt.title("Выполнение ПО, реализующего алгоритм поиска простых чисел", fontsize=10)
plt.xticks(fontsize=15)
plt.yticks(fontsize=15)
plt.ylabel("Быстродействие, KIPS", fontsize=12)
plt.tight_layout()
plt.savefig("./emu_compare.jpg", dpi=600)
