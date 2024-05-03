import numpy as np


def fuel_plate(x_value):
    template = """
[[cuboids]] # Fuel element
center = {{ x = {x}, y = 0.0, z = 0.0 }}
width = 0.005
depth = 1.0
height = 1.5
material_name = "U235"
material_composition_vector = [
  {{ material_name = "U238", material_fraction = 0.06 }},
  {{ material_name = "U235", material_fraction = 0.94 }},
]
order = 4
"""
    entry = template.format(x=x_value)
    return entry


def control_rod(x_value, y_value, z_value):
    template = """
[[cylinders]] # Control rod
center = {{ x = {x}, y = {y}, z = {z} }}
direction = {{ x = 0.0, y = 0.0, z = 1.0 }}
length = 4.0
radius = 0.02
material_name = "B10"
material_composition_vector = [
  {{ material_name = "B10", material_fraction = 1.0 }},
]
order = 5
"""
    entry = template.format(x=x_value, y=y_value, z=z_value)
    return entry


def generate_linear_grid(center_x, N, spacing):
    if N % 2 == 1:
        # For odd N, center_x is the middle value
        half_length = (N // 2) * spacing
        start_x = center_x - half_length
        stop_x = center_x + half_length
    else:
        # For even N, adjust so that center_x is between two central points
        half_length = (N // 2) * spacing - spacing / 2
        start_x = center_x - half_length
        stop_x = center_x + half_length

    return np.linspace(start_x, stop_x, N)


number_groups = 5
control_rod_spacing = 5 / 100  # cm
plates_per_group = 1
plate_spacing = 1.5 / 100  # cm
group_spacing = 10 / 100  # cm
group_width = plate_spacing * plates_per_group + group_spacing
group_grid = generate_linear_grid(center_x=0.0, N=number_groups, spacing=group_width)

control_rod_height = 1.64

with open("scripts/geometry_creation/geometries/geometry.toml", "w") as f:
    for group_coordinate in group_grid:

        for control_rod_y in [-0.6, -0.3, 0.0, 0.3, 0.6]:
            f.write(
                control_rod(
                    x_value=group_coordinate,
                    y_value=control_rod_y,
                    z_value=control_rod_height,
                )
            )

        for side in [-1, 1]:

            center_x = group_coordinate + side * control_rod_spacing
            plate_grid = generate_linear_grid(
                center_x=center_x, N=plates_per_group, spacing=plate_spacing
            )
            for plate_coordinate in plate_grid:
                f.write(fuel_plate(plate_coordinate))

    standard_components = """[[cylinders]] # Steel vessel.
center = { x = 0.0, y = 0.0, z = 0.0 }
direction = { x = 0.0, y = 0.0, z = 1.0 }
length = 3.0
radius = 1.0
material_name = "Fe54"
material_composition_vector = [
  { material_name = "Fe54", material_fraction = 1.0 },
]
order = 1

[[spheres]] # Top cap. 
center = { x = 0.0, y = 0.0, z = 1.5 }
radius = 1.0
material_name = "Fe54"
material_composition_vector = [
  { material_name = "Fe54", material_fraction = 1.0 },
]
order = 1

[[spheres]] # Bottom cap. 
center = { x = 0.0, y = 0.0, z = -1.5 }
radius = 1.0
material_name = "Fe54"
material_composition_vector = [
  { material_name = "Fe54", material_fraction = 1.0 },
]
order = 1

[[cylinders]] # Water inside. 
center = { x = 0.0, y = 0.0, z = 0.0 }
direction = { x = 0.0, y = 0.0, z = 1.0 }
length = 3.0
radius = 0.9
material_name = "H1"
material_composition_vector = [
  { material_name = "H1", material_fraction = 0.6666666667 },
  { material_name = "O16", material_fraction = 0.3333333333 },
]
order = 2

[[spheres]] # Water top cap. 
center = { x = 0.0, y = 0.0, z = 1.5 }
radius = 0.9
material_name = "H1"
material_composition_vector = [
  { material_name = "H1", material_fraction = 0.6666666667 },
  { material_name = "O16", material_fraction = 0.3333333333 },
]
order = 2

[[spheres]] # Water bottom cap. 
center = { x = 0.0, y = 0.0, z = -1.5 }
radius = 0.9
material_name = "H1"
material_composition_vector = [
  { material_name = "H1", material_fraction = 0.6666666667 },
  { material_name = "O16", material_fraction = 0.3333333333 },
]
order = 2

## Steel supports

[[cuboids]] # Top right support. 
center = { x = 0.0, y = 0.20, z = 0.75 }
width = 2.0
depth = 0.05
height = 0.05
material_name = "Fe54"
material_composition_vector = [
  { material_name = "Fe54", material_fraction = 1.0 },
]
order = 3

[[cuboids]] # Top left support. 
center = { x = 0.0, y = -0.20, z = 0.75 }
width = 2.0
depth = 0.05
height = 0.05
material_name = "Fe54"
material_composition_vector = [
  { material_name = "Fe54", material_fraction = 1.0 },
]
order = 3

[[cuboids]] # Bottom right support. 
center = { x = 0.0, y = 0.20, z = -0.75 }
width = 2.0
depth = 0.05
height = 0.05
material_name = "Fe54"
material_composition_vector = [
  { material_name = "Fe54", material_fraction = 1.0 },
]
order = 3

# Bottom left support. 
[[cuboids]] # Bottom left support. 
center = { x = 0.0, y = -0.20, z = -0.75 }
width = 2.0
depth = 0.05
height = 0.05
material_name = "Fe54"
material_composition_vector = [
  { material_name = "Fe54", material_fraction = 1.0 },
]
order = 3"""

    f.write(standard_components)
