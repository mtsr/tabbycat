use std::fmt::{Formatter, Result};

use derive_builder::Builder;

#[derive(Clone, Debug)]
pub struct AttrList<'a> (pub(crate) Vec<Vec<(Identity<'a>, Identity<'a>)>>);

#[derive(Clone, Debug)]
pub struct StmtList<'a>(pub(crate) Vec<Stmt<'a>>);

#[derive(Copy, Clone, Debug)]
pub enum GraphType {
    Graph,
    DiGraph,
}

#[derive(Copy, Clone, Debug)]
pub enum AttrType {
    Graph,
    Node,
    Edge,
}

#[derive(Clone, Debug)]
pub enum Identity<'a> {
    String(&'a str),
    Usize(usize),
    ISize(isize),
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    Bool(bool),
    I64(i64),
    U64(u64),
    I128(i128),
    U128(u128),
    Float(f32),
    Double(f64),
    Quoted(&'a str),
    #[cfg(feature="attributes")]
    ArrowName([Option<&'a str>; 4]),
    #[cfg(feature="attributes")]
    RGBA(u8, u8, u8, u8),
    #[cfg(feature="attributes")]
    HSV(f32, f32, f32),
}

#[derive(Builder, Clone, Debug)]
#[builder(pattern = "owned")]
pub struct Graph<'a> {
    graph_type: GraphType,
    strict: bool,
    #[builder(setter(strip_option))]
    id: Option<Identity<'a>>,
    stmts: StmtList<'a>,
}

#[derive(Clone, Debug)]
pub enum Stmt<'a> {
    Edge(Edge<'a>),
    Node {
        id: Identity<'a>,
        port: Option<Port<'a>>,
        attr: Option<AttrList<'a>>,
    },
    Attr(AttrType, AttrList<'a>),
    Equation(Identity<'a>, Identity<'a>),
    SubGraph(SubGraph<'a>),
}

#[derive(Clone, Debug)]
pub struct Edge<'a> {
    pub(crate) node: EdgeNode<'a>,
    pub(crate) body: Vec<EdgeBody<'a>>,
    pub(crate) attr: Option<AttrList<'a>>,
}

#[derive(Copy, Clone, Debug)]
pub enum EdgeOp {
    Arrow,
    Line,
}

#[derive(Clone, Debug)]
pub struct EdgeBody<'a> {
    pub(crate) node: EdgeNode<'a>,
    pub(crate) op: EdgeOp,
}

#[derive(Clone, Debug)]
pub enum EdgeNode<'a> {
    Node {
        id: Identity<'a>,
        port: Option<Port<'a>>,
    },
    SubGraph(SubGraph<'a>),
}

#[derive(Clone, Debug)]
pub enum SubGraph<'a> {
    SubGraph {
        id: Option<Identity<'a>>,
        stmts: Box<StmtList<'a>>,
    },
    Cluster(Box<StmtList<'a>>),
}

impl<'a> SubGraph<'a> {
    pub fn cluster(list: StmtList<'a>) -> Self {
        SubGraph::Cluster(Box::new(list))
    }
    pub fn subgraph(id: Option<Identity<'a>>, list: StmtList<'a>) -> Self {
        SubGraph::SubGraph { id, stmts: Box::new(list) }
    }
}

#[derive(Clone, Debug)]
pub enum Port<'a> {
    ID(Identity<'a>, Option<Compass>),
    Compass(Compass),
}

#[derive(Copy, Clone, Debug)]
pub enum Compass {
    North,
    NorthEast,
    Ease,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    Central,
}

impl<'a> IntoIterator for StmtList<'a> {
    type Item = Stmt<'a>;
    type IntoIter = std::vec::IntoIter<Stmt<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for AttrList<'a> {
    type Item = Vec<(Identity<'a>, Identity<'a>)>;
    type IntoIter = std::vec::IntoIter<Vec<(Identity<'a>, Identity<'a>)>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> From<bool> for Identity<'a> {
    fn from(flag: bool) -> Self {
        Identity::Bool(flag)
    }
}

impl<'a> From<isize> for Identity<'a> {
    fn from(number: isize) -> Self {
        Identity::ISize(number)
    }
}

impl<'a> From<usize> for Identity<'a> {
    fn from(number: usize) -> Self {
        Identity::Usize(number)
    }
}

impl<'a> From<i8> for Identity<'a> {
    fn from(number: i8) -> Self {
        Identity::I8(number)
    }
}

impl<'a> From<u8> for Identity<'a> {
    fn from(number: u8) -> Self {
        Identity::U8(number)
    }
}

impl<'a> From<u16> for Identity<'a> {
    fn from(number: u16) -> Self {
        Identity::U16(number)
    }
}

impl<'a> From<i16> for Identity<'a> {
    fn from(number: i16) -> Self {
        Identity::I16(number)
    }
}

impl<'a> From<u32> for Identity<'a> {
    fn from(number: u32) -> Self {
        Identity::U32(number)
    }
}

impl<'a> From<i32> for Identity<'a> {
    fn from(number: i32) -> Self {
        Identity::I32(number)
    }
}

impl<'a> From<u64> for Identity<'a> {
    fn from(number: u64) -> Self {
        Identity::U64(number)
    }
}

impl<'a> From<i64> for Identity<'a> {
    fn from(number: i64) -> Self {
        Identity::I64(number)
    }
}

impl<'a> From<i128> for Identity<'a> {
    fn from(number: i128) -> Self {
        Identity::I128(number)
    }
}


impl<'a> From<u128> for Identity<'a> {
    fn from(number: u128) -> Self {
        Identity::U128(number)
    }
}

impl<'a> From<f32> for Identity<'a> {
    fn from(number: f32) -> Self {
        Identity::Float(number)
    }
}

impl<'a> From<f64> for Identity<'a> {
    fn from(number: f64) -> Self {
        Identity::Double(number)
    }
}

impl<'a> Identity<'a> {
    pub fn id(data: &'a str) -> anyhow::Result<Self> {
        static PATTERN: &str = r#"^[a-zA-Z\x{80}-\x{ff}_][a-zA-Z\x{80}-\x{ff}\d_]*$"#;
        let re = regex::Regex::new(PATTERN).unwrap();
        if re.is_match(data) {
            Ok(Identity::String(data))
        } else {
            Err(anyhow::anyhow!("invalid identity format"))
        }
    }
    pub fn quoted(data: &'a str) -> Self {
        Identity::Quoted(data)
    }
}

impl<'a> Port<'a> {
    pub fn id(i: Identity<'a>) -> Self {
        Port::ID(i, None)
    }

    pub fn id_compass(i: Identity<'a>, c: Compass) -> Self {
        Port::ID(i, Some(c))
    }

    pub fn compass(c: Compass) -> Self {
        Port::Compass(c)
    }
}

impl<'a> std::fmt::Display for Graph<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.strict {
            write!(f, "strict ")
        } else {
            Ok(())
        }.and(
            match self.graph_type {
                GraphType::Graph =>
                    write!(f, "graph "),
                GraphType::DiGraph =>
                    write!(f, "digraph ")
            }
        ).and(
            match &self.id {
                Some(id) => write!(f, "{}", id),
                _ => Ok(())
            }
        ).and(
            write!(f, "{{{}}}", self.stmts)
        )
    }
}

impl std::fmt::Display for Compass {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use Compass::*;
        match self {
            North => write!(f, "n"),
            NorthEast => write!(f, "ne"),
            Ease => write!(f, "e"),
            SouthEast => write!(f, "se"),
            South => write!(f, "s"),
            SouthWest => write!(f, "sw"),
            West => write!(f, "w"),
            NorthWest => write!(f, "nw"),
            Central => write!(f, "c")
        }
    }
}

impl<'a> std::fmt::Display for Identity<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use Identity::*;
        match self {
            RGBA(r, g, b, a) => write!(f, "#{:x}{:x}{:x}{:x}", r, g, b, a),
            HSV(h, s, v) => write!(f, "{},+{},+{}", h, s, v),
            String(id) => write!(f, "{}", id),
            Usize(id) => write!(f, "{}", id),
            Float(id) => write!(f, "{}", id),
            Double(id) => write!(f, "{}", id),
            Quoted(id) => write!(f, "{:?}", id),
            ISize(id) => write!(f, "{}", id),
            I8(id) => write!(f, "{}", id),
            U8(id) => write!(f, "{}", id),
            I16(id) => write!(f, "{}", id),
            U16(id) => write!(f, "{}", id),
            I32(id) => write!(f, "{}", id),
            U32(id) => write!(f, "{}", id),
            I64(id) => write!(f, "{}", id),
            U64(id) => write!(f, "{}", id),
            I128(id) => write!(f, "{}", id),
            U128(id) => write!(f, "{}", id),
            Bool(flag) => write!(f, "{}", flag),
            #[cfg(feature="attributes")]
            ArrowName(names) => {
                names.iter().fold(Ok(()), |acc, x| {
                    acc.and(
                        match x {
                            None => Ok(()),
                            Some(e) => {
                                write!(f, "{}", e)
                            }
                        })
                })
            }
        }
    }
}

impl<'a> std::fmt::Display for Port<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Port::ID(id, Some(c)) =>
                write!(f, ":{}:{}", id, c),
            Port::ID(x, None) =>
                write!(f, ":{}", x),
            Port::Compass(x) =>
                write!(f, ":{}", x)
        }
    }
}

impl<'a> std::fmt::Display for AttrList<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.0.iter()
            .fold(Ok(()), |acc, list| {
                acc.and(write!(f, "["))
                    .and(list
                        .iter()
                        .fold(Ok(()), |acc, (x, y)| {
                            acc.and(write!(f, "{}={};", x, y))
                        }))
                    .and(write!(f, "]"))
            })
    }
}

