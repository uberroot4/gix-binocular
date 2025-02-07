from typing import Optional, List
from datetime import datetime

class PySig:
    """
    Represents a signature with a name, email, and timestamp.

    Attributes:
        name (str): The name of the signer.
        email (str): The email address of the signer.
        time (datetime): The time when the signature was created.
    """
    name: str
    email: str
    time: datetime


class PyGitDiffMetric:
    """
    Represents metrics associated with a git diff operation.

    Attributes:
        total_number_of_files_changed (int): The total number of files that were changed.
        total_number_of_insertions (int): The total number of insertions made.
        total_number_of_deletions (int): The total number of deletions made.
        commit (str): The commit identifier (hash) for the diff.
        parent (Optional[str]): The parent commit identifier, if available.
        committer (Optional[PySig]): The signature of the committer, if available.
        author (Optional[PySig]): The signature of the author, if available.
    """
    total_number_of_files_changed: int
    total_number_of_insertions: int
    total_number_of_deletions: int
    commit: str
    parent: Optional[str]
    committer: Optional[PySig]
    author: Optional[PySig]


class PyGitCommitMetric:
    """
    Represents metrics for a git commit.

    Attributes:
        commit_str (str): The commit identifier as a string.
        message (str): The commit message.
        committer (Optional[PySig]): The signature of the committer, if available.
        author (Optional[PySig]): The signature of the author, if available.
        branch (Optional[str]): The branch name associated with the commit, if applicable.
        parents (List[str]): A list of parent commit identifiers.
    """
    commit_str: str
    message: str
    committer: Optional[PySig]
    author: Optional[PySig]
    branch: Optional[str]
    parents: List[str]
