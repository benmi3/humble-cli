#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = "List all your purchased bundles",
    alias = "ls"
)]
pub struct List {
    /// Print bundle with the specified fields only
    #[arg(
        short,
        long,
        long_help = "Print bundle with the specified fields only. This can be used to chain commands together for automation. \
                 If fields are not set, all fields will be printed  \
                 Valid Fields: [key, name, size, claimed] \
                 Use example: --field key --field name "
    )]
    field: Vec<String>,

    /// Show claimed or unclaimed bundles only
    #[arg(
        short,
        long,
        long_help = "Show claimed or unclaimed bundles only. \
                    This is useful if you want to know which games or bundles you have not claimed yet.",
        value_name="value",
        value_parser=["all", "yes", "no"],
        default_value="all"
    )]
    claimed: String,
}
