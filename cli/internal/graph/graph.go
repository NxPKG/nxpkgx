// Package graph contains the CompleteGraph struct and some methods around it
package graph

import (
	gocontext "context"
	"fmt"
	"regexp"
	"sort"
	"strings"

	"github.com/hashicorp/go-hclog"
	"github.com/pyr-sh/dag"
	"github.com/nxpkg/nxpkg/cli/internal/env"
	"github.com/nxpkg/nxpkg/cli/internal/fs"
	"github.com/nxpkg/nxpkg/cli/internal/nodes"
	"github.com/nxpkg/nxpkg/cli/internal/runsummary"
	"github.com/nxpkg/nxpkg/cli/internal/taskhash"
	"github.com/nxpkg/nxpkg/cli/internal/nxpkgpath"
	"github.com/nxpkg/nxpkg/cli/internal/util"
	"github.com/nxpkg/nxpkg/cli/internal/workspace"
)

// CompleteGraph represents the common state inferred from the filesystem and pipeline.
// It is not intended to include information specific to a particular run.
type CompleteGraph struct {
	// WorkspaceGraph expresses the dependencies between packages
	WorkspaceGraph dag.AcyclicGraph

	// Pipeline is config from nxpkg.json
	Pipeline fs.Pipeline

	// WorkspaceInfos stores the package.json contents by package name
	WorkspaceInfos workspace.Catalog

	// GlobalHash is the hash of all global dependencies
	GlobalHash string

	RootNode string

	// Map of TaskDefinitions by taskID
	TaskDefinitions map[string]*fs.TaskDefinition
	RepoRoot        nxpkgpath.AbsoluteSystemPath

	TaskHashTracker *taskhash.Tracker
}

// GetPackageTaskVisitor wraps a `visitor` function that is used for walking the TaskGraph
// during execution (or dry-runs). The function returned here does not execute any tasks itself,
// but it helps curry some data from the Complete Graph and pass it into the visitor function.
func (g *CompleteGraph) GetPackageTaskVisitor(
	ctx gocontext.Context,
	taskGraph *dag.AcyclicGraph,
	frameworkInference bool,
	globalEnvMode util.EnvMode,
	getArgs func(taskID string) []string,
	logger hclog.Logger,
	execFunc func(ctx gocontext.Context, packageTask *nodes.PackageTask, taskSummary *runsummary.TaskSummary) error,
) func(taskID string) error {
	return func(taskID string) error {
		packageName, taskName := util.GetPackageTaskFromId(taskID)
		pkg, ok := g.WorkspaceInfos.PackageJSONs[packageName]
		if !ok {
			return fmt.Errorf("cannot find package %v for task %v", packageName, taskID)
		}

		// Check for root task
		var command string
		if cmd, ok := pkg.Scripts[taskName]; ok {
			command = cmd
		}

		if packageName == util.RootPkgName && commandLooksLikeNxpkg(command) {
			return fmt.Errorf("root task %v (%v) looks like it invokes nxpkg and might cause a loop", taskName, command)
		}

		taskDefinition, ok := g.TaskDefinitions[taskID]
		if !ok {
			return fmt.Errorf("Could not find definition for task")
		}

		// Task env mode is only independent when global env mode is `infer`.
		taskEnvMode := globalEnvMode
		if taskEnvMode == util.Infer {
			if taskDefinition.PassThroughEnv != nil {
				taskEnvMode = util.Strict
			} else {
				// If we're in infer mode we have just detected non-usage of strict env vars.
				// But our behavior's actual meaning of this state is `loose`.
				taskEnvMode = util.Loose
			}
		}

		// TODO: maybe we can remove this PackageTask struct at some point
		packageTask := &nodes.PackageTask{
			TaskID:          taskID,
			Task:            taskName,
			PackageName:     packageName,
			Pkg:             pkg,
			EnvMode:         taskEnvMode,
			Dir:             pkg.Dir.ToString(),
			TaskDefinition:  taskDefinition,
			Outputs:         taskDefinition.Outputs.Inclusions,
			ExcludedOutputs: taskDefinition.Outputs.Exclusions,
		}

		passThruArgs := getArgs(taskName)
		hash, err := g.TaskHashTracker.CalculateTaskHash(
			logger,
			packageTask,
			taskGraph.DownEdges(taskID),
			frameworkInference,
			passThruArgs,
		)

		// Not being able to construct the task hash is a hard error
		if err != nil {
			return fmt.Errorf("Hashing error: %v", err)
		}

		pkgDir := pkg.Dir
		packageTask.Hash = hash
		envVars := g.TaskHashTracker.GetEnvVars(taskID)
		expandedInputs := g.TaskHashTracker.GetExpandedInputs(packageTask)
		framework := g.TaskHashTracker.GetFramework(taskID)

		packageTask.Command = command

		envVarPassThroughMap, err := g.TaskHashTracker.EnvAtExecutionStart.FromWildcards(taskDefinition.PassThroughEnv)
		if err != nil {
			return err
		}

		specifiedEnvVarsPresentation := []string{}
		if taskDefinition.Env != nil {
			specifiedEnvVarsPresentation = taskDefinition.Env
		}

		summary := &runsummary.TaskSummary{
			TaskID:                 taskID,
			Task:                   taskName,
			Hash:                   hash,
			Package:                packageName,
			Dir:                    pkgDir.ToString(),
			Outputs:                taskDefinition.Outputs.Inclusions,
			ExcludedOutputs:        taskDefinition.Outputs.Exclusions,
			LogFileRelativePath:    packageTask.RepoRelativeSystemLogFile(),
			ResolvedTaskDefinition: taskDefinition,
			ExpandedInputs:         expandedInputs,
			ExpandedOutputs:        []nxpkgpath.AnchoredSystemPath{},
			Command:                command,
			CommandArguments:       passThruArgs,
			Framework:              framework,
			EnvMode:                taskEnvMode,
			EnvVars: runsummary.TaskEnvVarSummary{
				Specified: runsummary.TaskEnvConfiguration{
					Env:            specifiedEnvVarsPresentation,
					PassThroughEnv: taskDefinition.PassThroughEnv,
				},
				Configured:  env.EnvironmentVariableMap(envVars.BySource.Explicit).ToSecretHashable(),
				Inferred:    env.EnvironmentVariableMap(envVars.BySource.Matching).ToSecretHashable(),
				PassThrough: envVarPassThroughMap.ToSecretHashable(),
			},
			DotEnv:           taskDefinition.DotEnv,
			ExternalDepsHash: pkg.ExternalDepsHash,
		}

		if ancestors, err := g.getTaskGraphAncestors(taskGraph, packageTask.TaskID); err == nil {
			summary.Dependencies = ancestors
		}
		if descendents, err := g.getTaskGraphDescendants(taskGraph, packageTask.TaskID); err == nil {
			summary.Dependents = descendents
		}

		return execFunc(ctx, packageTask, summary)
	}
}

