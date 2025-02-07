import pygix_cartography as carto
from pygix_cartography import retrieve_commits
# from pygix_cartography import PyGitCommitMetric
# from pygix_cartography.pygix_cartography.functions import traverse_commit_graph
import pandas as pd
import io
import time
import logging

FORMAT = '%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s'
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.DEBUG)

GIT_DIR = '.'
NO_MERGES = False

print(carto.do_something())
print(carto.pygix_cartography.__all__)
print(carto.functions.__all__)
print("==============================")

start = time.time()
# traverse_result: list = carto.functions.traverse_commit_graph(
#     git_dir=GIT_DIR,
#     branches=["blame"],
#     # threads=4,
#     skip_merges=NO_MERGES)
# df = pd.DataFrame(traverse_result)
df = carto.retrieve_commits(
    GIT_DIR,
    ["blame"],
    NO_MERGES
)

committer_df = df[["commit_str", "committer"]]
# committer_df = committer_df[timelogs_df["timelogs"].apply(lambda x: len(x) > 0)]
# committer_df = committer_df.explode("committer")
committer_df = pd.concat(
    [committer_df[["commit_str", "committer"]].reset_index(drop=True), pd.json_normalize(committer_df["committer"])],
    axis=1)
committer_df = committer_df.drop(columns=["committer"]).rename(columns={
    "commit_str": "commit",
    "name": "committer_name",
    "email": "committer_email",
    "time": "committer_time",
})
print(committer_df[:10])
print(committer_df.info())
print(df)
# print(pd.json_normalize(df["committer"]))
print(df.info())
print("++++++++++++++++++++++++++++++++++++++++")

print(f"I am done {time.time() - start}")
del traverse_result, start, df

start = time.time()
traverse_result = carto.functions.get_diffs(
    git_dir=GIT_DIR,
    commitlist=["HEAD"],
    threads=4,
    # committish='fb900694931dc9de03d9dd065491290d1b814aa0',
    skip_merges=NO_MERGES)
# print(
#     f"gix_pyo3: {traverse_result}")

df = pd.DataFrame(traverse_result)
#
print("df")
print(df)
print("df.info()")
print(df.info())
for k, v in df['change_map'][0].items():
    print(f"{k}, {v}")

print(f"I am done {time.time() - start}")
