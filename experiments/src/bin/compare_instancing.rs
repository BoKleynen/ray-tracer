use cg_practicum::bvh::AxisSelection::*;
use cg_practicum::bvh::SplittingHeuristic::*;
use cg_practicum::bvh::{SplittingConfig, Z_AXIS};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let splitting_configs = [
        SplittingConfig {
            splitting_heuristic: SurfaceAreaHeuristic(12),
            axis_selection: Alternate(Z_AXIS),
        },
        SplittingConfig {
            splitting_heuristic: ObjectMedianSplit,
            axis_selection: Alternate(Z_AXIS),
        },
        SplittingConfig {
            splitting_heuristic: SpaceMedianSplit,
            axis_selection: Alternate(Z_AXIS),
        },
        SplittingConfig {
            splitting_heuristic: SpaceAverageSplit,
            axis_selection: Alternate(Z_AXIS),
        },
    ];

    Ok(())
}
