# skribble

> skribble is a fully typed atomic css compiler for rust, node, deno and the browser

## Motivation

Atomic CSS has been popularised by `tailwindcss`. However there are some limitations to the way it's
currently used.

- Strings are error prone and it would be better to piggy back off typed languages for
  automcompletion and surfacing errors.
- Composing libraries is difficult due to the specificity of CSS. If a component has declare `p-2`
  and you want to override that to `p-1` in your nested component. If the `p-2` appears after in the
  generated CSS code then it can not be overriden by the `p-1` class, regardless of intention.

## Contributing

[`devenv`](https://devenv.sh/) is used to provide a reproducible development environment for this
project. Follow the [getting started instructions](https://devenv.sh/getting-started/).

To automatically load the environment you should
[install direnv](https://devenv.sh/automatic-shell-activation/) and then load the `direnv`.

```bash
# The security mechanism didn't allow to load the .envrc. Since we trust it, let's allow it execution.
direnv allow .
```

At this point you should see the `nix` commands available in your terminal.
