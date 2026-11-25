import numpy as np
import time

start = time.perf_counter()

array = np.arange(0, 100000000, 1)

end = time.perf_counter()

print(f"Time taken: {end - start:.4f} seconds")