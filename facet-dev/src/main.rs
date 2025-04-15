use facet_ansi::Stylize as _;
use log::{LevelFilter, debug, error, warn};
use similar::ChangeTag;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

mod readme;
mod sample;
mod tuples;

#[derive(Debug, Clone)]
pub struct Job {
    pub path: PathBuf,
    pub old_content: Option<Vec<u8>>,
    pub new_content: Vec<u8>,
}

impl Job {
    pub fn is_noop(&self) -> bool {
        match &self.old_content {
            Some(old) => &self.new_content == old,
            None => self.new_content.is_empty(),
        }
    }

    /// Computes a summary of the diff between old_content and new_content.
    /// Returns (num_plus, num_minus): plus lines (insertions), minus lines (deletions).
    pub fn diff_plus_minus(&self) -> (usize, usize) {
        use similar::TextDiff;
        let old = match &self.old_content {
            Some(bytes) => String::from_utf8_lossy(bytes),
            None => "".into(),
        };
        let new = String::from_utf8_lossy(&self.new_content);
        let diff = TextDiff::from_lines(&old, &new);
        let mut plus = 0;
        let mut minus = 0;
        for change in diff.iter_all_changes() {
            match change.tag() {
                ChangeTag::Insert => plus += 1,
                ChangeTag::Delete => minus += 1,
                ChangeTag::Equal => {}
            }
        }
        (plus, minus)
    }

    pub fn show_diff(&self) {
        use facet_ansi::Stylize as _;
        use similar::{ChangeTag, TextDiff};

        let context_lines = 3;

        let old = match &self.old_content {
            Some(bytes) => String::from_utf8_lossy(bytes),
            None => "".into(),
        };
        let new = String::from_utf8_lossy(&self.new_content);
        let diff = TextDiff::from_lines(&old, &new);

        // Collect the changes for random access
        let changes: Vec<_> = diff.iter_all_changes().collect();

        // Identify the indices of changes (added/removed lines)
        let mut change_indices = vec![];
        for (i, change) in changes.iter().enumerate() {
            match change.tag() {
                ChangeTag::Insert | ChangeTag::Delete => change_indices.push(i),
                _ => {}
            }
        }

        let mut show_line = vec![false; changes.len()];
        // Mark lines to show: up to context_lines before/after each change
        for &idx in &change_indices {
            let start = idx.saturating_sub(context_lines);
            let end = (idx + context_lines + 1).min(changes.len());
            #[allow(clippy::needless_range_loop)]
            for i in start..end {
                show_line[i] = true;
            }
        }

        // Always show a few lines at the top and bottom of the diff for context,
        // in case the first or last lines are not changes.
        #[allow(clippy::needless_range_loop)]
        for i in 0..context_lines.min(changes.len()) {
            show_line[i] = true;
        }
        #[allow(clippy::needless_range_loop)]
        for i in changes.len().saturating_sub(context_lines)..changes.len() {
            show_line[i] = true;
        }

        let mut last_was_ellipsis = false;
        for (i, change) in changes.iter().enumerate() {
            if show_line[i] {
                match change.tag() {
                    ChangeTag::Insert => print!("{}", format!("    +{}", change).green()),
                    ChangeTag::Delete => print!("{}", format!("    -{}", change).red()),
                    ChangeTag::Equal => print!("{}", format!("    {}", change).dim()),
                }
                last_was_ellipsis = false;
            } else if !last_was_ellipsis {
                println!("{}", "    ...".dim());
                last_was_ellipsis = true;
            }
        }
        println!();
    }

    /// Applies the job by writing out the new_content to path and staging the file.
    pub fn apply(&self) -> std::io::Result<()> {
        use std::fs;
        use std::process::Command;
        fs::write(&self.path, &self.new_content)?;
        // Now stage it, best effort
        let _ = Command::new("git").arg("add").arg(&self.path).status();
        Ok(())
    }
}

