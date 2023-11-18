// Package run implements `nxpkg run`
// This file implements the logic for `nxpkg run --dry`
package run

import (
	gocontext "context"
	"sync"

	"github.com/pkg/errors"
	"github.com/nxpkg/nxpkg/cli/internal/cache"
	"github.com/nxpkg/nxpkg/cli/internal/cmdutil"
	"github.com/nxpkg/nxpkg/cli/internal/core"
	"github.com/nxpkg/nxpkg/cli/internal/env"
	"github.com/nxpkg/nxpkg/cli/internal/graph"
	"github.com/nxpkg/nxpkg/cli/internal/nodes"
	"github.com/nxpkg/nxpkg/cli/internal/runsummary"
	"github.com/nxpkg/nxpkg/cli/internal/taskhash"
	"github.com/nxpkg/nxpkg/cli/internal/util"
)

// DryRun gets all the info needed from tasks and prints out a summary, but doesn't actually
// execute the task.
func DryRun(
	ctx gocontext.Context,
	g *graph.CompleteGraph,
	rs *runSpec,
	engine *core.Engine,
	_ *taskhash.Tracker, // unused, but keep here for parity with RealRun method signature
	nxpkgCache cache.Cache,
	globalEnvMode util.EnvMode,
	_ env.EnvironmentVariableMap,
	_ env.EnvironmentVariableMap,
	base *cmdutil.CmdBase,
	summary runsummary.Meta,
) error {
	defer nxpkgCache.Shutdown()

	taskSummaries := []*runsummary.TaskSummary{}

	mu := sync.Mutex{}
	execFunc := func(ctx gocontext.Context, packageTask *nodes.PackageTask, taskSummary *runsummary.TaskSummary) error {
		// Assign some fallbacks if they were missing
		if taskSummary.Command == "" {
			taskSummary.Command = runsummary.MissingTaskLabel
		}

		if taskSummary.Framework == "" {
			if rs.Opts.runOpts.FrameworkInference {
				taskSummary.Framework = runsummary.NoFrameworkDetected
			} else {
				taskSummary.Framework = runsummary.FrameworkDetectionSkipped
			}
		}

		// This mutex is not _really_ required, since we are using Concurrency: 1 as an execution
		// option, but we add it here to match the shape of RealRuns execFunc.
		mu.Lock()
		defer mu.Unlock()
		taskSummaries = append(taskSummaries, taskSummary)
		return nil
	}

	// This setup mirrors a real run. We call engine.execute() with
	// a visitor function and some hardcoded execOpts.
	// Note: we do not currently attempt to parallelize the graph walking
	// (as we do in real execution)
	getArgs := func(taskID string) []string {
		return rs.ArgsForTask(taskID)
	}

	visitorFn := g.GetPackageTaskVisitor(ctx, engine.TaskGraph, rs.Opts.runOpts.FrameworkInference, globalEnvMode, getArgs, base.Logger, execFunc)
	execOpts := core.EngineExecutionOptions{
		Concurrency: 1,
		Parallel:    false,
	}

	if errs := engine.Execute(visitorFn, execOpts); len(errs) > 0 {
		for _, err := range errs {
			base.UI.Error(err.Error())
		}
		return errors.New("errors occurred during dry-run graph traversal")
	}

	// We walk the graph with no concurrency.
	// Populating the cache state is parallelizable.
	// Do this _after_ walking the graph.
	populateCacheState(nxpkgCache, taskSummaries)

	// Assign the Task Summaries to the main summary
	summary.RunSummary.Tasks = taskSummaries

	// The exitCode isn't really used by the Run Summary Close() method for dry runs
	// but we pass in a successful value to match Real Runs.
	return summary.Close(ctx, 0, g.WorkspaceInfos, base.UI)
}

func populateCacheState(nxpkgCache cache.Cache, taskSummaries []*runsummary.TaskSummary) {
	// We make at most 8 requests at a time for cache state.
	maxParallelRequests := 8
	taskCount := len(taskSummaries)

	parallelRequestCount := maxParallelRequests
	if taskCount < maxParallelRequests {
		parallelRequestCount = taskCount
	}

	queue := make(chan int, taskCount)

	wg := &sync.WaitGroup{}
	for i := 0; i < parallelRequestCount; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for index := range queue {
				task := taskSummaries[index]
				itemStatus := nxpkgCache.Exists(task.Hash)
				task.CacheSummary = runsummary.NewTaskCacheSummary(itemStatus)
			}
		}()
	}

	for index := range taskSummaries {
		queue <- index
	}
	close(queue)
	wg.Wait()
}
