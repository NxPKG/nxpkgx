// Package runsummary implements structs that report on a `nxpkg run` and `nxpkg run --dry`
package runsummary

import (
	"context"
	"fmt"
	"path/filepath"
	"time"

	"github.com/mitchellh/cli"
	"github.com/segmentio/ksuid"
	"github.com/nxpkg/nxpkg/cli/internal/ci"
	"github.com/nxpkg/nxpkg/cli/internal/client"
	"github.com/nxpkg/nxpkg/cli/internal/env"
	"github.com/nxpkg/nxpkg/cli/internal/spinner"
	"github.com/nxpkg/nxpkg/cli/internal/nxpkgpath"
	"github.com/nxpkg/nxpkg/cli/internal/util"
	"github.com/nxpkg/nxpkg/cli/internal/workspace"
)

// MissingTaskLabel is printed when a package is missing a definition for a task that is supposed to run
// E.g. if `nxpkg run build --dry` is run, and package-a doesn't define a `build` script in package.json,
// the RunSummary will print this, instead of the script (e.g. `next build`).
const MissingTaskLabel = "<NONEXISTENT>"

// NoFrameworkDetected is a string to identify when a workspace doesn't detect a framework
const NoFrameworkDetected = "<NO FRAMEWORK DETECTED>"

// FrameworkDetectionSkipped is a string to identify when framework detection was skipped
const FrameworkDetectionSkipped = "<FRAMEWORK DETECTION SKIPPED>"

// NOTE: When changing this, please ensure that the server side is updated to handle the new version on khulnasoft.com
// this is required to ensure safe handling of env vars (unknown run summary versions will be ignored on the server)
const runSummarySchemaVersion = "1"

type runType int

const (
	runTypeReal runType = iota
	runTypeDryText
	runTypeDryJSON
)

// Meta is a wrapper around the serializable RunSummary, with some extra information
// about the Run and references to other things that we need.
type Meta struct {
	RunSummary         *RunSummary
	repoRoot           nxpkgpath.AbsoluteSystemPath // used to write run summary
	repoPath           nxpkgpath.RelativeSystemPath
	singlePackage      bool
	shouldSave         bool
	spacesClient       *spacesClient
	runType            runType
	synthesizedCommand string
}

// RunSummary contains a summary of what happens in the `nxpkg run` command and why.
type RunSummary struct {
	ID                 ksuid.KSUID        `json:"id"`
	Version            string             `json:"version"`
	NxpkgVersion       string             `json:"nxpkgVersion"`
	Monorepo           bool               `json:"monorepo"`
	GlobalHashSummary  *GlobalHashSummary `json:"globalCacheInputs"`
	Packages           []string           `json:"packages"`
	EnvMode            util.EnvMode       `json:"envMode"`
	FrameworkInference bool               `json:"frameworkInference"`
	ExecutionSummary   *executionSummary  `json:"execution,omitempty"`
	Tasks              []*TaskSummary     `json:"tasks"`
	User               string             `json:"user"`
	SCM                *scmState          `json:"scm"`
}

// NewRunSummary returns a RunSummary instance
func NewRunSummary(
	startAt time.Time,
	repoRoot nxpkgpath.AbsoluteSystemPath,
	repoPath nxpkgpath.RelativeSystemPath,
	nxpkgVersion string,
	apiClient *client.APIClient,
	spacesClient *client.APIClient,
	runOpts util.RunOpts,
	packages []string,
	globalEnvMode util.EnvMode,
	envAtExecutionStart env.EnvironmentVariableMap,
	globalHashSummary *GlobalHashSummary,
	synthesizedCommand string,
) Meta {
	singlePackage := runOpts.SinglePackage
	profile := runOpts.Profile
	shouldSave := runOpts.Summarize
	spaceID := runOpts.ExperimentalSpaceID

	runType := runTypeReal
	if runOpts.DryRun {
		runType = runTypeDryText
		if runOpts.DryRunJSON {
			runType = runTypeDryJSON
		}
	}

	executionSummary := newExecutionSummary(synthesizedCommand, repoPath, startAt, profile)

	rsm := Meta{
		RunSummary: &RunSummary{
			ID:                 ksuid.New(),
			Version:            runSummarySchemaVersion,
			ExecutionSummary:   executionSummary,
			NxpkgVersion:       nxpkgVersion,
			Packages:           packages,
			EnvMode:            globalEnvMode,
			FrameworkInference: runOpts.FrameworkInference,
			Tasks:              []*TaskSummary{},
			GlobalHashSummary:  globalHashSummary,
			SCM:                getSCMState(envAtExecutionStart, repoRoot),
			User:               getUser(envAtExecutionStart, repoRoot),
			Monorepo:           !singlePackage,
		},
		runType:            runType,
		repoRoot:           repoRoot,
		singlePackage:      singlePackage,
		shouldSave:         shouldSave,
		synthesizedCommand: synthesizedCommand,
	}

	rsm.spacesClient = newSpacesClient(spaceID, spacesClient)
	if rsm.spacesClient.enabled {
		go rsm.spacesClient.start()
		payload := newSpacesRunCreatePayload(&rsm)
		rsm.spacesClient.createRun(payload)
	}

	return rsm
}