pub fn enqueue_readme_jobs(sender: std::sync::mpsc::Sender<Job>) {
    use std::path::Path;

    let workspace_dir = std::env::current_dir().unwrap();
    let entries = match fs_err::read_dir(&workspace_dir) {
        Ok(e) => e,
        Err(e) => {
            error!("Failed to read workspace directory ({})", e);
            return;
        }
    };

    let template_name = "README.md.in";

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                warn!("Skipping entry: {e}");
                continue;
            }
        };
        let crate_path = entry.path();

        if !crate_path.is_dir()
            || crate_path.file_name().is_some_and(|name| {
                let name = name.to_string_lossy();
                name.starts_with('.') || name.starts_with('_')
            })
        {
            continue;
        }

        let dir_name = crate_path.file_name().unwrap().to_string_lossy();
        if dir_name == "target" {
            continue;
        }

        let cargo_toml_path = crate_path.join("Cargo.toml");
        if !cargo_toml_path.exists() {
            continue;
        }

        let crate_name = dir_name.to_string();

        let template_path = if crate_name == "facet" {
            Path::new(template_name).to_path_buf()
        } else {
            crate_path.join(template_name)
        };

        if template_path.exists() {
            // Read the template file
            let template_input = match fs::read_to_string(&template_path) {
                Ok(s) => s,
                Err(e) => {
                    error!("Failed to read template {}: {e}", template_path.display());
                    continue;
                }
            };

            // Generate the README content using readme::generate
            let readme_content = readme::generate(readme::GenerateReadmeOpts {
                crate_name: crate_name.clone(),
                input: template_input,
            });

            // Determine the README.md output path
            let readme_path = crate_path.join("README.md");

            // Read old_content from README.md if exists, otherwise None
            let old_content = fs::read(&readme_path).ok();

            // Build the job
            let job = Job {
                path: readme_path,
                old_content,
                new_content: readme_content.into_bytes(),
            };

            // Send job
            if let Err(e) = sender.send(job) {
                error!("Failed to send job: {e}");
            }
        } else {
            error!("🚫 Missing template: {}", template_path.display().red());
        }
    }

    // Also handle the workspace README (the "facet" crate at root)
    let workspace_template_path = workspace_dir.join(template_name);
    if workspace_template_path.exists() {
        // Read the template file
        let template_input = match fs::read_to_string(&workspace_template_path) {
            Ok(s) => s,
            Err(e) => {
                error!(
                    "Failed to read template {}: {e}",
                    workspace_template_path.display()
                );
                return;
            }
        };

        // Generate the README content using readme::generate
        let readme_content = readme::generate(readme::GenerateReadmeOpts {
            crate_name: "facet".to_string(),
            input: template_input,
        });

        // Determine the README.md output path
        let readme_path = workspace_dir.join("README.md");

        // Read old_content from README.md if exists, otherwise None
        let old_content = fs::read(&readme_path).ok();

        // Build the job
        let job = Job {
            path: readme_path,
            old_content,
            new_content: readme_content.into_bytes(),
        };

        // Send job
        if let Err(e) = sender.send(job) {
            error!("Failed to send workspace job: {e}");
        }
    } else {
        error!(
            "🚫 {}",
            format!(
                "Template file {} not found for workspace. We looked at {}",
                template_name,
                workspace_template_path.display()
            )
            .red()
        );
    }
}

pub fn enqueue_tuple_job(sender: std::sync::mpsc::Sender<Job>) {
    use std::time::Instant;

    // Path where tuple impls should be written
    let base_path = Path::new("facet-core/src/impls_core/tuple.rs");

    debug!("Generating tuple impls for {}", base_path.display().blue());

    // Generate the tuple impls code
    let start = Instant::now();
    let output = tuples::generate();
    let content = output.into_bytes();
    let duration = start.elapsed();
    let size_mb = (content.len() as f64) / (1024.0 * 1024.0);
    let secs = duration.as_secs_f64();
    let mbps = if secs > 0.0 { size_mb / secs } else { 0.0 };
    debug!(
        "Generated and formatted tuple impls for {}: {:.2} MiB in {:.2} s ({:.2} MiB/s)",
        base_path.display().blue(),
        size_mb,
        secs,
        mbps.bright_magenta()
    );

    // Attempt to read existing file
    let old_content = fs::read(base_path).ok();

    let job = Job {
        path: base_path.to_path_buf(),
        old_content,
        new_content: content,
    };

    if let Err(e) = sender.send(job) {
        error!("Failed to send tuple job: {e}");
    }
}

