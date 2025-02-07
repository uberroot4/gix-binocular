import pandas as pd
from ..pygix_cartography import functions


def retrieve_diffs(git_dir: str = ".", commitlist: list[str] = ["HEAD"], threads: int = 1,
                   skip_merges: bool = False, breadth_first: bool = False, follow: bool = False,
                   limit: int = None) -> pd.DataFrame():
    result = functions.get_diffs(
        git_dir=git_dir,
        commitlist=commitlist,
        threads=threads,
        skip_merges=skip_merges,
        breadth_first=breadth_first,
        follow=follow,
        limit=limit)
    result_df = pd.DataFrame(result)
    del result

    return result_df
