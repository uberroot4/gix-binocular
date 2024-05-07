import gix_pyo3
import pandas as pd
import io
import time

start = time.time()
traverse_result = gix_pyo3.traverse(
    '/Users/rise/Repositories/Binocular',
    threads=4,
    # committish='fb900694931dc9de03d9dd065491290d1b814aa0',
    no_merges=True)
# print(
#     f"gix_pyo3: {traverse_result}")

df = pd.read_csv(io.StringIO(traverse_result), sep=",")

print(df)
print(df.info())

print(f"I am done {time.time() - start}")