pub fn enqueue_sample_job(sender: std::sync::mpsc::Sender<Job>) {
    use log::trace;
    use std::time::Instant;

    // Path where sample generated code should be written
    let rel_path = std::path::PathBuf::from("facet/src/sample_generated_code.rs");
    let workspace_dir = std::env::current_dir().unwrap();
    let target_path = workspace_dir.join(&rel_path);

    trace!(
        "Expanding sample code at {:?}",
        target_path.display().blue()
    );
    let start = Instant::now();

    // Generate the sample expanded and formatted code
    let code = sample::cargo_expand_and_format();
    let content = code.into_bytes();
    let size_mb = (content.len() as f64) / (1024.0 * 1024.0);

    let duration = start.elapsed();
    let secs = duration.as_secs_f64();
    let mbps = if secs > 0.0 { size_mb / secs } else { 0.0 };

    debug!(
        "Generated and formatted sample code for {}: {:.2} MiB in {:.2} s ({:.2} MiB/s)",
        rel_path.display().blue(),
        size_mb,
        secs,
        mbps.bright_magenta()
    );

    // Attempt to read existing file
    let old_content = fs::read(&target_path).ok();

    let job = Job {
        path: target_path,
        old_content,
        new_content: content,
    };

    if let Err(e) = sender.send(job) {
        error!("Failed to send sample job: {e}");
    }
}

pub fn enqueue_rustfmt_jobs(sender: std::sync::mpsc::Sender<Job>, staged_files: &StagedFiles) {
    use log::trace;
    use std::time::Instant;

    for path in &staged_files.clean {
        // Only process .rs files
        if let Some(ext) = path.extension() {
            if ext != "rs" {
                continue;
            }
        } else {
            continue;
        }

        trace!("rustfmt: formatting {}", path.display());

        let original = match fs::read(path) {
            Ok(val) => val,
            Err(e) => {
                error!(
                    "{} {}: {}",
                    "❌".red(),
                    path.display().to_string().blue(),
                    format!("Failed to read: {e}").dim()
                );
                continue;
            }
        };

        let size_mb = (original.len() as f64) / (1024.0 * 1024.0);

        // Format the content via rustfmt (edition 2024)
        let start = Instant::now();
        let cmd = Command::new("rustfmt")
            .arg("--edition")
            .arg("2024")
            .arg("--emit")
            .arg("stdout")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        let mut cmd = match cmd {
            Ok(child) => child,
            Err(e) => {
                error!("Failed to spawn rustfmt for {}: {}", path.display(), e);
                continue;
            }
        };

        // Write source to rustfmt's stdin
        {
            let mut stdin = cmd.stdin.take().expect("Failed to take rustfmt stdin");
            if stdin.write_all(&original).is_err() {
                error!(
                    "{} {}: {}",
                    "❌".red(),
                    path.display().to_string().blue(),
                    "Failed to write src to rustfmt".dim()
                );
                continue;
            }
        }

        let output = match cmd.wait_with_output() {
            Ok(out) => out,
            Err(e) => {
                error!("Failed to get rustfmt output for {}: {}", path.display(), e);
                continue;
            }
        };

        let duration = start.elapsed();
        let secs = duration.as_secs_f64();
        let mbps = if secs > 0.0 { size_mb / secs } else { 0.0 };
        debug!(
            "rustfmt: {} formatted {:.2} MiB in {:.2} s ({:.2} MiB/s)",
            path.display(),
            size_mb,
            secs,
            mbps.magenta()
        );

        if !output.status.success() {
            error!(
                "{} {}: rustfmt failed\n{}\n{}",
                "❌".red(),
                path.display().to_string().blue(),
                String::from_utf8_lossy(&output.stderr).dim(),
                String::from_utf8_lossy(&output.stdout).dim()
            );
            continue;
        }

        let formatted = output.stdout;

        // Only enqueue a job if the formatted output is different
        if formatted != original {
            let job = Job {
                path: path.clone(),
                old_content: Some(original),
                new_content: formatted,
            };
            if let Err(e) = sender.send(job) {
                error!("Failed to send rustfmt job for {}: {}", path.display(), e);
            }
        }
    }
}

