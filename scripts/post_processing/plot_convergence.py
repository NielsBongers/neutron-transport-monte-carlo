from datetime import datetime

import matplotlib.pyplot as plt
import pandas as pd

# df = pd.read_csv(Path(r"D:\Desktop\nuclear-rust\results\geometry\neutron_bins.csv"))

# print(df["fission_count"].sum())
# print(df["fission_count"].describe())

df = pd.read_csv(
    r"D:\Desktop\nuclear-rust\results\diagnostics\runs\Convergence analysis - 2024-11-03_19-08-16.164089200\convergence.csv"
)

# We only track starting from generation 5 by default
df = df[df["generation"] > 5]

plt.plot(df["generation"], df["convergence"])
plt.title("Convergence analysis")
plt.xlabel("Generation")
plt.ylabel("Convergence measure")
plt.grid()
plt.loglog()
plt.savefig(
    f"figures/general/{datetime.now().strftime('%d%m%Y - ')}Neutron Monte Carlo - convergence analysis - basic reactor.png",
    dpi=300,
)
plt.show()

# plt.plot(df[""])
