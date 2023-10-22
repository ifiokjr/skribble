# skribble

> skribble is a fully typed atomic css compiler for rust, node, deno and the browser

## Motivation

Atomic CSS has been popularised by `tailwindcss`. However there are some limitations to the way it's currently used.

- Strings are error prone and it would be better to piggy back off typed languages for auto-completion and highlighting typos.
- Composing libraries is difficult due to the specificity of CSS. If a component has consumed the class `.p-2` and you want to override this with the class `.p-1` in your parent component, the component will not receive the intended styles. Since the `.p-2` appears after `.p-1` in the generated CSS code then it can not be overriden by the `.p-1` class, regardless of intention. More information on overrides below.

### Overrides

One problem with atomic css libraries is that css selectors that appear later in the generated file have a higher priority than selectors which appear earlier.

In the css that is generated the `padding` property, added by the `.p::$px` class appears before the `padding-right` property, added by the `.pr::$2` class. This is usually what is desired.

```css
.p::$px {
    padding: 1px;
}

.pr::$2 {
    padding-right: 4px;
}
```

Imagine we want to set the padding on a component.

```jsx
import { c } from "skribble/client";

const OverridePadding = (props) => {
	return <div className={[c.p.$px, c.pr.$2].join(" ")}>{props.children}</div>;
};
```

The component `OverridePadding` will render correctly since the `padding-right` appears later in the css file. The div will have a `padding-right` of `4px`.

But, as shown below, when we try to do the opposite and override the `padding-right` with the `padding` property the `div` will still have a `padding-right` of `4px`.

```jsx
import { c } from "skribble/client";

const FailedOverridePadding = (props) => {
	return <div className={[c.pr.$2, c.p.$px].join(" ")}>{props.children}</div>;
};
```

The best way to fix this would be to automate removal of css classes when they are completely overridden by a class later in the class declaration. This is solved by `skribble`.

## Contributing

[`devenv`](https://devenv.sh/) is used to provide a reproducible development environment for this project. Follow the [getting started instructions](https://devenv.sh/getting-started/).

To automatically load the environment you should [install direnv](https://devenv.sh/automatic-shell-activation/) and then load the `direnv`.

```bash
# The security mechanism didn't allow to load the `.envrc`.
# Since we trust it, let's allow it execution.
direnv allow .
```

At this point you should see the `nix` commands available in your terminal.

### Upgrading `devenv`

If you have an outdated version of `devenv` you can update it by running the following commands. If you have an easier way, please create a PR and I'll update these docs.

```bash
nix profile list # find the index of the nxi package
nix profile remove <index>
nix profile install --accept-flake-config github:cachix/devenv/<version>
```

### Editor Setup

To setup recommended configuration for your favourite editor run the following commands.

```bash
setup:vscode # Setup vscode
setup:helix  # Setup helix configuration
```
