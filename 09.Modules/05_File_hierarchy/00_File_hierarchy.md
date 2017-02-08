[9.5 File hierarchy](http://rustbyexample.com/mod/split.html)

Modules can be mapped to a file/directory hierarchy.
Let's break down the visibility example in files:

```
$ tree .
.
|-- my
|   |-- [inaccessible.rs](./my/inaccessible.rs)
|   |-- [mod.rs](./my/mod.rs)
|   `-- [nested.rs](./my/nested.rs)
`-- [split.rs](./split.rs)
```

> $ rustc split.rs && ./split

```
called `my::function()`
called `function()`
called `my::indirect_access()`, that
> called `my::private_function()`
called `my::nested::function()`

```
