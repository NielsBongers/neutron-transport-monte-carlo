from pathlib import Path

import pandas as pd


def get_latest_folder(root: str) -> Path:
    folder_generator = Path(root).glob("**/*")

    folder_names = []

    for target_path in folder_generator:
        if target_path.is_file():
            continue

        folder_names.append(target_path)
    folder_names.sort()

    return folder_names[-1]


def get_latest_df(root: Path, file_name) -> None:
    df = pd.read_csv(root / file_name)
    return df
