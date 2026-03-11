# **Slint Quick Reference: `root`, `parent`, and `self`**

These keywords refer to different elements in the UI hierarchy.

**1. `self` — the current element**

* Refers to the element you are inside.
* Used when accessing the element’s own properties.

Example:

```
Image {
    width: 24px;
    height: 24px;

    x: (parent.width - self.width) * 0.5;
}
```

Here `self.width` = width of the `Image`.

---

**2. `parent` — the direct container**

* Refers to the element that contains the current element.

Example:

```
Rectangle {
    width: 200px;

    Image {
        width: 40px;
        x: (parent.width - self.width) * 0.5;
    }
}
```

Here `parent.width` = width of the `Rectangle`.

---

**3. `root` — the component itself**

* Refers to the top-level component where properties are defined.
* Used to access component properties from nested elements.

Example:

```
export component IconButton {
    in property <length> size: 24px;

    Image {
        width: root.size;
        height: root.size;
    }
}
```

Here `root.size` = property of the component.

---

**Quick Mental Model**

```
root   = component itself
parent = container of the element
self   = the element itself
```

Hierarchy example:

```
Component (root)
   └── Rectangle (parent)
         └── Image (self)
```

---
# **Slint Quick Reference: `in`, `out`, and `in-out` Properties**

Properties define **how data flows between components** in **Slint**.

---

**1. `in property` — input only**

* Value is set **by the parent**
* Component can **read it**
* Component **cannot change it**

Example:

```slint
export component Icon {
    in property <length> size;
}

Icon {
    size: 32px;
}
```

Flow:

```
Parent → Component
```

Use this when the component **only needs to receive data**.

---

**2. `out property` — output only**

* Value is **set by the component**
* Parent can **read it**
* Parent **cannot assign to it**

Example:

```slint
export component Status {
    out property <bool> ready;
}
```

Flow:

```
Component → Parent
```

Use this when the component **reports state back to the parent**.

---

**3. `in-out property` — two-way**

* Parent can **set the value**
* Component can **modify it**
* Changes propagate both ways

Example:

```slint
export component Counter {
    in-out property <int> value;

    TouchArea {
        clicked => { root.value += 1; }
    }
}
```

Flow:

```
Parent ⇄ Component
```

Use this when **both sides need to modify the value**.

---

**Quick Summary**

| Property Type | Parent Can Set | Component Can Change |
| ------------- | -------------- | -------------------- |
| `in`          | ✅ Yes          | ❌ No                 |
| `out`         | ❌ No           | ✅ Yes                |
| `in-out`      | ✅ Yes          | ✅ Yes                |
