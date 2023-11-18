import chalk from "chalk";
import ora from "ora";
import gradient from "gradient-string";

const BLUE = "#0099F7";
const RED = "#F11712";
const YELLOW = "#FFFF00";

export const nxpkgGradient = gradient(BLUE, RED);
export const nxpkgBlue = chalk.hex(BLUE);
export const nxpkgRed = chalk.hex(RED);
export const yellow = chalk.hex(YELLOW);

export const nxpkgLoader = (text: string) =>
  ora({
    text,
    spinner: {
      frames: ["   ", nxpkgBlue(">  "), nxpkgBlue(">> "), nxpkgBlue(">>>")],
    },
  });

export const info = (...args: Array<unknown>) => {
  log(nxpkgBlue.bold(">>>"), ...args);
};

export const bold = (...args: Array<string>) => {
  log(chalk.bold(...args));
};

export const dimmed = (...args: Array<string>) => {
  log(chalk.dim(...args));
};

export const item = (...args: Array<unknown>) => {
  log(nxpkgBlue.bold("  •"), ...args);
};

export const log = (...args: Array<unknown>) => {
  // eslint-disable-next-line no-console -- logger
  console.log(...args);
};

export const warn = (...args: Array<unknown>) => {
  // eslint-disable-next-line no-console -- warn logger
  console.error(yellow.bold(">>>"), ...args);
};

export const error = (...args: Array<unknown>) => {
  // eslint-disable-next-line no-console -- error logger
  console.error(nxpkgRed.bold(">>>"), ...args);
};
