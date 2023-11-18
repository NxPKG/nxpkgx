package runsummary

import (
	"github.com/nxpkg/nxpkg/cli/internal/env"
	"github.com/nxpkg/nxpkg/cli/internal/nxpkgpath"
)

// GlobalEnvConfiguration contains the environment variable inputs for the global hash
type GlobalEnvConfiguration struct {
	Env            []string `json:"env"`
	PassThroughEnv []string `json:"passThroughEnv"`
}

// GlobalEnvVarSummary contains the environment variables that impacted the global hash
type GlobalEnvVarSummary struct {
	Specified GlobalEnvConfiguration `json:"specified"`

	Configured  env.EnvironmentVariablePairs `json:"configured"`
	Inferred    env.EnvironmentVariablePairs `json:"inferred"`
	PassThrough env.EnvironmentVariablePairs `json:"passthrough"`
}

// GlobalHashSummary contains the pieces of data that impacted the global hash (then then impacted the task hash)
type GlobalHashSummary struct {
	GlobalCacheKey       string                                `json:"rootKey"`
	GlobalFileHashMap    map[nxpkgpath.AnchoredUnixPath]string `json:"files"`
	RootExternalDepsHash string                                `json:"hashOfExternalDependencies"`
	DotEnv               nxpkgpath.AnchoredUnixPathArray       `json:"globalDotEnv"`
	EnvVars              GlobalEnvVarSummary                   `json:"environmentVariables"`
}

// NewGlobalHashSummary creates a GlobalHashSummary struct from a set of fields.
func NewGlobalHashSummary(
	globalCacheKey string,
	fileHashMap map[nxpkgpath.AnchoredUnixPath]string,
	rootExternalDepsHash string,
	globalEnv []string,
	globalPassThroughEnv []string,
	globalDotEnv nxpkgpath.AnchoredUnixPathArray,
	resolvedEnvVars env.DetailedMap,
	resolvedPassThroughEnvVars env.EnvironmentVariableMap,
) *GlobalHashSummary {
	return &GlobalHashSummary{
		GlobalCacheKey:       globalCacheKey,
		GlobalFileHashMap:    fileHashMap,
		RootExternalDepsHash: rootExternalDepsHash,

		EnvVars: GlobalEnvVarSummary{
			Specified: GlobalEnvConfiguration{
				Env:            globalEnv,
				PassThroughEnv: globalPassThroughEnv,
			},
			Configured:  resolvedEnvVars.BySource.Explicit.ToSecretHashable(),
			Inferred:    resolvedEnvVars.BySource.Matching.ToSecretHashable(),
			PassThrough: resolvedPassThroughEnvVars.ToSecretHashable(),
		},

		DotEnv: globalDotEnv,
	}
}
