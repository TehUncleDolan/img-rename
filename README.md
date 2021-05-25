# img-rename

[![License](https://img.shields.io/badge/License-BSD%203--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)

img-rename is a command-line tool that allows you to rename pages of a book
(from` 000.ext` to `NNN.ext`) while handling DPR correctly (renamed
`AAA-BBB.ext`).

## How to install

You can download a pre-compiled executable for Linux, MacOS and Windows
operating systems
[on the release page](https://github.com/TehUncleDolan/img-rename/releases/latest),
then you should copy that executable to a location from your `$PATH` env.

You might need to run `chmod +x img-rename_amd64` or `chmod +x img-rename_darwin`.

## Usage

The simplest invocation only requires you to specify the directory where the
files you want to rename are.

```bash
img-rename my-book
```

You can also gives the path to several books:

```bash
img-rename "Tome 01" "Tome 02"
```
