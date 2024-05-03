import json
import re
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


class ENDFHandling:
    """Simple class for handling ENDF files."""

    def __init__(self, endf_file_path: Path):
        """Initializes the handler by opening a given ENDF path.

        Args:
            endf_file_path (Path): Path to ENDF file.
        """
        self.scattering_mt = [2]
        self.absorption_mt = range(102, 118)

        self.COLUMN_INCREMENT = 11

        self.endf_file_path = endf_file_path
        self.endf_data = {}

        self.import_endf()

        self.material_name = self.get_endf_name()

    def import_endf(self) -> dict:
        """Imports the ENDF file specified in __init__.

        Raises:
            FileNotFoundError: ENDF file path does not exist.

        Returns:
            dict: Dictionary containing the ENDF data, with MTs as the keys.
        """
        if not Path(self.endf_file_path).exists():
            print(f"Specified ENDF file not found: {self.endf_file_path}")
            raise FileNotFoundError

        with open(self.endf_file_path) as f:
            file_started = False
            new_section_started = False

            for index, line in enumerate(f):
                material_code = int(line[66:70].strip())
                mf = int(line[70:72].strip())
                mt = int(line[72:75].strip())
                line_number = int(line[75:81].strip())

                # Special starting code.
                if line_number == 99999:
                    file_started = True
                    new_section_started = True
                    continue
                # Skipping non-data or empty portions.
                if mf == 0 or mf == 1:
                    continue
                # Skipping the file header.
                if not file_started:
                    continue
                # Skipping the section header.
                if line_number <= 3:
                    continue

                # Adding the MT to the dictionary if it doesn't exist yet.
                if str(mt) not in self.endf_data.keys():
                    self.endf_data[str(mt)] = {"energy": [], "cross_section": []}
                else:
                    # If the MT exists but we're also on a new section, this must be a duplicate.
                    if new_section_started:
                        continue
                new_section_started = False

                # Extracting the line segments.
                number_values = []
                for increment in range(1, 7):
                    left_slice = self.COLUMN_INCREMENT * (increment - 1)
                    right_slice = self.COLUMN_INCREMENT * increment
                    number_value = line[left_slice:right_slice].strip()

                    # Skipping empty values.
                    if number_value == "":
                        continue

                    # Converting scientific notation format.
                    number_value = number_value.replace("E", "")
                    number_value = re.sub("(?<!^)-", "e-", number_value)
                    number_value = re.sub("(?<!^)\+", "e+", number_value)

                    number_value = float(number_value)
                    number_values.append(number_value)

                # Alternating energies/cross-sections.
                energies = number_values[0::2]
                cross_sections = number_values[1::2]

                # Pushing to their respective lists in the MT segment.
                self.endf_data[str(mt)]["energy"].extend(energies)
                self.endf_data[str(mt)]["cross_section"].extend(cross_sections)
        return self.endf_data

    def to_json(self, file_path: Path, saved_data: dict = None):
        """Saves the ENDF data as a JSON.

        Args:
            file_path (Path): File path to save to.
            saved_data (dict, optional): Subset to save. Defaults to None.
        """
        with open(file_path, "w") as f:

            if not saved_data:
                json.dump(self.endf_data, f, indent=4)
            else:
                json.dump(saved_data, f, indent=4)

    def plot_mt_cross_sections(self, mt_range: list = None):
        """Plotting a certain range of MTs.

        Args:
            mt_range (list, optional): Range to plot. Defaults to None.
        """
        for mt in self.endf_data.keys():
            if mt_range and int(mt) not in list(mt_range):
                continue

            energy = self.endf_data[mt]["energy"]
            cross_section = self.endf_data[mt]["cross_section"]

            plt.plot(energy, cross_section, label=f"MT{mt}")

        Path("figures").mkdir(exist_ok=True, parents=True)

        plt.legend()
        plt.loglog()
        plt.title("Cross-sections")
        plt.xlabel("Energy (eV)")
        plt.ylabel("Cross-section (barn)")
        plt.savefig(
            "figures/result.png",
            dpi=300,
        )
        plt.show()

    def get_subset(self, mt_range: list) -> dict:
        """Extracts a subset of MTs.

        Args:
            mt_range (list): Range to extract.

        Returns:
            dict: Extracted subset from the main data.
        """
        mt_subset = {}

        for mt in self.endf_data.keys():
            if int(mt) in mt_range:
                mt_subset[mt] = self.endf_data[str(mt)]

        return mt_subset

    def aggregate_mts(self, mt_range: list) -> tuple[np.array, np.array]:
        """Combines multiple MTs by interpolating their individual energies.

        Args:
            mt_range (list): Range of MTs to combine.

        Returns:
            tuple[np.array, np.array]: Tuple of energies and interpolated cross-sections.
        """
        scattering_subset = self.get_subset(mt_range)

        all_energies = []

        for key, value in scattering_subset.items():
            all_energies.extend(value["energy"])

        unique_energies = np.unique(all_energies)

        aggregate_cross_sections = np.zeros_like(unique_energies)

        for key, value in scattering_subset.items():
            mt_interpolation_results = np.interp(
                unique_energies,
                value["energy"],
                value["cross_section"],
                left=0,
                right=0,
            )
            aggregate_cross_sections += mt_interpolation_results

        return unique_energies, aggregate_cross_sections

    def write_files(self, file_path: Path, energies, cross_sections):
        """Creates files formatted with data. This is mainly for my personal projects.

        Args:
            file_path (Path): File path to write to.
            energies (_type_): Energies to write.
            cross_sections (_type_): Cross-sections to write.
        """
        with open(file_path, "w") as f:
            f.write("energy,cross_section\n")

            for energy, cross_section in zip(energies, cross_sections):
                f.write(f"{energy},{cross_section}\n")

    def create_material(self):
        """Creates a material folder with aggregated scattering and absorption cross-sections."""
        result_folder = Path("endf_data/ready_endfs") / Path(self.material_name)
        result_folder.mkdir(exist_ok=True, parents=True)

        scattering_energies, scattering_cross_sections = self.aggregate_mts(
            self.scattering_mt
        )
        absorption_energies, absorption_cross_sections = self.aggregate_mts(
            self.absorption_mt
        )

        self.write_files(
            result_folder / Path(self.material_name + "_aggregated_scattering.csv"),
            scattering_energies,
            scattering_cross_sections,
        )
        self.write_files(
            result_folder / Path(self.material_name + "_aggregated_absorption.csv"),
            absorption_energies,
            absorption_cross_sections,
        )

    def get_endf_name(self) -> str:
        """Extracts the name from the ENDF

        Raises:
            ValueError: _description_

        Returns:
            str: Name of the material, formatted as h-1: [lowercase element]-[isotope].
        """
        with open(self.endf_file_path, "r") as f:
            for index, line in enumerate(f):
                if index == 5:
                    line = line.replace(" ", "")

                    results = re.findall("[A-Za-z]+-[0-9]+M?", line)

                    if len(results) > 1:
                        raise ValueError

                    endf_name = results[0]

                    endf_name = endf_name.lower()

                    break

        return endf_name
