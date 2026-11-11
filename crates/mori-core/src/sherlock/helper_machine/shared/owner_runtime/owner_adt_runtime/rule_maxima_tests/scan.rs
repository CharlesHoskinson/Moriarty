//! Rule-sliced text scan of the Core rule code (test-only, no circuit).
//!
//! For one rule the scan starts at the family entry function that
//! `family_of_rule` names, removes every `match` arm whose pattern names
//! other rules only (and a `_` arm when a pure arm names this rule), and
//! follows free calls, `reads.<method>` calls into `owner_adt_runtime` and
//! `GuardId::<name>` uses into the guard arms of `eval_guard`. The kept text
//! is an over-approximation of the code the rule can run: every branch that
//! is not ruled out by a rule pattern stays in.
//!
//! From the kept text it extracts which `PlannedWrites` fields the rule can
//! set and the call sites of `PreImageView` reads. Call sites are summed over
//! all kept branches, so a read count is an upper bound when the kept text
//! has no loop; a loop is reported.
use std::collections::{BTreeMap, BTreeSet};

pub(super) struct Module {
    pub name: &'static str,
    pub texts: Vec<&'static str>,
}

#[derive(Clone)]
pub(super) struct FnDef {
    pub module: &'static str,
    pub name: String,
    pub body: String,
}

/// Comments removed, string and char literal contents blanked.
pub(super) fn strip(src: &str) -> String {
    let b: Vec<char> = src.chars().collect();
    let mut out = String::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        let c = b[i];
        if c == '/' && i + 1 < b.len() && b[i + 1] == '/' {
            while i < b.len() && b[i] != '\n' {
                i += 1;
            }
            continue;
        }
        if c == '/' && i + 1 < b.len() && b[i + 1] == '*' {
            i += 2;
            while i + 1 < b.len() && !(b[i] == '*' && b[i + 1] == '/') {
                i += 1;
            }
            i += 2;
            continue;
        }
        if c == '"' {
            out.push('"');
            i += 1;
            while i < b.len() && b[i] != '"' {
                if b[i] == '\\' {
                    i += 1;
                }
                i += 1;
            }
            out.push('"');
            i += 1;
            continue;
        }
        if c == '\'' {
            // char literal 'x' or '\n'; otherwise a lifetime
            if i + 2 < b.len() && b[i + 1] != '\\' && b[i + 2] == '\'' {
                out.push_str("' '");
                i += 3;
                continue;
            }
            if i + 1 < b.len() && b[i + 1] == '\\' {
                let mut j = i + 2;
                while j < b.len() && b[j] != '\'' {
                    j += 1;
                }
                out.push_str("' '");
                i = j + 1;
                continue;
            }
        }
        out.push(c);
        i += 1;
    }
    out
}

