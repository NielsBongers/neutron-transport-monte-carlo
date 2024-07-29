import argparse
import concurrent.futures
from pathlib import Path

import pandas as pd
from pyvtk import PointData, Scalars, StructuredGrid, VtkData


def csv_to_vtk(csv_file_path, vtk_folder, scalar_column, keep_existing=False):
    df = pd.read_csv(csv_file_path, low_memory=False)
    df = df.sort_values(by=["z", "y", "x"])  # Ensure correct data order

    nx, ny, nz = len(df["x"].unique()), len(df["y"].unique()), len(df["z"].unique())
    
    print(f"nx, ny, nz: {nx}, {ny}, {nz}")
    
    if len(df) != nx * ny * nz:
        print(f"Data in {csv_file_path} does not fit perfectly into a structured grid.")
        return -1

    # Reshape data
    try:
        points = df[["x", "y", "z"]].values.reshape((nz, ny, nx, 3), order="C")
        indices = (
            df[scalar_column].values.reshape((nz, ny, nx), order="C").astype(float)
        )
    except Exception as e:
        print(f"Reshaping failed for {csv_file_path}: {e}")
        return -1

    if "geometry" in Path(csv_file_path).stem: 
        vtk_file_path = Path("results/geometry/geometry.vtk")
    else: 
        vtk_file_path = Path(vtk_folder) / f"heat_diffusion_{int(float(Path(csv_file_path).stem) * 1000):05d}.vtk"

    if vtk_file_path.exists() and keep_existing:
        return 0

    try:
        grid = StructuredGrid(dimensions=(nx, ny, nz), points=points.flatten())
        data = PointData(Scalars(indices.flatten(), name=scalar_column))
        vtk_data = VtkData(grid, data)
        vtk_data.tofile(str(vtk_file_path), "binary")
        print(f"File written successfully for {csv_file_path} to {vtk_file_path}.")
    except Exception as e:
        print(f"Error during VTK file creation for {csv_file_path}: {e}")
        return -1

    return 1


def process_csv(csv_file_path):
    try:
        if "geometry" in str(csv_file_path.stem):
            scalar_column = "index"
            vtk_path = Path("results/geometry/vtk")
        elif "position_bin" in str(csv_file_path.stem):
            scalar_column = "fission_count"
        elif "bin" in str(csv_file_path.stem):
            scalar_column = "fission_count"
        elif "heat_diffusion" in str(csv_file_path):
            vtk_path = Path("results/heat_diffusion/vtk")
            scalar_column = "T"
        else:
            return

        print(f"Converting to vtk - {csv_file_path}")
        
        vtk_path.mkdir(exist_ok=True, parents=True)
        csv_to_vtk(csv_file_path, vtk_path, scalar_column, keep_existing=False)
    except Exception as e:
        # Happens if a CSV isn't fully written yet - just re-run the code.
        print(e)


def process_geometry():
    process_csv(Path(r"D:\Desktop\nuclear-rust\results\geometry\geometry.csv"))

def process_heat_diffusion():
    csv_files = list(Path("results/heat_diffusion").glob("**/*.csv"))
    with concurrent.futures.ProcessPoolExecutor(max_workers=20) as executor:
        list(executor.map(process_csv, csv_files), total=len(csv_files))

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Process CSV files")
    parser.add_argument("mode", choices=["geometry", "heat_diffusion"], help="Mode to run the script in")
    args = parser.parse_args()

    if args.mode == "geometry":
        process_geometry()
    elif args.mode == "heat_diffusion":
        process_heat_diffusion()
