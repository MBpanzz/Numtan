# NumTan

NumTan 鏄竴涓互鈥滃垏绾库€濅负鏍稿績姒傚康鐨勮交閲忕骇鏁板€艰绠楀伐鍏峰簱銆傛牳蹇冪畻娉曚娇鐢?Rust 瀹炵幇锛屽苟閫氳繃 PyO3 瀵煎嚭涓?Python 妯″潡銆傚畠閫傚悎鏁欏銆佸疄楠屻€佸皬鍨嬫暟鍊间换鍔″拰闇€瑕佸彲瑙ｉ噴杩唬杩囩▼鐨勫満鏅€?
褰撳墠鐗堟湰锛歚0.9.1`銆?
鑻辨枃鏂囨。锛歚README.md`

## 鍔熻兘姒傝

- v0.1锛氭暟鍊兼眰瀵间笌鏍囬噺鏂圭▼姹傛牴
- v0.2锛氬皬鍨嬪悜閲忋€佺煩闃典笌绾挎€х郴缁熷伐鍏?- v0.3锛氬父寰垎鏂圭▼鏄惧紡姝ヨ繘涓庤嚜閫傚簲 RK4
- v0.4锛歵anh-sinh 绉垎銆佹棤绌峰尯闂寸Н鍒嗕笌姝ｅ垏鐩稿叧鐗规畩鍑芥暟
- v0.5锛氱敤浜庣粯鍥惧拰鏁欏灞曠ず鐨勫彲瑙嗗寲鏁版嵁瀵煎嚭
- v0.6锛氭弿杩扮粺璁°€佺浉鍏虫€у拰鍥炲綊宸ュ叿
- v0.7锛氭彃鍊笺€侀噰鏍风綉鏍煎拰鏈夐檺宸垎
- v0.8锛氬椤瑰紡璁＄畻宸ュ叿
- v0.9锛氫俊鍙峰钩婊戙€佸嵎绉€佸綊涓€鍖栧拰宄板€兼娴?
## 瀹夎

浠庢湰鍦?wheel 瀹夎锛?
```bash
python -m pip install target/wheels/numtan-0.9.1-cp313-cp313-win_amd64.whl
```

寮€鍙戞ā寮忔瀯寤哄苟瀹夎锛?
```bash
python -m pip install maturin
maturin develop --release
```

鏋勫缓鍙戝竷鐢?wheel锛?
```bash
maturin build --release --compatibility pypi --auditwheel=repair
```

## 蹇€熷紑濮?
```python
import numtan as nt

root = nt.newton(lambda x: x * x - 2.0, 1.0)["root"]
slope = nt.tangent(lambda x: x**3, 2.0)
area = nt.tanh_sinh(lambda x: x * x, 0.0, 1.0)["value"]

print(root, slope, area)
```

杈撳嚭澶х害涓猴細

```text
1.4142135623730951 12.0 0.3333333333333333
```

## 甯哥敤绀轰緥

### 姹傚涓庢眰鏍?
```python
nt.tangent(lambda x: x**3, 2.0)
nt.gradient(lambda v: v[0] ** 2 + v[1] ** 2, [1.0, 2.0])
nt.newton(lambda x: x * x - 2.0, 1.0)
nt.halley(lambda x: x**3 - 8.0, 1.0)
nt.householder(lambda x: x**3 - 8.0, 1.0, 3)
```

### 绾挎€т唬鏁?
```python
nt.dot([1.0, 2.0], [3.0, 4.0])
nt.norm([3.0, 4.0])
nt.solve([[2.0, 1.0], [1.0, 3.0]], [1.0, 2.0])
```

### 浼樺寲

```python
nt.gradient_descent(lambda v: (v[0] - 2.0) ** 2, [0.0])
nt.tangent_minimize(lambda x: (x - 3.0) ** 2, 0.0)
nt.gauss_newton(lambda v: [v[0] - 2.0], [0.0])
```

### ODE 涓庣Н鍒?
```python
nt.rk4(lambda t, y: y, 1.0, 0.0, 1.0, 0.05)
nt.adaptive_rk4(lambda t, y: y, 1.0, 0.0, 1.0, 0.2)
nt.quad_inf(lambda x: __import__("math").exp(-x * x))
```

### 缁熻銆佹彃鍊笺€佸椤瑰紡鍜屼俊鍙峰鐞?
```python
nt.summary([1.0, 2.0, 3.0])
nt.linear_interpolate([0.0, 1.0], [0.0, 2.0], 0.25)
nt.polyval([1.0, 0.0, 1.0], 2.0)
nt.find_peaks([0.0, 2.0, 1.0, 3.0, 0.0], 1.5)
```

## 涓枃鏂囨。

- 鏂囨。鎬荤储寮曪細`docs/INDEX.zh-CN.md`
- 鑻辨枃鏂囨。鎬荤储寮曪細`docs/INDEX.md`
- 鑻辨枃 README锛歚README.md`
- 涓枃浣跨敤鎸囧崡锛歚docs/USAGE.zh-CN.md`
- 涓枃 API 鍙傝€冿細`docs/API.zh-CN.md`
- 涓枃鍙戝竷鎸囧崡锛歚docs/RELEASE.zh-CN.md`
- 鍙繍琛?Python 绀轰緥锛歚examples/python_demo.py`

## 娴嬭瘯

杩愯 Rust 娴嬭瘯锛?
```bash
cargo test
```

瀹夎鎵╁睍鍚庤繍琛?Python 娴嬭瘯锛?
```bash
python tests/python_smoke.py
python tests/api_surface.py
```

## 璁捐璇存槑

- 鏍稿績绠楁硶浣嶄簬 `src/core`锛屼笉渚濊禆 Python銆?- Python 缁戝畾浣嶄簬 `src/api`锛岃礋璐ｆ妸 Rust 缁撴灉杞崲涓?Python 鐨?`dict`銆乣list`銆乣tuple` 鍜?`float`銆?- 杩唬绠楁硶灏介噺杩斿洖 `history`锛屾柟渚挎暀瀛︺€佺粯鍥惧拰璋冭瘯銆?- Python 鍥炶皟涓殑寮傚父浼氳姝ｅ父浼犲洖 Python锛屼笉浼氬湪 Rust 渚?panic銆?
## 椤圭洰鐘舵€?
NumTan `0.9.1` 閫傚悎浣滀负棰勮鐗堟垨鍊欓€夊彂甯冪増鏈€傝嫢瑕佸彂甯冪ǔ瀹氱増 `1.0.0`锛屽缓璁户缁ˉ鍏呮洿澶氭暟鍊艰竟鐣屾祴璇曘€佸骞冲彴 wheel CI 鍜屽畬鏁磋嚜鍔ㄧ敓鎴?API 鏂囨。銆?