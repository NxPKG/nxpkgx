import { RULES } from "../constants";
import { Project } from "../utils/calculate-inputs";

const project = new Project(process.cwd());
const cacheKey = project.valid() ? project.key() : Math.random();

const settings = {
  nxpkg: {
    cacheKey,
  },
};

const config = {
  settings,
  plugins: ["nxpkg"],
  rules: {
    [`nxpkg/${RULES.noUndeclaredEnvVars}`]: "error",
  },
};

export default config;