// SpacesIsEnabled returns true if this run summary is going to send to a
// spaces backend
func (rsm *Meta) SpacesIsEnabled() bool {
	return rsm.spacesClient.enabled
}

// getPath returns a path to where the runSummary is written.
// The returned path will always be relative to the dir passsed in.
// We don't do a lot of validation, so `../../` paths are allowed.
func (rsm *Meta) getPath() nxpkgpath.AbsoluteSystemPath {
	filename := fmt.Sprintf("%s.json", rsm.RunSummary.ID)
	return rsm.repoRoot.UntypedJoin(filepath.Join(".nxpkg", "runs"), filename)
}

// Close wraps up the RunSummary at the end of a `nxpkg run`.
func (rsm *Meta) Close(ctx context.Context, exitCode int, workspaceInfos workspace.Catalog, ui cli.Ui) error {
	if rsm.runType == runTypeDryJSON || rsm.runType == runTypeDryText {
		return rsm.closeDryRun(workspaceInfos, ui)
	}

	rsm.RunSummary.ExecutionSummary.exitCode = exitCode
	rsm.RunSummary.ExecutionSummary.endedAt = time.Now()

	summary := rsm.RunSummary
	if err := writeChrometracing(summary.ExecutionSummary.profileFilename, ui); err != nil {
		ui.Error(fmt.Sprintf("Error writing tracing data: %v", err))
	}

	// TODO: printing summary to local, writing to disk, and sending to API
	// are all the same thing, we should use a strategy similar to cache save/upload to
	// do this in parallel.

	// Otherwise, attempt to save the summary
	// Warn on the error, but we don't need to throw an error
	if rsm.shouldSave {
		if err := rsm.save(); err != nil {
			ui.Warn(fmt.Sprintf("Error writing run summary: %v", err))
		}
	}

	rsm.printExecutionSummary(ui)
	if rsm.spacesClient.enabled {
		rsm.sendToSpace(ctx, ui)
	} else {
		// Print any errors if the client is not enabled, since it could have
		// been disabled at runtime due to an issue.
		rsm.spacesClient.printErrors(ui)
	}

	return nil
}

func (rsm *Meta) sendToSpace(ctx context.Context, ui cli.Ui) {
	rsm.spacesClient.finishRun(rsm)
	func() {
		_ = spinner.WaitFor(ctx, rsm.spacesClient.Close, ui, "...sending run summary...", 1000*time.Millisecond)
	}()

	rsm.spacesClient.printErrors(ui)

	url := rsm.spacesClient.run.URL
	if url != "" {
		ui.Output(fmt.Sprintf("Run: %s", url))
		ui.Output("")
	}
}

// closeDryRun wraps up the Run Summary at the end of `nxpkg run --dry`.
// Ideally this should be inlined into Close(), but RunSummary doesn't currently
// have context about whether a run was real or dry.
func (rsm *Meta) closeDryRun(workspaceInfos workspace.Catalog, ui cli.Ui) error {
	// Render the dry run as json
	if rsm.runType == runTypeDryJSON {
		rendered, err := rsm.FormatJSON()
		if err != nil {
			return err
		}

		ui.Output(string(rendered))
		return nil
	}

	return rsm.FormatAndPrintText(workspaceInfos, ui)
}

// TrackTask makes it possible for the consumer to send information about the execution of a task.
func (summary *RunSummary) TrackTask(taskID string) (func(outcome executionEventName, err error, exitCode *int), *TaskExecutionSummary) {
	return summary.ExecutionSummary.run(taskID)
}

func (summary *RunSummary) getFailedTasks() []*TaskSummary {
	failed := []*TaskSummary{}

	for _, t := range summary.Tasks {
		if *t.Execution.exitCode != 0 {
			failed = append(failed, t)
		}
	}
	return failed
}

// Save saves the run summary to a file
func (rsm *Meta) save() error {
	json, err := rsm.FormatJSON()
	if err != nil {
		return err
	}

	// summaryPath will always be relative to the dir passsed in.
	// We don't do a lot of validation, so `../../` paths are allowed
	summaryPath := rsm.getPath()

	if err := summaryPath.EnsureDir(); err != nil {
		return err
	}

	return summaryPath.WriteFile(json, 0644)
}

// CloseTask posts the result of the Task to Spaces
func (rsm *Meta) CloseTask(task *TaskSummary, logs []byte) {
	if rsm.spacesClient.enabled {
		rsm.spacesClient.postTask(task, logs)
	}
}

func getUser(envVars env.EnvironmentVariableMap, dir nxpkgpath.AbsoluteSystemPath) string {
	var username string

	if ci.IsCi() {
		vendor := ci.Info()
		username = envVars[vendor.UsernameEnvVar]
	}

	return username
}
