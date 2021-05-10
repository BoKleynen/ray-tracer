macro_rules! space_median_split_impl {
    ($fdir: ident, $dir: ident) => {
        fn $fdir(shapes: Vec<S>) -> (Vec<S>, Vec<S>) {
            let split = shapes
                .iter()
                .map(|sample| sample.bbox().centroid().$dir)
                .sum::<f64>()
                / shapes.len() as f64;
            shapes
                .into_iter()
                .partition(|shape| shape.bbox().centroid().$dir < split)
        }
    };
}
