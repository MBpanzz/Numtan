# NumTan 浣跨敤鎸囧崡

鏈枃妗ｄ粙缁?NumTan `0.9.2` 鐨勪富瑕?Python 浣跨敤鏂瑰紡銆?
## 瀵煎叆

```python
import math
import numtan as nt
```

## 鏁板€兼眰瀵?
浣跨敤 `tangent` 浼拌涓€鍏冨嚱鏁板湪鏌愪竴鐐圭殑瀵兼暟锛?
```python
slope = nt.tangent(lambda x: x**3, 2.0)
print(slope)
```

浣跨敤 `gradient` 浼拌澶氬厓鏍囬噺鍑芥暟鐨勬搴︼細

```python
grad = nt.gradient(lambda v: v[0] ** 2 + 3.0 * v[1] ** 2, [2.0, 1.0])
print(grad)
```

## 鏂圭▼姹傛牴

姹傛牴鍑芥暟杩斿洖瀛楀吀锛屽寘鍚?`root`銆乣converged`銆乣iterations` 鍜?`history`銆?
```python
result = nt.newton(lambda x: x * x - 2.0, 1.0)
print(result["root"])
print(result["history"][0])
```

Halley 鍜?Householder 閫傚悎鍏夋粦鍑芥暟鐨勫眬閮ㄥ揩閫熸敹鏁涳細

```python
print(nt.halley(lambda x: x**3 - 8.0, 1.0)["root"])
print(nt.householder(lambda x: x**3 - 8.0, 1.0, 3)["root"])
```

## 绾挎€т唬鏁?
鍚戦噺浣跨敤 Python 鍒楄〃浼犲叆锛岀煩闃典娇鐢ㄨ涓诲簭浜岀淮鍒楄〃銆?
```python
print(nt.dot([1.0, 2.0], [3.0, 4.0]))
print(nt.norm([3.0, 4.0]))
print(nt.mat_vec([[1.0, 2.0], [3.0, 4.0]], [1.0, 1.0]))
print(nt.solve([[2.0, 1.0], [1.0, 3.0]], [1.0, 2.0]))
```

## 浼樺寲

浼樺寲鍑芥暟杩斿洖瀛楀吀锛屽寘鍚?`point`銆乣value`銆乣converged`銆乣iterations` 鍜?`history`銆?
```python
result = nt.gradient_descent(
    lambda v: (v[0] - 2.0) ** 2 + (v[1] + 1.0) ** 2,
    [0.0, 0.0],
    step_size=0.01,
)
print(result["point"])
```

鏍囬噺鏈€灏忓寲鍜岄┗鐐规悳绱細

```python
print(nt.tangent_minimize(lambda x: (x - 3.0) ** 2, 0.0)["point"])
print(nt.stationary_newton(lambda x: (x - 4.0) ** 2, 0.0)["point"])
```

Gauss-Newton 鐢ㄤ簬闈炵嚎鎬ф渶灏忎簩涔樻畫宸郴缁燂細

```python
print(nt.gauss_newton(lambda v: [v[0] - 2.0, v[1] + 3.0], [0.0, 0.0])["point"])
```

## 甯稿井鍒嗘柟绋?
ODE 姹傝В鍣ㄦ帴鏀?`f(t, y)`锛岃繑鍥炵敱 `t` 鍜?`y` 瀛楀吀缁勬垚鐨勫垪琛ㄣ€?
```python
path = nt.rk4(lambda t, y: y, 1.0, 0.0, 1.0, 0.05)
print(path[-1])
```

鍙敤姝ヨ繘鍣細

```python
nt.euler(lambda t, y: y, 1.0, 0.0, 1.0, 0.1)
nt.midpoint(lambda t, y: y, 1.0, 0.0, 1.0, 0.1)
nt.rk4(lambda t, y: y, 1.0, 0.0, 1.0, 0.1)
nt.adaptive_rk4(lambda t, y: y, 1.0, 0.0, 1.0, 0.2)
```

## 鏁板€肩Н鍒?
绉垎鍑芥暟杩斿洖瀛楀吀锛屽寘鍚?`value`銆乣error` 鍜?`levels`銆?
```python
finite = nt.tanh_sinh(lambda x: x * x, 0.0, 1.0)
gaussian = nt.quad_inf(lambda x: math.exp(-x * x))
print(finite["value"], gaussian["value"])
```