fn is_ident(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn word_at(t: &[char], i: usize, w: &str) -> bool {
    let wc: Vec<char> = w.chars().collect();
    if i + wc.len() > t.len() || t[i..i + wc.len()] != wc[..] {
        return false;
    }
    let before = i == 0 || !is_ident(t[i - 1]);
    let after = i + wc.len() == t.len() || !is_ident(t[i + wc.len()]);
    before && after
}

/// Index of the char after the bracket that closes the one at `open`.
fn close_of(t: &[char], open: usize) -> usize {
    let mut depth = 0i64;
    let mut i = open;
    while i < t.len() {
        match t[i] {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => {
                depth -= 1;
                if depth == 0 {
                    return i + 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    t.len()
}

pub(super) fn functions(module: &'static str, text: &str) -> Vec<FnDef> {
    let t: Vec<char> = text.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < t.len() {
        if word_at(&t, i, "fn") {
            let mut j = i + 2;
            while j < t.len() && t[j].is_whitespace() {
                j += 1;
            }
            let s = j;
            while j < t.len() && is_ident(t[j]) {
                j += 1;
            }
            let name: String = t[s..j].iter().collect();
            let mut paren = 0i64;
            let mut k = j;
            let mut body = None;
            while k < t.len() {
                match t[k] {
                    '(' | '[' => paren += 1,
                    ')' | ']' => paren -= 1,
                    '{' if paren == 0 => {
                        body = Some(k);
                        break;
                    }
                    ';' if paren == 0 => break,
                    _ => {}
                }
                k += 1;
            }
            if let (Some(b), false) = (body, name.is_empty()) {
                let e = close_of(&t, b);
                out.push(FnDef {
                    module,
                    name,
                    body: t[b..e].iter().collect(),
                });
                i = b + 1;
                continue;
            }
        }
        i += 1;
    }
    out
}

fn idents(t: &str) -> Vec<String> {
    let mut v = Vec::new();
    let mut cur = String::new();
    for c in t.chars() {
        if is_ident(c) {
            cur.push(c);
        } else if !cur.is_empty() {
            v.push(std::mem::take(&mut cur));
        }
    }
    if !cur.is_empty() {
        v.push(cur);
    }
    v
}

struct Arm {
    pattern: String,
    body: String,
}

/// Arms of a match block `{ ... }` (text without the outer braces).
fn arms(block: &[char]) -> Vec<Arm> {
    let mut out = Vec::new();
    let mut i = 0;
    let n = block.len();
    loop {
        while i < n && (block[i].is_whitespace() || block[i] == ',') {
            i += 1;
        }
        if i >= n {
            break;
        }
        let ps = i;
        let mut depth = 0i64;
        while i + 1 < n && !(depth == 0 && block[i] == '=' && block[i + 1] == '>') {
            match block[i] {
                '(' | '[' | '{' => depth += 1,
                ')' | ']' | '}' => depth -= 1,
                _ => {}
            }
            i += 1;
        }
        if i + 1 >= n {
            break;
        }
        let pattern: String = block[ps..i].iter().collect();
        i += 2;
        while i < n && block[i].is_whitespace() {
            i += 1;
        }
        let bs = i;
        if i < n && block[i] == '{' {
            i = close_of(block, i);
        } else {
            let mut depth = 0i64;
            while i < n {
                match block[i] {
                    '(' | '[' | '{' => depth += 1,
                    ')' | ']' | '}' => depth -= 1,
                    ',' if depth == 0 => break,
                    _ => {}
                }
                i += 1;
            }
        }
        out.push(Arm {
            pattern,
            body: block[bs..i.min(n)].iter().collect(),
        });
    }
    out
}

/// `text` with the match arms that cannot run for `rule` removed.
pub(super) fn prune(text: &str, rule: &str, rules: &BTreeSet<String>) -> String {
    let t: Vec<char> = text.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < t.len() {
        if word_at(&t, i, "match") && i + 5 < t.len() && t[i + 5].is_whitespace() {
            // scrutinee up to the first `{` at bracket depth 0
            let mut j = i + 5;
            let mut depth = 0i64;
            while j < t.len() {
                match t[j] {
                    '(' | '[' => depth += 1,
                    ')' | ']' => depth -= 1,
                    '{' if depth == 0 => break,
                    _ => {}
                }
                j += 1;
            }
            if j >= t.len() {
                out.extend(&t[i..]);
                break;
            }
            let e = close_of(&t, j);
            let all = arms(&t[j + 1..e - 1]);
            let pure_hit = all.iter().any(|a| {
                let ids = idents(&a.pattern);
                ids.iter().any(|x| x == rule)
                    && ids.iter().all(|x| x == "RuleIdC" || rules.contains(x))
            });
            out.extend(&t[i..=j]);
            for a in &all {
                let ids = idents(&a.pattern);
                let named: Vec<&String> = ids.iter().filter(|x| rules.contains(*x)).collect();
                let wildcard = a.pattern.trim() == "_";
                let keep = if !named.is_empty() {
                    named.iter().any(|x| *x == rule)
                } else {
                    !(wildcard && pure_hit)
                };
                if keep {
                    out.push_str(&a.pattern);
                    out.push_str(" => ");
                    out.push_str(&prune(&a.body, rule, rules));
                    out.push_str(",\n");
                }
            }
            out.push('}');
            i = e;
            continue;
        }
        out.push(t[i]);
        i += 1;
    }
    out
}

/// Free calls `name(` and `path::name(` with their path module (if any).
fn free_calls(text: &str) -> Vec<(Option<String>, String)> {
    let t: Vec<char> = text.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < t.len() {
        if is_ident(t[i]) && (i == 0 || !is_ident(t[i - 1])) {
            let s = i;
            while i < t.len() && is_ident(t[i]) {
                i += 1;
            }
            if i < t.len() && t[i] == '(' && !(s > 0 && t[s - 1] == '.') {
                let name: String = t[s..i].iter().collect();
                let module = if s >= 2 && t[s - 1] == ':' && t[s - 2] == ':' {
                    let mut k = s - 2;
                    while k > 0 && is_ident(t[k - 1]) {
                        k -= 1;
                    }
                    Some(t[k..s - 2].iter().collect::<String>())
                } else {
                    None
                };
                out.push((module, name));
            }
            continue;
        }
        i += 1;
    }
    out
}

/// Method calls `recv.name(` for a fixed receiver.
fn method_calls(text: &str, recv: &str) -> Vec<String> {
    let pat = format!("{recv}.");
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(p) = rest.find(&pat) {
        let before_ok = p == 0 || !rest[..p].ends_with(|c: char| is_ident(c));
        let after = &rest[p + pat.len()..];
        let name: String = after.chars().take_while(|c| is_ident(*c)).collect();
        if before_ok && after[name.len()..].starts_with('(') && !name.is_empty() {
            out.push(name);
        }
        rest = &rest[p + pat.len()..];
    }
    out
}

pub(super) fn count_word(text: &str, w: &str) -> usize {
    let t: Vec<char> = text.chars().collect();
    (0..t.len()).filter(|i| word_at(&t, *i, w)).count()
}

/// Number of `.name(` call sites.
pub(super) fn count_method(text: &str, name: &str) -> usize {
    let pat = format!(".{name}(");
    text.matches(&pat).count()
}

pub(super) struct Index {
    pub fns: Vec<FnDef>,
    pub rules: BTreeSet<String>,
    pub guard_arms: BTreeMap<String, String>,
}

impl Index {
    pub fn new(modules: &[Module], rules: BTreeSet<String>, eval_guard_body: &str) -> Self {
        let mut fns = Vec::new();
        for m in modules {
            for t in &m.texts {
                fns.extend(functions(m.name, &strip(t)));
            }
        }
        let g = strip(eval_guard_body);
        let gt: Vec<char> = g.chars().collect();
        let mut guard_arms = BTreeMap::new();
        if let Some(p) = g.find("match g {") {
            let open = g[..p + 8].chars().count();
            let e = close_of(&gt, open);
            for a in arms(&gt[open + 1..e - 1]) {
                for id in idents(&a.pattern) {
                    if id != "GuardId" {
                        guard_arms.insert(id, a.body.clone());
                    }
                }
            }
        }
        Self {
            fns,
            rules,
            guard_arms,
        }
    }

    fn find(&self, module: Option<&str>, here: &'static str, name: &str) -> Vec<usize> {
        let by = |m: &str| -> Vec<usize> {
            (0..self.fns.len())
                .filter(|i| self.fns[*i].name == name && self.fns[*i].module == m)
                .collect()
        };
        if let Some(m) = module {
            let v = by(m);
            if !v.is_empty() {
                return v;
            }
        }
        let v = by(here);
        if !v.is_empty() {
            return v;
        }
        // Defined only in other modules: follow every definition (an
        // over-approximation when the name is ambiguous).
        (0..self.fns.len())
            .filter(|i| self.fns[*i].name == name)
            .collect()
    }

    /// Rule-sliced reachable text from `entry` (module, fn name), the names
    /// of the functions reached, and the free-call names with no definition
    /// in the scanned files.
    pub fn region(
        &self,
        rule: &str,
        entry: (&'static str, &str),
    ) -> (String, Vec<String>, BTreeSet<String>) {
        let mut seen = BTreeSet::new();
        let mut unresolved = BTreeSet::new();
        let mut work: Vec<usize> = self.find(Some(entry.0), entry.0, entry.1);
        assert!(!work.is_empty(), "entry {}::{} not found", entry.0, entry.1);
        let mut text = String::new();
        let mut reached = Vec::new();
        let mut guards_done = BTreeSet::new();
        while let Some(f) = work.pop() {
            if !seen.insert(f) {
                continue;
            }
            let d = &self.fns[f];
            reached.push(format!("{}::{}", d.module, d.name));
            let body = prune(&d.body, rule, &self.rules);
            for (m, n) in free_calls(&body) {
                let found = self.find(m.as_deref(), d.module, &n);
                if found.is_empty() {
                    unresolved.insert(n);
                }
                work.extend(found);
            }
            for recv in ["reads", "adt_reads()"] {
                for n in method_calls(&body, recv) {
                    for g in self.find(Some("owner_adt_runtime"), "owner_adt_runtime", &n) {
                        work.push(g);
                    }
                }
            }
            for (gname, garm) in &self.guard_arms {
                if body.contains(&format!("GuardId::{gname}")) && guards_done.insert(gname.clone())
                {
                    reached.push(format!("guard::{gname}"));
                    text.push_str(garm);
                    text.push('\n');
                }
            }
            text.push_str(&body);
            text.push('\n');
        }
        (text, reached, unresolved)
    }
}

/// `field = <value>` or `field: <value>` with a value other than `None`.
pub(super) fn sets_field(text: &str, field: &str) -> bool {
    let t: Vec<char> = text.chars().collect();
    let n = t.len();
    for i in 0..n {
        if !word_at(&t, i, field) {
            continue;
        }
        let mut j = i + field.chars().count();
        while j < n && t[j].is_whitespace() {
            j += 1;
        }
        let assign = j < n && t[j] == '=' && !(j + 1 < n && t[j + 1] == '=');
        let colon = j < n && t[j] == ':' && !(j + 1 < n && t[j + 1] == ':');
        if !(assign || colon) {
            continue;
        }
        j += 1;
        while j < n && t[j].is_whitespace() {
            j += 1;
        }
        let rest: String = t[j..(j + 4).min(n)].iter().collect();
        if rest != "None" {
            return true;
        }
    }
    false
}
