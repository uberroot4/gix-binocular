import gix_pyo3
import pandas as pd
import io
import time

GIT_DIR = '/Users/rise/Downloads/24s/2024ss-se-pr-group/24ss-se-pr-inso-01/repo'
NO_MERGES = False

start = time.time()
traverse_result = gix_pyo3.traverse_commit_graph(
    git_dir=GIT_DIR,
    branches=["master"],
    # threads=4,
    # committish='fb900694931dc9de03d9dd065491290d1b814aa0',
    no_merges=NO_MERGES)
# print(
#     f"gix_pyo3: {traverse_result}")

df = pd.read_csv(io.StringIO(traverse_result), sep=",")

# print(df)
# print(df.info())

print(f"I am done {time.time() - start}")
del traverse_result, start, df

start = time.time()
traverse_result = gix_pyo3.get_diffs(
    git_dir=GIT_DIR,
    threads=4,
    # committish='fb900694931dc9de03d9dd065491290d1b814aa0',
    no_merges=NO_MERGES)
# print(
#     f"gix_pyo3: {traverse_result}")

df = pd.read_csv(io.StringIO(traverse_result), sep=",")

print(df)
print(df.info())

print(f"I am done {time.time() - start}")