`tan_sinh` 鐢ㄤ簬浠?`a` 鍒版鏃犵┓鐨勫崐鏃犵┓鍖洪棿锛?
```python
print(nt.tan_sinh(lambda x: math.exp(-x), 0.0)["value"])
```

## 姝ｅ垏涓庝笁瑙掓墿灞?
```python
print(nt.tanpi(0.25))
print(nt.tan_deg(45.0))
print(nt.tan_grad(50.0))
print(nt.tanint(0.5))
print(nt.atanint(1.0))
print(nt.complex_tan(0.5, 0.25))
```

## 鍙鍖栨暟鎹?
NumTan 杩斿洖鏅€?Python 鏁版嵁锛屽彲浜ょ粰浠绘剰缁樺浘搴撲娇鐢ㄣ€?
```python
lines = nt.tangent_lines(lambda x: x * x, 0.0, 2.0, [0.5, 1.0, 1.5])
animation = nt.newton_animation_data(lambda x: x * x - 2.0, 1.0)
field = nt.ode_direction_field(lambda t, y: y, (0.0, 1.0), (0.0, 2.0), 8, 8)
```

## 缁熻涓庡洖褰?
```python
print(nt.summary([1.0, 2.0, 3.0, 4.0]))
print(nt.correlation([1.0, 2.0, 3.0], [2.0, 4.0, 6.0]))
print(nt.linear_regression([1.0, 2.0, 3.0], [3.0, 5.0, 7.0]))
print(nt.polynomial_regression([0.0, 1.0, 2.0], [1.0, 3.0, 7.0], 2))
```

## 鎻掑€间笌缃戞牸

```python
print(nt.linspace(0.0, 1.0, 5))
print(nt.sample_grid(lambda x: x * x, 0.0, 1.0, 5))
print(nt.linear_interpolate([0.0, 1.0], [0.0, 2.0], 0.25))
print(nt.lagrange_interpolate([0.0, 1.0, 2.0], [1.0, 2.0, 5.0], 1.5))
print(nt.finite_difference([0.0, 1.0, 4.0], 1.0))
```

## 澶氶」寮忓伐鍏?
澶氶」寮忕郴鏁版寜鍗囧箓鎺掑垪銆備緥濡?`[1.0, 0.0, 1.0]` 琛ㄧず `1 + x^2`銆?
```python
print(nt.polyval([1.0, 0.0, 1.0], 2.0))
print(nt.polyder([1.0, 0.0, 1.0]))
print(nt.polyint([0.0, 2.0], constant=1.0))
print(nt.polyadd([1.0, 2.0], [3.0]))
print(nt.polymul([1.0, 1.0], [1.0, -1.0]))
print(nt.polyroot([-4.0, 0.0, 1.0], 3.0))
```

## 淇″彿澶勭悊

```python
print(nt.moving_average([1.0, 3.0, 5.0], 2))
print(nt.exponential_smooth([0.0, 10.0], 0.5))
print(nt.convolve([1.0, 2.0], [1.0, 1.0]))
print(nt.normalize([2.0, 4.0, 6.0]))
print(nt.find_peaks([0.0, 2.0, 1.0, 3.0, 0.0], 1.5))
```

## 寮傚父澶勭悊

鍙傛暟闈炴硶鏃朵細鎶涘嚭 `ValueError`銆侾ython 鍥炶皟鍐呴儴鎶涘嚭鐨勫紓甯镐細鍘熸牱浼犲洖 Python銆?
```python
try:
    nt.tangent(lambda x: (_ for _ in ()).throw(RuntimeError("boom")), 1.0)
except RuntimeError as exc:
    print(exc)
```
