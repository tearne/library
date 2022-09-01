from statistics import quantiles
import numpy as np
from scipy import stats

array = [1,2,3,4,5,5,5,5,5]
percentile = 40

p = quantiles(array, n=100)[percentile]
print("Statistics says", p)

p = np.percentile(np.array(array), percentile)
print("NumPy says", p)

p = stats.scoreatpercentile(array, percentile)
print("SciPy says", p)
