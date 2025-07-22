use std::convert::identity;

use emeraldc_lexer::{Span, WideTokenKind};
use emeraldc_parser::{
    Binary, BinaryOperator, Declaration, Expression, FatalParserError,
    Function, Identifier, Let, NodeError, Parenthesized, ParsedNode, Statement,
};
use maud::Render;

pub struct Visualizer<'s> {
    source: &'s str,
}

impl<'s> Visualizer<'s> {
    pub fn visualize(
        tree: impl Iterator<
            Item = Result<ParsedNode<Declaration>, FatalParserError>,
        >,
        source: &'s str,
    ) -> String {
        let mut visualizer = Self::new(source);
        let tree = visualizer.visualize_tree(tree);
        let tree_window = visualizer.visualize_tree_window(tree);
        visualizer.visualize_page(tree_window)
    }

    fn new(source: &'s str) -> Self {
        Self { source }
    }

    fn visualize_tree(
        &mut self,
        tree: impl Iterator<
            Item = Result<ParsedNode<Declaration>, FatalParserError>,
        >,
    ) -> maud::PreEscaped<String> {
        let mut buffer = String::new();
        for declaration in tree {
            declaration.visualize(self).render_to(&mut buffer);
        }
        maud::PreEscaped(buffer)
    }

    fn visualize_tree_window(
        &mut self,
        tree: maud::PreEscaped<String>,
    ) -> maud::PreEscaped<String> {
        maud::html! {
            div id="tree-window" {
                (tree)
            }
        }
        .render()
    }

    fn visualize_page(&mut self, tree_window: impl maud::Render) -> String {
        maud::html! {
            (maud::DOCTYPE)
            html {
                head {
                    meta charset="utf-8";
                    meta name="author" content="Georgiy Ozhegov";
                    title { "Parse Tree Visualization" }
                }
                body {
                    main {
                        (tree_window)
                    }
                }
            }
        }
        .into_string()
    }

    pub fn text_of(&self, span: &Span) -> &'s str {
        &self.source[span.start..span.end]
    }
}

trait Visualize {
    fn visualize(self, v: &Visualizer) -> Box<dyn maud::Render>;
}

impl Visualize for Result<ParsedNode<Declaration>, FatalParserError> {
    fn visualize(self, v: &Visualizer) -> Box<dyn maud::Render> {
        let inner = match self {
            Ok(node) => node.visualize(v),
            Err(error) => error.visualize(v),
        };
        let element = maud::html! {
            div class="node" {
                (inner)
            }
        };
        Box::new(element)
    }
}

impl<V> Visualize for ParsedNode<V>
where
    V: Visualize,
{
    fn visualize(self, v: &Visualizer) -> Box<dyn maud::Render> {
        let inner = match self.node {
            Ok(node) => node.visualize(v),
            Err(error) => error.visualize(v),
        };
        let element = maud::html! {
            div class="parsed-node" data-span-start=(self.span.start) data-span-end=(self.span.end) data-text=(v.text_of(&self.span)) {
                (inner)
            }
        };
        Box::new(element)
    }
}

impl Visualize for Declaration {
    fn visualize(self, v: &Visualizer) -> Box<dyn maud::Render> {
        let inner = match self {
            Self::Function(function) => function.visualize(v),
        };
        let element = maud::html! {
            div class="declaration" {
                (inner)
            }
        };
        Box::new(element)
    }
}

impl Visualize for Function {
    fn visualize(self, v: &Visualizer) -> Box<dyn maud::Render> {
        let element = maud::html! {
            div class="function"{
                (self._introducer.visualize(v))
                (self.identifier.visualize(v))
                (self._open_round.visualize(v))
                (self._close_round.visualize(v))
                (self.body.visualize(v))
                (self._end.visualize(v))
            }
        };
        Box::new(element)
    }
}

impl Visualize for Vec<ParsedNode<Statement>> {
    fn visualize(self, v: &Visualizer) -> Box<dyn maud::Render> {
        let mut inner = String::new();
        for statement in self {
            statement.visualize(v).render_to(&mut inner);
        }
        let element = maud::html! {
            div class="function-body" {
                (maud::PreEscaped(inner))
            }
        };
        Box::new(element)
    }
}

impl Visualize for Statement {
    fn visualize(self, v: &Visualizer) -> Box<dyn maud::Render> {
        let inner = match self {
            Self::Let(let_) => let_.visualize(v),
        };
        let element = maud::html! {
            div class="statement" {
                (inner)
            }
        };
        Box::new(element)
    }
}

impl Visualize for Let {
    fn visualize(self, v: &Visualizer) -> Box<dyn maud::Render> {
        let element = maud::html! {
            div class="let" {
                (self._introducer.visualize(v))
                (self.identifier.visualize(v))
                (self._equal.visualize(v))
                (self.value.visualize(v))
            }
        };
        Box::new(element)
    }
}

impl Visualize for Expression {
    fn visualize(self, v: &Visualizer) -> Box<dyn maud::Render> {
        let inner = match self {
            Self::Integer => {
                let element = maud::html! {
                    div class="integer" {}
                };
                Box::new(element)
            }
            Self::Variable(identifier) => identifier.visualize(v),
            Self::Binary(binary) => binary.visualize(v),
            Self::Parenthesized(parenthesized) => parenthesized.visualize(v),
        };
        let element = maud::html! {
            div class="expression" {
                (inner)
            }
        };
        Box::new(element)
    }
}

impl Visualize for Binary {
    fn visualize(self, v: &Visualizer) -> Box<dyn maud::Render> {
        let element = maud::html! {
            div class="binary" {
                (self.left.visualize(v))
                (self.operator.visualize(v))
                (self.right.visualize(v))
            }
        };
        Box::new(element)
    }
}

impl Visualize for BinaryOperator {
    fn visualize(self, _v: &Visualizer) -> Box<dyn maud::Render> {
        let element = maud::html! {
            div class="binary-operator" data-operator=(format!("{self:?}")) {}
        };
        Box::new(element)
    }
}

impl Visualize for Parenthesized {
    fn visualize(self, v: &Visualizer) -> Box<dyn maud::Render> {
        let element = maud::html! {
            div class="parenthesized" {
                (self._open_round.visualize(v))
                (self.inner.visualize(v))
                (self._close_round.visualize(v))
            }
        };
        Box::new(element)
    }
}

impl Visualize for Identifier {
    fn visualize(self, _v: &Visualizer) -> Box<dyn maud::Render> {
        let element = maud::html! {
            div class="identifier" {}
        };
        Box::new(element)
    }
}

impl Visualize for WideTokenKind {
    fn visualize(self, _v: &Visualizer) -> Box<dyn maud::Render> {
        let element = maud::html! {
            div class="token" data-kind=(format!("{self:?}")) {}
        };
        Box::new(element)
    }
}

impl Visualize for NodeError {
    fn visualize(self, _v: &Visualizer) -> Box<dyn maud::Render> {
        let element = maud::html! {
            div class="node-error" data-message=(self) {}
        };
        Box::new(element)
    }
}

impl Visualize for FatalParserError {
    fn visualize(self, _v: &Visualizer) -> Box<dyn maud::Render> {
        let element = maud::html! {
            div class="fatal-error" data-message=(self) {}
        };
        Box::new(element)
    }
}
