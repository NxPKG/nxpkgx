//go:build rust
// +build rust

package fs

import (
	"github.com/nxpkg/nxpkg/cli/internal/ffi"
	"github.com/nxpkg/nxpkg/cli/internal/nxpkgpath"
)

// GetNxpkgDataDir returns a directory outside of the repo
// where nxpkg can store data files related to nxpkg.
func GetNxpkgDataDir() nxpkgpath.AbsoluteSystemPath {
	dir := ffi.GetNxpkgDataDir()
	return nxpkgpath.AbsoluteSystemPathFromUpstream(dir)
}
