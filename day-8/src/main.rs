use std::iter;

static INPUT: &str = include_str!("../input.txt");

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let raw_data = raw_data()?;

    let root = Node::new(raw_data)?;

    println!("Metadata sum is {}", root.metadata_sum());
    println!("Root value is {}", root.value());

    Ok(())
}

struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn new(data: impl IntoIterator<Item = usize>) -> Result<Self> {
        fn new_inner(data: &mut impl Iterator<Item = usize>) -> Result<Node> {
            let n_children = data.next().ok_or("Missing data count")?;
            let n_metadata = data.next().ok_or("Missing metadata count")?;

            let children = iter::repeat_with(|| new_inner(data)).take(n_children).collect::<Result<_>>()?;
            let metadata = data.take(n_metadata).collect();

            Ok(Node { children, metadata })
        }

        new_inner(&mut data.into_iter())
    }

    fn metadata_sum(&self) -> usize {
        let children = self.children.iter().map(|c| c.metadata_sum()).sum::<usize>();
        let direct = self.metadata.iter().sum::<usize>();

        children + direct
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.metadata.iter().sum::<usize>()
        } else {
            self.metadata.iter().flat_map(|idx| {
                match idx {
                    0 => None,
                    idx => self.children.get(idx - 1).map(|n| n.value()),
                }
            }).sum::<usize>()
        }
    }
}

fn raw_data() -> Result<Vec<usize>> {
    INPUT.split_whitespace().map(|n| n.parse().map_err(Into::into)).collect()
}
