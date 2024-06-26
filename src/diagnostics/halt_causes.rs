use std::fmt;

#[derive(Default)]
pub enum SimulationHaltCauses {
    #[default]
    NotHalted,
    HitNeutronCap,
    HitGenerationCap,
    NoNeutrons,
    HitFissionCap,
}

impl fmt::Display for SimulationHaltCauses {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SimulationHaltCauses::NotHalted => write!(f, "Simulation did not halt."),
            SimulationHaltCauses::HitNeutronCap => {
                write!(f, "Neutron cap.")
            }
            SimulationHaltCauses::HitGenerationCap => {
                write!(f, "Generation cap.")
            }
            SimulationHaltCauses::NoNeutrons => {
                write!(f, "No neutrons.")
            }
            SimulationHaltCauses::HitFissionCap => {
                write!(f, "Fission cap.")
            }
        }
    }
}
