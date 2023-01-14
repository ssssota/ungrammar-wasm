mod utils;

use serde::Serialize;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() {
    utils::set_panic_hook();
}

#[derive(Debug, Clone, Serialize)]
pub struct Grammar(Vec<Node>);

#[derive(Debug, Clone, Serialize)]
pub struct Node {
    pub name: String,
    pub rule: Rule,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum Rule {
    Labeled { label: String, rule: Box<Rule> },
    Node { node: String },
    Token { token: String },
    Seq { rules: Vec<Rule> },
    Alt { rules: Vec<Rule> },
    Opt { rule: Box<Rule> },
    Rep { rule: Box<Rule> },
}

impl From<ungrammar::Grammar> for Grammar {
    fn from(grammar: ungrammar::Grammar) -> Self {
        let mut nodes = vec![];
        for node in grammar.iter() {
            let node = &grammar[node];
            nodes.push(Node {
                name: node.name.clone(),
                rule: rule_to_rule(&grammar, &node.rule),
            })
        }
        Grammar(nodes)
    }
}

fn rule_to_rule(grammar: &ungrammar::Grammar, rule: &ungrammar::Rule) -> Rule {
    match rule {
        ungrammar::Rule::Labeled { label, rule } => Rule::Labeled {
            label: label.clone(),
            rule: Box::new(rule_to_rule(grammar, rule)),
        },
        ungrammar::Rule::Node(node) => Rule::Node {
            node: grammar[*node].name.clone(),
        },
        ungrammar::Rule::Token(token) => Rule::Token {
            token: grammar[*token].name.clone(),
        },
        ungrammar::Rule::Seq(rules) => {
            let mut rs = vec![];
            for r in rules {
                rs.push(rule_to_rule(grammar, r));
            }
            Rule::Seq { rules: rs }
        }
        ungrammar::Rule::Alt(rules) => {
            let mut rs = vec![];
            for r in rules {
                rs.push(rule_to_rule(grammar, r));
            }
            Rule::Alt { rules: rs }
        }
        ungrammar::Rule::Opt(rule) => Rule::Opt {
            rule: Box::new(rule_to_rule(grammar, rule)),
        },
        ungrammar::Rule::Rep(rule) => Rule::Rep {
            rule: Box::new(rule_to_rule(grammar, rule)),
        },
    }
}

#[wasm_bindgen]
pub fn parse(ungram: String) -> Result<JsValue, JsValue> {
    let grammar = ungrammar::Grammar::from_str(&ungram).map_err(|err| err.to_string())?;
    Ok(serde_wasm_bindgen::to_value(&Grammar::from(grammar))?)
}

// fn grammar_to_json(grammar: &ungrammar::Grammar, mut obj: write_json::Object<'_>) {
//     for node in grammar.iter() {
//         let node = &grammar[node];
//         rule_to_json(grammar, &node.rule, obj.object(&node.name));
//     }
// }

// fn rule_to_json(grammar: &ungrammar::Grammar, rule: &ungrammar::Rule, mut obj: write_json::Object) {
//     match rule {
//         ungrammar::Rule::Labeled { label, rule } => {
//             obj.string("label", label);
//             rule_to_json(grammar, rule, obj.object("rule"))
//         }
//         ungrammar::Rule::Node(node) => {
//             obj.string("node", &grammar[*node].name);
//         }
//         ungrammar::Rule::Token(token) => {
//             obj.string("token", &grammar[*token].name);
//         }
//         ungrammar::Rule::Seq(rules) | ungrammar::Rule::Alt(rules) => {
//             let tag = match rule {
//                 ungrammar::Rule::Seq(_) => "seq",
//                 ungrammar::Rule::Alt(_) => "alt",
//                 _ => unreachable!(),
//             };
//             let mut array = obj.array(tag);
//             for rule in rules {
//                 rule_to_json(grammar, rule, array.object());
//             }
//         }
//         ungrammar::Rule::Opt(arg) | ungrammar::Rule::Rep(arg) => {
//             let tag = match rule {
//                 ungrammar::Rule::Opt(_) => "opt",
//                 ungrammar::Rule::Rep(_) => "rep",
//                 _ => unreachable!(),
//             };
//             rule_to_json(grammar, arg, obj.object(tag));
//         }
//     }
// }
