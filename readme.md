# Boom 💥 

A fast & simple boilerplate generator, built with Rust.

<img src="https://user-images.githubusercontent.com/2598660/139595667-d44b9b10-c389-41ad-ac43-de08e5b249cd.gif" width="600" />


## Installing boom

**This package is not yet downloadable on Brew or other package managers, so for now installing it takes slightly more work.**

1. Clone this repo into the directory of your choice:
```bash
git clone https://github.com/t4t5/boom.git
```

2. Compile the code:
```bash
cd boom && cargo build
```

3. Set an alias for "boom" in your `.zshrc` or `.bash_profile`:
```bash
alias boom="{YOUR_DIR}/boom/target/debug/boom"
```

## Using boom

Boom will automatically create a `.boom` folder in your home directory. This is where all your boilerplate generators will be stored.

You can add this directory to your version-controlled [dotfiles](https://thoughtbot.com/upcase/videos/intro-to-dotfiles) so that you don't lose your library of boilerplate templates.

### Adding a boom template

To create a new boom template, run:

```bash
boom new <boom-template-name>
```

A new `<boom-template-name>` directory will be created in your `.boom` folder, with a `boilerplate` directory and an `init.sh` file inside it.

Add the files you want for your boilerplate in the `boilerplate` directory, and the commands you want to run when starting a project using this boilerplate in `init.sh` (like "npm install" for example).

### Using your new boom template

```bash
boom <boom-template-name> <your-project-name>
```

This will create a new folder called `<your-project-name>` in whatever directory you're currently in, using the `<boom-template-name>` boilerplate.

### Adding others' templates

We encourage you to share your boom templates in a public dotfiles repo! That way, others can find and use your templates in their own projects.

To quickly clone someone else's template, you first need to install [github-clone](https://github.com/HR/github-clone):

```bash
pip install github-clone
```

Now let's say you want to add [t4t5's eleventy template](https://github.com/t4t5/dotfiles/tree/master/boom/templates/eleventy) to your own boom templates. You can then simply run:

```bash
boom add https://github.com/t4t5/dotfiles/tree/master/boom/templates/eleventy
```

You can now use this boilerplate using:

```bash
boom eleventy <your-project-name>
```

## Todos

- [ ] Remove dependency on [github-clone](https://github.com/HR/github-clone)
- [ ] Show the output of scripts from templates' `init.sh` (so that you can see progress of `npm install` for example)
- [ ] Support calling `boom` command without any arguments. Show an interactive UI where you can pick from your templates.
