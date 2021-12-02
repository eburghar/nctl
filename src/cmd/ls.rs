fn indent(size: usize) -> String {
	const INDENT: &'static str = "    ";
	(0..size)
		.map(|_| INDENT)
		.fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}

enum FileType {
	File,
	Dir,
}

enum State {
	Start,
	Response,
	Prop,
	ResourceType,
}

struct Context {
	state: State,
	namespaces: Option<Namespace>,
	name: String,
	file_type: Option<FileType>,
	files: Vec<String>,
	dirs: Vec<String>,
}

impl Context {
	fn new() -> Context {
		Context {
			state: State::Start,
			namespaces: None,
			name: String::new(),
			file_type: None,
			files: Vec::new(),
			dirs: Vec::new(),
		}
	}
}

pub fn cmd(conf: &Config, args: &Ls, Opts { verbose, .. }: &Opts) -> Result<()> {
	let accesspath: Vec<&str> = args.dirs.split(':').collect();
	if accesspath.len() != 2 {
		Err(anyhow!("dst should be config:path"))?;
	}
	let access = match conf.access.get(accesspath[0]) {
		Some(access) => access,
		None => Err(anyhow!("config {} hast not been defined", accesspath[0]))?,
	};
	let url = format!(
		"https://{}/remote.php/dav/files/{}/{}",
		&access.host, &access.user, &dstname
	);
	let r2 = propfind("https:/localhost/remote.php/dav/files/test")
		.auth("test", "test")
		.call();
	let text = r2.into_string().unwrap();
	println!("{}", text);
	//let files = Vec::new();
	let mut context = Context::new();
	let parser = EventReader::from_str(&text);
	let mut depth = 0;
	for e in parser {
		match context.state {
			State::Start => match e {
				Ok(XmlEvent::StartElement {
					name, namespace, ..
				}) => {
					println!("{}+{}({:?})", indent(depth), name, namespace);
					depth += 1;
					if context.namespaces == None {
						context.namespaces = Some(namespace);
					}
					if name.to_string() == "{DAV:}d:response" {
						context.state = State::Response
					}
				}
				Ok(XmlEvent::EndElement { name }) => {
					depth -= 1;
					println!("{}-{}", indent(depth), name);
				}
				Err(e) => {
					println!("Error: {}", e);
					break;
				}
				_ => {}
			},
			State::Response => match e {
				Ok(XmlEvent::StartElement { name, .. }) => {}
				Err(e) => {
					println!("Error: {}", e);
					break;
				}
				_ => {}
			},
			State::Prop => {}
			State::ResourceType => {}
		}
	}
}