pub fn show_jobs_and_apply_if_consent_is_given(jobs: &mut [Job]) {
    use console::{Emoji, style};
    use std::io::{self, Write};

    // Use menu crate (https://docs.rs/menu/)
    use menu::{Item, ItemType, Menu, Runner};

    // Emojis for display
    static ACTION_REQUIRED: Emoji<'_, '_> = Emoji("🚧", "");
    static DIFF: Emoji<'_, '_> = Emoji("📝", "");
    static OK: Emoji<'_, '_> = Emoji("✅", "");
    static CANCEL: Emoji<'_, '_> = Emoji("❌", "");

    jobs.sort_by_key(|job| job.path.clone());

    if jobs.is_empty() {
        println!(
            "{}",
            style("All generated files are up-to-date and all Rust files are formatted properly")
                .green()
                .bold()
        );
        return;
    }

    println!(
        "\n{}\n{}\n",
        style(format!(
            "{} GENERATION CHANGES {}",
            ACTION_REQUIRED, ACTION_REQUIRED
        ))
        .on_black()
        .bold()
        .yellow()
        .italic()
        .underlined(),
        style(format!(
            "The following {} file{} would be updated/generated:",
            jobs.len(),
            if jobs.len() == 1 { "" } else { "s" }
        ))
        .magenta()
    );
    for (idx, job) in jobs.iter().enumerate() {
        let (plus, minus) = job.diff_plus_minus();
        println!(
            "  {}. {} {}{}",
            style(idx + 1).bold().cyan(),
            style(job.path.display()).yellow(),
            if plus > 0 {
                format!("+{}", plus).green().to_string()
            } else {
                String::new()
            },
            if minus > 0 {
                format!(" -{}", minus).red().to_string()
            } else {
                String::new()
            }
        );
    }

    // Handler functions as standalone functions (for menu crate API)
    fn apply_changes_callback(_args: &menu::Parameter, user_data: &mut Vec<Job>) -> Result<(), menu::Error> {
        use console::style;
        static OK: Emoji<'_, '_> = Emoji("✅", "");
        static CANCEL: Emoji<'_, '_> = Emoji("❌", "");
        println!();
        for job in user_data.iter() {
            if let Err(e) = std::fs::write(&job.path, &job.new_content) {
                eprintln!("{} Failed to write {}: {}", CANCEL, job.path.display(), e);
                std::process::exit(1);
            } else {
                let add_status = std::process::Command::new("git")
                    .arg("add")
                    .arg(&job.path)
                    .status();
                if let Err(e) = add_status {
                    eprintln!("{} Failed to stage {}: {}", CANCEL, job.path.display(), e);
                    std::process::exit(1);
                }
                println!(
                    "{} {} updated and staged.",
                    OK,
                    style(job.path.display()).green()
                );
            }
        }
        println!("{} All changes applied and staged.", OK);
        std::process::exit(0);
    }
    fn show_diffs_callback(_args: &menu::Parameter, user_data: &mut Vec<Job>) -> Result<(), menu::Error> {
        use console::style;
        static DIFF: Emoji<'_, '_> = Emoji("📝", "");
        println!("\n{}\n", style("Showing diffs...").bold());
        for job in user_data.iter() {
            println!(
                "\n{} Diff for {}:",
                DIFF,
                style(job.path.display()).bold().blue()
            );
            job.show_diff();
        }
        println!("\n-- End of diffs --");
        Ok(())
    }
    fn cancel_callback(_args: &menu::Parameter, _user_data: &mut Vec<Job>) -> Result<(), menu::Error> {
        static CANCEL: Emoji<'_, '_> = Emoji("❌", "");
        println!("{} Changes were not applied.", CANCEL);
        std::process::exit(0);
    }
    fn abort_callback(_args: &menu::Parameter, _user_data: &mut Vec<Job>) -> Result<(), menu::Error> {
        static CANCEL: Emoji<'_, '_> = Emoji("❌", "");
        println!("{} Aborted.", CANCEL);
        std::process::exit(1);
    }

    // The menu expects a mutable Vec for parameter passing; collect jobs into a Vec
    let mut jobs_vec = jobs.to_vec();

    // Define menu items
    static MENU_LABEL: &str = "What do you want to do?";
    let apply_item = Item {
        command: "apply",
        help: Some("🚀 Apply the above changes"),
        item_type: ItemType::Callback {
            function: apply_changes_callback,
            parameters: &[],
        },
    };
    let diff_item = Item {
        command: "diff",
        help: Some("🔍 Diff: Show details of all diffs"),
        item_type: ItemType::Callback {
            function: show_diffs_callback,
            parameters: &[],
        },
    };
    let cancel_item = Item {
        command: "cancel",
        help: Some("🛑 Cancel: Abort without changing files"),
        item_type: ItemType::Callback {
            function: cancel_callback,
            parameters: &[],
        },
    };
    let abort_item = Item {
        command: "abort",
        help: Some("💥 Abort: Exit with error"),
        item_type: ItemType::Callback {
            function: abort_callback,
            parameters: &[],
        },
    };
    let items = [&apply_item, &diff_item, &cancel_item, &abort_item];
    let menu = Menu {
        label: MENU_LABEL,
        items: &items,
        entry: None,
        exit: None,
    };

    // Create a buffer for input
    let mut buffer = [0u8; 64];
    let mut runner = Runner::new(menu, &mut buffer, io::stdout(), &mut jobs_vec);

    // Menu description
    println!();
    let joined_cmds = items
        .iter()
        .map(|i| i.command.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    println!(
        "Type one of: {}.",
        joined_cmds
    );
    println!("Or type {} for help.", style("help").cyan());

    // Main menu interaction loop
    loop {
        print!("{} > ", style(MENU_LABEL).dim());
        io::stdout().flush().ok();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input.");
            continue;
        }
        let input_trim = input.trim();
        // Process the input
        match runner.run(input_trim.as_bytes()) {
            Ok(_) => {
                // Continue running or exit (callbacks handle exit)
            }
            Err(e) => {
                println!("{}", style(format!("Menu error: {:?}", e)).red());
            }
        }
    }
}


