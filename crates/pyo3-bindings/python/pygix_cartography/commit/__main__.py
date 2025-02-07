import pandas as pd
from ..pygix_cartography import functions


def retrieve_commits(git_dir: str = ".", branches: list[str] = ["HEAD"], skip_merges: bool = False) -> pd.DataFrame():
    traverse_result = functions.traverse_commit_graph(
        git_dir=git_dir,
        branches=branches,
        skip_merges=skip_merges)
    traverse_result_df = pd.DataFrame(traverse_result).rename(columns={
        "commit_str": "commit_hash",
    })
    del traverse_result

    traverse_result_df = (pd.concat({i: pd.json_normalize(x) for i, x in traverse_result_df.pop('committer').items()})
                          .reset_index(level=1, drop=True)
                          .join(traverse_result_df)
                          .reset_index(drop=True))
    traverse_result_df = traverse_result_df.rename(columns={
        "name": "committer_name",
        "email": "committer_email",
        "time": "committer_time",
    })
    traverse_result_df = (pd.concat({i: pd.json_normalize(x) for i, x in traverse_result_df.pop('author').items()})
                          .reset_index(level=1, drop=True)
                          .join(traverse_result_df)
                          .reset_index(drop=True))
    traverse_result_df = traverse_result_df.rename(columns={
        "name": "author_name",
        "email": "author_email",
        "time": "author_time",
    })
    return traverse_result_df