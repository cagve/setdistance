import numpy as np
from matplotlib import pyplot as plt

data = [(5, 7), (3, 4), (6, 4), (1, 2)]


data = np.asarray(data)
print(data)

xmin, xmax = 0, 4
ymin, ymax = 0, 4
ax = plt.axis((xmin,xmax,ymin,ymax))


plt.scatter(data[:,0], data[:,1], c='b') # markers

plt.show()