impl<'a> std::fmt::Display for Stmt<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use Stmt as S;
        match self {
            S::Equation(a, b) =>
                write!(f, "{}={}", a, b),
            S::Edge(edge) =>
                write!(f, "{}", edge),
            S::Node { id, port, attr } => {
                write!(f, "{}", id)
                    .and(match port {
                        None => Ok(()),
                        Some(p) => write!(f, "{}", p)
                    })
                    .and(match attr {
                        None => Ok(()),
                        Some(a) => write!(f, "{}", a)
                    })
            }
            S::Attr(t, list) => {
                match t {
                    AttrType::Node => write!(f, "node {}", list),
                    AttrType::Graph => write!(f, "graph {}", list),
                    AttrType::Edge => write!(f, "edge {}", list)
                }
            }
            S::SubGraph(sub) => {
                write!(f, "{}", sub)
            }
        }
    }
}

impl<'a> std::fmt::Display for StmtList<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.0
            .iter()
            .fold(Ok(()), |acc, x| {
                acc.and(write!(f, "{};", x))
            })
    }
}

impl<'a> std::fmt::Display for SubGraph<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            SubGraph::SubGraph { id, stmts } => {
                write!(f, "subgraph ")
                    .and(
                        match id {
                            Some(id) => {
                                write!(f, "{} ", id)
                            }
                            _ => Ok(())
                        }
                    ).and(
                    write!(f, "{{{}}}", stmts)
                )
            }
            SubGraph::Cluster(stmts) => {
                write!(f, "{{{}}}", stmts)
            }
        }
    }
}

impl<'a> std::fmt::Display for EdgeNode<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            EdgeNode::Node { id, port } =>
                write!(f, "{}", id)
                    .and(match port {
                        Some(port) => write!(f, "{}", port),
                        _ => Ok(())
                    }),
            EdgeNode::SubGraph(graph) => {
                write!(f, "{}", graph)
            }
        }
    }
}

impl<'a> std::fmt::Display for EdgeBody<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.op {
            EdgeOp::Arrow => write!(f, "->"),
            EdgeOp::Line => write!(f, "--")
        }.and(
            write!(f, "{}", self.node)
        )
    }
}

impl<'a> std::fmt::Display for Edge<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.node)
            .and(self.body.iter().fold(Ok(()), |acc, x| {
                acc.and(write!(f, "{}", x))
            }))
            .and(match &self.attr {
                Some(x) => write!(f, "{}", x),
                _ => Ok(())
            })
    }
}

impl<'a> AttrList<'a> {
    pub fn new() -> Self {
        AttrList(Vec::new())
    }
    pub fn new_bracket(mut self) -> Self {
        self.0.push(Vec::new());
        self
    }
    pub fn extend<I: IntoIterator<Item=(Identity<'a>, Identity<'a>)>>(mut self, iter: I) -> Self {
        if self.0.is_empty() {
            self = self.new_bracket();
        }
        self.0.last_mut().unwrap().extend(iter);
        self
    }
    pub fn extend_list<I: IntoIterator<Item=Vec<(Identity<'a>, Identity<'a>)>>>(mut self, iter: I) -> Self {
        self.0.extend(iter);
        self
    }
    pub fn add(mut self, key: Identity<'a>, value: Identity<'a>) -> Self {
        if self.0.is_empty() {
            self = self.new_bracket();
        }
        self.0.last_mut().unwrap().push((key, value));
        self
    }
    pub fn add_pair(self, pair: AttrPair<'a>) -> Self {
        self.add(pair.0, pair.1)
    }
}

impl<'a> StmtList<'a> {
    pub fn new() -> Self {
        StmtList(Vec::new())
    }

    pub fn add(mut self, stmt: Stmt<'a>) -> Self {
        self.0.push(stmt);
        self
    }

    pub fn extend<I: IntoIterator<Item=Stmt<'a>>>(mut self, iter: I) -> Self {
        self.0.extend(iter);
        self
    }

    pub fn add_node(mut self, id: Identity<'a>, port: Option<Port<'a>>, attr: Option<AttrList<'a>>) -> Self {
        self.0.push(Stmt::Node {
            id,
            port,
            attr,
        });
        self
    }

    pub fn add_attr(mut self, attr_type: AttrType, attr_list: AttrList<'a>) -> Self {
        self.0.push(Stmt::Attr(
            attr_type,
            attr_list,
        ));
        self
    }

    pub fn add_edge(mut self, edge: Edge<'a>) -> Self {
        self.0.push(Stmt::Edge(
            edge
        ));
        self
    }

    pub fn add_subgraph(mut self, sub: SubGraph<'a>) -> Self {
        self.0.push(Stmt::SubGraph(
            sub
        ));
        self
    }

    pub fn add_equation(mut self, a: Identity<'a>, b: Identity<'a>) -> Self {
        self.0.push(Stmt::Equation(
            a, b,
        ));
        self
    }
}

impl<'a> Edge<'a> {
    pub fn head_node(id: Identity<'a>, port: Option<Port<'a>>) -> Self {
        Edge {
            node: EdgeNode::Node {
                id,
                port,
            },
            body: vec![],
            attr: None,
        }
    }
    pub fn head_subgraph(sub: SubGraph<'a>) -> Self {
        Edge {
            node: EdgeNode::SubGraph(sub),
            body: vec![],
            attr: None,
        }
    }
    pub fn line_to_node(mut self, id: Identity<'a>, port: Option<Port<'a>>) -> Self {
        self.body.push(
            EdgeBody {
                node: EdgeNode::Node {
                    id,
                    port,
                },
                op: EdgeOp::Line,
            }
        );
        self
    }
    pub fn line_to_subgraph(mut self, sub: SubGraph<'a>) -> Self {
        self.body.push(
            EdgeBody {
                node: EdgeNode::SubGraph(sub),
                op: EdgeOp::Line,
            }
        );
        self
    }
    pub fn arrow_to_node(mut self, id: Identity<'a>, port: Option<Port<'a>>) -> Self {
        self.body.push(
            EdgeBody {
                node: EdgeNode::Node {
                    id,
                    port,
                },
                op: EdgeOp::Arrow,
            }
        );
        self
    }
    pub fn arrow_to_subgraph(mut self, sub: SubGraph<'a>) -> Self {
        self.body.push(
            EdgeBody {
                node: EdgeNode::SubGraph(sub),
                op: EdgeOp::Arrow,
            }
        );
        self
    }
    pub fn add_attrlist(mut self, list: AttrList<'a>) -> Self {
        if self.attr.is_none() {
            self.attr.replace(list);
        } else {
            self.attr.as_mut().unwrap().0.extend(list.0);
            self.attr.as_mut().unwrap().0.push(Vec::new());
        }
        self
    }
    pub fn add_attribute(mut self, key: Identity<'a>, value: Identity<'a>) -> Self {
        if self.attr.is_none() {
            self.attr.replace(AttrList(vec![vec![(key, value)]]));
        } else {
            let vec = &mut self.attr.as_mut().unwrap().0;
            if vec.is_empty() {
                vec.push(vec![(key, value)]);
            } else {
                vec.last_mut().unwrap().push((key, value));
            }
        }
        self
    }
    pub fn add_attrpair(self, pair: AttrPair<'a>) -> Self {
        self.add_attribute(pair.0, pair.1)
    }
}

pub type AttrPair<'a> = (Identity<'a>, Identity<'a>);
#[cfg(feature="attributes")]
pub mod attributes {
    #![allow(non_snake_case)]

    use crate::{AttrPair, Identity};
    use std::hint::unreachable_unchecked;

    macro_rules! attribute_from {
        ($id:ident, $t:ty) => {
            pub fn $id<'a>(value: $t) -> AttrPair<'a> {
                (Identity::String(stringify!($id)), Identity::from(value))
            }
        };
    }

