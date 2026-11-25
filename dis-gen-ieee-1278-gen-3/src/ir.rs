use slotmap::{SlotMap, new_key_type};

new_key_type! {
    pub struct NodeId;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveType {
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Int8,
    Int16,
    Int32,
    Int64,
    Float32,
    Float64,
    Boolean,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumType {
    Enum8,
    Enum16,
    Enum32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeRef {
    Primitive(PrimitiveType),
    Record(NodeId),
    Enum(EnumType),
}

#[derive(Debug)]
pub enum NodeKind {
    Schema,

    Pdu { name: String, pdu_type: u8 },

    Record { name: String },

    Field { name: String, field_type: TypeRef },
}

#[derive(Debug)]
pub struct Node {
    pub kind: NodeKind,

    // Strict tree relationships
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EdgeKind {
    Reference,
    DependsOn,
    Custom(String),
}

#[derive(Debug)]
pub struct Edge {
    pub from: NodeId,
    pub to: NodeId,
    pub kind: EdgeKind,
}

pub struct Schema {
    nodes: SlotMap<NodeId, Node>,
    edges: Vec<Edge>,
    root: NodeId,
}

impl Schema {
    pub fn new() -> Self {
        let mut nodes = SlotMap::with_key();

        let root = nodes.insert(Node {
            kind: NodeKind::Schema,
            parent: None,
            children: Vec::new(),
        });

        Self {
            nodes,
            edges: Vec::new(),
            root,
        }
    }

    pub fn root(&self) -> NodeId {
        self.root
    }

    pub fn get(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }

    fn insert_child(&mut self, parent: NodeId, kind: NodeKind) -> NodeId {
        let id = self.nodes.insert(Node {
            kind,
            parent: Some(parent),
            children: Vec::new(),
        });

        self.nodes[parent].children.push(id);

        id
    }

    pub fn add_pdu(&mut self, name: impl Into<String>, pdu_type: u8) -> NodeId {
        self.insert_child(
            self.root,
            NodeKind::Pdu {
                name: name.into(),
                pdu_type,
            },
        )
    }

    pub fn add_record(&mut self, name: impl Into<String>) -> NodeId {
        self.insert_child(self.root, NodeKind::Record { name: name.into() })
    }

    pub fn add_field(
        &mut self,
        parent: NodeId,
        name: impl Into<String>,
        field_type: TypeRef,
    ) -> NodeId {
        assert!(
            matches!(
                self.nodes[parent].kind,
                NodeKind::Pdu { .. } | NodeKind::Record { .. }
            ),
            "Fields can only belong to PDUs or records"
        );

        if let TypeRef::Record(record_id) = field_type {
            assert!(
                matches!(
                    self.nodes.get(record_id).map(|n| &n.kind),
                    Some(NodeKind::Record { .. })
                ),
                "Field references an invalid record"
            );
        }

        self.insert_child(
            parent,
            NodeKind::Field {
                name: name.into(),
                field_type,
            },
        )
    }

    pub fn add_edge(&mut self, from: NodeId, to: NodeId, kind: EdgeKind) {
        assert!(self.nodes.contains_key(from));
        assert!(self.nodes.contains_key(to));

        self.edges.push(Edge { from, to, kind });
    }

    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    pub fn children(&self, id: NodeId) -> impl Iterator<Item = (NodeId, &Node)> {
        self.nodes[id]
            .children
            .iter()
            .map(|child| (*child, &self.nodes[*child]))
    }
}
