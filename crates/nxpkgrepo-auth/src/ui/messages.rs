use nxpkgrepo_ui::{BOLD, CYAN, UI};

pub fn print_cli_authorized(user: &str, ui: &UI) {
    println!(
        "
{} Nxpkgrepo CLI authorized for {}
{}
{}
",
        ui.rainbow(">>> Success!"),
        user,
        ui.apply(
            CYAN.apply_to("To connect to your Remote Cache, run the following in any nxpkgrepo:")
        ),
        ui.apply(BOLD.apply_to("  npx nxpkg link"))
    );
}
