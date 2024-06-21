[![progress-banner](https://backend.codecrafters.io/progress/git/15c4a312-c64b-497d-9e0d-e72d5804998b)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

This is a starting point for Rust solutions to the
["Build Your Own Git" Challenge](https://codecrafters.io/challenges/git).

In this challenge, you'll build a small Git implementation that's capable of
initializing a repository, creating commits and cloning a public repository.
Along the way we'll learn about the `.git` directory, Git objects (blobs,
commits, trees etc.), Git's transfer protocols and more.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

# Passing the first stage

The entry point for your Git implementation is in `src/main.rs`. Study and
uncomment the relevant code, and push your changes to pass the first stage:

```sh
git add .
git commit -m "pass 1st stage" # any msg
git push origin master
```

That's all!

# Stage 2 & beyond

Note: This section is for stages 2 and beyond.

1. Ensure you have `cargo (1.54)` installed locally
1. Run `./your_git.sh` to run your Git implementation, which is implemented in
   `src/main.rs`. This command compiles your Rust project, so it might be slow
   the first time you run it. Subsequent runs will be fast.
1. Commit your changes and run `git push origin master` to submit your solution
   to CodeCrafters. Test output will be streamed to your terminal.

# Testing locally

The `your_git.sh` script is expected to operate on the `.git` folder inside the
current working directory. If you're running this inside the root of this
repository, you might end up accidentally damaging your repository's `.git`
folder.

We suggest executing `your_git.sh` in a different folder when testing locally.
For example:

```sh
mkdir -p /tmp/testing && cd /tmp/testing
/path/to/your/repo/your_git.sh init
```

To make this easier to type out, you could add a
[shell alias](https://shapeshed.com/unix-alias/):

```sh
alias mygit=/path/to/your/repo/your_git.sh

mkdir -p /tmp/testing && cd /tmp/testing
mygit init
```

# Testing via codecrafters

```
codecrafters test
```

Visit https://codecrafters.io/cli to install

# Resources

[git-objects](https://git-scm.com/book/en/v2/Git-Internals-Git-Objects)

[git-bottom-up](http://ftp.newartisans.com/pub/git.from.bottom.up.pdf)

[pygit](https://benhoyt.com/writings/pygit/)

[git-smart-http](https://www.git-scm.com/docs/http-protocol)

[git-clone](https://stefan.saasen.me/articles/git-clone-in-haskell-from-the-bottom-up/)

[gitprotocol-pack](https://github.com/git/git/blob/795ea8776befc95ea2becd8020c7a284677b4161/Documentation/gitprotocol-pack.txt)

[gitformat-pack](https://github.com/git/git/blob/795ea8776befc95ea2becd8020c7a284677b4161/Documentation/gitformat-pack.txt)

[protocol-capabilities](https://github.com/git/git/blob/795ea8776befc95ea2becd8020c7a284677b4161/Documentation/gitprotocol-capabilities.txt)

[protocol-common](https://github.com/git/git/blob/795ea8776befc95ea2becd8020c7a284677b4161/Documentation/gitprotocol-common.txt)

[Unpacking Git packfiles](https://codewords.recurse.com/issues/three/unpacking-git-packfiles)

[Sneaky Git number encoding](https://medium.com/@concertdaw/sneaky-git-number-encoding-ddcc5db5329f)

[git-smart-https](https://stackoverflow.com/questions/68062812/what-does-the-git-smart-https-protocol-fully-look-like-in-all-its-glory)