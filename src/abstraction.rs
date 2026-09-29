use crate::semantic::normalize;

const MAX_GROUPS: usize = 256;
const MAX_ALIASES: usize = 8;

#[derive(Clone, Debug, PartialEq)]
pub struct ConceptGroup {
    pub canonical: String,
    pub aliases: Vec<String>,
    pub evidence: u16,
    pub confidence: f32,
}

#[derive(Clone, Debug, Default)]
pub struct ConceptAbstraction {
    groups: Vec<ConceptGroup>,
}

impl ConceptAbstraction {
    pub fn learn_from_text(&mut self, text: &str) -> usize {
        let normalized = normalize(text);
        let mut learned = 0usize;
        for sentence in normalized.split(['.', ';', '!', '?', '\n']) {
            let s = sentence.trim();
            for marker in [
                " con goi la ",
                " cung goi la ",
                " nghia la ",
                " tuong duong voi ",
                " dong nghia voi ",
            ] {
                if let Some((a, b)) = s.split_once(marker) {
                    if self.link(a, b, 0.95) {
                        learned += 1;
                    }
                    break;
                }
            }
        }
        learned
    }

    pub fn link(&mut self, a: &str, b: &str, confidence: f32) -> bool {
        let a = clean(a);
        let b = clean(b);
        if a.is_empty() || b.is_empty() || a == b {
            return false;
        }

        let ai = self.group_index(&a);
        let bi = self.group_index(&b);
        match (ai, bi) {
            (Some(i), Some(j)) if i == j => {
                let g = &mut self.groups[i];
                g.evidence = g.evidence.saturating_add(1);
                g.confidence = (g.confidence * 0.8 + confidence * 0.2).clamp(0.0, 1.0);
            }
            (Some(i), Some(j)) => {
                let (keep, remove) = if i < j { (i, j) } else { (j, i) };
                let removed = self.groups.remove(remove);
                let g = &mut self.groups[keep];
                for alias in removed
                    .aliases
                    .into_iter()
                    .chain(std::iter::once(removed.canonical))
                {
                    if g.aliases.len() < MAX_ALIASES
                        && alias != g.canonical
                        && !g.aliases.contains(&alias)
                    {
                        g.aliases.push(alias);
                    }
                }
                g.evidence = g.evidence.saturating_add(removed.evidence).saturating_add(1);
                g.confidence = g.confidence.max(removed.confidence).max(confidence);
            }
            (Some(i), None) => add_alias(&mut self.groups[i], b, confidence),
            (None, Some(i)) => add_alias(&mut self.groups[i], a, confidence),
            (None, None) => {
                if self.groups.len() >= MAX_GROUPS {
                    let idx = self
                        .groups
                        .iter()
                        .enumerate()
                        .min_by_key(|(_, g)| g.evidence)
                        .map(|(i, _)| i)
                        .unwrap_or(0);
                    self.groups.remove(idx);
                }
                self.groups.push(ConceptGroup {
                    canonical: a,
                    aliases: vec![b],
                    evidence: 1,
                    confidence: confidence.clamp(0.0, 1.0),
                });
            }
        }
        true
    }

    pub fn canonical_phrase(&self, phrase: &str) -> String {
        let p = clean(&normalize(phrase));
        if let Some(i) = self.group_index(&p) {
            return self.groups[i].canonical.clone();
        }
        p
    }

    pub fn canonicalize_text(&self, text: &str) -> String {
        let mut out = normalize(text);
        let mut replacements: Vec<(String, String)> = self
            .groups
            .iter()
            .flat_map(|g| {
                g.aliases
                    .iter()
                    .map(move |a| (a.clone(), g.canonical.clone()))
            })
            .collect();
        replacements.sort_by_key(|(a, _)| std::cmp::Reverse(a.len()));
        for (alias, canonical) in replacements {
            out = replace_phrase(&out, &alias, &canonical);
        }
        out
    }

    pub fn groups(&self) -> &[ConceptGroup] {
        &self.groups
    }

    pub fn len(&self) -> usize {
        self.groups.len()
    }

    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    fn group_index(&self, phrase: &str) -> Option<usize> {
        self.groups.iter().position(|g| {
            g.canonical == phrase || g.aliases.iter().any(|a| a == phrase)
        })
    }
}

fn add_alias(group: &mut ConceptGroup, alias: String, confidence: f32) {
    if alias != group.canonical
        && !group.aliases.contains(&alias)
        && group.aliases.len() < MAX_ALIASES
    {
        group.aliases.push(alias);
    }
    group.evidence = group.evidence.saturating_add(1);
    group.confidence = (group.confidence * 0.8 + confidence * 0.2).clamp(0.0, 1.0);
}

fn clean(s: &str) -> String {
    s.trim()
        .trim_matches(|c: char| matches!(c, ',' | ':' | '.' | '?' | '!'))
        .split_whitespace()
        .take(12)
        .collect::<Vec<_>>()
        .join(" ")
}

fn replace_phrase(text: &str, alias: &str, canonical: &str) -> String {
    if alias.is_empty() || alias == canonical {
        return text.to_string();
    }
    let padded = format!(" {text} ");
    let from = format!(" {alias} ");
    let to = format!(" {canonical} ");
    padded.replace(&from, &to).trim().to_string()
}
