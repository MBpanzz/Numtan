use approx::assert_abs_diff_eq;

#[test]
fn tangent_matches_polynomial_derivative() {
    let function = |x: f64| x.powi(3);
    assert_abs_diff_eq!(
        numtan::core::derivative::tangent(&function, 2.0),
        12.0,
        epsilon = 1e-5
    );
}

#[test]
fn gradient_matches_quadratic_surface() {
    let function = |point: &[f64]| point[0] * point[0] + 3.0 * point[1] * point[1];
    let gradient = numtan::core::derivative::gradient(&function, &[2.0, 1.0]);
    assert_abs_diff_eq!(gradient[0], 4.0, epsilon = 1e-5);
    assert_abs_diff_eq!(gradient[1], 6.0, epsilon = 1e-5);
}

#[test]
fn newton_finds_square_root_two() {
    let function = |x: f64| x * x - 2.0;
    let result = numtan::core::root::newton(&function, 1.0, 1e-10, 50).unwrap();
    assert!(result.converged);
    assert_abs_diff_eq!(result.root, 2.0_f64.sqrt(), epsilon = 1e-8);
}

#[test]
fn halley_and_householder_find_root() {
    let function = |x: f64| x * x * x - 8.0;
    let halley = numtan::core::root::halley(&function, 1.0, 1e-10, 50).unwrap();
    let householder = numtan::core::root::householder(&function, 1.0, 3, 1e-10, 50).unwrap();
    assert!(halley.converged);
    assert!(householder.converged);
    assert_abs_diff_eq!(halley.root, 2.0, epsilon = 1e-8);
    assert_abs_diff_eq!(householder.root, 2.0, epsilon = 1e-8);
}

#[test]
fn linalg_solves_small_system() {
    let dot = numtan::core::linalg::dot(&[1.0, 2.0], &[3.0, 4.0]).unwrap();
    let product =
        numtan::core::linalg::mat_vec(&[vec![1.0, 2.0], vec![3.0, 4.0]], &[1.0, 1.0]).unwrap();
    let solution =
        numtan::core::linalg::solve(vec![vec![2.0, 1.0], vec![1.0, 3.0]], vec![1.0, 2.0]).unwrap();
    assert_abs_diff_eq!(dot, 11.0, epsilon = 1e-12);
    assert_abs_diff_eq!(product[0], 3.0, epsilon = 1e-12);
    assert_abs_diff_eq!(product[1], 7.0, epsilon = 1e-12);
    assert_abs_diff_eq!(solution[0], 0.2, epsilon = 1e-12);
    assert_abs_diff_eq!(solution[1], 0.6, epsilon = 1e-12);
}

#[test]
fn statistics_reject_undefined_constant_vector_operations() {
    assert!(numtan::core::stats::correlation(&[1.0, 1.0], &[2.0, 3.0]).is_err());
    assert!(numtan::core::stats::linear_regression(&[1.0, 1.0], &[2.0, 3.0]).is_err());
}

#[test]
fn optimizers_reach_simple_minimum() {
    let scalar = |x: f64| (x - 3.0).powi(2);
    let vector = |point: &[f64]| (point[0] - 2.0).powi(2) + (point[1] + 1.0).powi(2);
    let tangent = numtan::core::optimize::tangent_minimize(&scalar, 0.0, 0.1, 1e-7, 1000);
    let gradient = numtan::core::optimize::gradient_descent(&vector, &[0.0, 0.0], 0.1, 1e-7, 1000);
    assert!(tangent.converged);
    assert!(gradient.converged);
    assert_abs_diff_eq!(tangent.point[0], 3.0, epsilon = 1e-5);
    assert_abs_diff_eq!(gradient.point[0], 2.0, epsilon = 1e-5);
    assert_abs_diff_eq!(gradient.point[1], -1.0, epsilon = 1e-5);
}

#[test]
fn rk4_solves_exponential_growth() {
    let function = |_t: f64, y: f64| y;
    let points = numtan::core::ode::rk4(&function, 1.0, 0.0, 1.0, 0.05);
    let last = points.last().unwrap();
    assert_abs_diff_eq!(last.y, std::f64::consts::E, epsilon = 1e-6);
}

#[test]
fn euler_midpoint_and_adaptive_rk4_return_valid_paths() {
    let function = |_t: f64, y: f64| y;
    let euler = numtan::core::ode::euler(&function, 1.0, 0.0, 0.2, 0.1);
    let midpoint = numtan::core::ode::midpoint(&function, 1.0, 0.0, 0.2, 0.1);
    let adaptive = numtan::core::ode::adaptive_rk4(&function, 1.0, 0.0, 1.0, 0.2, 1e-8);
    assert_eq!(euler.len(), 3);
    assert_eq!(midpoint.len(), 3);
    assert_abs_diff_eq!(
        adaptive.last().unwrap().y,
        std::f64::consts::E,
        epsilon = 1e-6
    );
}

#[test]
fn tanh_sinh_integrates_unit_interval() {
    let function = |x: f64| x * x;
    let result = numtan::core::integrate::tanh_sinh(&function, 0.0, 1.0, 1e-10, 12);
    assert_abs_diff_eq!(result.value, 1.0 / 3.0, epsilon = 1e-8);
}

#[test]
fn infinite_quadrature_handles_gaussian() {
    let function = |x: f64| (-x * x).exp();
    let result = numtan::core::integrate::quad_inf(&function, 1e-8, 12);
    assert_abs_diff_eq!(result.value, std::f64::consts::PI.sqrt(), epsilon = 1e-6);
}

#[test]
fn trig_helpers_match_standard_values() {
    assert_abs_diff_eq!(numtan::core::trig::tanpi(0.25), 1.0, epsilon = 1e-12);
    assert_abs_diff_eq!(numtan::core::trig::tan_deg(45.0), 1.0, epsilon = 1e-12);
    assert_abs_diff_eq!(numtan::core::trig::tan_grad(50.0), 1.0, epsilon = 1e-12);
}

#[test]
fn trigonometric_integrals_and_complex_tan_are_finite() {
    let atanint = numtan::core::trig::atanint(1.0);
    let tanint = numtan::core::trig::tanint(0.5);
    let complex = numtan::core::trig::complex_tan(0.5, 0.25);
    assert!(atanint.is_finite());
    assert!(tanint.is_finite());
    assert!(complex.0.is_finite());
    assert!(complex.1.is_finite());
}