    macro_rules! attribute_quoted {
        ($id:ident) => {
            pub fn $id<'a>(value: &'a str) -> AttrPair<'a> {
                (Identity::String(stringify!($id)), Identity::quoted(value))
            }
        };
    }

    attribute_from!(Damping, f64);
    attribute_from!(K, f64);
    attribute_quoted!(URL);
    attribute_quoted!(_backgroud);
    attribute_from!(area, f64);
    attribute_from!(arrowsize, f64);
    attribute_from!(center, bool);
    attribute_quoted!(charset);
    attribute_quoted!(class);
    attribute_quoted!(colorscheme);
    attribute_quoted!(comment);
    attribute_from!(compound, bool);
    attribute_from!(concentrate, bool);
    attribute_from!(constraint, bool);
    attribute_from!(decorate, bool);
    attribute_from!(defaultdist, f64);
    attribute_from!(dim, u8);
    attribute_from!(dimen, u8);
    attribute_from!(diredgeconstraints, bool);
    attribute_from!(distortion, f64);
    attribute_from!(dpi, f64);
    attribute_quoted!(edgeURL);
    attribute_quoted!(edgehref);
    attribute_quoted!(edgetarget);
    attribute_quoted!(edgetooltip);
    attribute_from!(epsilon, f64);
    attribute_quoted!(fontname);
    attribute_quoted!(fontnames);
    attribute_quoted!(fontpath);
    attribute_from!(fontsize, f64);
    attribute_from!(forcelables, bool);
    attribute_from!(gradientangle, i32);
    attribute_quoted!(group);
    attribute_quoted!(headURL);
    attribute_from!(headclip, bool);
    attribute_quoted!(headhref);
    attribute_quoted!(headlabel);
    attribute_quoted!(headtarget);
    attribute_quoted!(headtooltip);
    attribute_from!(height, f64);
    attribute_quoted!(href);
    attribute_quoted!(id);
    attribute_quoted!(image);
    attribute_quoted!(imagepath);
    attribute_quoted!(imagepos);
    attribute_from!(imagescale, bool);
    attribute_from!(inputscale, f64);
    attribute_quoted!(label);
    attribute_quoted!(labelURL);
    attribute_from!(label_scheme, i32);
    attribute_from!(labelangle, f64);
    attribute_from!(labeldistance, f64);
    attribute_from!(labelfloat, bool);
    attribute_quoted!(labelfontname);
    attribute_from!(labelfontsize, f64);
    attribute_quoted!(labelhref);
    attribute_quoted!(labeljust);
    attribute_quoted!(labelloc);
    attribute_quoted!(labeltarget);
    attribute_quoted!(labeltooltip);
    attribute_from!(landscape, bool);
    attribute_quoted!(layerlistsep);
    attribute_quoted!(layersep);
    attribute_quoted!(layout);
    attribute_from!(len, f64);
    attribute_from!(levels, i32);
    attribute_from!(levelsgap, f64);
    attribute_quoted!(lhead);
    attribute_from!(lheight, f64);
    attribute_quoted!(ltail);
    attribute_from!(lwidth, f64);
    attribute_from!(margin, f64);
    attribute_from!(maxiter, i32);
    attribute_from!(mclimit, f64);
    attribute_from!(mindist, f64);
    attribute_from!(minlen, i32);
    attribute_quoted!(mode);
    attribute_quoted!(model);
    attribute_from!(mosek, bool);
    attribute_from!(newrank, bool);
    attribute_from!(nodesep, f64);
    attribute_from!(nojustify, bool);
    attribute_from!(normalize, f64);
    attribute_from!(notranslate, bool);
    attribute_from!(nslimit, f64);
    attribute_from!(nslimit1, f64);
    attribute_quoted!(ordering);
    attribute_from!(orientation, f64);
    attribute_from!(overlap_scaling, f64);
    attribute_from!(overlap_shrink, bool);
    attribute_from!(pad, f64);
    attribute_from!(page, f64);
    attribute_from!(penwidth, f64);
    attribute_from!(peripheries, i32);
    attribute_from!(pin, bool);
    attribute_from!(quantum, f64);
    attribute_from!(ranksep, f64);
    attribute_from!(ratio, f64);
    attribute_from!(regular, bool);
    attribute_from!(remincross, bool);
    attribute_from!(repulsiveforce, f64);
    attribute_from!(resolution, f64);
    attribute_quoted!(root);
    attribute_from!(rotate, i32);
    attribute_from!(rotation, f64);
    attribute_quoted!(samehead);
    attribute_quoted!(sametail);
    attribute_from!(samplepoints, i32);
    attribute_from!(scale, f64);
    attribute_from!(searchsize, i32);
    attribute_quoted!(shapefile);
    attribute_from!(showboxes, i32);
    attribute_from!(sides, i32);
    attribute_from!(size, f64);
    attribute_from!(skew, f64);
    attribute_from!(sortv, i32);
    attribute_quoted!(stylesheet);
    attribute_quoted!(tailURL);
    attribute_from!(tailclip, bool);
    attribute_quoted!(tailhref);
    attribute_quoted!(taillabel);
    attribute_quoted!(tailtarget);
    attribute_quoted!(tailtooltip);
    attribute_quoted!(target);
    attribute_quoted!(tooltip);
    attribute_from!(truecolor, bool);
    attribute_from!(voro_margin, f64);
    attribute_from!(weight, f64);
    attribute_from!(width, f64);
    attribute_quoted!(xdotversion);
    attribute_quoted!(xlabel);
    attribute_from!(z, f64);
    attribute_from!(bgcolor, Color);
    attribute_from!(color, Color);
    attribute_from!(fillcolor, Color);
    attribute_from!(labelfontcolor, Color);
    attribute_from!(pencolor, Color);
    attribute_from!(shape, Shape);
    pub fn arrowhead<'a>(value: ArrowShape) -> AttrPair<'a> {
        (Identity::String("arrowhead"), Identity::ArrowName([Some(arrow_str(value)), None, None, None]))
    }

    pub fn arrowhead2<'a>(a: ArrowShape, b: ArrowShape) -> AttrPair<'a> {
        (Identity::String("arrowhead"), Identity::ArrowName([Some(arrow_str(a)), Some(arrow_str(b)), None, None]))
    }

    pub fn arrowhead3<'a>(a: ArrowShape, b: ArrowShape, c: ArrowShape) -> AttrPair<'a> {
        (Identity::String("arrowhead"), Identity::ArrowName([Some(arrow_str(a)), Some(arrow_str(b)), Some(arrow_str(c)), None]))
    }

    pub fn arrowhead4<'a>(a: ArrowShape, b: ArrowShape, c: ArrowShape, d: ArrowShape) -> AttrPair<'a> {
        (Identity::String("arrowhead"), Identity::ArrowName([Some(arrow_str(a)), Some(arrow_str(b)), Some(arrow_str(c)), Some(arrow_str(d))]))
    }

    pub fn arrowtail<'a>(value: ArrowShape) -> AttrPair<'a> {
        (Identity::String("arrowtail"), Identity::ArrowName([Some(arrow_str(value)), None, None, None]))
    }

    pub fn arrowtail2<'a>(a: ArrowShape, b: ArrowShape) -> AttrPair<'a> {
        (Identity::String("arrowtail"), Identity::ArrowName([Some(arrow_str(a)), Some(arrow_str(b)), None, None]))
    }

    pub fn arrowtail3<'a>(a: ArrowShape, b: ArrowShape, c: ArrowShape) -> AttrPair<'a> {
        (Identity::String("arrowtail"), Identity::ArrowName([Some(arrow_str(a)), Some(arrow_str(b)), Some(arrow_str(c)), None]))
    }

    pub fn arrowtail4<'a>(a: ArrowShape, b: ArrowShape, c: ArrowShape, d: ArrowShape) -> AttrPair<'a> {
        (Identity::String("arrowtail"), Identity::ArrowName([Some(arrow_str(a)), Some(arrow_str(b)), Some(arrow_str(c)), Some(arrow_str(d))]))
    }


    #[derive(Debug)]
    pub enum Shape {
        Box,
        Polygon,
        Ellipse,
        Oval,
        Circle,
        Point,
        Egg,
        Triangle,
        Plaintext,
        Plain,
        Diamond,
        Trapezium,
        Parallelogram,
        House,
        Pentagon,
        Hexagon,
        Septagon,
        Octagon,
        Doublecircle,
        Doubleoctagon,
        Tripleoctagon,
        Invtriangle,
        Invtrapezium,
        Invhouse,
        Mdiamond,
        Msquare,
        Mcircle,
        Rect,
        Rectangle,
        Square,
        Star,
        None,
        Underline,
        Cylinder,
        Note,
        Tab,
        Folder,
        Box3d,
        Component,
        Promoter,
        Cds,
        Terminator,
        Utr,
        Primersite,
        Restrictionsite,
        Fivepoverhang,
        Threepoverhang,
        Noverhang,
        Assembly,
        Signature,
        Insulator,
        Ribosite,
        Rnastab,
        Proteasesite,
        Proteinstab,
        Rpromoter,
        Rarrow,
        Larrow,
        Lpromoter,
    }

    pub enum ArrowShape {
        Olbox,
        Olcrow,
        Olcurve,
        Olicurve,
        Oldiamond,
        Oldot,
        Olinv,
        Olnone,
        Olnormal,
        Oltee,
        Olvee,
        Orbox,
        Orcrow,
        Orcurve,
        Oricurve,
        Ordiamond,
        Ordot,
        Orinv,
        Ornone,
        Ornormal,
        Ortee,
        Orvee,
        Lbox,
        Lcrow,
        Lcurve,
        Licurve,
        Ldiamond,
        Ldot,
        Linv,
        Lnone,
        Lnormal,
        Ltee,
        Lvee,
        Rbox,
        Rcrow,
        Rcurve,
        Ricurve,
        Rdiamond,
        Rdot,
        Rinv,
        Rnone,
        Rnormal,
        Rtee,
        Rvee,
        Box,
        Crow,
        Curve,
        Icurve,
        Diamond,
        Dot,
        Inv,
        None,
        Normal,
        Tee,
        Vee,
    }

    pub enum Color {
        Rgb(u8, u8, u8),
        Rgba(u8, u8, u8, u8),
        HSV(f32, f32, f32),
        Aliceblue,
        Antiquewhite,
        Antiquewhite1,
        Antiquewhite2,
        Antiquewhite3,
        Antiquewhite4,
        Aqua,
        Aquamarine,
        Aquamarine1,
        Aquamarine2,
        Aquamarine3,
        Aquamarine4,
        Azure,
        Azure1,
        Azure2,
        Azure3,
        Azure4,
        Beige,
        Bisque,
        Bisque1,
        Bisque2,
        Bisque3,
        Bisque4,
        Black,
        Blanchedalmond,
        Blue,
        Blue1,
        Blue2,
        Blue3,
        Blue4,
        Blueviolet,
        Brown,
        Brown1,
        Brown2,
        Brown3,
        Brown4,
        Burlywood,
        Burlywood1,
        Burlywood2,
        Burlywood3,
        Burlywood4,
        Cadetblue,
        Cadetblue1,
        Cadetblue2,
        Cadetblue3,
        Cadetblue4,
        Chartreuse,
        Chartreuse1,
        Chartreuse2,
        Chartreuse3,
        Chartreuse4,
        Chocolate,
        Chocolate1,
        Chocolate2,
        Chocolate3,
        Chocolate4,
        Coral,
        Coral1,
        Coral2,
        Coral3,
        Coral4,
        Cornflowerblue,
        Cornsilk,
        Cornsilk1,
        Cornsilk2,
        Cornsilk3,
        Cornsilk4,
        Crimson,
        Cyan,
        Cyan1,
        Cyan2,
        Cyan3,
        Cyan4,
        Darkblue,
        Darkcyan,
        Darkgoldenrod,
        Darkgoldenrod1,
        Darkgoldenrod2,
        Darkgoldenrod3,
        Darkgoldenrod4,
        Darkgray,
        Darkgreen,
        Darkgrey,
        Darkkhaki,
        Darkmagenta,
        Darkolivegreen,
        Darkolivegreen1,
        Darkolivegreen2,
        Darkolivegreen3,
        Darkolivegreen4,
        Darkorange,
        Darkorange1,
        Darkorange2,
        Darkorange3,
        Darkorange4,
        Darkorchid,
        Darkorchid1,
        Darkorchid2,
        Darkorchid3,
        Darkorchid4,
        Darkred,
        Darksalmon,
        Darkseagreen,
        Darkseagreen1,
        Darkseagreen2,
        Darkseagreen3,
        Darkseagreen4,
        Darkslateblue,
        Darkslategray,
        Darkslategray1,
        Darkslategray2,
        Darkslategray3,
        Darkslategray4,
        Darkslategrey,
        Darkturquoise,
        Darkviolet,
        Deeppink,
        Deeppink1,
        Deeppink2,
        Deeppink3,
        Deeppink4,
        Deepskyblue,
        Deepskyblue1,
        Deepskyblue2,
        Deepskyblue3,
        Deepskyblue4,
        Dimgray,
        Dimgrey,
        Dodgerblue,
        Dodgerblue1,
        Dodgerblue2,
        Dodgerblue3,
        Dodgerblue4,
        Firebrick,
        Firebrick1,
        Firebrick2,
        Firebrick3,
        Firebrick4,
        Floralwhite,
        Forestgreen,
        Fuchsia,
        Gainsboro,
        Ghostwhite,
        Gold,
        Gold1,
        Gold2,
        Gold3,
        Gold4,
        Goldenrod,
        Goldenrod1,
        Goldenrod2,
        Goldenrod3,
        Goldenrod4,
        Gray,
        Gray0,
        Gray1,
        Gray10,
        Gray100,
        Gray11,
        Gray12,
        Gray13,
        Gray14,
        Gray15,
        Gray16,
        Gray17,
        Gray18,
        Gray19,
        Gray2,
        Gray20,
        Gray21,
        Gray22,
        Gray23,
        Gray24,
        Gray25,
        Gray26,
        Gray27,
        Gray28,
        Gray29,
        Gray3,
        Gray30,
        Gray31,
        Gray32,
        Gray33,
        Gray34,
        Gray35,
        Gray36,
        Gray37,
        Gray38,
        Gray39,
        Gray4,
        Gray40,
        Gray41,
        Gray42,
        Gray43,
        Gray44,
        Gray45,
        Gray46,
        Gray47,
        Gray48,
        Gray49,
        Gray5,
        Gray50,
        Gray51,
        Gray52,
        Gray53,
        Gray54,
        Gray55,
        Gray56,
        Gray57,
        Gray58,
        Gray59,
        Gray6,
        Gray60,
        Gray61,
        Gray62,
        Gray63,
        Gray64,
        Gray65,
        Gray66,
        Gray67,
        Gray68,
        Gray69,
        Gray7,
        Gray70,
        Gray71,
        Gray72,
        Gray73,
        Gray74,
        Gray75,
        Gray76,
        Gray77,
        Gray78,
        Gray79,
        Gray8,
        Gray80,
        Gray81,
        Gray82,
        Gray83,
        Gray84,
        Gray85,
        Gray86,
        Gray87,
        Gray88,
        Gray89,
        Gray9,
        Gray90,
        Gray91,
        Gray92,
        Gray93,
        Gray94,
        Gray95,
        Gray96,
        Gray97,
        Gray98,
        Gray99,
        Green,
        Green1,
        Green2,
        Green3,
        Green4,
        Greenyellow,
        Grey,
        Grey0,
        Grey1,
        Grey10,
        Grey100,
        Grey11,
        Grey12,
        Grey13,
        Grey14,
        Grey15,
        Grey16,
        Grey17,
        Grey18,
        Grey19,
        Grey2,
        Grey20,
        Grey21,
        Grey22,
        Grey23,
        Grey24,
        Grey25,
        Grey26,
        Grey27,
        Grey28,
        Grey29,
        Grey3,
        Grey30,
        Grey31,
        Grey32,
        Grey33,
        Grey34,
        Grey35,
        Grey36,
        Grey37,
        Grey38,
        Grey39,
        Grey4,
        Grey40,
        Grey41,
        Grey42,
        Grey43,
        Grey44,
        Grey45,
        Grey46,
        Grey47,
        Grey48,
        Grey49,
        Grey5,
        Grey50,
        Grey51,
        Grey52,
        Grey53,
        Grey54,
        Grey55,
        Grey56,
        Grey57,
        Grey58,
        Grey59,
        Grey6,
        Grey60,
        Grey61,
        Grey62,
        Grey63,
        Grey64,
        Grey65,
        Grey66,
        Grey67,
        Grey68,
        Grey69,
        Grey7,
        Grey70,
        Grey71,
        Grey72,
        Grey73,
        Grey74,
        Grey75,
        Grey76,
        Grey77,
        Grey78,
        Grey79,
        Grey8,
        Grey80,
        Grey81,
        Grey82,
        Grey83,
        Grey84,
        Grey85,
        Grey86,
        Grey87,
        Grey88,
        Grey89,
        Grey9,
        Grey90,
        Grey91,
        Grey92,
        Grey93,
        Grey94,
        Grey95,
        Grey96,
        Grey97,
        Grey98,
        Grey99,
        Honeydew,
        Honeydew1,
        Honeydew2,
        Honeydew3,
        Honeydew4,
        Hotpink,
        Hotpink1,
        Hotpink2,
        Hotpink3,
        Hotpink4,
        Indianred,
        Indianred1,
        Indianred2,
        Indianred3,
        Indianred4,
        Indigo,
        Invis,
        Ivory,
        Ivory1,
        Ivory2,
        Ivory3,
        Ivory4,
        Khaki,
        Khaki1,
        Khaki2,
        Khaki3,
        Khaki4,
        Lavender,
        Lavenderblush,
        Lavenderblush1,
        Lavenderblush2,
        Lavenderblush3,
        Lavenderblush4,
        Lawngreen,
        Lemonchiffon,
        Lemonchiffon1,
        Lemonchiffon2,
        Lemonchiffon3,
        Lemonchiffon4,
        Lightblue,
        Lightblue1,
        Lightblue2,
        Lightblue3,
        Lightblue4,
        Lightcoral,
        Lightcyan,
        Lightcyan1,
        Lightcyan2,
        Lightcyan3,
        Lightcyan4,
        Lightgoldenrod,
        Lightgoldenrod1,
        Lightgoldenrod2,
        Lightgoldenrod3,
        Lightgoldenrod4,
        Lightgoldenrodyellow,
        Lightgray,
        Lightgreen,
        Lightgrey,
        Lightpink,
        Lightpink1,
        Lightpink2,
        Lightpink3,
        Lightpink4,
        Lightsalmon,
        Lightsalmon1,
        Lightsalmon2,
        Lightsalmon3,
        Lightsalmon4,
        Lightseagreen,
        Lightskyblue,
        Lightskyblue1,
        Lightskyblue2,
        Lightskyblue3,
        Lightskyblue4,
        Lightslateblue,
        Lightslategray,
        Lightslategrey,
        Lightsteelblue,
        Lightsteelblue1,
        Lightsteelblue2,
        Lightsteelblue3,
        Lightsteelblue4,
        Lightyellow,
        Lightyellow1,
        Lightyellow2,
        Lightyellow3,
        Lightyellow4,
        Lime,
        Limegreen,
        Linen,
        Magenta,
        Magenta1,
        Magenta2,
        Magenta3,
        Magenta4,
        Maroon,
        Maroon1,
        Maroon2,
        Maroon3,
        Maroon4,
        Mediumaquamarine,
        Mediumblue,
        Mediumorchid,
        Mediumorchid1,
        Mediumorchid2,
        Mediumorchid3,
        Mediumorchid4,
        Mediumpurple,
        Mediumpurple1,
        Mediumpurple2,
        Mediumpurple3,
        Mediumpurple4,
        Mediumseagreen,
        Mediumslateblue,
        Mediumspringgreen,
        Mediumturquoise,
        Mediumvioletred,
        Midnightblue,
        Mintcream,
        Mistyrose,
        Mistyrose1,
        Mistyrose2,
        Mistyrose3,
        Mistyrose4,
        Moccasin,
        Navajowhite,
        Navajowhite1,
        Navajowhite2,
        Navajowhite3,
        Navajowhite4,
        Navy,
        Navyblue,
        None,
        Oldlace,
        Olive,
        Olivedrab,
        Olivedrab1,
        Olivedrab2,
        Olivedrab3,
        Olivedrab4,
        Orange,
        Orange1,
        Orange2,
        Orange3,
        Orange4,
        Orangered,
        Orangered1,
        Orangered2,
        Orangered3,
        Orangered4,
        Orchid,
        Orchid1,
        Orchid2,
        Orchid3,
        Orchid4,
        Palegoldenrod,
        Palegreen,
        Palegreen1,
        Palegreen2,
        Palegreen3,
        Palegreen4,
        Paleturquoise,
        Paleturquoise1,
        Paleturquoise2,
        Paleturquoise3,
        Paleturquoise4,
        Palevioletred,
        Palevioletred1,
        Palevioletred2,
        Palevioletred3,
        Palevioletred4,
        Papayawhip,
        Peachpuff,
        Peachpuff1,
        Peachpuff2,
        Peachpuff3,
        Peachpuff4,
        Peru,
        Pink,
        Pink1,
        Pink2,
        Pink3,
        Pink4,
        Plum,
        Plum1,
        Plum2,
        Plum3,
        Plum4,
        Powderblue,
        Purple,
        Purple1,
        Purple2,
        Purple3,
        Purple4,
        Red,
        Red1,
        Red2,
        Red3,
        Red4,
        Rosybrown,
        Rosybrown1,
        Rosybrown2,
        Rosybrown3,
        Rosybrown4,
        Royalblue,
        Royalblue1,
        Royalblue2,
        Royalblue3,
        Royalblue4,
        Saddlebrown,
        Salmon,
        Salmon1,
        Salmon2,
        Salmon3,
        Salmon4,
        Sandybrown,
        Seagreen,
        Seagreen1,
        Seagreen2,
        Seagreen3,
        Seagreen4,
        Seashell,
        Seashell1,
        Seashell2,
        Seashell3,
        Seashell4,
        Sienna,
        Sienna1,
        Sienna2,
        Sienna3,
        Sienna4,
        Silver,
        Skyblue,
        Skyblue1,
        Skyblue2,
        Skyblue3,
        Skyblue4,
        Slateblue,
        Slateblue1,
        Slateblue2,
        Slateblue3,
        Slateblue4,
        Slategray,
        Slategray1,
        Slategray2,
        Slategray3,
        Slategray4,
        Slategrey,
        Snow,
        Snow1,
        Snow2,
        Snow3,
        Snow4,
        Springgreen,
        Springgreen1,
        Springgreen2,
        Springgreen3,
        Springgreen4,
        Steelblue,
        Steelblue1,
        Steelblue2,
        Steelblue3,
        Steelblue4,
        Tan,
        Tan1,
        Tan2,
        Tan3,
        Tan4,
        Teal,
        Thistle,
        Thistle1,
        Thistle2,
        Thistle3,
        Thistle4,
        Tomato,
        Tomato1,
        Tomato2,
        Tomato3,
        Tomato4,
        Transparent,
        Turquoise,
        Turquoise1,
        Turquoise2,
        Turquoise3,
        Turquoise4,
        Violet,
        Violetred,
        Violetred1,
        Violetred2,
        Violetred3,
        Violetred4,
        Wheat,
        Wheat1,
        Wheat2,
        Wheat3,
        Wheat4,
        White,
        Whitesmoke,
        Yellow,
        Yellow1,
        Yellow2,
        Yellow3,
        Yellow4,
        Yellowgreen,
    }

    impl<'a> From<Shape> for Identity<'a> {
        fn from(shape: Shape) -> Self {
            Identity::String(match shape {
                Shape::Box => "box",
                Shape::Polygon => "polygon",
                Shape::Ellipse => "ellipse",
                Shape::Oval => "oval",
                Shape::Circle => "circle",
                Shape::Point => "point",
                Shape::Egg => "egg",
                Shape::Triangle => "triangle",
                Shape::Plaintext => "plaintext",
                Shape::Plain => "plain",
                Shape::Diamond => "diamond",
                Shape::Trapezium => "trapezium",
                Shape::Parallelogram => "parallelogram",
                Shape::House => "house",
                Shape::Pentagon => "pentagon",
                Shape::Hexagon => "hexagon",
                Shape::Septagon => "septagon",
                Shape::Octagon => "octagon",
                Shape::Doublecircle => "doublecircle",
                Shape::Doubleoctagon => "doubleoctagon",
                Shape::Tripleoctagon => "tripleoctagon",
                Shape::Invtriangle => "invtriangle",
                Shape::Invtrapezium => "invtrapezium",
                Shape::Invhouse => "invhouse",
                Shape::Mdiamond => "Mdiamond",
                Shape::Msquare => "Msquare",
                Shape::Mcircle => "Mcircle",
                Shape::Rect => "rect",
                Shape::Rectangle => "rectangle",
                Shape::Square => "square",
                Shape::Star => "star",
                Shape::None => "none",
                Shape::Underline => "underline",
                Shape::Cylinder => "cylinder",
                Shape::Note => "note",
                Shape::Tab => "tab",
                Shape::Folder => "folder",
                Shape::Box3d => "box3d",
                Shape::Component => "component",
                Shape::Promoter => "promoter",
                Shape::Cds => "cds",
                Shape::Terminator => "terminator",
                Shape::Utr => "utr",
                Shape::Primersite => "primersite",
                Shape::Restrictionsite => "restrictionsite",
                Shape::Fivepoverhang => "fivepoverhang",
                Shape::Threepoverhang => "threepoverhang",
                Shape::Noverhang => "noverhang",
                Shape::Assembly => "assembly",
                Shape::Signature => "signature",
                Shape::Insulator => "insulator",
                Shape::Ribosite => "ribosite",
                Shape::Rnastab => "rnastab",
                Shape::Proteasesite => "proteasesite",
                Shape::Proteinstab => "proteinstab",
                Shape::Rpromoter => "rpromoter",
                Shape::Rarrow => "rarrow",
                Shape::Larrow => "larrow",
                Shape::Lpromoter => "lpromoter",
            })
        }
    }


    fn arrow_str(ashape: ArrowShape) -> &'static str {
        match ashape {
            ArrowShape::Olbox => "olbox",
            ArrowShape::Olcrow => "olcrow",
            ArrowShape::Olcurve => "olcurve",
            ArrowShape::Olicurve => "olicurve",
            ArrowShape::Oldiamond => "oldiamond",
            ArrowShape::Oldot => "oldot",
            ArrowShape::Olinv => "olinv",
            ArrowShape::Olnone => "olnone",
            ArrowShape::Olnormal => "olnormal",
            ArrowShape::Oltee => "oltee",
            ArrowShape::Olvee => "olvee",
            ArrowShape::Orbox => "orbox",
            ArrowShape::Orcrow => "orcrow",
            ArrowShape::Orcurve => "orcurve",
            ArrowShape::Oricurve => "oricurve",
            ArrowShape::Ordiamond => "ordiamond",
            ArrowShape::Ordot => "ordot",
            ArrowShape::Orinv => "orinv",
            ArrowShape::Ornone => "ornone",
            ArrowShape::Ornormal => "ornormal",
            ArrowShape::Ortee => "ortee",
            ArrowShape::Orvee => "orvee",
            ArrowShape::Lbox => "lbox",
            ArrowShape::Lcrow => "lcrow",
            ArrowShape::Lcurve => "lcurve",
            ArrowShape::Licurve => "licurve",
            ArrowShape::Ldiamond => "ldiamond",
            ArrowShape::Ldot => "ldot",
            ArrowShape::Linv => "linv",
            ArrowShape::Lnone => "lnone",
            ArrowShape::Lnormal => "lnormal",
            ArrowShape::Ltee => "ltee",
            ArrowShape::Lvee => "lvee",
            ArrowShape::Rbox => "rbox",
            ArrowShape::Rcrow => "rcrow",
            ArrowShape::Rcurve => "rcurve",
            ArrowShape::Ricurve => "ricurve",
            ArrowShape::Rdiamond => "rdiamond",
            ArrowShape::Rdot => "rdot",
            ArrowShape::Rinv => "rinv",
            ArrowShape::Rnone => "rnone",
            ArrowShape::Rnormal => "rnormal",
            ArrowShape::Rtee => "rtee",
            ArrowShape::Rvee => "rvee",
            ArrowShape::Box => "box",
            ArrowShape::Crow => "crow",
            ArrowShape::Curve => "curve",
            ArrowShape::Icurve => "icurve",
            ArrowShape::Diamond => "diamond",
            ArrowShape::Dot => "dot",
            ArrowShape::Inv => "inv",
            ArrowShape::None => "none",
            ArrowShape::Normal => "normal",
            ArrowShape::Tee => "tee",
            ArrowShape::Vee => "vee"
        }
    }

    impl<'a> From<Color> for Identity<'a> {
        fn from(xc: Color) -> Self {
            if let Color::Rgb(r, g, b) = xc {
                return Identity::RGBA(r, g, b, 255);
            }
            if let Color::Rgba(r, g, b, a) = xc {
                return Identity::RGBA(r, g, b, a);
            }
            if let Color::HSV(h, s, v) = xc {
                return Identity::HSV(h, s, v);
            }
            Identity::String(match xc {
                Color::Aliceblue => "aliceblue",
                Color::Antiquewhite => "antiquewhite",
                Color::Antiquewhite1 => "antiquewhite1",
                Color::Antiquewhite2 => "antiquewhite2",
                Color::Antiquewhite3 => "antiquewhite3",
                Color::Antiquewhite4 => "antiquewhite4",
                Color::Aqua => "aqua",
                Color::Aquamarine => "aquamarine",
                Color::Aquamarine1 => "aquamarine1",
                Color::Aquamarine2 => "aquamarine2",
                Color::Aquamarine3 => "aquamarine3",
                Color::Aquamarine4 => "aquamarine4",
                Color::Azure => "azure",
                Color::Azure1 => "azure1",
                Color::Azure2 => "azure2",
                Color::Azure3 => "azure3",
                Color::Azure4 => "azure4",
                Color::Beige => "beige",
                Color::Bisque => "bisque",
                Color::Bisque1 => "bisque1",
                Color::Bisque2 => "bisque2",
                Color::Bisque3 => "bisque3",
                Color::Bisque4 => "bisque4",
                Color::Black => "black",
                Color::Blanchedalmond => "blanchedalmond",
                Color::Blue => "blue",
                Color::Blue1 => "blue1",
                Color::Blue2 => "blue2",
                Color::Blue3 => "blue3",
                Color::Blue4 => "blue4",
                Color::Blueviolet => "blueviolet",
                Color::Brown => "brown",
                Color::Brown1 => "brown1",
                Color::Brown2 => "brown2",
                Color::Brown3 => "brown3",
                Color::Brown4 => "brown4",
                Color::Burlywood => "burlywood",
                Color::Burlywood1 => "burlywood1",
                Color::Burlywood2 => "burlywood2",
                Color::Burlywood3 => "burlywood3",
                Color::Burlywood4 => "burlywood4",
                Color::Cadetblue => "cadetblue",
                Color::Cadetblue1 => "cadetblue1",
                Color::Cadetblue2 => "cadetblue2",
                Color::Cadetblue3 => "cadetblue3",
                Color::Cadetblue4 => "cadetblue4",
                Color::Chartreuse => "chartreuse",
                Color::Chartreuse1 => "chartreuse1",
                Color::Chartreuse2 => "chartreuse2",
                Color::Chartreuse3 => "chartreuse3",
                Color::Chartreuse4 => "chartreuse4",
                Color::Chocolate => "chocolate",
                Color::Chocolate1 => "chocolate1",
                Color::Chocolate2 => "chocolate2",
                Color::Chocolate3 => "chocolate3",
                Color::Chocolate4 => "chocolate4",
                Color::Coral => "coral",
                Color::Coral1 => "coral1",
                Color::Coral2 => "coral2",
                Color::Coral3 => "coral3",
                Color::Coral4 => "coral4",
                Color::Cornflowerblue => "cornflowerblue",
                Color::Cornsilk => "cornsilk",
                Color::Cornsilk1 => "cornsilk1",
                Color::Cornsilk2 => "cornsilk2",
                Color::Cornsilk3 => "cornsilk3",
                Color::Cornsilk4 => "cornsilk4",
                Color::Crimson => "crimson",
                Color::Cyan => "cyan",
                Color::Cyan1 => "cyan1",
                Color::Cyan2 => "cyan2",
                Color::Cyan3 => "cyan3",
                Color::Cyan4 => "cyan4",
                Color::Darkblue => "darkblue",
                Color::Darkcyan => "darkcyan",
                Color::Darkgoldenrod => "darkgoldenrod",
                Color::Darkgoldenrod1 => "darkgoldenrod1",
                Color::Darkgoldenrod2 => "darkgoldenrod2",
                Color::Darkgoldenrod3 => "darkgoldenrod3",
                Color::Darkgoldenrod4 => "darkgoldenrod4",
                Color::Darkgray => "darkgray",
                Color::Darkgreen => "darkgreen",
                Color::Darkgrey => "darkgrey",
                Color::Darkkhaki => "darkkhaki",
                Color::Darkmagenta => "darkmagenta",
                Color::Darkolivegreen => "darkolivegreen",
                Color::Darkolivegreen1 => "darkolivegreen1",
                Color::Darkolivegreen2 => "darkolivegreen2",
                Color::Darkolivegreen3 => "darkolivegreen3",
                Color::Darkolivegreen4 => "darkolivegreen4",
                Color::Darkorange => "darkorange",
                Color::Darkorange1 => "darkorange1",
                Color::Darkorange2 => "darkorange2",
                Color::Darkorange3 => "darkorange3",
                Color::Darkorange4 => "darkorange4",
                Color::Darkorchid => "darkorchid",
                Color::Darkorchid1 => "darkorchid1",
                Color::Darkorchid2 => "darkorchid2",
                Color::Darkorchid3 => "darkorchid3",
                Color::Darkorchid4 => "darkorchid4",
                Color::Darkred => "darkred",
                Color::Darksalmon => "darksalmon",
                Color::Darkseagreen => "darkseagreen",
                Color::Darkseagreen1 => "darkseagreen1",
                Color::Darkseagreen2 => "darkseagreen2",
                Color::Darkseagreen3 => "darkseagreen3",
                Color::Darkseagreen4 => "darkseagreen4",
                Color::Darkslateblue => "darkslateblue",
                Color::Darkslategray => "darkslategray",
                Color::Darkslategray1 => "darkslategray1",
                Color::Darkslategray2 => "darkslategray2",
                Color::Darkslategray3 => "darkslategray3",
                Color::Darkslategray4 => "darkslategray4",
                Color::Darkslategrey => "darkslategrey",
                Color::Darkturquoise => "darkturquoise",
                Color::Darkviolet => "darkviolet",
                Color::Deeppink => "deeppink",
                Color::Deeppink1 => "deeppink1",
                Color::Deeppink2 => "deeppink2",
                Color::Deeppink3 => "deeppink3",
                Color::Deeppink4 => "deeppink4",
                Color::Deepskyblue => "deepskyblue",
                Color::Deepskyblue1 => "deepskyblue1",
                Color::Deepskyblue2 => "deepskyblue2",
                Color::Deepskyblue3 => "deepskyblue3",
                Color::Deepskyblue4 => "deepskyblue4",
                Color::Dimgray => "dimgray",
                Color::Dimgrey => "dimgrey",
                Color::Dodgerblue => "dodgerblue",
                Color::Dodgerblue1 => "dodgerblue1",
                Color::Dodgerblue2 => "dodgerblue2",
                Color::Dodgerblue3 => "dodgerblue3",
                Color::Dodgerblue4 => "dodgerblue4",
                Color::Firebrick => "firebrick",
                Color::Firebrick1 => "firebrick1",
                Color::Firebrick2 => "firebrick2",
                Color::Firebrick3 => "firebrick3",
                Color::Firebrick4 => "firebrick4",
                Color::Floralwhite => "floralwhite",
                Color::Forestgreen => "forestgreen",
                Color::Fuchsia => "fuchsia",
                Color::Gainsboro => "gainsboro",
                Color::Ghostwhite => "ghostwhite",
                Color::Gold => "gold",
                Color::Gold1 => "gold1",
                Color::Gold2 => "gold2",
                Color::Gold3 => "gold3",
                Color::Gold4 => "gold4",
                Color::Goldenrod => "goldenrod",
                Color::Goldenrod1 => "goldenrod1",
                Color::Goldenrod2 => "goldenrod2",
                Color::Goldenrod3 => "goldenrod3",
                Color::Goldenrod4 => "goldenrod4",
                Color::Gray => "gray",
                Color::Gray0 => "gray0",
                Color::Gray1 => "gray1",
                Color::Gray10 => "gray10",
                Color::Gray100 => "gray100",
                Color::Gray11 => "gray11",
                Color::Gray12 => "gray12",
                Color::Gray13 => "gray13",
                Color::Gray14 => "gray14",
                Color::Gray15 => "gray15",
                Color::Gray16 => "gray16",
                Color::Gray17 => "gray17",
                Color::Gray18 => "gray18",
                Color::Gray19 => "gray19",
                Color::Gray2 => "gray2",
                Color::Gray20 => "gray20",
                Color::Gray21 => "gray21",
                Color::Gray22 => "gray22",
                Color::Gray23 => "gray23",
                Color::Gray24 => "gray24",
                Color::Gray25 => "gray25",
                Color::Gray26 => "gray26",
                Color::Gray27 => "gray27",
                Color::Gray28 => "gray28",
                Color::Gray29 => "gray29",
                Color::Gray3 => "gray3",
                Color::Gray30 => "gray30",
                Color::Gray31 => "gray31",
                Color::Gray32 => "gray32",
                Color::Gray33 => "gray33",
                Color::Gray34 => "gray34",
                Color::Gray35 => "gray35",
                Color::Gray36 => "gray36",
                Color::Gray37 => "gray37",
                Color::Gray38 => "gray38",
                Color::Gray39 => "gray39",
                Color::Gray4 => "gray4",
                Color::Gray40 => "gray40",
                Color::Gray41 => "gray41",
                Color::Gray42 => "gray42",
                Color::Gray43 => "gray43",
                Color::Gray44 => "gray44",
                Color::Gray45 => "gray45",
                Color::Gray46 => "gray46",
                Color::Gray47 => "gray47",
                Color::Gray48 => "gray48",
                Color::Gray49 => "gray49",
                Color::Gray5 => "gray5",
                Color::Gray50 => "gray50",
                Color::Gray51 => "gray51",
                Color::Gray52 => "gray52",
                Color::Gray53 => "gray53",
                Color::Gray54 => "gray54",
                Color::Gray55 => "gray55",
                Color::Gray56 => "gray56",
                Color::Gray57 => "gray57",
                Color::Gray58 => "gray58",
                Color::Gray59 => "gray59",
                Color::Gray6 => "gray6",
                Color::Gray60 => "gray60",
                Color::Gray61 => "gray61",
                Color::Gray62 => "gray62",
                Color::Gray63 => "gray63",
                Color::Gray64 => "gray64",
                Color::Gray65 => "gray65",
                Color::Gray66 => "gray66",
                Color::Gray67 => "gray67",
                Color::Gray68 => "gray68",
                Color::Gray69 => "gray69",
                Color::Gray7 => "gray7",
                Color::Gray70 => "gray70",
                Color::Gray71 => "gray71",
                Color::Gray72 => "gray72",
                Color::Gray73 => "gray73",
                Color::Gray74 => "gray74",
                Color::Gray75 => "gray75",
                Color::Gray76 => "gray76",
                Color::Gray77 => "gray77",
                Color::Gray78 => "gray78",
                Color::Gray79 => "gray79",
                Color::Gray8 => "gray8",
                Color::Gray80 => "gray80",
                Color::Gray81 => "gray81",
                Color::Gray82 => "gray82",
                Color::Gray83 => "gray83",
                Color::Gray84 => "gray84",
                Color::Gray85 => "gray85",
                Color::Gray86 => "gray86",
                Color::Gray87 => "gray87",
                Color::Gray88 => "gray88",
                Color::Gray89 => "gray89",
                Color::Gray9 => "gray9",
                Color::Gray90 => "gray90",
                Color::Gray91 => "gray91",
                Color::Gray92 => "gray92",
                Color::Gray93 => "gray93",
                Color::Gray94 => "gray94",
                Color::Gray95 => "gray95",
                Color::Gray96 => "gray96",
                Color::Gray97 => "gray97",
                Color::Gray98 => "gray98",
                Color::Gray99 => "gray99",
                Color::Green => "green",
                Color::Green1 => "green1",
                Color::Green2 => "green2",
                Color::Green3 => "green3",
                Color::Green4 => "green4",
                Color::Greenyellow => "greenyellow",
                Color::Grey => "grey",
                Color::Grey0 => "grey0",
                Color::Grey1 => "grey1",
                Color::Grey10 => "grey10",
                Color::Grey100 => "grey100",
                Color::Grey11 => "grey11",
                Color::Grey12 => "grey12",
                Color::Grey13 => "grey13",
                Color::Grey14 => "grey14",
                Color::Grey15 => "grey15",
                Color::Grey16 => "grey16",
                Color::Grey17 => "grey17",
                Color::Grey18 => "grey18",
                Color::Grey19 => "grey19",
                Color::Grey2 => "grey2",
                Color::Grey20 => "grey20",
                Color::Grey21 => "grey21",
                Color::Grey22 => "grey22",
                Color::Grey23 => "grey23",
                Color::Grey24 => "grey24",
                Color::Grey25 => "grey25",
                Color::Grey26 => "grey26",
                Color::Grey27 => "grey27",
                Color::Grey28 => "grey28",
                Color::Grey29 => "grey29",
                Color::Grey3 => "grey3",
                Color::Grey30 => "grey30",
                Color::Grey31 => "grey31",
                Color::Grey32 => "grey32",
                Color::Grey33 => "grey33",
                Color::Grey34 => "grey34",
                Color::Grey35 => "grey35",
                Color::Grey36 => "grey36",
                Color::Grey37 => "grey37",
                Color::Grey38 => "grey38",
                Color::Grey39 => "grey39",
                Color::Grey4 => "grey4",
                Color::Grey40 => "grey40",
                Color::Grey41 => "grey41",
                Color::Grey42 => "grey42",
                Color::Grey43 => "grey43",
                Color::Grey44 => "grey44",
                Color::Grey45 => "grey45",
                Color::Grey46 => "grey46",
                Color::Grey47 => "grey47",
                Color::Grey48 => "grey48",
                Color::Grey49 => "grey49",
                Color::Grey5 => "grey5",
                Color::Grey50 => "grey50",
                Color::Grey51 => "grey51",
                Color::Grey52 => "grey52",
                Color::Grey53 => "grey53",
                Color::Grey54 => "grey54",
                Color::Grey55 => "grey55",
                Color::Grey56 => "grey56",
                Color::Grey57 => "grey57",
                Color::Grey58 => "grey58",
                Color::Grey59 => "grey59",
                Color::Grey6 => "grey6",
                Color::Grey60 => "grey60",
                Color::Grey61 => "grey61",
                Color::Grey62 => "grey62",
                Color::Grey63 => "grey63",
                Color::Grey64 => "grey64",
                Color::Grey65 => "grey65",
                Color::Grey66 => "grey66",
                Color::Grey67 => "grey67",
                Color::Grey68 => "grey68",
                Color::Grey69 => "grey69",
                Color::Grey7 => "grey7",
                Color::Grey70 => "grey70",
                Color::Grey71 => "grey71",
                Color::Grey72 => "grey72",
                Color::Grey73 => "grey73",
                Color::Grey74 => "grey74",
                Color::Grey75 => "grey75",
                Color::Grey76 => "grey76",
                Color::Grey77 => "grey77",
                Color::Grey78 => "grey78",
                Color::Grey79 => "grey79",
                Color::Grey8 => "grey8",
                Color::Grey80 => "grey80",
                Color::Grey81 => "grey81",
                Color::Grey82 => "grey82",
                Color::Grey83 => "grey83",
                Color::Grey84 => "grey84",
                Color::Grey85 => "grey85",
                Color::Grey86 => "grey86",
                Color::Grey87 => "grey87",
                Color::Grey88 => "grey88",
                Color::Grey89 => "grey89",
                Color::Grey9 => "grey9",
                Color::Grey90 => "grey90",
                Color::Grey91 => "grey91",
                Color::Grey92 => "grey92",
                Color::Grey93 => "grey93",
                Color::Grey94 => "grey94",
                Color::Grey95 => "grey95",
                Color::Grey96 => "grey96",
                Color::Grey97 => "grey97",
                Color::Grey98 => "grey98",
                Color::Grey99 => "grey99",
                Color::Honeydew => "honeydew",
                Color::Honeydew1 => "honeydew1",
                Color::Honeydew2 => "honeydew2",
                Color::Honeydew3 => "honeydew3",
                Color::Honeydew4 => "honeydew4",
                Color::Hotpink => "hotpink",
                Color::Hotpink1 => "hotpink1",
                Color::Hotpink2 => "hotpink2",
                Color::Hotpink3 => "hotpink3",
                Color::Hotpink4 => "hotpink4",
                Color::Indianred => "indianred",
                Color::Indianred1 => "indianred1",
                Color::Indianred2 => "indianred2",
                Color::Indianred3 => "indianred3",
                Color::Indianred4 => "indianred4",
                Color::Indigo => "indigo",
                Color::Invis => "invis",
                Color::Ivory => "ivory",
                Color::Ivory1 => "ivory1",
                Color::Ivory2 => "ivory2",
                Color::Ivory3 => "ivory3",
                Color::Ivory4 => "ivory4",
                Color::Khaki => "khaki",
                Color::Khaki1 => "khaki1",
                Color::Khaki2 => "khaki2",
                Color::Khaki3 => "khaki3",
                Color::Khaki4 => "khaki4",
                Color::Lavender => "lavender",
                Color::Lavenderblush => "lavenderblush",
                Color::Lavenderblush1 => "lavenderblush1",
                Color::Lavenderblush2 => "lavenderblush2",
                Color::Lavenderblush3 => "lavenderblush3",
                Color::Lavenderblush4 => "lavenderblush4",
                Color::Lawngreen => "lawngreen",
                Color::Lemonchiffon => "lemonchiffon",
                Color::Lemonchiffon1 => "lemonchiffon1",
                Color::Lemonchiffon2 => "lemonchiffon2",
                Color::Lemonchiffon3 => "lemonchiffon3",
                Color::Lemonchiffon4 => "lemonchiffon4",
                Color::Lightblue => "lightblue",
                Color::Lightblue1 => "lightblue1",
                Color::Lightblue2 => "lightblue2",
                Color::Lightblue3 => "lightblue3",
                Color::Lightblue4 => "lightblue4",
                Color::Lightcoral => "lightcoral",
                Color::Lightcyan => "lightcyan",
                Color::Lightcyan1 => "lightcyan1",
                Color::Lightcyan2 => "lightcyan2",
                Color::Lightcyan3 => "lightcyan3",
                Color::Lightcyan4 => "lightcyan4",
                Color::Lightgoldenrod => "lightgoldenrod",
                Color::Lightgoldenrod1 => "lightgoldenrod1",
                Color::Lightgoldenrod2 => "lightgoldenrod2",
                Color::Lightgoldenrod3 => "lightgoldenrod3",
                Color::Lightgoldenrod4 => "lightgoldenrod4",
                Color::Lightgoldenrodyellow => "lightgoldenrodyellow",
                Color::Lightgray => "lightgray",
                Color::Lightgreen => "lightgreen",
                Color::Lightgrey => "lightgrey",
                Color::Lightpink => "lightpink",
                Color::Lightpink1 => "lightpink1",
                Color::Lightpink2 => "lightpink2",
                Color::Lightpink3 => "lightpink3",
                Color::Lightpink4 => "lightpink4",
                Color::Lightsalmon => "lightsalmon",
                Color::Lightsalmon1 => "lightsalmon1",
                Color::Lightsalmon2 => "lightsalmon2",
                Color::Lightsalmon3 => "lightsalmon3",
                Color::Lightsalmon4 => "lightsalmon4",
                Color::Lightseagreen => "lightseagreen",
                Color::Lightskyblue => "lightskyblue",
                Color::Lightskyblue1 => "lightskyblue1",
                Color::Lightskyblue2 => "lightskyblue2",
                Color::Lightskyblue3 => "lightskyblue3",
                Color::Lightskyblue4 => "lightskyblue4",
                Color::Lightslateblue => "lightslateblue",
                Color::Lightslategray => "lightslategray",
                Color::Lightslategrey => "lightslategrey",
                Color::Lightsteelblue => "lightsteelblue",
                Color::Lightsteelblue1 => "lightsteelblue1",
                Color::Lightsteelblue2 => "lightsteelblue2",
                Color::Lightsteelblue3 => "lightsteelblue3",
                Color::Lightsteelblue4 => "lightsteelblue4",
                Color::Lightyellow => "lightyellow",
                Color::Lightyellow1 => "lightyellow1",
                Color::Lightyellow2 => "lightyellow2",
                Color::Lightyellow3 => "lightyellow3",
                Color::Lightyellow4 => "lightyellow4",
                Color::Lime => "lime",
                Color::Limegreen => "limegreen",
                Color::Linen => "linen",
                Color::Magenta => "magenta",
                Color::Magenta1 => "magenta1",
                Color::Magenta2 => "magenta2",
                Color::Magenta3 => "magenta3",
                Color::Magenta4 => "magenta4",
                Color::Maroon => "maroon",
                Color::Maroon1 => "maroon1",
                Color::Maroon2 => "maroon2",
                Color::Maroon3 => "maroon3",
                Color::Maroon4 => "maroon4",
                Color::Mediumaquamarine => "mediumaquamarine",
                Color::Mediumblue => "mediumblue",
                Color::Mediumorchid => "mediumorchid",
                Color::Mediumorchid1 => "mediumorchid1",
                Color::Mediumorchid2 => "mediumorchid2",
                Color::Mediumorchid3 => "mediumorchid3",
                Color::Mediumorchid4 => "mediumorchid4",
                Color::Mediumpurple => "mediumpurple",
                Color::Mediumpurple1 => "mediumpurple1",
                Color::Mediumpurple2 => "mediumpurple2",
                Color::Mediumpurple3 => "mediumpurple3",
                Color::Mediumpurple4 => "mediumpurple4",
                Color::Mediumseagreen => "mediumseagreen",
                Color::Mediumslateblue => "mediumslateblue",
                Color::Mediumspringgreen => "mediumspringgreen",
                Color::Mediumturquoise => "mediumturquoise",
                Color::Mediumvioletred => "mediumvioletred",
                Color::Midnightblue => "midnightblue",
                Color::Mintcream => "mintcream",
                Color::Mistyrose => "mistyrose",
                Color::Mistyrose1 => "mistyrose1",
                Color::Mistyrose2 => "mistyrose2",
                Color::Mistyrose3 => "mistyrose3",
                Color::Mistyrose4 => "mistyrose4",
                Color::Moccasin => "moccasin",
                Color::Navajowhite => "navajowhite",
                Color::Navajowhite1 => "navajowhite1",
                Color::Navajowhite2 => "navajowhite2",
                Color::Navajowhite3 => "navajowhite3",
                Color::Navajowhite4 => "navajowhite4",
                Color::Navy => "navy",
                Color::Navyblue => "navyblue",
                Color::None => "none",
                Color::Oldlace => "oldlace",
                Color::Olive => "olive",
                Color::Olivedrab => "olivedrab",
                Color::Olivedrab1 => "olivedrab1",
                Color::Olivedrab2 => "olivedrab2",
                Color::Olivedrab3 => "olivedrab3",
                Color::Olivedrab4 => "olivedrab4",
                Color::Orange => "orange",
                Color::Orange1 => "orange1",
                Color::Orange2 => "orange2",
                Color::Orange3 => "orange3",
                Color::Orange4 => "orange4",
                Color::Orangered => "orangered",
                Color::Orangered1 => "orangered1",
                Color::Orangered2 => "orangered2",
                Color::Orangered3 => "orangered3",
                Color::Orangered4 => "orangered4",
                Color::Orchid => "orchid",
                Color::Orchid1 => "orchid1",
                Color::Orchid2 => "orchid2",
                Color::Orchid3 => "orchid3",
                Color::Orchid4 => "orchid4",
                Color::Palegoldenrod => "palegoldenrod",
                Color::Palegreen => "palegreen",
                Color::Palegreen1 => "palegreen1",
                Color::Palegreen2 => "palegreen2",
                Color::Palegreen3 => "palegreen3",
                Color::Palegreen4 => "palegreen4",
                Color::Paleturquoise => "paleturquoise",
                Color::Paleturquoise1 => "paleturquoise1",
                Color::Paleturquoise2 => "paleturquoise2",
                Color::Paleturquoise3 => "paleturquoise3",
                Color::Paleturquoise4 => "paleturquoise4",
                Color::Palevioletred => "palevioletred",
                Color::Palevioletred1 => "palevioletred1",
                Color::Palevioletred2 => "palevioletred2",
                Color::Palevioletred3 => "palevioletred3",
                Color::Palevioletred4 => "palevioletred4",
                Color::Papayawhip => "papayawhip",
                Color::Peachpuff => "peachpuff",
                Color::Peachpuff1 => "peachpuff1",
                Color::Peachpuff2 => "peachpuff2",
                Color::Peachpuff3 => "peachpuff3",
                Color::Peachpuff4 => "peachpuff4",
                Color::Peru => "peru",
                Color::Pink => "pink",
                Color::Pink1 => "pink1",
                Color::Pink2 => "pink2",
                Color::Pink3 => "pink3",
                Color::Pink4 => "pink4",
                Color::Plum => "plum",
                Color::Plum1 => "plum1",
                Color::Plum2 => "plum2",
                Color::Plum3 => "plum3",
                Color::Plum4 => "plum4",
                Color::Powderblue => "powderblue",
                Color::Purple => "purple",
                Color::Purple1 => "purple1",
                Color::Purple2 => "purple2",
                Color::Purple3 => "purple3",
                Color::Purple4 => "purple4",
                Color::Red => "red",
                Color::Red1 => "red1",
                Color::Red2 => "red2",
                Color::Red3 => "red3",
                Color::Red4 => "red4",
                Color::Rosybrown => "rosybrown",
                Color::Rosybrown1 => "rosybrown1",
                Color::Rosybrown2 => "rosybrown2",
                Color::Rosybrown3 => "rosybrown3",
                Color::Rosybrown4 => "rosybrown4",
                Color::Royalblue => "royalblue",
                Color::Royalblue1 => "royalblue1",
                Color::Royalblue2 => "royalblue2",
                Color::Royalblue3 => "royalblue3",
                Color::Royalblue4 => "royalblue4",
                Color::Saddlebrown => "saddlebrown",
                Color::Salmon => "salmon",
                Color::Salmon1 => "salmon1",
                Color::Salmon2 => "salmon2",
                Color::Salmon3 => "salmon3",
                Color::Salmon4 => "salmon4",
                Color::Sandybrown => "sandybrown",
                Color::Seagreen => "seagreen",
                Color::Seagreen1 => "seagreen1",
                Color::Seagreen2 => "seagreen2",
                Color::Seagreen3 => "seagreen3",
                Color::Seagreen4 => "seagreen4",
                Color::Seashell => "seashell",
                Color::Seashell1 => "seashell1",
                Color::Seashell2 => "seashell2",
                Color::Seashell3 => "seashell3",
                Color::Seashell4 => "seashell4",
                Color::Sienna => "sienna",
                Color::Sienna1 => "sienna1",
                Color::Sienna2 => "sienna2",
                Color::Sienna3 => "sienna3",
                Color::Sienna4 => "sienna4",
                Color::Silver => "silver",
                Color::Skyblue => "skyblue",
                Color::Skyblue1 => "skyblue1",
                Color::Skyblue2 => "skyblue2",
                Color::Skyblue3 => "skyblue3",
                Color::Skyblue4 => "skyblue4",
                Color::Slateblue => "slateblue",
                Color::Slateblue1 => "slateblue1",
                Color::Slateblue2 => "slateblue2",
                Color::Slateblue3 => "slateblue3",
                Color::Slateblue4 => "slateblue4",
                Color::Slategray => "slategray",
                Color::Slategray1 => "slategray1",
                Color::Slategray2 => "slategray2",
                Color::Slategray3 => "slategray3",
                Color::Slategray4 => "slategray4",
                Color::Slategrey => "slategrey",
                Color::Snow => "snow",
                Color::Snow1 => "snow1",
                Color::Snow2 => "snow2",
                Color::Snow3 => "snow3",
                Color::Snow4 => "snow4",
                Color::Springgreen => "springgreen",
                Color::Springgreen1 => "springgreen1",
                Color::Springgreen2 => "springgreen2",
                Color::Springgreen3 => "springgreen3",
                Color::Springgreen4 => "springgreen4",
                Color::Steelblue => "steelblue",
                Color::Steelblue1 => "steelblue1",
                Color::Steelblue2 => "steelblue2",
                Color::Steelblue3 => "steelblue3",
                Color::Steelblue4 => "steelblue4",
                Color::Tan => "tan",
                Color::Tan1 => "tan1",
                Color::Tan2 => "tan2",
                Color::Tan3 => "tan3",
                Color::Tan4 => "tan4",
                Color::Teal => "teal",
                Color::Thistle => "thistle",
                Color::Thistle1 => "thistle1",
                Color::Thistle2 => "thistle2",
                Color::Thistle3 => "thistle3",
                Color::Thistle4 => "thistle4",
                Color::Tomato => "tomato",
                Color::Tomato1 => "tomato1",
                Color::Tomato2 => "tomato2",
                Color::Tomato3 => "tomato3",
                Color::Tomato4 => "tomato4",
                Color::Transparent => "transparent",
                Color::Turquoise => "turquoise",
                Color::Turquoise1 => "turquoise1",
                Color::Turquoise2 => "turquoise2",
                Color::Turquoise3 => "turquoise3",
                Color::Turquoise4 => "turquoise4",
                Color::Violet => "violet",
                Color::Violetred => "violetred",
                Color::Violetred1 => "violetred1",
                Color::Violetred2 => "violetred2",
                Color::Violetred3 => "violetred3",
                Color::Violetred4 => "violetred4",
                Color::Wheat => "wheat",
                Color::Wheat1 => "wheat1",
                Color::Wheat2 => "wheat2",
                Color::Wheat3 => "wheat3",
                Color::Wheat4 => "wheat4",
                Color::White => "white",
                Color::Whitesmoke => "whitesmoke",
                Color::Yellow => "yellow",
                Color::Yellow1 => "yellow1",
                Color::Yellow2 => "yellow2",
                Color::Yellow3 => "yellow3",
                Color::Yellow4 => "yellow4",
                Color::Yellowgreen => "yellowgreen",
                _ => unsafe {unreachable_unchecked()}
            })
        }
    }
}

