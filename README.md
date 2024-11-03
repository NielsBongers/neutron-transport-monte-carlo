# Neutron Transport Monte Carlo 

![Rust Main](https://github.com/NielsBongers/neutron-transport-monte-carlo/actions/workflows/rust.yml/badge.svg?branch=main&event=push)

## Overview 

This is an energy-dependent neutron transport Monte Carlo simulation developed entirely in Rust, which can be used to design and simulate simple nuclear reactors. 

<img src="figures/03052024 - Neutron Monte Carlo - ParaView - k=1 reactor with control rods, stretched.png" width="600" alt="ParaView visualization of a reactor consisting of five 94% U-235 plates.">

## Features 

### Neutron multiplication factor 

Estimating the neutron multiplication factor $k$ is crucial. This is defined as: 

$k^{i+1} = \frac{n^{i+1}}{n^i}$ 

By simply tracking the number of neutrons in each generation, this can be estimated. The result is noisy, so it is averaged over a number of generations. For $k \gg 1$, a large neutron count cap should be used, to ensure a sufficient number of generations is simulated before the simulation halts. 

### Neutron flux and fission distribution 

If $k$ has been estimated and the geometry has been optimized to $k \approx 1$, or the $k$-estimate is not relevant, the maximum specified neutron count can be set to be enforced. This shuffles the vector containing the neutrons for the current generatino, then truncates it to the maximum neutron count, thus allowing for an extended simulation without infinitely increasing the memory usage. The distribution can be saved and later viewed with ParaView by enabling bin tracking and specifying a region to investigate. 

```TOML
track_bins = true 

[bin_parameters]
center = { x = 0.0, y = 0.0, z = 0.0 }
length_count = 200
depth_count = 200
height_count = 200
total_length = 2.0
total_depth = 2.0
total_height = 2.0
```

The simulation sums up all neutrons per bin, as well as all fissions per bin, allowing for heat generation modelling. An example with control rods optimized for $k = 1.005$ shows the flux is concentrated in the lower level of the reactor. 

<img src="figures/02052024 - Neutron Monte Carlo - ParaView - k=1.027 reactor with control rods at 1.70.png" width="400" alt="ParaView visualization of a reactor.">

### Materials 

Compound materials involving multiple nuclei (like water, or U-235/U-238) are implemented by looking at their relative total cross-sections and sampling based on that. In the top image, the light plates are 94% U-235, with the dark spots being the 6% U-238. 

### Moderation 

The simulation includes elastic scattering with general nuclei, reducing the neutron's energy. For the majority of nuclei, interaction cross-sections increase dramatically for neutrons with lower energies, so that neutron moderation is crucial to reactor operation. 

For a given scattering angle $\theta$, which is sampled isotropically, and nucleus mass $A$, the ratio of the final energy $E'$ to the incident energy $E$ is:[^1]

$\frac{E'}{E} = \frac{A^2+1+2A\cos \theta}{(A + 1)^2}$

Following a scattering event, the energy-dependent cross-sections for all materials are updated. Optionally, by setting the ```maximum_neutron_energy_difference``` parameter in the simulation's configuration file, this energy update can be skipped. This is relevant for simulations that only include heavy materials, where moderation is not relevant, so that the cost of updating the cross-sections can be avoided. 

### Constructive solid geometry 

To allow for more complex geometries, a basic form of [Constructive Solid Geometry](https://en.wikipedia.org/wiki/Constructive_solid_geometry) (CGS) has been implemented. This includes spheres, cuboids, and cylinders. The simulation automatically calculates bounding boxes for these as well. At every iteration, the neutron checks whether it is in the bounding box, and if so, if it is also in the specific geometry object. To achieve construction of different geometries, a specific order must be specified, with higher-order objects superceding lower-order objects, so that geometries can be added and subtracted. 

An order of -1 and lower is used for background materials (like water or shielding): these are ignored in the bounds-check that determines the maximum extent of the simulation, beyond which any neutrons are considered to have escaped and are discarded. 

As an example, the following TOML entry is for the center U-235/U-238 plate shown in the top image, showing its material composition, order, center position and dimensions. 

```toml 
[[cuboids]]
center = { x = 0.35, y = 0.0, z = 0.0 }
width = 0.020
depth = 0.50
height = 0.50
material_name = "U235"
material_composition_vector = [
  { material_name = "U238", material_fraction = 0.06 },
  { material_name = "U235", material_fraction = 0.94 },
]
order = 1
```

This method can be used to construct quite advanced geometries. The following is a top-down view of the reactor, with the steel power vessel and fuel supports, uranium fuel plates, and boron control rods extending down between the plates. 

<img src="figures/03052024 - Neutron Monte Carlo - nuclear reactor model geometry - top-down.png" width="800" alt="Example of constructive solid geometry.">

### Power estimation 

The simulation allows for removing any individual neutron reaching a certain specified total runtime. This results in a clear simulated runtime, which can be used for power estimation. 

As an example, the plate-reactor simulation was ran with plates of 2 cm thickness (giving $k \approx 1$), for a given timespan after which neutrons are discarded, and with a certain total number of neutrons/generation, with any excess also discarded. Summing up the total number of fissions occurring, converting that to total energy produced, and dividing by the total runtime gives an estimate of the power. 

Results here are as expected: doubling the number of neutrons doubles the power produced. For small runtimes, the power estimation is inaccurate, but with longer runtimes ($>5 \cdot 10^{-6}$, power estimates converge), likely because the neutron distribution has not fully equilibriated for shorter timespans. 

<img src="figures/07042024 - Neutron Monte Carlo - power estimates - different runtimes.png" width="400" alt="Neutron moderation by water.">

### Thermal behavior 

Reaching a stable equilibrium where an assembly has $k_\text{prompt} < 1$ and $k \approx 1$ is comparatively easy - a bisection search works there, for simple geometries. At that point, an arbitrary total power level/total number of neutrons can be specified, with no change in the neutron distribution. The main power generation constraint in the design of a reactor is then the heat developed in the fuel: this has to be evacuated into the coolant (generally water, optionally boiling, or various gases). If the fuel is too thick and the power level too high, the center of the fuel assemblies will start to melt and deform. 

To include this in the simulation as an optimization parameter, we use the results for the fission bins per fuel position to derive the heat generation per position at a given total number of neutrons. This can then be scaled to any number of neutrons or power level. That power per position can be used as a source term in a heat diffusion simulation, assuming that all energy from a fission is deposited locally.[^2] 

Rust does not currently have good support for sparse matrix solvers, so that large-scale heat diffusion is difficult to simulate - backward solvers like Crank-Nicolson are difficult to implement on a larger scale without sparse matrices. Therefore, instead, a simple forward method with the finite volume method is used, operating directly on the simulation's power bins. An example of a result of a section of plate, cooled by water, is shown below. This is for the reactor shown in the header image, which produces 123.3 kW. 

The mean temperature drops monotonically from the initially specified 600 K (intended to make plotting in ParaView easier), while the maximum in the center of the plates first increases, then decreases as it reaches an equilibrium. 

<img src="figures/29072024 - Neutron Monte Carlo - plate temperatures at around 100 kW.png" width="400" alt="Temperatures in the fuel plates.">

The associated temperature distribution for a cross-section of the plates is shown below, at a higher power level (a $10^6$ multiplier). Variations in the temperatures are caused by local fission intensity differences. 

<img src="figures/29072024 - Neutron Monte Carlo - Complex geometry heat diffusion.png" width="400" alt="Temperature distribution in the fuel plates.">

### ParaView visualization 

Results for geometry and neutron flux/fission bin counts are added to a CSV file with the format: 

```
x,y,z,count
-1.00000,-1.00000,-1.00000,0
-1.00000,-1.00000,-0.99000,0
```

This allows it to be loaded into [ParaView](https://www.paraview.org), and visualized using the filter "Table To Structured Grid" (accessible with ```Ctrl+Spacebar```). The Whole Extent in the filter should be set to ```length_count``` minus 1. Following this, the filter can be applied. Switching from Solid Color to "count" shows the flux per location. 

<img src="figures/15042024 - Neutron Monte Carlo - ParaView example guide loading CSVs.png" width="600" alt="Neutron moderation by water.">

Clipping the results, then using Threshold with logarithmic intervals has proven to work well for visualization and checking. 

To visualize geometry instead of neutron fluxes, essentially the same steps can be taken but with ```results/geometry/geometry.csv```. At that point, setting "Interpret Values as Categories" in the Color Map Editor makes the results easy to interpret, especially in combination with the use of the Threshold filter to only show certain materials. In generating these geometries, a 1 MeV neutron is placed at each location, and the materials are sampled. The geometry is therefore partially stochastic and energy-dependent. 

## Installation 

Rust makes installation easy: clone the repository, enter the folder, and execute ```cargo run```, which will install all the necessary crates and run in debug mode. After installation, ensure everything works correctly with ```cargo test```, then proceed with ```cargo run --release``` to execute in release mode, which is _significantly_ faster than debug mode, but does not include various safeguards. By the default, the run result should look something like this. 

```
=== Simulation completed ===

    - Results -
  Duration:                             00:00:42.083 hh:mm:ss:ms
  Maximum generation value:                      101
  Averaged k:                                  1.003
  Initial neutron count:                        1000
  Total neutrons:                             458137
  Total fissions:                               8312
  Total energy produced:                 0.000000258 J
  Power:                                       0.026 W
  Halt cause: Generation cap.

  - Settings -
  Track creation:                               true
  Track positions:                             false
  Track energies:                              false
  Track bins:                                   true
```

## Getting started 

The easiest option to get started is to experiment with the configuration files under ```config/simulation/default.toml``` and ```config/geometries/plate_reactor.toml```. Create a new geometry, enable ```plot_geometry```, then load the resulting ```results/geometry/geometry.csv``` data into ParaView using the method outlined above. Then, iteratively modify the geometry and try different simulation settings. 

To gather more data, modify the files under ```src/diagnostics``` and in ```src/simulation/simulation.rs```: the entire simulation loop is created from there. Adding more fields to the simulation struct and tracking those in the main loop is easy. 

Additional energy-dependent material data can be loaded in from ENDFs. Details on this can be found under a [different repository](https://github.com/NielsBongers/endf-handling), created specifically for this project.

## Updates 

### 28-07-2024 - Major changes in heat diffusion code 

The heat diffusion code was completely reworked, resulting in massive speed-ups, and overall far more usable code. The new system works by creating an aggregated CSV file with fission locations, which is then loaded in by the heat diffusion code, distributed over bins, and used as source terms. This is far more flexible and modular than the previous approach where the fission bins were used directly, and boundary conditions determined using those. 

Along side these changes, a lot of the other code was also modified to be easier to use and more efficient, including the general configuration file layout, the multithreaded parts, file writing etc. 

### 03-11-2024 - Variance reduction and convergence analysis 

I have added basic convergence analysis using the neutron position bins. As a metric, I am using a form of mean relative difference between timesteps: 

$\frac{1}{N} \sum_{i=1}^N \left| \frac{n_i^t}{n_T^t} - \frac{n_i^{t-1}}{n_T^{t-1}} \right|$

This normalizes the neutrons per bin $i$ for a timestep $t$ and $t+1$ over the total number of neutrons in the vector at those timesteps, so $n_T^t$ and $n_T^{t+1}$, respectively. 

This gives very reasonable results: we converge at a rate of around -1.14; per order of magnitude of neutron generations, we decrease our convergence measure by a bit more than an order of magnitude. 

<img src="figures/03112024 - Neutron Monte Carlo - convergence analysis - basic reactor.png" width="400" alt="Convergence analysis.">

Aside from this, I have extended the code with basic variance reduction. There are now options to remove a randomly sampled number of neutrons if the population exceeds a set number, or to sample from the existing distribution to instead add more neutrons. 

```
# Variance reduction 
variance_reduction = true                             # Resets the neutron count to specified_neutron_count each generation by removing or sampling. 
specified_neutron_count = 10000                       # Target neutron count. 
```

This allows estimation of distributions for geometries where $k \neq 1$, and which therefore either exponentially increases or decreases. 

## Future development 

### Flux through shapes 

The diagnostics-code will be extended at some point by adding user-defined geometries (like planes, spheres etc.) with neutron flux through these tallied. This could be relevant for shielding studies and radiation leakage. 

### Statistics 

Currently, the software allows for exporting all neutron data, or manually saving multiple runs and post-processing these. In the future, support for simple statistical tests may be added, to estimate whether changing a certain parameter is statistically relevant. 

### Optimization 

One of the goals of the simulation is to design a simple reactor, including power and heat generation. To allow for easier design, a simple set of optimization tools could be added later, perhaps based on gradient-based methods, differential evolution, or perhaps Nelder-Mead, in order to optimize different parameters like the number of fuel units, dimensions, locations etc. 

## Examples 

### Godiva validation 

As a validation case, results from the [Godiva-device](https://en.wikipedia.org/wiki/Godiva_device) were taken. According to Burgio2004 ([Time resolved MCNP neutron transport simulation on multiplying media: GODIVA benchmark](https://indico.ictp.it/event/a0335/session/147/contribution/87/material/0/0.pdf)), a spherical mass of 94\% U-235/6\% U-238 reaches criticality (that is, $k=1$) at $r = 0.087037$. This case is used as an integration test to ensure accurate results. 

As can be seen in the following figure, we initially have a spike in the estimated $k$, because the simulation starts, by default, with 100 initial neutrons at $r = 0$, and because of the low mean-free path $\lambda$, it takes several generations for the neutrons to diffuse outwards - therefore, initially, $k$ is overestimated. However, the results rapidly converge to the correct results. 

The results here show an increase in variance as the number of generations increases: each of these simulations was ran with a maximum of $N = 20.000$ neutrons, with the simulation halting if this value is hit. This maximum can be increased at will. The more generations, the fewer simulations remained without halting, resulting in increased variance. To reach accurate results, a limited number of generations can be used, or simulations can be cut-off based on simulated time instead. 

If enabled, all neutron counts per generation are saved under ```/results/diagnostics``` in a specific run directory, as ```generation_counts.csv```, which can be analyzed further. 

<img src="figures/04022024 - Neutron Monte Carlo - k estimates with confidence intervals, corrected - Godiva.png" width="600" alt="Godiva validation.">

### Spherical assemblies 

A more general simulation showcasing the increase in $k$ for increasingly large radii, with $k \approx 2.5$ for $r \to \infty$. This is used as another validation test: in an infinite medium without any escaping neutrons, the estimated $k$ should be within 5% of $k = 2.5$ for the test to pass. 

<img src="figures/04022024 - Neutron Monte Carlo - k estimate per radius.png" width="350" alt="Neutron moderation by water.">

### Plate reactor 

A reactor consisting of five plates of equal thickness, and $0.5 \times 0.5$ m in height/width is used as a basic showcase, resulting in the neutron flux distributions and material compositions shown in the title image. The dark spots on the fuel plates there is U-238, with the lighter red being U-235. The following figure gives the locations of fissions in the reactor following a run. It is clear that the central plates have a large number of fissions, with the exterior plates almost nothing. This corresponds with the fluxes that can be observed in ParaView. 

<img src="figures/01042024 - Neutron Monte Carlo - ParaView - 3.8 cm plates - with water and geometry - fission locations.png" width="500" alt="Neutron moderation by water.">

### Detailed constructive solid geometry 

By populating TOML files with Python, complex assemblies can be generated easily. The following is a number of densely packed fuel plates with control rods interspersed. 

<img src="figures/02052024 - Neutron Monte Carlo - ParaView - geometry attempt with lots of plates.png" width="500" alt="Neutron moderation by water.">

## Motivation

I've always been curious about physics and engineering, and especially simulations. I went through Lamarsh's _Introduction to Nuclear Engineering_ several years ago, and after reading about the [origins of the Monte Carlo method](https://en.wikipedia.org/wiki/Monte_Carlo_method#History), I implemented some toy examples for $k$-estimation in Python, then several more extensive examples in C++. I completed my Masters in Chemical Process Engineering with a thesis based around Gibbs minimization for gas phase modelling in a transient high-temperature reactor, in C++ too, and started working in a start-up. In my spare time, I worked on CDF, wrote a [SIMPLE-implementation](https://github.com/NielsBongers/SIMPLE-CFD), then continued with OpenFOAM instead, but continued to be curious about going back to nuclear reactor modelling at some point. 

After noticing how pleasant Rust is in a [first small project](https://github.com/NielsBongers/rust-orbital-debris), I decided to come back and implement the Monte Carlo properly in November 2023, and have been working on it during weekends since then. This is just a hobby project, and there are definitely errors and inefficiencies. Don't use it for anything relevant. 

[^1]: Stacey, _Nuclear Reactor Physics_, 2nd edition, Wiley-VCH, 2007, eq. 1.19, and Duderstadt, _Nuclear Reactor Analysis_, Wiley, 1976, eq. 2.65. 

[^2]: This is not entirely accurate: around 93% of the energy (ignoring neutrinos) is deposited locally, largely through fragments, but the remainder is released as γ particles. That is too difficult to include in the simulation (for now), so that this assumption is made as a form of worst-case scenario for heat generation. 
