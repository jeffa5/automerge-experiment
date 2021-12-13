extern crate hex;
extern crate uuid;
extern crate web_sys;

use automerge_protocol as amp;
use std::cmp::Eq;

pub const HEAD: ElemId = ElemId(OpId(0, 0));
pub const ROOT: OpId = OpId(0, 0);

const ROOT_STR: &str = "_root";
const HEAD_STR: &str = "_head";

#[derive(Debug)]
pub enum Export {
    Id(OpId),
    Special(String),
    Prop(usize),
}

pub trait Exportable {
    fn export(&self) -> Export;
}

pub trait Importable {
    fn wrap(id: OpId) -> Self;
    fn from(s: &str) -> Option<Self>
    where
        Self: std::marker::Sized;
}

impl OpId {
    #[inline]
    pub fn counter(&self) -> u64 {
        self.0
    }
    #[inline]
    pub fn actor(&self) -> usize {
        self.1
    }
}

impl Exportable for ObjId {
    fn export(&self) -> Export {
        if self.0 == ROOT {
            Export::Special(ROOT_STR.to_owned())
        } else {
            Export::Id(self.0)
        }
    }
}

impl Exportable for &ObjId {
    fn export(&self) -> Export {
        if self.0 == ROOT {
            Export::Special(ROOT_STR.to_owned())
        } else {
            Export::Id(self.0)
        }
    }
}

impl Exportable for ElemId {
    fn export(&self) -> Export {
        if self == &HEAD {
            Export::Special(HEAD_STR.to_owned())
        } else {
            Export::Id(self.0)
        }
    }
}

impl Exportable for OpId {
    fn export(&self) -> Export {
        Export::Id(*self)
    }
}

impl Exportable for Key {
    fn export(&self) -> Export {
        match self {
            Key::Map(p) => Export::Prop(*p),
            Key::Seq(e) => e.export(),
        }
    }
}

impl Importable for ObjId {
    fn wrap(id: OpId) -> Self {
        ObjId(id)
    }
    fn from(s: &str) -> Option<Self> {
        if s == ROOT_STR {
            Some(ROOT.into())
        } else {
            None
        }
    }
}

impl Importable for OpId {
    fn wrap(id: OpId) -> Self {
        id
    }
    fn from(s: &str) -> Option<Self> {
        if s == ROOT_STR {
            Some(ROOT)
        } else {
            None
        }
    }
}

impl Importable for ElemId {
    fn wrap(id: OpId) -> Self {
        ElemId(id)
    }
    fn from(s: &str) -> Option<Self> {
        if s == HEAD_STR {
            Some(HEAD)
        } else {
            None
        }
    }
}

impl From<OpId> for ObjId {
    fn from(o: OpId) -> Self {
        ObjId(o)
    }
}

impl From<OpId> for ElemId {
    fn from(o: OpId) -> Self {
        ElemId(o)
    }
}

impl From<String> for Prop {
    fn from(p: String) -> Self {
        Prop::Map(p)
    }
}

impl From<&str> for Prop {
    fn from(p: &str) -> Self {
        Prop::Map(p.to_owned())
    }
}

impl From<usize> for Prop {
    fn from(index: usize) -> Self {
        Prop::Seq(index)
    }
}

impl From<f64> for Prop {
    fn from(index: f64) -> Self {
        Prop::Seq(index as usize)
    }
}

impl From<OpId> for Key {
    fn from(id: OpId) -> Self {
        Key::Seq(ElemId(id))
    }
}

impl From<ElemId> for Key {
    fn from(e: ElemId) -> Self {
        Key::Seq(e)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub enum Key {
    /// Index into a cache with the actual key.
    Map(usize),
    Seq(ElemId),
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum Prop {
    Map(String),
    Seq(usize),
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct Patch {}

impl Key {
    pub fn elemid(&self) -> Option<ElemId> {
        match self {
            Key::Map(_) => None,
            Key::Seq(id) => Some(*id),
        }
    }
}

#[derive(Debug, Clone, PartialOrd, Ord, Eq, PartialEq, Copy, Hash, Default)]
pub struct OpId(pub u64, pub usize);

#[derive(Debug, Clone, Copy, PartialOrd, Eq, PartialEq, Ord, Hash, Default)]
pub struct ObjId(pub OpId);

#[derive(Debug, Clone, Copy, PartialOrd, Eq, PartialEq, Ord, Hash, Default)]
pub struct ElemId(pub OpId);

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Op {
    pub change: usize,
    pub id: OpId,
    pub action: amp::OpType,
    pub obj: ObjId,
    pub key: Key,
    pub succ: Vec<OpId>,
    pub pred: Vec<OpId>,
    pub insert: bool,
}

impl Op {
    pub fn is_del(&self) -> bool {
        matches!(self.action, amp::OpType::Del(_))
    }

    pub fn overwrites(&self, other: &Op) -> bool {
        self.pred.iter().any(|i| i == &other.id)
    }

    pub fn elemid(&self) -> Option<ElemId> {
        if self.insert {
            Some(ElemId(self.id))
        } else {
            self.key.elemid()
        }
    }

    #[allow(dead_code)]
    pub fn dump(&self) -> String {
        match &self.action {
            amp::OpType::Set(value) if self.insert => format!("i:{}", value),
            amp::OpType::Set(value) => format!("s:{}", value),
            amp::OpType::Make(obj) => format!("make{}", obj),
            amp::OpType::Inc(val) => format!("inc:{}", val),
            amp::OpType::Del(_) => "del".to_string(),
            amp::OpType::MultiSet(_) => "multiset".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Peer {}
