import matplotlib.pyplot as plt

input_lengths = [25, 250, 1350]
slow = [1056, 88548, 2542304]
fast = [1652, 20875, 95557]

plt.figure()
plt.scatter(input_lengths, slow, zorder = 3)
plt.scatter(input_lengths, fast, zorder = 3)
plt.plot(input_lengths, fast, linestyle = "--", color = "m")
plt.xlabel("input length [characters]")
plt.ylabel("avg execution time [ns]")
plt.grid()
plt.show()
