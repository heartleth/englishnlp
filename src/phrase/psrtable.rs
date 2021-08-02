use std::collections::HashMap;
use super::*;

pub type Coord = (usize, Part, usize);

#[derive(Clone)]
pub struct Item {
    pub structure :Vec<ExpectmentParent>,
    pub parent :Coord,
    pub forward :Coord,
    pub is_essential :bool
}

pub struct Table {
    pub m: HashMap<Coord, Item>
}
impl Table {
    pub fn new()->Table {
        Table { m: HashMap::new() }
    }
    pub fn has(&self, p :usize, part :Part, nth :usize)->bool {
        self.m.contains_key(&(p, part, nth))
    }
    pub fn get(&self, p :usize, part :Part, nth :usize)->Option<&Item> {
        self.m.get(&(p, part, nth))
    }
    pub fn check(&self, p :usize, part :Part)->usize {
        self.m.keys().filter(|(ps, pr, _)| (*ps, *pr)==(p, part)).count()
    }
    pub fn set(&mut self, p :usize, part :Part, forward :Coord, parent: Coord, rule :Vec<ExpectmentParent>, gp :bool)->usize {
        let n = self.m.keys().filter(|(ps, pr, _)| (*ps, *pr)==(p, part)).count();
        self.m.insert((p, part, n), Item { structure: rule, parent: parent, forward: forward, is_essential: gp });
        return n;
    }
    pub fn delete(&mut self, p :usize, part :Part, nth :usize)->Option<Item> {
        self.m.remove(&(p, part, nth))
    }
    pub fn delete_family(&mut self, p :usize, part :Part, nth :usize) {
        let mut now = (p, part, nth);
        loop {
            if let Some(k) = self.delete(now.0, now.1, now.2) {
                if k.is_essential {
                    let tmp = k.parent;
                    now = tmp;
                }
                else {
                    break;
                }
            }
            else {
                break;
            }
        }
    }
    pub fn find_childs(&self, p :usize, part :Part, nth :usize)->Vec<Coord> {
        let mut r :Vec<Coord> = self.m.iter().filter(|(_, b)| b.parent==(p, part, nth)).map(|(a, _)|*a).collect();
        r.sort_by_key(|e|e.0);
        r
    }
    fn tree_<'w>(&self, p :usize, part :Part, nth :usize, s :&'w [Word<'w>])->std::result::Result<DiagramNode<'w>, ()> {
        let m = self.find_childs(p, part, nth);
        if m.is_empty() {
            return Ok(DiagramNode::new(DiagramNodeEnum::Leaf(s[p].determine(part).unwrap()), part));
        }
        else {
            let k :Vec<_> = m.iter().map(|(p, pr, nth)| self.tree_(*p, *pr, *nth, s)).collect();
            let mut nk = Vec::new();
            for elem in k {
                nk.push(elem?);
            }
            return Ok(DiagramNode::new(DiagramNodeEnum::Node(nk), part));
        }
    }
    pub fn tree<'w>(&self, crd :Coord, s :&'w [Word<'w>])->std::result::Result<DiagramNode<'w>, ()> {
        let mut ret = HashMap::new();
        let mut now = crd;
        while now.1 != Part::None {
            let e = self.get(now.0, now.1, now.2).ok_or(())?;
            ret.insert(now, e.clone());
            now = e.forward;
        }
        let new = Table { m: ret };
        let mut fc = new.find_childs(0, Part::None, 0);
        if fc.len() == 0 {
            return Err(());
        }
        let root = fc.remove(0);
        
        new.tree_(root.0, root.1, root.2, s)
    }
}