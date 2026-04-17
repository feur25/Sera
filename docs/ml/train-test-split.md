# train_test_split

## Signature

```python
X_train, X_test, y_train, y_test = sp.train_test_split(
    x,
    y,
    test_size: float = 0.2,
    random_state: int = 42,
)
```

---

## Description

Splits data into training and test sets using a Fisher-Yates shuffle seeded by `random_state` (splitmix64 PRNG). Deterministic for the same seed.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `x` | `ndarray (n, p)` | — | Feature matrix |
| `y` | `ndarray (n,)` | — | Target vector (float) |
| `test_size` | `float` | `0.2` | Fraction of data in the test set |
| `random_state` | `int` | `42` | Random seed |

---

## Returns

| Return | Type | Description |
|--------|------|-------------|
| `X_train` | `list[list[float]]` | Training features |
| `X_test` | `list[list[float]]` | Test features |
| `y_train` | `list[float]` | Training targets |
| `y_test` | `list[float]` | Test targets |

---

## Example

<details>
<summary><strong>Basic split</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(1000, 5)
y = (X @ np.random.randn(5)).astype(np.float64)

X_train, X_test, y_train, y_test = sp.train_test_split(X, y, 0.2, 42)

print(f"Train: {len(y_train)} samples")
print(f"Test:  {len(y_test)} samples")
```

</details>

<details>
<summary><strong>Full pipeline</strong></summary>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(500, 4)
y = (X @ np.random.randn(4)).astype(np.float64)

X_train, X_test, y_train, y_test = sp.train_test_split(X, y, 0.25, 0)

scaler = sp.StandardScaler()
X_train = np.array(scaler.fit_transform(np.array(X_train)))
X_test = np.array(scaler.transform(np.array(X_test)))

model = sp.Ridge(alpha=0.5)
model.fit(X_train, np.array(y_train))

preds = model.predict(X_test)
r2 = sp.r2_score(y_test, preds)
print(f"Test R²: {r2:.4f}")
```

</details>
