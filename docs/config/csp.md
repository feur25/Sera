# CSP-safe Mode

<div class="lang-en">

Strict Content Security Policies block inline `<script>`. The `csp_safe()` method extracts JS into a `<script type="application/json">` payload + a single nonce-able loader.

## Python

```python
import seraplot as sp

chart = sp.line([1,2,3,4], [10,20,15,25]).csp_safe()
```

Apply your CSP `script-src 'nonce-sp-nonce'` and the chart still renders.

</div>

<div class="lang-fr">

Les CSP strictes bloquent les `<script>` inline. La méthode `csp_safe()` extrait le JS dans un payload `<script type="application/json">` + un loader unique compatible nonce.

## Python

```python
import seraplot as sp

chart = sp.line([1,2,3,4], [10,20,15,25]).csp_safe()
```

Applique ta CSP `script-src 'nonce-sp-nonce'` et le chart se rend toujours.

</div>
