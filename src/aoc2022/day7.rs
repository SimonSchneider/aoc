use anyhow::{anyhow, Result};
use iter_tools::Itertools;
use std::str::FromStr;

#[derive(Debug)]
struct FileSys {
    nodes: Vec<Node>,
}

#[derive(Debug)]
struct Node {
    name: String,
    parent: Option<usize>,
    node: INode,
}

impl Node {
    fn is_dir(&self) -> bool {
        matches!(self.node, INode::Dir { .. })
    }

    fn size(&self, fs: &FileSys) -> usize {
        match self.node {
            INode::Dir {
                children_pointer,
                children_num,
            } => fs.nodes[children_pointer..(children_pointer + children_num)]
                .iter()
                .fold(0, |sum, n| sum + n.size(fs)),
            INode::File { size } => size,
        }
    }
}

#[derive(Debug)]
enum INode {
    Dir {
        children_pointer: usize,
        children_num: usize,
    },
    File {
        size: usize,
    },
}

impl TryFrom<Commands> for FileSys {
    type Error = anyhow::Error;

    fn try_from(cmds: Commands) -> std::result::Result<Self, Self::Error> {
        let (_, file_sys) = cmds.0.into_iter().try_fold(
            (
                0,
                FileSys {
                    nodes: vec![Node {
                        name: "/".to_string(),
                        parent: None,
                        node: INode::Dir {
                            children_pointer: 0,
                            children_num: 0,
                        },
                    }],
                },
            ),
            |(curr, mut fs), cmd| -> Result<(usize, FileSys)> {
                match (cmd, fs.nodes.get(curr)) {
                    (
                        Command::Cd { arg },
                        Some(Node {
                            parent: Some(par), ..
                        }),
                    ) if arg == ".." => {
                        return Ok((*par, fs));
                    }
                    (Command::Cd { arg }, _) if arg == "/" => {
                        return Ok((0, fs));
                    }
                    (
                        Command::Cd { arg },
                        Some(Node {
                            node:
                                INode::Dir {
                                    children_pointer,
                                    children_num,
                                },
                            ..
                        }),
                    ) => {
                        let (dest, _) = &fs.nodes
                            [*children_pointer..(*children_pointer + *children_num)]
                            .iter()
                            .enumerate()
                            .find(|(_, e)| e.name == arg)
                            .ok_or_else(|| anyhow!("child not found"))?;
                        return Ok((dest + children_pointer, fs));
                    }
                    (
                        Command::Ls { output },
                        Some(Node {
                            node: INode::Dir { .. },
                            ..
                        }),
                    ) => {
                        fs.nodes[curr].node = INode::Dir {
                            children_pointer: fs.nodes.len(),
                            children_num: output.len(),
                        };
                        fs.nodes.append(
                            &mut output
                                .iter()
                                .map(|o| match o {
                                    LsOutput::Dir(name) => Node {
                                        name: name.clone(),
                                        parent: Some(curr),
                                        node: INode::Dir {
                                            children_pointer: 0,
                                            children_num: 0,
                                        },
                                    },
                                    LsOutput::File { name, size } => Node {
                                        name: name.clone(),
                                        parent: Some(curr),
                                        node: INode::File { size: *size },
                                    },
                                })
                                .collect(),
                        )
                    }
                    _ => return Err(anyhow!("illegal command")),
                }
                Ok((curr, fs))
            },
        )?;
        Ok(file_sys)
    }
}

#[derive(Debug, Default)]
struct Commands(Vec<Command>);

#[derive(Debug)]
enum Command {
    Cd { arg: String },
    Ls { output: Vec<LsOutput> },
}

#[derive(Debug)]
enum LsOutput {
    Dir(String),
    File { name: String, size: usize },
}

impl FromStr for FileSys {
    type Err = anyhow::Error;

    fn from_str(inp: &str) -> std::result::Result<Self, Self::Err> {
        let cmds = inp
            .lines()
            .try_fold(Commands::default(), |mut cmds, line| {
                if let Some(cmd) = line.strip_prefix("$ ") {
                    if let Some(cd) = cmd.strip_prefix("cd ") {
                        cmds.0.push(Command::Cd {
                            arg: cd.to_string(),
                        })
                    } else if cmd.starts_with("ls") {
                        cmds.0.push(Command::Ls { output: vec![] })
                    } else {
                        return Err(anyhow!("illegal command: {}", line));
                    }
                } else if let Some(Command::Ls { output }) = cmds.0.last_mut() {
                    if let Some(dir) = line.strip_prefix("dir ") {
                        output.push(LsOutput::Dir(dir.to_string()))
                    } else {
                        let (s, name) = line
                            .split_once(' ')
                            .ok_or_else(|| anyhow!("illegal line: {}", line))?;
                        let size: usize = s.parse()?;
                        output.push(LsOutput::File {
                            size,
                            name: name.to_string(),
                        })
                    }
                } else {
                    return Err(anyhow!("unexpected line: {}", line));
                }
                Ok(cmds)
            })?;
        FileSys::try_from(cmds)
    }
}

pub fn first(inp: &str) -> Result<String> {
    let file_sys: FileSys = inp.parse()?;
    let res: usize = file_sys
        .nodes
        .iter()
        .filter(|n| n.is_dir())
        .map(|n| n.size(&file_sys))
        .filter(|s| *s <= 100_000)
        .sum();
    Ok(res.to_string())
}

pub fn second(inp: &str) -> Result<String> {
    let file_sys: FileSys = inp.parse()?;
    let used_size = file_sys
        .nodes
        .first()
        .ok_or_else(|| anyhow!("no root node found"))?
        .size(&file_sys);
    let min_amount_to_delete = 30_000_000 - (70_000_000 - used_size);
    dbg!(used_size, min_amount_to_delete);
    let (_, res) = file_sys
        .nodes
        .iter()
        .filter(|n| n.is_dir())
        .map(|n| (n.name.clone(), n.size(&file_sys)))
        .filter(|(_, s)| s >= &min_amount_to_delete)
        .sorted_by_key(|(_, s)| *s)
        .next()
        .ok_or_else(|| anyhow!("no deletion candidate found"))?;
    Ok(res.to_string())
}
