from pathlib import Path

import pandas as pd
from pyvtk import PointData, Scalars, StructuredGrid, VtkData
from tqdm import tqdm


def csv_to_vtk(csv_file_path, vtk_folder, scalar_column, keep_existing=False):
    df = pd.read_csv(csv_file_path)
    df = df.sort_values(by=["z", "y", "x"])  # Ensure correct data order

    nx, ny, nz = len(df["x"].unique()), len(df["y"].unique()), len(df["z"].unique())
    if len(df) != nx * ny * nz:
        print("Data does not fit perfectly into a structured grid.")
        return -1

    # Reshape data
    try:
        points = df[["x", "y", "z"]].values.reshape((nz, ny, nx, 3), order="C")
        indices = (
            df[scalar_column].values.reshape((nz, ny, nx), order="C").astype(float)
        )
    except Exception as e:
        print(f"Reshaping failed: {e}")
        return -1

    vtk_file_path = Path(vtk_folder) / (Path(csv_file_path).stem + ".vtk")
    if vtk_file_path.exists() and keep_existing:
        return 0

    try:
        grid = StructuredGrid(dimensions=(nx, ny, nz), points=points.flatten())
        data = PointData(Scalars(indices.flatten(), name=scalar_column))
        vtk_data = VtkData(grid, data)
        vtk_data.tofile(str(vtk_file_path), "binary")
        print("File written successfully.")
    except Exception as e:
        print(f"Error during VTK file creation: {e}")
        return -1

    return 1


csv_files = list(Path("results/geometry").glob("**/*.csv"))
vtk_path = Path("results/geometry")

csv_files = [
    Path(r"D:\Desktop\nuclear-rust\results\geometry/geometry.csv"),
]


# for csv_file_path in tqdm(csv_files, desc="Converting", total=len(csv_files)):
for csv_file_path in csv_files:
    try:
        if "geometry" in str(csv_file_path.stem):
            scalar_column = "index"
        if "position_bin" in str(csv_file_path.stem):
            scalar_column = "fission_count"
        if "bin" in str(csv_file_path.stem):
            scalar_column = "fission_count"
        if "heat_diffusion" in str(csv_file_path):
            scalar_column = "T"

        print("Converting to vtk")

        csv_to_vtk(csv_file_path, vtk_path, scalar_column, keep_existing=False)
    except Exception as e:
        # Happens if a CSV isn't fully written yet - just re-run the code.
        print(e)
        continue

print("\a")
