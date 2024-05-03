use log::warn;

/// Struct used for binary search. Contains information on the value obtained and the convergence.
pub struct BinarySearchResult {
    pub index: usize,
    pub value: f64,
    pub is_exact: bool,
    pub num_iterations: usize,
}

impl std::fmt::Display for BinarySearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Binary search result:\n\tindex: {}\n\tvalue: {}\n\tis_exact: {}\n\tnum_iterations: {}\n",
            self.index, self.value, self.is_exact, self.num_iterations
        )
    }
}

/// Performs binary search based on an input vector and a target value, and returns an instance of ```BinarySearchResult```.
pub fn binary_search(input_vector: &Vec<f64>, target_value: f64) -> Option<BinarySearchResult> {
    let vector_length = input_vector.len();

    let mut left_index = 0 as usize;
    let mut right_index = vector_length;

    let max_iterations = 100;

    for (iteration_counter, _) in (0..max_iterations).enumerate() {
        let center_index = (right_index + left_index) / 2;
        let center_value = input_vector[center_index];

        if target_value == center_value {
            let successful_convergence = BinarySearchResult {
                index: center_index as usize,
                value: center_value,
                is_exact: true,
                num_iterations: iteration_counter,
            };
            return Some(successful_convergence);
        }
        if left_index == center_index {
            let successful_convergence = BinarySearchResult {
                index: center_index as usize,
                value: center_value,
                is_exact: false,
                num_iterations: iteration_counter,
            };
            return Some(successful_convergence);
        }

        if target_value < center_value {
            right_index = center_index;
        }
        if target_value > center_value {
            left_index = center_index
        }
    }
    None
}

/// Interpolates values for the linear interpolation function.
pub fn interpolate_values(
    left_x: f64,
    right_x: f64,
    left_y: f64,
    right_y: f64,
    target_x: f64,
) -> f64 {
    let derivative = (right_y - left_y) / (right_x - left_x);
    let distance = target_x - left_x;
    let target_y = left_y + derivative * distance;
    target_y
}

/// Linearly interpolates two vectors using binary search.
pub fn linear_interpolation(
    x_vector: &Vec<f64>,
    y_vector: &Vec<f64>,
    target_x: f64,
) -> (f64, BinarySearchResult) {
    // Skipping the binary search in case there is no data.
    // This occurs if a material isn't fissionable, for example - we just set it to have a vector with 0.0 then.
    if y_vector.len() == 1 {
        let dummy_binary_search_result = BinarySearchResult {
            index: 0 as usize,
            value: 0.0,
            is_exact: false,
            num_iterations: 0 as usize,
        };
        return (0.0, dummy_binary_search_result);
    }

    let binary_search_result =
        binary_search(&x_vector, target_x).expect("Binary search failed to converge.");

    if binary_search_result.is_exact {
        return (y_vector[binary_search_result.index], binary_search_result);
    }

    if binary_search_result.index == x_vector.len() - 1 {
        warn!("Value is outside range - returning maximum value.");
        return (y_vector[binary_search_result.index], binary_search_result);
    }

    let target_index = binary_search_result.index;

    let left_x = x_vector[target_index];
    let right_x = x_vector[target_index + 1];

    let left_y = y_vector[target_index];
    let right_y = y_vector[target_index + 1];

    let target_y = interpolate_values(left_x, right_x, left_y, right_y, target_x);
    (target_y, binary_search_result)
}

/// Returns Watt parameters through linear interpolation. Finds the index of the _a_ parameter and uses that to also get the _b_ parameter without running linear interpolation twice.
pub fn get_watt_parameters(
    energy_vector: &Vec<f64>,
    a_vector: &Vec<f64>,
    b_vector: &Vec<f64>,
    energy: f64,
) -> (f64, f64) {
    let mev_energy = energy / 1e6;

    let (a_value, binary_search_result) =
        linear_interpolation(&energy_vector, &a_vector, mev_energy);

    let target_index = binary_search_result.index;

    if target_index + 1 >= energy_vector.len() {
        return (a_value, b_vector[target_index]);
    }

    let left_x = energy_vector[target_index];
    let right_x = energy_vector[target_index + 1];

    let left_b = b_vector[target_index];
    let right_b = b_vector[target_index + 1];

    let b_value = interpolate_values(left_x, right_x, left_b, right_b, mev_energy);

    (a_value, b_value)
}
