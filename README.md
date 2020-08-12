# vidmerger

> A wrapper around ffmpeg which simplifies merging multiple videos 🎞

Vidmerger is a command-line-tool which uses `ffmpeg` to merge multiple video-files with the same file-extension together into one file called `output.<format>`. It includes a usage help which you can print out by `vidmerger --help` 😃

## Installing / Getting started

### 1️⃣ Install prerequisites 🧐

[Homebrew](https://brew.sh) (Mac):

```bash
brew install ffmpeg
```

[Chocolatey](https://chocolatey.org/) (Windows):

```bash
choco install ffmpeg
```

Ensure that it was installed successfully:

```fish
↪ ffmpeg -version | head -n 1
ffmpeg version 4.2.2 Copyright (c) 2000-2019 the FFmpeg developers
```

### 2️⃣ Install vidmerger 🤖

[Homebrew 🍺](https://github.com/TGotwig/homebrew-vidmerger/blob/master/vidmerger.rb) (Mac):

```bash
brew tap tgotwig/vidmerger
brew install vidmerger
```

if you want to upgrade, then do a `brew update-reset && brew upgrade vidmerger`.

---

Linux 🐧

```bash
wget https://github.com/TGotwig/vidmerger/releases/download/v0.1.2/vidmerger-linux.tar.gz
tar xf vidmerger-linux.tar.gz -C /usr/bin/
rm vidmerger-linux.tar.gz
```

---

Windows 🏳️‍🌈

The next lines require that you opened `Internet Explorer` for at least one time and having [WinRAR](https://chocolatey.org/packages/winrar) in your PATH

```ps
Invoke-WebRequest https://github.com/TGotwig/vidmerger/releases/download/v0.1.2/vidmerger-win.rar -OutFile vidmerger-win.rar
UnRAR.exe x -o- vidmerger-win.rar
Move-Item vidmerger.exe C:\Windows\System32
Remove-Item vidmerger-win.rar
```

### 3️⃣ Use vidmerger 🎬

First of all lets see what we have:

```fish
↪ exa -lh | awk '{print $2,$7}'
Size
328k 1.mp4
328k 2.mp4
```

We want to merge all videos with the ending `mp4`, so we run it with `-f` for `format`:

```fish
↪ vidmerger . -f mp4

Order of merging 👇

file '1.mp4'
file '2.mp4'

Calling: 'ffmpeg -y -f concat -i data/input.txt -c copy data/output.mp4' 🚀

Successfully generated 'output.mp4'! 😆🎞
```

It prints us that it has generated `output.mp4` in the order you can see above 😃 It looks for each file with the given ending `mp4` except `output.mp4` and merges it to that. If the format would be `mkv`, the generated file would be named `output.mkv` 😊

```fish
↪ exa -lh | awk '{print $2,$7}'
Size
328k 1.mp4
328k 2.mp4
664k output.mp4
```

There it is! 🎉🎊🥳📺

## Run it without installing / Getting started

You can also use Docker to run vidmerger without installing anything except Docker, hosted on [Dockerhub](https://hub.docker.com/r/tgotwig/vidmerger), you can use it like so:

```bash
docker container run -it --rm -v <PATH-TO-VIDS>:/data -e format=<FORMAT> tgotwig/vidmerger
```

## Developing

### Built With

Some crates inside of [Cargo.toml](Cargo.toml) under `dependencies`.

### Prerequisites

- [Rust 2018+](https://www.rust-lang.org/tools/install)
- [youtube-dl](http://ytdl-org.github.io/youtube-dl/download.html)
- [ffmpeg](https://ffmpeg.org/download.html)

### Setting up Dev

Once you are done with installing the prerequisites, you should run `make` (or  [nmake](https://docs.microsoft.com/en-us/cpp/build/reference/nmake-reference?view=vs-2019) for Windows) to see if everything runs smooth:

```shell
git clone https://github.com/TGotwig/vidmerger.git
cd vidmerger
make
```

### Building

Run `make build` to build for Mac, Linux and Windows. You can find the compressed files for Github under `target/tars`, and the uncompressed files besides `target/tars`.

### Deploying / Publishing

Take care that the versions in the following files increases:

- [Cargo.toml](Cargo.toml)
- [Makefile](Makefile)
- [src/cli.yaml](src/cli.yaml)
- [vidmerger.nuspec](vidmerger.nuspec)
- [README.md](README.md)

## Versioning

We use [SemVer](http://semver.org/) for versioning.

## Tests

There are some basic shell-tests, which you can run by `make test`.

## Style guide

We are using [rust-clippy](rust-clippy) and [rls-vscode](https://github.com/rust-lang/rls-vscode) ✨

## Licensing

[MIT License](LICENSE.md)

[![Linkedin Badge](https://img.shields.io/badge/-Thomas_Gotwig-blue?style=flat-square&logo=Linkedin&logoColor=white&link=https://www.linkedin.com/in/thomas-gotwig-244a35177/)](https://www.linkedin.com/in/thomas-gotwig-244a35177/)
[![Gmail Badge](https://img.shields.io/badge/-tgotwig@gmail.com-c14438?style=flat-square&logo=Gmail&logoColor=white&link=mailto:tgotwig@gmail.com)](mailto:tgotwig@gmail.com)

![github stats](https://github-readme-stats.vercel.app/api?username=tgotwig&show_icons=true)
