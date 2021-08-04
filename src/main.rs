use anyhow::{bail, Result};
use indoc::indoc;
use std::collections::VecDeque;
use std::env;
use std::fmt;
use std::io::prelude::*;
use std::iter::Peekable;
use std::path::PathBuf;

fn _fmt_defs(
    shared: bool,
    request: bool,
) -> (&'static str, &'static str, &'static str, &'static str) {
    let (s_sig, s_arg) = if shared {
        ("Shared.Model ->", "shared")
    } else {
        ("", "")
    };

    let (r_sig, r_arg) = if request {
        ("Request.With Params ->", "req")
    } else {
        ("", "")
    };

    (s_sig, r_sig, s_arg, r_arg)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PageType {
    Static,
    Sandbox,
    Element,
    Advanced,
}

impl PageType {
    fn from(string: &str) -> Option<Self> {
        match string {
            "static" => Some(Self::Static),
            "sandbox" => Some(Self::Sandbox),
            "element" => Some(Self::Element),
            "advanced" => Some(Self::Advanced),
            _ => None,
        }
    }

    fn exposing_template(self) -> &'static str {
        match self {
            PageType::Static => "page",
            PageType::Sandbox => "page, Model, Msg",
            PageType::Element => "page, Model, Msg",
            PageType::Advanced => "page, Model, Msg",
        }
    }

    fn page_template(self, shared: bool, request: bool) -> String {
        let (_, _, s_arg, r_arg) = _fmt_defs(shared, request);
        match self {
            PageType::Static => format!(
                indoc! {r###"
                page : Shared.Model -> Request.With Params -> Page
                page shared req =
                    Page.static
                        {{ view = view {s_arg} {r_arg}
                        }}
                "###},
                s_arg = s_arg,
                r_arg = r_arg
            ),

            PageType::Sandbox => format!(
                indoc! {r###"
                page : Shared.Model -> Request.With Params -> Page.With Model Msg
                page shared req =
                    Page.sandbox
                        {{ init = init {s_arg} {r_arg}
                        , update = update {s_arg} {r_arg}
                        , view = view {s_arg} {r_arg}
                        }}
                "###},
                s_arg = s_arg,
                r_arg = r_arg
            ),

            PageType::Element => format!(
                indoc! {r###"
                page : Shared.Model -> Request.With Params -> Page.With Model Msg
                page shared req =
                    Page.element
                        {{ init = init {s_arg} {r_arg}
                        , update = update {s_arg} {r_arg}
                        , view = view {s_arg} {r_arg}
                        , subscriptions = subscriptions {s_arg} {r_arg}
                        }}
                "###},
                s_arg = s_arg,
                r_arg = r_arg
            ),

            PageType::Advanced => format!(
                indoc! {r###"
                page : Shared.Model -> Request.With Params -> Page.With Model Msg
                page shared req =
                    Page.advanced
                        {{ init = init {s_arg} {r_arg}
                        , update = update {s_arg} {r_arg}
                        , view = view {s_arg} {r_arg}
                        , subscriptions = subscriptions {s_arg} {r_arg}
                        }}
                "###},
                s_arg = s_arg,
                r_arg = r_arg
            ),
        }
    }

    fn init_template(self, shared: bool, request: bool) -> String {
        let (s_sig, r_sig, s_arg, r_arg) = _fmt_defs(shared, request);

        match self {
            PageType::Static => "".into(),

            PageType::Sandbox => format!(
                indoc! {r###"
                init : {s_sig} {r_sig} Model
                init {s_arg} {r_arg} =
                    {{}}
                "###},
                s_sig = s_sig,
                s_arg = s_arg,
                r_sig = r_sig,
                r_arg = r_arg
            ),

            PageType::Element => format!(
                indoc! {r###"
                init : {s_sig} {r_sig} (Model, Cmd Msg)
                init {s_arg} {r_arg} =
                    ({{}}, Cmd.none)
                "###},
                s_sig = s_sig,
                s_arg = s_arg,
                r_sig = r_sig,
                r_arg = r_arg
            ),

            PageType::Advanced => format!(
                indoc! {r###"
                init : {s_sig} {r_sig} (Model, Effect Msg)
                init {s_arg} {r_arg} =
                    ({{}}, Effect.none)
                "###},
                s_sig = s_sig,
                s_arg = s_arg,
                r_sig = r_sig,
                r_arg = r_arg
            ),
        }
    }

    fn update_template(self: Self, shared: bool, request: bool) -> String {
        let (s_sig, r_sig, s_arg, r_arg) = _fmt_defs(shared, request);

        match self {
            PageType::Static => "".into(),

            PageType::Sandbox => format!(
                indoc! {r###"
                update : {s_sig} {r_sig} Msg -> Model -> Model
                update {s_arg} {r_arg} msg model =
                    case msg of
                        _ ->
                            model
                "###},
                s_sig = s_sig,
                s_arg = s_arg,
                r_sig = r_sig,
                r_arg = r_arg
            ),

            PageType::Element => format!(
                indoc! {r###"
                update : {s_sig} {r_sig} Msg -> Model -> ( Model, Cmd Msg )
                update {s_arg} {r_arg} msg model =
                    case msg of
                        _ ->
                            ( model, Cmd.none )
                "###},
                s_sig = s_sig,
                s_arg = s_arg,
                r_sig = r_sig,
                r_arg = r_arg
            ),

            PageType::Advanced => format!(
                indoc! {r###"
                update : {s_sig} {r_sig} Msg -> Model -> ( Model, Effect Msg )
                update {s_arg} {r_arg} msg model =
                    case msg of
                        _ ->
                            ( model, Effect.none )
                "###},
                s_sig = s_sig,
                s_arg = s_arg,
                r_sig = r_sig,
                r_arg = r_arg
            ),
        }
    }

    fn view_template(self, shared: bool, request: bool) -> String {
        let (s_sig, r_sig, s_arg, r_arg) = _fmt_defs(shared, request);

        match self {
            PageType::Static => format!(
                indoc! {r###"
                view : {s_sig} {r_sig} View msg
                view {s_arg} {r_arg} =
                    View.placeholder "Hello World"
                "###},
                s_sig = s_sig,
                s_arg = s_arg,
                r_sig = r_sig,
                r_arg = r_arg
            ),

            PageType::Sandbox | PageType::Element | PageType::Advanced => format!(
                indoc! {r###"
                view : {s_sig} {r_sig} Model -> View Msg
                view {s_arg} {r_arg} model =
                    View.placeholder "Hello World"
                "###},
                s_sig = s_sig,
                s_arg = s_arg,
                r_sig = r_sig,
                r_arg = r_arg
            ),
        }
    }

    fn subscriptions_template(self, shared: bool, request: bool) -> String {
        let (s_sig, r_sig, s_arg, r_arg) = _fmt_defs(shared, request);

        match self {
            PageType::Static | PageType::Sandbox => "".into(),

            PageType::Element | PageType::Advanced => format!(
                indoc! {r###"
                subscriptions : {s_sig} {r_sig} Model -> Sub Msg
                subscriptions {s_arg} {r_arg} model =
                    Sub.none
                "###},
                s_sig = s_sig,
                s_arg = s_arg,
                r_sig = r_sig,
                r_arg = r_arg
            ),
        }
    }
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    exposing: Option<String>,
}

impl Module {
    fn parse(line: String, lines: &mut Peekable<impl Iterator<Item = String>>) -> Result<Self> {
        let name = line.split_whitespace().skip(1).next().map_or_else(
            || bail!(format!("Failed to parse: {}", &line)),
            |l| Ok(l.to_string()),
        )?;

        if !line.contains("exposing") {
            return Ok(Self {
                name,
                exposing: None,
            });
        }

        let mut exposing: String = line
            .chars()
            .skip_while(|c| c != &'(')
            .skip(1)
            .take_while(|c| c != &')')
            .collect();

        if !line.ends_with(')') {
            while let Some(line) = lines.next() {
                exposing.extend(line.chars().take_while(|c| c != &')'));
                if line.ends_with(')') {
                    break;
                }
            }
        }

        Ok(Self {
            name,
            exposing: Some(exposing),
        })
    }
}

#[derive(Debug, Clone)]
struct Function {
    lines: Vec<String>,
}
impl Function {
    fn parse(
        line: String,
        next_lines: &mut Peekable<impl Iterator<Item = String>>,
    ) -> Result<Self> {
        let name = line.split_whitespace().next().map_or_else(
            || bail!(format!("Failed to parse: {}", &line)),
            |l| Ok(l.to_string()),
        )?;

        let mut lines = vec![line];
        while let Some(line) = next_lines.peek() {
            if line.trim().is_empty()
                || line.starts_with(' ')
                || line.starts_with('\t')
                || line.starts_with(&format!("{} ", &name))
            {
                lines.push(line.into());
                next_lines.next();
            } else {
                break;
            }
        }

        let func = Self { lines };
        Ok(func)
    }
}

#[derive(Debug, Clone)]
enum Block {
    Module(Module),
    Import(Module),
    Init(Function),
    View(Function),
    Update(Function),
    Subscriptions(Function),
    Page(Function),
    Other(String),
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Module(m) => match &m.exposing {
                Some(exposing) => {
                    writeln!(f, "module {} exposing ({})", m.name, exposing)?;
                }
                None => {
                    writeln!(f, "module {}", m.name)?;
                }
            },

            Self::Import(m) => match &m.exposing {
                Some(exposing) => {
                    writeln!(f, "import {} exposing ({})", m.name, exposing)?;
                }
                None => {
                    writeln!(f, "import {}", m.name)?;
                }
            },

            Self::Init(b)
            | Self::Update(b)
            | Self::View(b)
            | Self::Subscriptions(b)
            | Self::Page(b) => {
                writeln!(f, "")?;
                for line in b.lines.iter() {
                    writeln!(f, "{}", line)?;
                }
                writeln!(f, "")?;
            }

            Self::Other(b) => {
                writeln!(f, "{}", b)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
struct Page {
    blocks: Vec<Block>,
}

impl Page {
    fn parse(text: &String) -> Result<Self> {
        let mut page = Self::default();
        let mut lines = text.lines().map(|l| l.trim_end().to_string()).peekable();

        while let Some(line) = lines.next() {
            if line.starts_with("module ") {
                let module = Module::parse(line.into(), &mut lines)?;
                page.blocks.push(Block::Module(module));
            } else if line.starts_with("import ") {
                let module = Module::parse(line.into(), &mut lines)?;
                page.blocks.push(Block::Import(module));
            } else if line.starts_with("init ") {
                let func = Function::parse(line.into(), &mut lines)?;
                page.blocks.push(Block::Init(func));
            } else if line.starts_with("update ") {
                let func = Function::parse(line.into(), &mut lines)?;
                page.blocks.push(Block::Update(func));
            } else if line.starts_with("view ") {
                let func = Function::parse(line.into(), &mut lines)?;
                page.blocks.push(Block::View(func));
            } else if line.starts_with("subscriptions ") {
                let func = Function::parse(line.into(), &mut lines)?;
                page.blocks.push(Block::Subscriptions(func));
            } else if line.starts_with("page ") {
                let func = Function::parse(line.into(), &mut lines)?;
                page.blocks.push(Block::Page(func));
            } else {
                page.blocks.push(Block::Other(line.into()));
            }
        }
        Ok(page)
    }

    fn to(mut self, pagetype: PageType, shared: bool, request: bool) -> Self {
        let mut blocks = vec![];

        if shared
            && !self.blocks.iter().any(|b| match b {
                Block::Import(m) => m.name == "Shared",
                _ => false,
            })
        {
            blocks.push(Block::Import(Module {
                name: "Shared".into(),
                exposing: None,
            }))
        };

        if request
            && !self.blocks.iter().any(|b| match b {
                Block::Import(m) => m.name == "Request",
                _ => false,
            })
        {
            blocks.push(Block::Import(Module {
                name: "Request".into(),
                exposing: Some("Request".into()),
            }))
        };

        if let Some(page) = self.blocks.iter_mut().find_map(|b| match b {
            Block::Import(m) => {
                if m.name == "Page" {
                    Some(m)
                } else {
                    None
                }
            }
            _ => None,
        }) {
            page.exposing = Some(
                page.exposing
                    .as_ref()
                    .map(|e| format!("Page, {}", e))
                    .unwrap_or_else(|| "Page".into()),
            )
        } else {
            blocks.push(Block::Import(Module {
                name: "Page".into(),
                exposing: Some("Page".into()),
            }))
        };

        if pagetype == PageType::Advanced
            && !self.blocks.iter().any(|b| match b {
                Block::Import(m) => m.name == "Effect",
                _ => false,
            })
        {
            blocks.push(Block::Import(Module {
                name: "Effect".into(),
                exposing: Some("Effect".into()),
            }))
        };

        for block in self.blocks.into_iter() {
            match block {
                Block::Module(b) => {
                    if let Some(import) = blocks.iter_mut().find_map(|b_| match b_ {
                        Block::Import(i) => {
                            if i.name
                                == format!("Gen.Params.{}", b.name.trim_start_matches("Pages."))
                            {
                                Some(i)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }) {
                        import.exposing = Some(
                            import
                                .exposing
                                .as_ref()
                                .map(|e| format!("Params, {}", e))
                                .unwrap_or_else(|| "Params".into()),
                        );
                    } else {
                        blocks.push(Block::Import(Module {
                            name: format!("Gen.Params.{}", b.name.trim_start_matches("Pages.")),
                            exposing: Some("Params".into()),
                        }))
                    }

                    blocks.insert(
                        0,
                        Block::Module(Module {
                            name: b.name,
                            exposing: Some(pagetype.exposing_template().into()),
                        }),
                    );
                }

                Block::Init(b) => {
                    blocks.push(Block::Init(Function {
                        lines: pagetype
                            .init_template(shared, request)
                            .lines()
                            .map(String::from)
                            .collect(),
                    }));
                    blocks.push(Block::Other(
                        b.lines
                            .iter()
                            .map(|l| format!("-- {}", l))
                            .collect::<Vec<String>>()
                            .join("\n"),
                    ));
                }

                Block::Update(b) => {
                    blocks.push(Block::Update(Function {
                        lines: pagetype
                            .update_template(shared, request)
                            .lines()
                            .map(String::from)
                            .collect(),
                    }));
                    blocks.push(Block::Other(
                        b.lines
                            .iter()
                            .map(|l| format!("-- {}", l))
                            .collect::<Vec<String>>()
                            .join("\n"),
                    ));
                }

                Block::View(b) => {
                    blocks.push(Block::View(Function {
                        lines: pagetype
                            .view_template(shared, request)
                            .lines()
                            .map(String::from)
                            .collect(),
                    }));
                    blocks.push(Block::Other(
                        b.lines
                            .iter()
                            .map(|l| format!("-- {}", l))
                            .collect::<Vec<String>>()
                            .join("\n"),
                    ));
                }

                Block::Subscriptions(b) => {
                    blocks.push(Block::Subscriptions(Function {
                        lines: pagetype
                            .subscriptions_template(shared, request)
                            .lines()
                            .map(String::from)
                            .collect(),
                    }));
                    blocks.push(Block::Other(
                        b.lines
                            .iter()
                            .map(|l| format!("-- {}", l))
                            .collect::<Vec<String>>()
                            .join("\n"),
                    ));
                }

                Block::Page(b) => {
                    blocks.push(Block::Page(Function {
                        lines: pagetype
                            .page_template(shared, request)
                            .lines()
                            .map(String::from)
                            .collect(),
                    }));
                    blocks.push(Block::Other(
                        b.lines
                            .iter()
                            .map(|l| format!("-- {}", l))
                            .collect::<Vec<String>>()
                            .join("\n"),
                    ));
                }

                b => blocks.push(b),
            }
        }

        if !blocks.iter().any(|b| matches!(b, Block::Page(..))) {
            blocks.push(Block::Other(pagetype.page_template(shared, request)));
        }

        if pagetype != PageType::Static {
            if !blocks.iter().any(|b| match b {
                Block::Other(text) => text.starts_with("type alias Model ="),
                _ => false,
            }) {
                blocks.push(Block::Other("\ntype alias Model = {}\n\n".into()));
            }

            if !blocks.iter().any(|b| match b {
                Block::Other(text) => text.starts_with("type Msg ") || text.trim() == "type Msg",
                _ => false,
            }) {
                blocks.push(Block::Other("\ntype Msg = ReplaceMe\n\n".into()));
            }

            if pagetype != PageType::Sandbox {
                if !blocks.iter().any(|b| matches!(b, Block::Subscriptions(..))) {
                    blocks.push(Block::Other(
                        pagetype.subscriptions_template(shared, request),
                    ));
                }
            }

            if !blocks.iter().any(|b| matches!(b, Block::Init(..))) {
                blocks.push(Block::Other(pagetype.init_template(shared, request)));
            }

            if !blocks.iter().any(|b| matches!(b, Block::Update(..))) {
                blocks.push(Block::Other(pagetype.update_template(shared, request)));
            }
        }

        if !blocks.iter().any(|b| matches!(b, Block::View(..))) {
            blocks.push(Block::Other(pagetype.view_template(shared, request)));
        }

        Self { blocks }
    }
}

impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        for b in self.blocks.iter() {
            write!(f, "{}", b)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
struct Cli {
    version: bool,
    help: bool,
    dry_run: bool,
    shared: bool,
    request: bool,
    path: Option<PathBuf>,
    template: Option<PageType>,
}

impl Cli {
    fn parse(args: env::Args) -> Result<Self> {
        let mut args: VecDeque<String> = args.skip(1).collect();
        let mut cli = Self::default();

        while let Some(arg) = args.pop_front() {
            match arg.as_str() {
                // Flags
                "-h" | "--help" => {
                    cli.help = true;
                }

                "-V" | "--version" => {
                    cli.version = true;
                }

                "--" => {
                    if cli.path.is_none() {
                        cli.path = args.pop_front().map(PathBuf::from);
                    }
                    return Ok(cli);
                }

                "-s" | "--shared" => cli.shared = true,

                "-r" | "--request" => cli.request = true,

                "--dry-run" => cli.dry_run = true,

                // path
                arg => {
                    if cli.path.is_none() {
                        cli.path = Some(arg.into());
                    } else if cli.template.is_none() {
                        cli.template = PageType::from(arg);
                    }
                }
            }
        }
        Ok(cli)
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse(env::args()).unwrap_or_else(|e| {
        eprintln!("error: {}", e);
        std::process::exit(1);
    });

    if cli.help {
        let usage = format!(r###"
    {} [FLAG]... [OPTION]... [PATH] [TEMPLATE]"###, env!("CARGO_PKG_NAME"));

        let flags = r###"
    --                 Denotes the end of command-line flags and options
    -s  --shared       Pass the shared model to the page functions
    -r  --request      Pass the request object to the page functions
    -d  --dry-run      Print the result without overwriting file
    -h, --help         Print help information
    -V, --version      Print version information"###;

        let args = r###"
    <PATH>        Path to focus on, or enter if directory
    <TEMPLATE>    Specify the target page template.
                    Options are - static|element|sandbox|advanced"###;

        let help = format!(
            "{} {}\n{}\n{}\n\nUSAGE:{}\n\nFLAGS:{}\n\nARGS:{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_AUTHORS"),
            env!("CARGO_PKG_DESCRIPTION"),
            usage,
            flags,
            args,
        );
        let help = help.trim();

        println!("{}", help);
        Ok(())
    } else if cli.version {
        println!("xplr {}", env!("CARGO_PKG_VERSION"));
        Ok(())
    } else if let Some((path, template)) =
        cli.path.as_ref().and_then(|p| cli.template.map(|t| (p, t)))
    {
        let text = std::fs::read_to_string(&path)?;

        let page = Page::parse(&text)?.to(template, cli.shared, cli.request);

        if cli.dry_run {
            println!("{}", page);
        } else {
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(path)?;

            writeln!(file, "{}", page)?;
        }
        Ok(())
    } else {
        bail!("missing operand\nTry 'rm --help' for more information.");
    }
}
