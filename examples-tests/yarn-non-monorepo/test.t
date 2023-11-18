  $ . ${TESTDIR}/../setup.sh non-monorepo yarn
  \d+\.\d+\.\d+ (re)

# run twice and make sure it works
  $ yarn nxpkg build lint --output-logs=errors-only
  yarn run v\d+\.\d+\.\d+ (re)
  \$ (.*)node_modules/.bin/nxpkg build lint --output-logs=errors-only (re)
  \xe2\x80\xa2 Running build, lint (esc)
  \xe2\x80\xa2 Remote caching disabled (esc)
  
   Tasks:    2 successful, 2 total
  Cached:    0 cached, 2 total
    Time:\s*[\.0-9ms]+  (re)
  
  Done in [\.0-9]+m?s\. (re)

  $ yarn nxpkg build lint --output-logs=errors-only
  yarn run v\d+\.\d+\.\d+ (re)
  \$ (.*)node_modules/.bin/nxpkg build lint --output-logs=errors-only (re)
  \xe2\x80\xa2 Running build, lint (esc)
  \xe2\x80\xa2 Remote caching disabled (esc)
  
   Tasks:    2 successful, 2 total
  Cached:    2 cached, 2 total
    Time:\s*[\.0-9ms]+ >>> FULL NXPKG (re)
  
  Done in [\.0-9]+m?s\. (re)

  $ git diff
