use approx::assert_abs_diff_eq;

#[test]
fn visualization_exports_plot_ready_data() {
    let function = |x: f64| x * x;
    let lines = numtan::core::visualize::tangent_lines(&function, 0.0, 2.0, &[1.0]);
    let field = numtan::core::visualize::direction_field(&|_x, y| y, (0.0, 1.0), (0.0, 1.0), 2, 3);
    let animation =
        numtan::core::visualize::newton_animation(&|x| x * x - 2.0, 1.0, 1e-10, 20).unwrap();
    assert_eq!(lines.len(), 1);
    assert_eq!(field.len(), 6);
    assert!(!animation.is_empty());
    assert_abs_diff_eq!(lines[0].slope, 2.0, epsilon = 1e-5);
}

#[test]
fn statistics_and_regression_are_consistent() {
    let values = [1.0, 2.0, 3.0, 4.0];
    let summary = numtan::core::stats::summary(&values).unwrap();
    let regression =
        numtan::core::stats::linear_regression(&[1.0, 2.0, 3.0], &[3.0, 5.0, 7.0]).unwrap();
    let coefficients =
        numtan::core::stats::polynomial_regression(&[0.0, 1.0, 2.0], &[1.0, 3.0, 7.0], 2).unwrap();
    assert_abs_diff_eq!(summary.mean, 2.5, epsilon = 1e-12);
    assert_abs_diff_eq!(summary.variance, 1.25, epsilon = 1e-12);
    assert_abs_diff_eq!(regression.slope, 2.0, epsilon = 1e-12);
    assert_abs_diff_eq!(regression.intercept, 1.0, epsilon = 1e-12);
    assert_abs_diff_eq!(coefficients[0], 1.0, epsilon = 1e-10);
    assert_abs_diff_eq!(coefficients[1], 1.0, epsilon = 1e-10);
    assert_abs_diff_eq!(coefficients[2], 1.0, epsilon = 1e-10);
}

#[test]
fn interpolation_and_grid_helpers_work() {
    let grid = numtan::core::interpolate::sample_grid(&|x| x * x, 0.0, 2.0, 3);
    let linear =
        numtan::core::interpolate::linear_interpolate(&[0.0, 1.0], &[0.0, 2.0], 0.25).unwrap();
    let lagrange =
        numtan::core::interpolate::lagrange_interpolate(&[0.0, 1.0, 2.0], &[1.0, 2.0, 5.0], 1.5)
            .unwrap();
    let diff = numtan::core::interpolate::finite_difference(&[0.0, 1.0, 4.0], 1.0).unwrap();
    assert_eq!(grid.len(), 3);
    assert_abs_diff_eq!(linear, 0.5, epsilon = 1e-12);
    assert_abs_diff_eq!(lagrange, 3.25, epsilon = 1e-12);
    assert_abs_diff_eq!(diff[1], 2.0, epsilon = 1e-12);
}

#[test]
fn polynomial_tools_cover_basic_algebra() {
    let coefficients = [1.0, 0.0, 1.0];
    assert_abs_diff_eq!(
        numtan::core::polynomial::evaluate(&coefficients, 2.0),
        5.0,
        epsilon = 1e-12
    );
    assert_eq!(
        numtan::core::polynomial::derivative(&coefficients),
        vec![0.0, 2.0]
    );
    assert_eq!(
        numtan::core::polynomial::add(&[1.0, 2.0], &[3.0]),
        vec![4.0, 2.0]
    );
    assert_eq!(
        numtan::core::polynomial::multiply(&[1.0, 1.0], &[1.0, -1.0]),
        vec![1.0, 0.0, -1.0]
    );
    let root = numtan::core::polynomial::root(&[-4.0, 0.0, 1.0], 3.0, 1e-10, 50).unwrap();
    assert_abs_diff_eq!(root, 2.0, epsilon = 1e-8);
}

#[test]
fn signal_helpers_smooth_convolve_and_detect_peaks() {
    let moving = numtan::core::signal::moving_average(&[1.0, 3.0, 5.0], 2).unwrap();
    let smooth = numtan::core::signal::exponential_smooth(&[0.0, 10.0], 0.5).unwrap();
    let conv = numtan::core::signal::convolve(&[1.0, 2.0], &[1.0, 1.0]).unwrap();
    let normalized = numtan::core::signal::normalize(&[2.0, 4.0, 6.0]).unwrap();
    let peaks = numtan::core::signal::find_peaks(&[0.0, 2.0, 1.0, 3.0, 0.0], 1.5);
    assert_eq!(moving, vec![1.0, 2.0, 4.0]);
    assert_eq!(smooth, vec![0.0, 5.0]);
    assert_eq!(conv, vec![1.0, 3.0, 2.0]);
    assert_eq!(normalized, vec![0.0, 0.5, 1.0]);
    assert_eq!(peaks.len(), 2);
}