#[derive(Debug, Clone)]
struct Options {
    check: bool,
}

fn main() {
    facet_testhelpers::setup();
    // Accept allowed log levels: trace, debug, error, warn, info
    if let Ok(log_level) = std::env::var("RUST_LOG") {
        let allowed = ["trace", "debug", "error", "warn", "info"];
        let log_level_lc = log_level.to_lowercase();
        if allowed.contains(&log_level_lc.as_str()) {
            let level = match log_level_lc.as_str() {
                "trace" => LevelFilter::Trace,
                "debug" => LevelFilter::Debug,
                "info" => LevelFilter::Info,
                "warn" => LevelFilter::Warn,
                "error" => LevelFilter::Error,
                _ => LevelFilter::Info,
            };
            log::set_max_level(level);
        } else {
            // Default to Info if not allowed
            log::set_max_level(LevelFilter::Info);
        }
    }

    // Parse opts from args
    let args: Vec<String> = std::env::args().collect();
    let mut opts = Options { check: false };
    for arg in &args[1..] {
        if arg == "--check" || arg == "-c" {
            opts.check = true;
        }
    }

    // Check if current directory has a Cargo.toml with [workspace]
    let cargo_toml_path = std::env::current_dir().unwrap().join("Cargo.toml");
    let cargo_toml_content =
        fs_err::read_to_string(cargo_toml_path).expect("Failed to read Cargo.toml");
    if !cargo_toml_content.contains("[workspace]") {
        error!(
            "🚫 {}",
            "Cargo.toml does not contain [workspace] (you must run codegen from the workspace root)"
                .red()
        );
        std::process::exit(1);
    }

    let staged_files = match collect_staged_files() {
        Ok(sf) => sf,
        Err(e) => {
            error!("Failed to collect staged files: {e}");
            std::process::exit(1);
        }
    };

    // Use a channel to collect jobs from all tasks.
    use std::sync::mpsc;
    let (sender, receiver) = mpsc::channel();

    // Start threads for each codegen job enqueuer
    let send1 = sender.clone();
    let handle_readme = std::thread::spawn(move || {
        enqueue_readme_jobs(send1);
    });
    let send2 = sender.clone();
    let handle_tuple = std::thread::spawn(move || {
        enqueue_tuple_job(send2);
    });
    let send3 = sender.clone();
    let handle_sample = std::thread::spawn(move || {
        enqueue_sample_job(send3);
    });
    // Rustfmt job: enqueue formatting for staged .rs files
    let send4 = sender.clone();
    let handle_rustfmt = std::thread::spawn(move || {
        enqueue_rustfmt_jobs(send4, &staged_files);
    });

    // Drop original sender so the channel closes when all workers finish
    drop(sender);

    // Collect jobs
    let mut jobs: Vec<Job> = Vec::new();
    for job in receiver {
        jobs.push(job);
    }

    // Wait for all job enqueuers to finish
    handle_readme.join().unwrap();
    handle_tuple.join().unwrap();
    handle_sample.join().unwrap();
    handle_rustfmt.join().unwrap();

    if jobs.is_empty() {
        println!("{}", "No codegen changes detected.".green().bold());
        return;
    }

    if opts.check {
        let mut any_diffs = false;
        for job in &jobs {
            // Compare old_content (current file content) to new_content (generated content)
            let disk_content = std::fs::read(&job.path).unwrap_or_default();
            if disk_content != job.new_content {
                error!(
                    "Diff detected in {}",
                    job.path.display().to_string().yellow().bold()
                );
                any_diffs = true;
            }
        }
        if any_diffs {
            // Print a big banner with error message about generated files
            error!(
                "┌────────────────────────────────────────────────────────────────────────────┐"
            );
            error!(
                "│                                                                            │"
            );
            error!(
                "│  GENERATED FILES HAVE CHANGED - RUN `just codegen` TO UPDATE THEM          │"
            );
            error!(
                "│                                                                            │"
            );
            error!(
                "│  For README.md files:                                                      │"
            );
            error!(
                "│                                                                            │"
            );
            error!(
                "│  • Don't edit README.md directly - edit the README.md.in template instead  │"
            );
            error!(
                "│  • Then run `just codegen` to regenerate the README.md files               │"
            );
            error!(
                "│  • A pre-commit hook is set up by cargo-husky to do just that              │"
            );
            error!(
                "│                                                                            │"
            );
            error!(
                "│  See CONTRIBUTING.md                                                       │"
            );
            error!(
                "│                                                                            │"
            );
            error!(
                "└────────────────────────────────────────────────────────────────────────────┘"
            );
            std::process::exit(1);
        } else {
            println!("{}", "✅ All generated files up to date.".green().bold());
        }
    } else {
        // Remove no-op jobs (where the content is unchanged).
        jobs.retain(|job| !job.is_noop());
        show_jobs_and_apply_if_consent_is_given(&mut jobs);
    }
}

