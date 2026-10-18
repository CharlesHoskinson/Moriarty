// ------------------------------------------------------- iterative Drop C8
fn dummy_expr() -> CoreExpr { CoreExpr::Literal { value: CoreValue::Bool { b: false } } }
fn dummy_tree() -> CoreTree { CoreTree::Leaf { n: i128_class::KIntC::InRange(0) } }

fn take_value_children(v: &mut CoreValue, tw: &mut Vec<Box<CoreTree>>) {
    if let CoreValue::Tree { tree } = v {
        tw.push(Box::new(std::mem::replace(tree, dummy_tree())));
    }
}
fn take_expr_children(e: &mut CoreExpr, ew: &mut Vec<Box<CoreExpr>>, tw: &mut Vec<Box<CoreTree>>) {
    match e {
        CoreExpr::Literal { value } => take_value_children(value, tw),
        CoreExpr::Var { .. } => {}
        CoreExpr::Let { value, body, .. } => {
            ew.push(std::mem::replace(value, Box::new(dummy_expr())));
            ew.push(std::mem::replace(body, Box::new(dummy_expr())));
        }
        CoreExpr::Binary { left, right, .. } => {
            ew.push(std::mem::replace(left, Box::new(dummy_expr())));
            ew.push(std::mem::replace(right, Box::new(dummy_expr())));
        }
        CoreExpr::Unary { operand, .. } => {
            ew.push(std::mem::replace(operand, Box::new(dummy_expr())));
        }
        CoreExpr::If { cond, then_expr, else_expr } => {
            ew.push(std::mem::replace(cond, Box::new(dummy_expr())));
            ew.push(std::mem::replace(then_expr, Box::new(dummy_expr())));
            ew.push(std::mem::replace(else_expr, Box::new(dummy_expr())));
        }
        CoreExpr::Call { args, .. } => {
            for a in std::mem::take(args) { ew.push(Box::new(a)); }
        }
        CoreExpr::Match { scrutinee, arms } => {
            ew.push(std::mem::replace(scrutinee, Box::new(dummy_expr())));
            for arm in std::mem::take(arms) { ew.push(Box::new(arm.body)); }
        }
    }
}
fn take_tree_children(t: &mut CoreTree, tw: &mut Vec<Box<CoreTree>>) {
    if let CoreTree::Branch { left, right } = t {
        tw.push(std::mem::replace(left, Box::new(dummy_tree())));
        tw.push(std::mem::replace(right, Box::new(dummy_tree())));
    }
}
fn drain_worklists(ew: &mut Vec<Box<CoreExpr>>, tw: &mut Vec<Box<CoreTree>>) {
    loop {
        if let Some(mut b) = ew.pop() { take_expr_children(&mut b, ew, tw); continue; }
        if let Some(mut b) = tw.pop() { take_tree_children(&mut b, tw); continue; }
        break;
    }
}
impl Drop for CoreExpr {
    fn drop(&mut self) {
        let mut ew: Vec<Box<CoreExpr>> = Vec::new();
        let mut tw: Vec<Box<CoreTree>> = Vec::new();
        take_expr_children(self, &mut ew, &mut tw);
        drain_worklists(&mut ew, &mut tw);
    }
}
impl Drop for CoreTree {
    fn drop(&mut self) {
        let mut tw: Vec<Box<CoreTree>> = Vec::new();
        take_tree_children(self, &mut tw);
        let mut ew: Vec<Box<CoreExpr>> = Vec::new();
        drain_worklists(&mut ew, &mut tw);
    }
}
impl Drop for CoreProgram {
    fn drop(&mut self) {
        let mut ew: Vec<Box<CoreExpr>> = Vec::new();
        let mut tw: Vec<Box<CoreTree>> = Vec::new();
        ew.push(Box::new(std::mem::replace(&mut self.entry, dummy_expr())));
        for f in std::mem::take(&mut self.functions) { ew.push(Box::new(f.body)); }
        drain_worklists(&mut ew, &mut tw);
    }
}

