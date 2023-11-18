(globalThis.NXPKGPACK = globalThis.NXPKGPACK || []).push(["output/crates_nxpkgpack-tests_tests_snapshot_basic_top-level-await_input_3adb52._.js", {

"[project]/crates/nxpkgpack-tests/tests/snapshot/basic/top-level-await/input/Actions.js [test] (ecmascript)": (({ r: __nxpkgpack_require__, f: __nxpkgpack_require_context__, i: __nxpkgpack_import__, s: __nxpkgpack_esm__, v: __nxpkgpack_export_value__, n: __nxpkgpack_export_namespace__, c: __nxpkgpack_cache__, l: __nxpkgpack_load__, j: __nxpkgpack_dynamic__, p: __nxpkgpack_resolve_absolute_path__, U: __nxpkgpack_relative_url__, R: __nxpkgpack_resolve_module_id_path__, g: global, __dirname, k: __nxpkgpack_refresh__ }) => (() => {
"use strict";

// import() doesn't care about whether a module is an async module or not
__nxpkgpack_esm__({
    "AlternativeCreateUserAction": ()=>AlternativeCreateUserAction,
    "CreateUserAction": ()=>CreateUserAction
});
const UserApi = __nxpkgpack_require__("[project]/crates/nxpkgpack-tests/tests/snapshot/basic/top-level-await/input/UserAPI.js [test] (ecmascript, loader)")(__nxpkgpack_import__);
const CreateUserAction = async (name)=>{
    console.log("Creating user", name);
    // These are normal awaits, because they are in an async function
    const { createUser } = await UserApi;
    await createUser(name);
};
const AlternativeCreateUserAction = async (name)=>{
    const { createUser } = await __nxpkgpack_require__("[project]/crates/nxpkgpack-tests/tests/snapshot/basic/top-level-await/input/UserAPI.js [test] (ecmascript, loader)")(__nxpkgpack_import__);
    await createUser(name);
}; // Note: Using await import() at top-level doesn't make much sense
 //       except in rare cases. It will import modules sequentially.

})()),
"[project]/crates/nxpkgpack-tests/tests/snapshot/basic/top-level-await/input/index.js [test] (ecmascript)": (({ r: __nxpkgpack_require__, f: __nxpkgpack_require_context__, i: __nxpkgpack_import__, s: __nxpkgpack_esm__, v: __nxpkgpack_export_value__, n: __nxpkgpack_export_namespace__, c: __nxpkgpack_cache__, l: __nxpkgpack_load__, j: __nxpkgpack_dynamic__, p: __nxpkgpack_resolve_absolute_path__, U: __nxpkgpack_relative_url__, R: __nxpkgpack_resolve_module_id_path__, g: global, __dirname, k: __nxpkgpack_refresh__ }) => (() => {
"use strict";

var __NXPKGPACK__imported__module__$5b$project$5d2f$crates$2f$nxpkgpack$2d$tests$2f$tests$2f$snapshot$2f$basic$2f$top$2d$level$2d$await$2f$input$2f$Actions$2e$js__$5b$test$5d$__$28$ecmascript$29$__ = __nxpkgpack_import__("[project]/crates/nxpkgpack-tests/tests/snapshot/basic/top-level-await/input/Actions.js [test] (ecmascript)");
"__NXPKGPACK__ecmascript__hoisting__location__";
;
(async ()=>{
    await __NXPKGPACK__imported__module__$5b$project$5d2f$crates$2f$nxpkgpack$2d$tests$2f$tests$2f$snapshot$2f$basic$2f$top$2d$level$2d$await$2f$input$2f$Actions$2e$js__$5b$test$5d$__$28$ecmascript$29$__["CreateUserAction"]("John");
    console.log("created user John");
})();

})()),
}]);

//# sourceMappingURL=crates_nxpkgpack-tests_tests_snapshot_basic_top-level-await_input_3adb52._.js.map