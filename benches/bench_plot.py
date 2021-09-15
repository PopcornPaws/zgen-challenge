import matplotlib.pyplot as plt
import numpy as np

input_lengths = np.array([25, 250, 1350])
slow = np.array([1056, 88548, 2542304])
fast = np.array([1652, 20875, 95557])

A_fast = np.vstack((np.ones_like(input_lengths), input_lengths)).T
A_slow = np.vstack((np.ones_like(input_lengths), input_lengths, input_lengths**2)).T

x_fast = np.linalg.inv(A_fast.T @ A_fast) @ A_fast.T @ fast
x_slow = np.linalg.inv(A_slow.T @ A_slow) @ A_slow.T @ slow

input_domain = np.linspace(0, 1350, 1351)

plt.figure()
plt.scatter(input_lengths, slow, zorder = 3, color = "m")
plt.scatter(input_lengths, fast, zorder = 3, color = "c")
plt.plot(input_domain, x_slow[0] + x_slow[1] * input_domain + x_slow[2] * input_domain**2, linestyle = ":", color = "r")
plt.plot(input_domain, x_fast[0] + x_fast[1] * input_domain, linestyle = "--", color = "b")
plt.xlabel("input length [characters]")
plt.ylabel("avg execution time [ns]")
plt.legend(["slow fit", "fast fit", "slow measurements", "fast measurements"])
plt.grid()
plt.show()