// GetPipelineFromWorkspace returns the Unmarshaled fs.Pipeline struct from nxpkg.json in the given workspace.
func (g *CompleteGraph) GetPipelineFromWorkspace(workspaceName string, isSinglePackage bool) (fs.Pipeline, error) {
	nxpkgConfig, err := g.GetNxpkgConfigFromWorkspace(workspaceName, isSinglePackage)

	if err != nil {
		return nil, err
	}

	return nxpkgConfig.Pipeline, nil
}

// GetNxpkgConfigFromWorkspace returns the Unmarshaled fs.NxpkgJSON from nxpkg.json in the given workspace.
func (g *CompleteGraph) GetNxpkgConfigFromWorkspace(workspaceName string, isSinglePackage bool) (*fs.NxpkgJSON, error) {
	cachedNxpkgConfig, ok := g.WorkspaceInfos.NxpkgConfigs[workspaceName]

	if ok {
		return cachedNxpkgConfig, nil
	}

	var workspacePackageJSON *fs.PackageJSON
	if pkgJSON, err := g.GetPackageJSONFromWorkspace(workspaceName); err == nil {
		workspacePackageJSON = pkgJSON
	} else {
		return nil, err
	}

	// Note: pkgJSON.Dir for the root workspace will be an empty string, and for
	// other workspaces, it will be a relative path.
	workspaceAbsolutePath := workspacePackageJSON.Dir.RestoreAnchor(g.RepoRoot)
	nxpkgConfig, err := fs.LoadNxpkgConfig(workspaceAbsolutePath, workspacePackageJSON, isSinglePackage)

	// If we failed to load a NxpkgConfig, bubble up the error
	if err != nil {
		return nil, err
	}

	// add to cache
	g.WorkspaceInfos.NxpkgConfigs[workspaceName] = nxpkgConfig

	return g.WorkspaceInfos.NxpkgConfigs[workspaceName], nil
}

// GetPackageJSONFromWorkspace returns an Unmarshaled struct of the package.json in the given workspace
func (g *CompleteGraph) GetPackageJSONFromWorkspace(workspaceName string) (*fs.PackageJSON, error) {
	if pkgJSON, ok := g.WorkspaceInfos.PackageJSONs[workspaceName]; ok {
		return pkgJSON, nil
	}

	return nil, fmt.Errorf("No package.json for %s", workspaceName)
}

// getTaskGraphAncestors gets all the ancestors for a given task in the graph.
// "ancestors" are all tasks that the given task depends on.
func (g *CompleteGraph) getTaskGraphAncestors(taskGraph *dag.AcyclicGraph, taskID string) ([]string, error) {
	ancestors, err := taskGraph.Ancestors(taskID)
	if err != nil {
		return nil, err
	}
	stringAncestors := []string{}
	for _, dep := range ancestors {
		// Don't leak out internal root node name, which are just placeholders
		if !strings.Contains(dep.(string), g.RootNode) {
			stringAncestors = append(stringAncestors, dep.(string))
		}
	}

	sort.Strings(stringAncestors)
	return stringAncestors, nil
}

// getTaskGraphDescendants gets all the descendants for a given task in the graph.
// "descendants" are all tasks that depend on the given taskID.
func (g *CompleteGraph) getTaskGraphDescendants(taskGraph *dag.AcyclicGraph, taskID string) ([]string, error) {
	descendents, err := taskGraph.Descendents(taskID)
	if err != nil {
		return nil, err
	}
	stringDescendents := []string{}
	for _, dep := range descendents {
		// Don't leak out internal root node name, which are just placeholders
		if !strings.Contains(dep.(string), g.RootNode) {
			stringDescendents = append(stringDescendents, dep.(string))
		}
	}
	sort.Strings(stringDescendents)
	return stringDescendents, nil
}

var _isNxpkg = regexp.MustCompile(`(?:^|\s)nxpkg(?:$|\s)`)

func commandLooksLikeNxpkg(command string) bool {
	return _isNxpkg.MatchString(command)
}
