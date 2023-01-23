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

`nix` is used to provide a reproducible development environment for this project. To get started

- [install nix](https://nix.dev/tutorials/install-nix) via the provided instructions
- edit either `~/.config/nix/nix.conf` or `/etc/nix/nix.conf` and add:

  ```
  experimental-features = nix-command flakes
  ```

  This is needed to expose the Nix 2.0 CLI and flakes support that are hidden behind feature-flags.

- [install direnv(https://direnv.net/docs/installation.html)
- clone this repository

  ```bash
  git clone https://github.com/ifiokjr/skribble
  cd skribble
  ```

- load the `direnv`

  ```bash
  # The security mechanism didn't allow to load the .envrc. Since we trust it, let's allow it execution.
  direnv allow .
  ```

At this point you should see the `nix` commands available in your terminal.
