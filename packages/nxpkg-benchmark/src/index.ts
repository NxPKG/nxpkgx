import cp from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import fse from "fs-extra";
import ndjson from "ndjson";
import {
  DEFAULT_EXEC_OPTS,
  getCommitDetails,
  REPO_PATH,
  setup,
  NXPKG_BIN,
} from "./helpers";

const REPETITIONS = 5;

const DEFAULT_CACHE_PATH = path.join(
  REPO_PATH,
  "node_modules",
  ".cache",
  "nxpkg"
);

const ALT_CACHE_PATH = path.join(
  REPO_PATH,
  "node_modules",
  ".cache",
  "nxpkg-benchmark"
);

type Timing = number;

interface Benchmark {
  name: string;
  unit: string;
  value: number;
  range?: string;
  extra?: string;
}

interface TBirdEvent {
  commitSha: string;
  commitTimestamp: Date;
  platform: string;
  benchmark: string;
  durationMs: number;
}

function cleanNxpkgCache(): void {
  if (fs.existsSync(DEFAULT_CACHE_PATH)) {
    console.log("clearing cache");
    fs.rmSync(DEFAULT_CACHE_PATH, { recursive: true });
  }
}

function cleanBuild(): Array<Timing> {
  const timings: Array<Timing> = [];
  const isLocal = process.argv[process.argv.length - 1] === "--local";
  // We aren't really benchmarking this one, it OOMs if run in full parallel
  // on GH actions
  const repetitions = isLocal ? REPETITIONS : 1;
  const concurrency = isLocal ? "" : " --concurrency=1";
  for (let i = 0; i < repetitions; i++) {
    // clean first, we'll leave the cache in place for subsequent builds
    cleanNxpkgCache();
    const start = new Date().getTime();
    cp.execSync(`${NXPKG_BIN} run build${concurrency}`, DEFAULT_EXEC_OPTS);
    const end = new Date().getTime();
    const timing = end - start;
    timings.push(timing);
  }
  return timings;
}

function cachedBuild(): Array<Timing> {
  const timings: Array<Timing> = [];
  for (let i = 0; i < REPETITIONS; i++) {
    const start = new Date().getTime();
    cp.execSync(`${NXPKG_BIN} run build`, DEFAULT_EXEC_OPTS);
    const end = new Date().getTime();
    const timing = end - start;
    timings.push(timing);
  }
  return timings;
}

function saveCache() {
  // Remove any existing backup
  if (fs.existsSync(ALT_CACHE_PATH)) {
    fs.rmSync(ALT_CACHE_PATH, { recursive: true });
  }
  // copy the current cache to the backup
  if (fs.existsSync(DEFAULT_CACHE_PATH)) {
    fse.copySync(DEFAULT_CACHE_PATH, ALT_CACHE_PATH, { recursive: true });
  } else {
    // make an empty cache
    fs.mkdirSync(ALT_CACHE_PATH, { recursive: true });
  }
}

function restoreSavedCache() {
  // Remove any existing cache
  if (fs.existsSync(DEFAULT_CACHE_PATH)) {
    fs.rmSync(DEFAULT_CACHE_PATH, { recursive: true });
  }
  // Copy the backed-up cache to the real cache
  fse.copySync(ALT_CACHE_PATH, DEFAULT_CACHE_PATH, { recursive: true });
}

function cachedBuildWithDelta(): Array<Timing> {
  // Save existing cache just once, we'll restore from it each time
  saveCache();

  // Edit a file in place
  const file = path.join(
    REPO_PATH,
    "packages",
    "crew",
    "important-feature-0",
    "src",
    "lib",
    "important-component-0",
    "important-component-0.tsx"
  );
  const contents = fs.readFileSync(file).toString("utf-8");
  // make a small edit
  const updated = contents.replace("-0!", "-0!!");
  fs.writeFileSync(file, updated);

  const timings: Array<Timing> = [];
  for (let i = 0; i < REPETITIONS; i++) {
    // Make sure we're starting with the cache from before we make the source code edit
    restoreSavedCache();
    const start = new Date().getTime();
    cp.execSync(`${NXPKG_BIN} run build`, DEFAULT_EXEC_OPTS);
    const end = new Date().getTime();
    const timing = end - start;
    timings.push(timing);
  }
  return timings;
}

function cachedBuildWithDependencyChange(): Array<Timing> {
  // Save existing cache just once, we'll restore from it each time
  saveCache();

  // Edit a dependency
  const file = path.join(REPO_PATH, "apps", "navigation", "package.json");
  const contents = JSON.parse(fs.readFileSync(file).toString("utf-8"));
  contents.dependencies["crew-important-feature-0"] = "*";
  fs.writeFileSync(file, JSON.stringify(contents, null, 2));

  const timings: Array<Timing> = [];
  for (let i = 0; i < REPETITIONS; i++) {
    // Make sure we're starting with the cache from before we made the dependency edit
    restoreSavedCache();
    const start = new Date().getTime();
    cp.execSync(`${NXPKG_BIN} run build`, DEFAULT_EXEC_OPTS);
    const end = new Date().getTime();
    const timing = end - start;
    timings.push(timing);
  }
  return timings;
}

class Benchmarks {
  private readonly benchmarks: Array<Benchmark> = [];
  private readonly tbirdEvents: Array<TBirdEvent> = [];

  constructor(
    private readonly benchmarkFile: string,
    private readonly tinybirdFile: string,
    private readonly commitSha: string,
    private readonly commitTimestamp: Date,
    private readonly platform: string
  ) {}

  run(name: string, b: () => Array<Timing>) {
    console.log(name);
    const timings = b();
    const max = Math.max(...timings);
    const min = Math.min(...timings);
    const avg = timings.reduce((memo, t) => memo + t, 0) / timings.length;
    this.benchmarks.push({
      name,
      value: avg,
      unit: "ms",
      range: String(max - min),
    });
    timings.forEach((t) => {
      this.tbirdEvents.push({
        commitSha: this.commitSha,
        commitTimestamp: this.commitTimestamp,
        platform: this.platform,
        benchmark: name,
        durationMs: t,
      });
    });
  }

  flush() {
    console.log(JSON.stringify(this.benchmarks, null, 2));
    fs.writeFileSync(
      this.benchmarkFile,
      JSON.stringify(this.benchmarks, null, 2)
    );
    const stream = ndjson.stringify();
    const fd = fs.openSync(this.tinybirdFile, "w");
    stream.on("data", (line) => {
      fs.writeSync(fd, line as string);
    });
    this.tbirdEvents.forEach((t) => {
      stream.write(t);
    });
    stream.end();
    fs.closeSync(fd);
  }
}

cp.execSync(`${NXPKG_BIN} --version`, { stdio: "inherit" });

const { commitSha, commitTimestamp } = getCommitDetails();
const platform = process.env.RUNNER_OS ?? "unknown";

console.log("setup");
setup();
const benchmark = new Benchmarks(
  "benchmarks.json",
  "tinybird.ndjson",
  commitSha,
  commitTimestamp,
  platform
);
benchmark.run("Clean Build", cleanBuild);
benchmark.run("Cached Build - No Change", cachedBuild);
benchmark.run("Cached Build - Code Change", cachedBuildWithDelta);
benchmark.run(
  "Cached Build - Dependency Change",
  cachedBuildWithDependencyChange
);
benchmark.flush();
