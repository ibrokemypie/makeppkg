# makeppkg
### (make, patch, package)
[![Build Status](https://travis-ci.org/ibrokemypie/makeppkg.svg?branch=master)](https://travis-ci.org/ibrokemypie/makeppkg)

## Installation
#### Aur:
[makeppkg](https://aur.archlinux.org/packages/makeppkg/)

[makeppkg-git](https://aur.archlinux.org/packages/makeppkg-git/)


#### Source:
```
git clone https://github.com/ibrokemypie/makeppkg

cd makeppkg

cargo build --release
```

Binary will be built at `./target/release/makeppkg`

## Usage
`$ makeppkg -l <makeppkg patch dir location>  <makepkg arguments>`

Patch dir location defaults to `$HOME/.config/makeppkg`

If no patches are found, makeppkg will simply wrap makepkg.


## Details
Makeppkg will search the makeppkg patch dir for a folder named after the package being build (taken from the PKGBUILD, the first name found if it is a multi package script).

If the patch directory is found, it will be searched for files ending in `.patch` or `.diff`.

Makeppkg will then apply patches to the PKGBUILD if any are found, getting the destination file name from the patch contents, falling back to the base name of the patch file.

Makeppkg will compute the appropriate checksums for the other patches found then insert to the PKGBUILD at the beginning of the prepare step.

Makeppkg only adds patches to be applied to the PKGBUILD rather than patching files directly so that the original checksums are still used to confirm sources before the patches apply.


## Example
// TODO: PRODUCE WORKING EXAMPLE

The usecase for this program is to allow the automated patching of packages that are updated, working as a makepkg wrapper that defaults to no action.

An example would be applying patches to the PKGBUILD and kernel config file provided by the linux-zen package to use modprobed-db and enable MuQSS.

By using makeppkg the changes would be applied every time the package is built automatically.