#[derive(Debug)]
pub struct StagedFiles {
    /// Files that are staged (in the index) and not dirty (working tree matches index).
    pub clean: Vec<PathBuf>,
    /// Files that are staged and dirty (index does NOT match working tree).
    pub dirty: Vec<PathBuf>,
    /// Files that are untracked or unstaged (not added to the index).
    pub unstaged: Vec<PathBuf>,
}

// -- Formatting support types --

#[derive(Debug)]
pub struct FormatCandidate {
    pub path: PathBuf,
    pub original: Vec<u8>,
    pub formatted: Vec<u8>,
    pub diff: Option<String>,
}

pub fn collect_staged_files() -> io::Result<StagedFiles> {
    // Run `git status --porcelain`
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()?;

    if !output.status.success() {
        panic!("Failed to run `git status --porcelain`");
    }
    let stdout = String::from_utf8_lossy(&output.stdout);

    log::trace!("Parsing {} output:", "`git status --porcelain`".blue());
    log::trace!("---\n{}\n---", stdout);

    let mut clean = Vec::new();
    let mut dirty = Vec::new();
    let mut unstaged = Vec::new();

    for line in stdout.lines() {
        log::trace!("Parsing git status line: {:?}", line.dim());
        // E.g. "M  src/main.rs", "A  foo.rs", "AM foo/bar.rs"
        if line.len() < 3 {
            log::trace!("Skipping short line: {:?}", line.dim());
            continue;
        }
        let x = line.chars().next().unwrap();
        let y = line.chars().nth(1).unwrap();
        let path = line[3..].to_string();

        log::trace!(
            "x: {:?}, y: {:?}, path: {:?}",
            x.magenta(),
            y.cyan(),
            (&path).dim()
        );

        // Staged and not dirty (to be formatted/committed)
        if x != ' ' && x != '?' && y == ' ' {
            log::debug!(
                "{} {}",
                "-> clean (staged, not dirty):".green().bold(),
                path.as_str().blue()
            );
            clean.push(PathBuf::from(&path));
        }
        // Staged + dirty (index does not match worktree; skip and warn)
        else if x != ' ' && x != '?' && y != ' ' {
            log::debug!(
                "{} {}",
                "-> dirty (staged and dirty):".yellow().bold(),
                path.as_str().blue()
            );
            dirty.push(PathBuf::from(&path));
        }
        // Untracked or unstaged files (may be useful for warning)
        else if x == '?' {
            log::debug!(
                "{} {}",
                "-> unstaged/untracked:".cyan().bold(),
                path.as_str().blue()
            );
            unstaged.push(PathBuf::from(&path));
        } else {
            log::debug!("{} {}", "-> not categorized:".red(), path.as_str().blue());
        }
    }
    Ok(StagedFiles {
        clean,
        dirty,
        unstaged,
    })
}
