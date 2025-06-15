import datetime

import pygix_cartography as carto
import polars as pl
import time
import logging
import pathlib

FORMAT = '%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s'
logging.basicConfig(format=FORMAT)
# logging.getLogger("carto").setLevel(logging.WARN)
# logging.getLogger().setLevel(logging.DEBUG)

GIT_DIR = '/Users/rise/Repositories/Binocular'
SNAPSHOT_STORAGE_LOCATION = "/Users/rise/Repositories/se-analytics/gix-test/crates/pyo3-bindings/snapshots"
NO_MERGES = False

assert pathlib.Path(GIT_DIR).exists()
assert pathlib.Path(f"{GIT_DIR}/.git").exists()
assert pathlib.Path(SNAPSHOT_STORAGE_LOCATION).exists()

project_stem = pathlib.Path(GIT_DIR).stem
print(f"project_stem = {project_stem}")
PROJECT_SNAPSHOT_LOCATION_ROOT = f"{SNAPSHOT_STORAGE_LOCATION}/{project_stem}"
del SNAPSHOT_STORAGE_LOCATION
pathlib.Path(PROJECT_SNAPSHOT_LOCATION_ROOT).mkdir(parents=False, exist_ok=True)
slug = datetime.datetime.now().strftime("%Y_%d_%m-%H_%M_%S")

snapshot_filename = f"{PROJECT_SNAPSHOT_LOCATION_ROOT}/{slug}.ndjson"
del PROJECT_SNAPSHOT_LOCATION_ROOT
assert not pathlib.Path(snapshot_filename).exists()
print(f"Storing snapshot at '{snapshot_filename}'")

# print(carto.do_something())
# print(carto.pygix_cartography.__all__)
# print(carto.functions.__all__)
# print("==============================")

# start = time.time()
# # traverse_result: list = carto.functions.traverse_commit_graph(
# #     git_dir=GIT_DIR,
# #     branches=["blame"],
# #     # threads=4,
# #     skip_merges=NO_MERGES)
# # df = pd.DataFrame(traverse_result)
# df = carto.retrieve_commits(
#     GIT_DIR,
#     ["feature/156"],
#     NO_MERGES
# )
# #
# print(df[:3])
# print(df.info())
# # print(df)
# # print(pd.json_normalize(df["committer"]))
# # print(df.info())
# print("++++++++++++++++++++++++++++++++++++++++")
#
# print(f"I am done {time.time() - start}")
# del start, df

start = time.time()
# df: pl.DataFrame = carto.retrieve_diffs(
#     git_dir=GIT_DIR,
#     commitlist=["0ae15d0912ca4b8b15210b13917398b0e35278f6", "HEAD"],
#     threads=4,
#     skip_merges=NO_MERGES)

df: pl.DataFrame = carto.commit.traverse_commit_graph(
    git_dir=GIT_DIR,
    # source_commit_hash="aef1d11147979277e57f12083bccdcdc7991ce14", # 1
    # target_commit_hash="6b975c3281c3cbb9fbbaef1ae27163db2ecc0eb8"
    # source_commit_hash="6b975c3281c3cbb9fbbaef1ae27163db2ecc0eb8", # 2
    # target_commit_hash="a66d86c5547bbcd85cb9c4ccbe9ebc1849985bd3"
    # source_commit_hash="59403507ca8e2c324dda6a96f056f00cccba175a", # 3
)

print("df")
print(df[["commit", "commit_dt", "author_dt"]])
print("df.info()")

# print(df["commit"])

print(f"I am done {time.time() - start}")
# df.write_ndjson(file=snapshot_filename)
