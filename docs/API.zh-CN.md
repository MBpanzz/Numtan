# NumTan API 鍙傝€?
鏈枃妗ｅ垪鍑?NumTan `0.9.2` 瀵煎嚭鐨?Python 鍏叡鍑芥暟銆?
## 姹傚 API

- `tangent(function, x, method="central") -> float`
- `gradient(function, point) -> list[float]`

## 姹傛牴 API

- `newton(function, x0, tol=1e-10, max_iter=50) -> dict`
- `halley(function, x0, tol=1e-10, max_iter=50) -> dict`
- `householder(function, x0, order=3, tol=1e-10, max_iter=50) -> dict`

姹傛牴缁撴灉瀛楀吀鍖呭惈锛?
- `root`锛氭渶缁堟牴浼拌鍊?- `converged`锛氭槸鍚︽敹鏁?- `iterations`锛氳凯浠ｆ鏁?- `history`锛氭瘡涓€姝ヨ凯浠ｆ暟鎹?
## 绾挎€т唬鏁?API

- `dot(left, right) -> float`
- `norm(vector) -> float`
- `add(left, right) -> list[float]`
- `sub(left, right) -> list[float]`
- `scale(vector, factor) -> list[float]`
- `mat_vec(matrix, vector) -> list[float]`
- `solve(matrix, rhs) -> list[float]`

## 浼樺寲 API

- `gradient_descent(function, start, step_size=0.01, tol=1e-8, max_iter=1000) -> dict`
- `tangent_minimize(function, start, step_size=0.01, tol=1e-8, max_iter=1000) -> dict`
- `stationary_newton(function, start, tol=1e-8, max_iter=100) -> dict`
- `gauss_newton(function, start, tol=1e-8, max_iter=100) -> dict`

浼樺寲缁撴灉瀛楀吀鍖呭惈锛?
- `point`锛氭渶缁堢偣
- `value`锛氭渶缁堢洰鏍囧嚱鏁板€?- `converged`锛氭槸鍚︽敹鏁?- `iterations`锛氳凯浠ｆ鏁?- `history`锛氭瘡涓€姝ヨ凯浠ｆ暟鎹?
## ODE API

- `euler(function, y0, t0, t1, step) -> list[dict]`
- `midpoint(function, y0, t0, t1, step) -> list[dict]`
- `rk4(function, y0, t0, t1, step) -> list[dict]`
- `adaptive_rk4(function, y0, t0, t1, initial_step, tol=1e-8) -> list[dict]`

ODE 杞ㄨ抗瀛楀吀鍖呭惈 `t` 鍜?`y`銆?
## 绉垎 API

- `tanh_sinh(function, a, b, tol=1e-10, max_levels=12) -> dict`
- `tan_sinh(function, a=0.0, tol=1e-10, max_levels=12) -> dict`
- `quad_inf(function, tol=1e-10, max_levels=12) -> dict`

绉垎缁撴灉瀛楀吀鍖呭惈 `value`銆乣error` 鍜?`levels`銆?
## 涓夎鎵╁睍 API

- `tanpi(x) -> float`
- `tanint(x) -> float`
- `atanint(x) -> float`
- `complex_tan(re, im) -> tuple[float, float]`
- `tan_deg(x) -> float`
- `tan_grad(x) -> float`

## 鍙鍖?API

- `tangent_lines(function, x_start, x_end, centers) -> list[dict]`
- `newton_animation_data(function, x0, tol=1e-10, max_iter=50) -> list[dict]`
- `ode_direction_field(function, x_range, y_range, x_count, y_count) -> list[dict]`

绾挎瀛楀吀鍖呭惈 `x_start`銆乣y_start`銆乣x_end`銆乣y_end`銆乣center` 鍜?`slope`銆?
鏂瑰悜鍦哄瓧鍏稿寘鍚?`x`銆乣y`銆乣dx` 鍜?`dy`銆?
## 缁熻 API

- `mean(values) -> float`
- `variance(values, sample=False) -> float`
- `summary(values) -> dict`
- `covariance(left, right, sample=False) -> float`
- `correlation(left, right) -> float`
- `linear_regression(x, y) -> dict`
- `polynomial_regression(x, y, degree) -> list[float]`

## 鎻掑€?API

- `linspace(start, end, count) -> list[float]`
- `sample_grid(function, start, end, count) -> list[dict]`
- `linear_interpolate(x, y, query) -> float`
- `lagrange_interpolate(x, y, query) -> float`
- `finite_difference(values, spacing) -> list[float]`

## 澶氶」寮?API

- `polyval(coefficients, x) -> float`
- `polyder(coefficients) -> list[float]`
- `polyint(coefficients, constant=0.0) -> list[float]`
- `polyadd(left, right) -> list[float]`
- `polymul(left, right) -> list[float]`
- `polyroot(coefficients, guess, tol=1e-10, max_iter=50) -> float`

## 淇″彿澶勭悊 API

- `moving_average(values, window) -> list[float]`
- `exponential_smooth(values, alpha) -> list[float]`
- `convolve(signal, kernel) -> list[float]`
- `normalize(values) -> list[float]`
- `find_peaks(values, threshold) -> list[dict]`

宄板€煎瓧鍏稿寘鍚?`index` 鍜?`value`銆?